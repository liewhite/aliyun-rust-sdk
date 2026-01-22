//! 类型定义 - 自动生成，请勿手动修改

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAvailableResourceResponseAvailableResourcesAvailableResourceItemSupportResourcesSupportResourceItem {
    /// 网络类型。
    #[serde(rename = "AddressType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_type: Option<String>,
    /// IP地址类型。
    #[serde(rename = "AddressIPVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_ip_version: Option<String>,
}

impl DescribeAvailableResourceResponseAvailableResourcesAvailableResourceItemSupportResourcesSupportResourceItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.address_type {
            params.push(("AddressType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.address_ip_version {
            params.push(("AddressIPVersion".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAvailableResourceResponseAvailableResourcesAvailableResourceItemSupportResources {
    /// 支持的资源。
    #[serde(rename = "SupportResource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_resource: Option<Vec<DescribeAvailableResourceResponseAvailableResourcesAvailableResourceItemSupportResourcesSupportResourceItem>>,
}

impl DescribeAvailableResourceResponseAvailableResourcesAvailableResourceItemSupportResources {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.support_resource {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("SupportResource.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAvailableResourceResponseAvailableResourcesAvailableResourceItem {
    /// 备可用区。
    #[serde(rename = "SlaveZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slave_zone_id: Option<String>,
    /// 主可用区。
    #[serde(rename = "MasterZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_zone_id: Option<String>,
    #[serde(rename = "SupportResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_resources: Option<DescribeAvailableResourceResponseAvailableResourcesAvailableResourceItemSupportResources>,
}

impl DescribeAvailableResourceResponseAvailableResourcesAvailableResourceItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.slave_zone_id {
            params.push(("SlaveZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.master_zone_id {
            params.push(("MasterZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.support_resources {
            for (k, v2) in v.to_query_params() {
                params.push((format!("SupportResources.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAvailableResourceResponseAvailableResources {
    /// 可用区及支持的资源列表。
    #[serde(rename = "AvailableResource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_resource: Option<Vec<DescribeAvailableResourceResponseAvailableResourcesAvailableResourceItem>>,
}

impl DescribeAvailableResourceResponseAvailableResources {
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
pub struct DescribeRegionsResponseRegionsRegionItem {
    /// 地域服务的终端节点地址。
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
}

impl DescribeRegionsResponseRegionsRegionItem {
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
pub struct DescribeZonesResponseZonesZoneItemSlaveZonesSlaveZoneItem {
    /// 备可用区ID。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 备可用区名称。
    #[serde(rename = "LocalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_name: Option<String>,
}

impl DescribeZonesResponseZonesZoneItemSlaveZonesSlaveZoneItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.local_name {
            params.push(("LocalName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeZonesResponseZonesZoneItemSlaveZones {
    /// 主可用区对应的备可用区列表。
    #[serde(rename = "SlaveZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slave_zone: Option<Vec<DescribeZonesResponseZonesZoneItemSlaveZonesSlaveZoneItem>>,
}

impl DescribeZonesResponseZonesZoneItemSlaveZones {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.slave_zone {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("SlaveZone.{}", i + 1);
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
    /// 可用区ID。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 可用区名称。
    #[serde(rename = "LocalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_name: Option<String>,
    #[serde(rename = "SlaveZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slave_zones: Option<DescribeZonesResponseZonesZoneItemSlaveZones>,
}

impl DescribeZonesResponseZonesZoneItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.local_name {
            params.push(("LocalName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.slave_zones {
            for (k, v2) in v.to_query_params() {
                params.push((format!("SlaveZones.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeZonesResponseZones {
    /// 可用区列表。
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

/// 标签列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateLoadBalancerRequestTagItem {
    /// 实例的标签键。N的取值范围：**1~20**。一旦输入该值，则不允许为空字符串。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 实例的标签值。N的取值范围：**1~20**。一旦输入该值，可以为空字符串。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreateLoadBalancerRequestTagItem {
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
pub struct DescribeLoadBalancerAttributeResponseTagsTagItem {
    /// 实例的标签键。N的取值范围：**1**~**20**。一旦输入该值，则不允许为空字符串。
    #[serde(rename = "TagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    /// 实例的标签值。N的取值范围：**1**~**20**。一旦输入该值，可以为空字符串。
    #[serde(rename = "TagValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
}

impl DescribeLoadBalancerAttributeResponseTagsTagItem {
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
pub struct DescribeLoadBalancerAttributeResponseTags {
    /// 标签列表。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<DescribeLoadBalancerAttributeResponseTagsTagItem>>,
}

impl DescribeLoadBalancerAttributeResponseTags {
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
pub struct DescribeLoadBalancerAttributeResponseListenerPorts {
    /// 传统型负载均衡实例前端使用的端口。
    #[serde(rename = "ListenerPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_port: Option<Vec<i32>>,
}

impl DescribeLoadBalancerAttributeResponseListenerPorts {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.listener_port {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("ListenerPort.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 传统型负载均衡实例前端使用的协议。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeLoadBalancerAttributeResponseListenerPortsAndProtocalListenerPortAndProtocalItem {
    /// 传统型负载均衡实例前端使用的协议。
    #[serde(rename = "ListenerProtocal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_protocal: Option<String>,
    /// 实例前端使用的端口。
    #[serde(rename = "ListenerPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_port: Option<i32>,
}

impl DescribeLoadBalancerAttributeResponseListenerPortsAndProtocalListenerPortAndProtocalItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.listener_protocal {
            params.push(("ListenerProtocal".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.listener_port {
            params.push(("ListenerPort".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeLoadBalancerAttributeResponseListenerPortsAndProtocal {
    /// 监听端口或协议。
    #[serde(rename = "ListenerPortAndProtocal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_port_and_protocal: Option<Vec<DescribeLoadBalancerAttributeResponseListenerPortsAndProtocalListenerPortAndProtocalItem>>,
}

impl DescribeLoadBalancerAttributeResponseListenerPortsAndProtocal {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.listener_port_and_protocal {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("ListenerPortAndProtocal.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 实例前端使用的端口和协议。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeLoadBalancerAttributeResponseListenerPortsAndProtocolListenerPortAndProtocolItem {
    /// 实例前端使用的端口。
    #[serde(rename = "ListenerPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_port: Option<i32>,
    /// 实例前端使用的协议。
    #[serde(rename = "ListenerProtocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_protocol: Option<String>,
    /// 是否启用监听转发。
    #[serde(rename = "ListenerForward")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_forward: Option<String>,
    /// 监听端口和协议描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 转发到的目的监听端口，必须是已经存在的HTTPS监听端口。
    #[serde(rename = "ForwardPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_port: Option<i32>,
}

impl DescribeLoadBalancerAttributeResponseListenerPortsAndProtocolListenerPortAndProtocolItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.listener_port {
            params.push(("ListenerPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.listener_protocol {
            params.push(("ListenerProtocol".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.listener_forward {
            params.push(("ListenerForward".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.forward_port {
            params.push(("ForwardPort".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeLoadBalancerAttributeResponseListenerPortsAndProtocol {
    /// 实例前端使用的端口和协议。
    #[serde(rename = "ListenerPortAndProtocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_port_and_protocol: Option<Vec<DescribeLoadBalancerAttributeResponseListenerPortsAndProtocolListenerPortAndProtocolItem>>,
}

impl DescribeLoadBalancerAttributeResponseListenerPortsAndProtocol {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.listener_port_and_protocol {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("ListenerPortAndProtocol.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 实例的后端服务器列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeLoadBalancerAttributeResponseBackendServersBackendServerItem {
    /// 后端服务器类型。
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 后端服务器的权重。
    #[serde(rename = "Weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    /// 后端服务器描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// ENI或ECI的实例ID。
    #[serde(rename = "ServerIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_ip: Option<String>,
    /// 后端服务器实例ID。
    #[serde(rename = "ServerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
}

impl DescribeLoadBalancerAttributeResponseBackendServersBackendServerItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.weight {
            params.push(("Weight".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.server_ip {
            params.push(("ServerIp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.server_id {
            params.push(("ServerId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeLoadBalancerAttributeResponseBackendServers {
    /// 实例的后端服务器列表。
    #[serde(rename = "BackendServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_server: Option<Vec<DescribeLoadBalancerAttributeResponseBackendServersBackendServerItem>>,
}

impl DescribeLoadBalancerAttributeResponseBackendServers {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.backend_server {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("BackendServer.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 标签列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeLoadBalancersRequestTagItem {
    /// 资源的标签键。N的取值范围：**1~20**。一旦输入该值，则不允许为空字符串。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 资源的标签值。N的取值范围：**1~20**。一旦输入该值，可以为空字符串。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DescribeLoadBalancersRequestTagItem {
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

/// 标签列表
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeLoadBalancersResponseLoadBalancersLoadBalancerItemTagsTagItem {
    /// 标签值。
    #[serde(rename = "TagValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
    /// 标签键。
    #[serde(rename = "TagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
}

impl DescribeLoadBalancersResponseLoadBalancersLoadBalancerItemTagsTagItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.tag_value {
            params.push(("TagValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_key {
            params.push(("TagKey".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeLoadBalancersResponseLoadBalancersLoadBalancerItemTags {
    /// 标签列表
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<DescribeLoadBalancersResponseLoadBalancersLoadBalancerItemTagsTagItem>>,
}

impl DescribeLoadBalancersResponseLoadBalancersLoadBalancerItemTags {
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

/// 数组格式，返回负载均衡实例列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeLoadBalancersResponseLoadBalancersLoadBalancerItem {
    /// 私网负载均衡实例的专有网络ID。
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 实例创建时间戳。
    #[serde(rename = "CreateTimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time_stamp: Option<i64>,
    /// 负载均衡实例ID。
    #[serde(rename = "LoadBalancerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_id: Option<String>,
    /// 实例创建时间，格式为：`YYYY-MM-DDThh:mm:ssZ`。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 负载均衡实例付费模式，取值：
    #[serde(rename = "PayType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_type: Option<String>,
    /// 实例的网络类型。取值：
    #[serde(rename = "AddressType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_type: Option<String>,
    /// 私网负载均衡实例的网络类型，取值：
    #[serde(rename = "NetworkType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    /// IP版本，可以设置为**ipv4**或者**ipv6**。
    #[serde(rename = "AddressIPVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_ip_version: Option<String>,
    /// 负载均衡实例的名称。
    #[serde(rename = "LoadBalancerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_name: Option<String>,
    /// 监听的带宽峰值，单位：Mbps。
    #[serde(rename = "Bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<i32>,
    /// 负载均衡实例服务地址。
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// 实例的备可用区ID。
    #[serde(rename = "SlaveZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slave_zone_id: Option<String>,
    /// 实例的主可用区ID。
    #[serde(rename = "MasterZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_zone_id: Option<String>,
    /// 公网计费方式。取值：
    #[serde(rename = "InternetChargeTypeAlias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internet_charge_type_alias: Option<String>,
    /// 负载均衡实例的性能规格。
    #[serde(rename = "LoadBalancerSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_spec: Option<String>,
    /// 负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 设置修改保护状态的原因，长度为1~80个字符，必须以字母或中文开头，支持数字、半角句号（.）、下划线（_）和短划线（-）。
    #[serde(rename = "ModificationProtectionReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modification_protection_reason: Option<String>,
    /// 负载均衡修改保护状态：
    #[serde(rename = "ModificationProtectionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modification_protection_status: Option<String>,
    /// 私网负载均衡实例的交换机ID。
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 负载均衡实例状态。取值：
    #[serde(rename = "LoadBalancerStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_status: Option<String>,
    /// 企业资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 公网类型实例付费方式。取值：
    #[serde(rename = "InternetChargeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internet_charge_type: Option<String>,
    /// 负载均衡删除保护状态。取值：
    #[serde(rename = "DeleteProtection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_protection: Option<String>,
    /// 传统型负载均衡实例的地域名称。
    #[serde(rename = "RegionIdAlias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id_alias: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<DescribeLoadBalancersResponseLoadBalancersLoadBalancerItemTags>,
    /// 实例计费方式，取值：
    #[serde(rename = "InstanceChargeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_charge_type: Option<String>,
}

impl DescribeLoadBalancersResponseLoadBalancersLoadBalancerItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.vpc_id {
            params.push(("VpcId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time_stamp {
            params.push(("CreateTimeStamp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.load_balancer_id {
            params.push(("LoadBalancerId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.pay_type {
            params.push(("PayType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.address_type {
            params.push(("AddressType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.network_type {
            params.push(("NetworkType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.address_ip_version {
            params.push(("AddressIPVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.load_balancer_name {
            params.push(("LoadBalancerName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bandwidth {
            params.push(("Bandwidth".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.address {
            params.push(("Address".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.slave_zone_id {
            params.push(("SlaveZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.master_zone_id {
            params.push(("MasterZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.internet_charge_type_alias {
            params.push(("InternetChargeTypeAlias".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.load_balancer_spec {
            params.push(("LoadBalancerSpec".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.modification_protection_reason {
            params.push(("ModificationProtectionReason".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.modification_protection_status {
            params.push(("ModificationProtectionStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("VSwitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.load_balancer_status {
            params.push(("LoadBalancerStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.internet_charge_type {
            params.push(("InternetChargeType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.delete_protection {
            params.push(("DeleteProtection".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id_alias {
            params.push(("RegionIdAlias".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Tags.{}", k), v2));
            }
        }
        if let Some(ref v) = self.instance_charge_type {
            params.push(("InstanceChargeType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeLoadBalancersResponseLoadBalancers {
    /// 数组格式，返回负载均衡实例列表。
    #[serde(rename = "LoadBalancer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer: Option<Vec<DescribeLoadBalancersResponseLoadBalancersLoadBalancerItem>>,
}

impl DescribeLoadBalancersResponseLoadBalancers {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.load_balancer {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("LoadBalancer.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 标签列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeLoadBalancerListenersRequestTagItem {
    /// 资源的标签键。N的取值范围：**1~20**。一旦输入该值，则不允许为空字符串。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 资源的标签值。N的取值范围：**1~20**。一旦输入该值，可以为空字符串。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DescribeLoadBalancerListenersRequestTagItem {
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

/// HTTP监听配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeLoadBalancerListenersResponseListenersItemHTTPListenerConfig {
    /// 健康检查HTTP协议版本。
    #[serde(rename = "HealthCheckHttpVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_http_version: Option<String>,
    /// 是否通过`XForwardedFor_ClientSrcPort`头字段获取访问负载均衡实例客户端的端口。取值：
    #[serde(rename = "XForwardedFor_ClientSrcPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_client_src_port: Option<String>,
    /// 服务器上配置的Cookie。
    #[serde(rename = "Cookie")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie: Option<String>,
    /// 是否开启Gzip压缩。取值：
    #[serde(rename = "Gzip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gzip: Option<String>,
    /// 健康检查的端口。
    #[serde(rename = "HealthCheckConnectPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_connect_port: Option<i32>,
    /// 每次健康检查响应的最大超时间，单位：秒。
    #[serde(rename = "HealthCheckTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_timeout: Option<i32>,
    /// 健康检查协议。
    #[serde(rename = "HealthCheckType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_type: Option<String>,
    /// Cookie超时时间。单位：秒。
    #[serde(rename = "CookieTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie_timeout: Option<i32>,
    /// 健康检查的域名。
    #[serde(rename = "HealthCheckDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_domain: Option<String>,
    /// 健康检查连续失败多少次后，将后端服务器的健康检查状态由**成功**判定为**失败**。
    #[serde(rename = "UnhealthyThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unhealthy_threshold: Option<i32>,
    /// 是否通过`SLB-ID`头字段获取负载均衡实例ID。取值：
    #[serde(rename = "XForwardedFor_SLBID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_slbid: Option<String>,
    /// HTTP转发至HTTPS的监听端口。
    #[serde(rename = "ForwardPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_port: Option<i32>,
    /// 健康检查正常的HTTP状态码。
    #[serde(rename = "HealthCheckHttpCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_http_code: Option<String>,
    /// 是否开启HTTP至HTTPS监听转发。取值：
    #[serde(rename = "ListenerForward")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_forward: Option<String>,
    /// 是否通过`XForwardedFor`的方式获取来访者真实IP。取值：
    #[serde(rename = "XForwardedFor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for: Option<String>,
    /// 指定连接空闲超时时间。单位：秒。取值范围：**1**~**60**。
    #[serde(rename = "IdleTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_timeout: Option<i32>,
    /// 指定请求超时时间。单位：秒。取值范围：**1**~**180**。
    #[serde(rename = "RequestTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_timeout: Option<i32>,
    /// 健康检查的时间间隔，单位：秒。
    #[serde(rename = "HealthCheckInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_interval: Option<i32>,
    /// 是否通过`XForwardedFor_SLBPORT`头字段获取负载均衡实例的监听端口。取值：
    #[serde(rename = "XForwardedFor_SLBPORT")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_slbport: Option<String>,
    /// 健康检查的URI。
    #[serde(rename = "HealthCheckURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_uri: Option<String>,
    /// Cookie的处理方式。取值：
    #[serde(rename = "StickySessionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticky_session_type: Option<String>,
    /// 健康检查连续成功多少次后，将后端服务器的健康检查状态由**失败**判定为**成功**。
    #[serde(rename = "HealthyThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthy_threshold: Option<i32>,
    /// 是否通过`X-Forwarded-Proto`头字段获取负载均衡实例的监听协议。取值：
    #[serde(rename = "XForwardedFor_proto")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_proto: Option<String>,
    /// 是否通过`SLB-IP`头字段获取客户端请求的VIP（Virtual IP address）。取值：
    #[serde(rename = "XForwardedFor_SLBIP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_slbip: Option<String>,
    /// 是否开启会话保持。取值：
    #[serde(rename = "StickySession")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticky_session: Option<String>,
    /// 健康检查方式。取值：**head**或**get**。
    #[serde(rename = "HealthCheckMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_method: Option<String>,
    /// 是否开启健康检查。取值：
    #[serde(rename = "HealthCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<String>,
}

impl DescribeLoadBalancerListenersResponseListenersItemHTTPListenerConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.health_check_http_version {
            params.push(("HealthCheckHttpVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.x_forwarded_for_client_src_port {
            params.push(("XForwardedFor_ClientSrcPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cookie {
            params.push(("Cookie".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.gzip {
            params.push(("Gzip".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_connect_port {
            params.push(("HealthCheckConnectPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_timeout {
            params.push(("HealthCheckTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_type {
            params.push(("HealthCheckType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cookie_timeout {
            params.push(("CookieTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_domain {
            params.push(("HealthCheckDomain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.unhealthy_threshold {
            params.push(("UnhealthyThreshold".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.x_forwarded_for_slbid {
            params.push(("XForwardedFor_SLBID".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.forward_port {
            params.push(("ForwardPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_http_code {
            params.push(("HealthCheckHttpCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.listener_forward {
            params.push(("ListenerForward".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.x_forwarded_for {
            params.push(("XForwardedFor".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.idle_timeout {
            params.push(("IdleTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.request_timeout {
            params.push(("RequestTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_interval {
            params.push(("HealthCheckInterval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.x_forwarded_for_slbport {
            params.push(("XForwardedFor_SLBPORT".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_uri {
            params.push(("HealthCheckURI".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sticky_session_type {
            params.push(("StickySessionType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.healthy_threshold {
            params.push(("HealthyThreshold".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.x_forwarded_for_proto {
            params.push(("XForwardedFor_proto".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.x_forwarded_for_slbip {
            params.push(("XForwardedFor_SLBIP".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sticky_session {
            params.push(("StickySession".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_method {
            params.push(("HealthCheckMethod".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check {
            params.push(("HealthCheck".to_string(), v.to_string()));
        }
        params
    }
}

/// HTTPS监听配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeLoadBalancerListenersResponseListenersItemHTTPSListenerConfig {
    /// 是否通过`XForwardedFor_ClientCertClientVerify`头字段获取对访问负载均衡实例客户端证书的校验结果。取值：
    #[serde(rename = "XForwardedFor_ClientCertClientVerify")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_client_cert_client_verify: Option<String>,
    /// 健康检查HTTP协议版本。
    #[serde(rename = "HealthCheckHttpVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_http_version: Option<String>,
    /// 是否通过`XForwardedFor_ClientSrcPort`头字段获取访问负载均衡实例客户端的端口。取值：
    #[serde(rename = "XForwardedFor_ClientSrcPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_client_src_port: Option<String>,
    /// 服务器上配置的Cookie。
    #[serde(rename = "Cookie")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie: Option<String>,
    /// 是否开启Gzip压缩。取值：
    #[serde(rename = "Gzip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gzip: Option<String>,
    /// 是否开启`HTTP 2.0`特性。取值：
    #[serde(rename = "EnableHttp2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_http2: Option<String>,
    /// CA证书ID。
    #[serde(rename = "CACertificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_certificate_id: Option<String>,
    /// 健康检查的端口。
    #[serde(rename = "HealthCheckConnectPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_connect_port: Option<i32>,
    /// 每次健康检查响应的最大超时间。单位：秒。
    #[serde(rename = "HealthCheckTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_timeout: Option<i32>,
    /// 健康检查协议。
    #[serde(rename = "HealthCheckType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_type: Option<String>,
    /// Cookie超时时间。单位：秒。
    #[serde(rename = "CookieTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie_timeout: Option<i32>,
    /// 健康检查的域名。
    #[serde(rename = "HealthCheckDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_domain: Option<String>,
    /// 健康检查连续失败多少次后，将后端服务器的健康检查状态由**成功**判定为**失败**。
    #[serde(rename = "UnhealthyThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unhealthy_threshold: Option<i32>,
    /// 是否通过`SLB-ID`头字段获取负载均衡实例ID。取值：
    #[serde(rename = "XForwardedFor_SLBID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_slbid: Option<String>,
    /// 是否通过`XForwardedFor_ClientCertSubjectDN`头字段获取访问负载均衡实例客户端证书的所有者信息。取值：
    #[serde(rename = "XForwardedFor_ClientCertSubjectDN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_client_cert_subject_dn: Option<String>,
    /// 健康检查正常的HTTP状态码。
    #[serde(rename = "HealthCheckHttpCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_http_code: Option<String>,
    /// 是否通过`XForwardedFor_ClientCertFingerprint`头字段获取访问负载均衡实例客户端证书的指纹。取值：
    #[serde(rename = "XForwardedFor_ClientCertFingerprint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_client_cert_fingerprint: Option<String>,
    /// 是否开启通过`XForwardedFor`的方式获取来访者真实IP。取值：
    #[serde(rename = "XForwardedFor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for: Option<String>,
    /// 指定请求超时时间，单位：秒。取值范围：**1**~**180**。
    #[serde(rename = "RequestTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_timeout: Option<i32>,
    /// 指定连接空闲超时时间。单位：秒。取值范围：**1**~**60**。
    #[serde(rename = "IdleTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_timeout: Option<i32>,
    /// 服务器证书的ID。
    #[serde(rename = "ServerCertificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificate_id: Option<String>,
    /// 健康检查的时间间隔。单位：秒。
    #[serde(rename = "HealthCheckInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_interval: Option<i32>,
    /// 是否通过`XForwardedFor_SLBPORT`头字段获取负载均衡实例的监听端口。取值：
    #[serde(rename = "XForwardedFor_SLBPORT")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_slbport: Option<String>,
    /// 健康检查的URI。
    #[serde(rename = "HealthCheckURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_uri: Option<String>,
    /// Cookie的处理方式。
    #[serde(rename = "StickySessionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticky_session_type: Option<String>,
    /// 是否通过`XForwardedFor_ClientCertIssuerDN`头字段获取访问负载均衡实例客户端证书的发行者信息。取值：
    #[serde(rename = "XForwardedFor_ClientCertIssuerDN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_client_cert_issuer_dn: Option<String>,
    /// 健康检查连续成功多少次后，将后端服务器的健康检查状态由**失败**判定为**成功**。
    #[serde(rename = "HealthyThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthy_threshold: Option<i32>,
    /// 是否通过`X-Forwarded-Proto`头字段获取负载均衡实例的监听协议。取值：
    #[serde(rename = "XForwardedFor_proto")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_proto: Option<String>,
    /// 是否通过`SLB-IP`头字段获取客户端请求的VIP（Virtual IP address）。取值：
    #[serde(rename = "XForwardedFor_SLBIP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_slbip: Option<String>,
    /// 是否开启会话保持。取值：
    #[serde(rename = "StickySession")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticky_session: Option<String>,
    /// 健康检查方式。
    #[serde(rename = "HealthCheckMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_method: Option<String>,
    /// 安全策略包含HTTPS可选的TLS协议版本和配套的加密算法套件。
    #[serde(rename = "TLSCipherPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_cipher_policy: Option<String>,
    /// 是否开启健康检查。取值：
    #[serde(rename = "HealthCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<String>,
}

impl DescribeLoadBalancerListenersResponseListenersItemHTTPSListenerConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.x_forwarded_for_client_cert_client_verify {
            params.push(("XForwardedFor_ClientCertClientVerify".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_http_version {
            params.push(("HealthCheckHttpVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.x_forwarded_for_client_src_port {
            params.push(("XForwardedFor_ClientSrcPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cookie {
            params.push(("Cookie".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.gzip {
            params.push(("Gzip".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_http2 {
            params.push(("EnableHttp2".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ca_certificate_id {
            params.push(("CACertificateId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_connect_port {
            params.push(("HealthCheckConnectPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_timeout {
            params.push(("HealthCheckTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_type {
            params.push(("HealthCheckType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cookie_timeout {
            params.push(("CookieTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_domain {
            params.push(("HealthCheckDomain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.unhealthy_threshold {
            params.push(("UnhealthyThreshold".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.x_forwarded_for_slbid {
            params.push(("XForwardedFor_SLBID".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.x_forwarded_for_client_cert_subject_dn {
            params.push(("XForwardedFor_ClientCertSubjectDN".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_http_code {
            params.push(("HealthCheckHttpCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.x_forwarded_for_client_cert_fingerprint {
            params.push(("XForwardedFor_ClientCertFingerprint".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.x_forwarded_for {
            params.push(("XForwardedFor".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.request_timeout {
            params.push(("RequestTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.idle_timeout {
            params.push(("IdleTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.server_certificate_id {
            params.push(("ServerCertificateId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_interval {
            params.push(("HealthCheckInterval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.x_forwarded_for_slbport {
            params.push(("XForwardedFor_SLBPORT".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_uri {
            params.push(("HealthCheckURI".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sticky_session_type {
            params.push(("StickySessionType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.x_forwarded_for_client_cert_issuer_dn {
            params.push(("XForwardedFor_ClientCertIssuerDN".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.healthy_threshold {
            params.push(("HealthyThreshold".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.x_forwarded_for_proto {
            params.push(("XForwardedFor_proto".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.x_forwarded_for_slbip {
            params.push(("XForwardedFor_SLBIP".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sticky_session {
            params.push(("StickySession".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_method {
            params.push(("HealthCheckMethod".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tls_cipher_policy {
            params.push(("TLSCipherPolicy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check {
            params.push(("HealthCheck".to_string(), v.to_string()));
        }
        params
    }
}

/// TCP监听配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeLoadBalancerListenersResponseListenersItemTCPListenerConfig {
    /// 健康检查正常的HTTP状态码。
    #[serde(rename = "HealthCheckHttpCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_http_code: Option<String>,
    /// 设置连接优雅中断超时时间。单位：秒。
    #[serde(rename = "ConnectionDrainTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_drain_timeout: Option<i32>,
    /// 是否开启了会话保持。单位：秒。
    #[serde(rename = "PersistenceTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistence_timeout: Option<i32>,
    /// 健康检查的时间间隔，单位为秒。
    #[serde(rename = "HealthCheckInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_interval: Option<i32>,
    /// 健康检查的URI。
    #[serde(rename = "HealthCheckURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_uri: Option<String>,
    /// 健康检查使用的端口。
    #[serde(rename = "HealthCheckConnectPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_connect_port: Option<i32>,
    /// 连接超时时间。单位：秒。
    #[serde(rename = "EstablishedTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub established_timeout: Option<i32>,
    /// 健康检查协议。
    #[serde(rename = "HealthCheckType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_type: Option<String>,
    /// 健康检查超时时间。单位：秒。
    #[serde(rename = "HealthCheckConnectTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_connect_timeout: Option<i32>,
    /// 监听绑定的主备服务器组ID。
    #[serde(rename = "MasterSlaveServerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_slave_server_group_id: Option<String>,
    /// 健康检查连续成功多少次后，将后端服务器的健康检查状态由**失败**判定为**成功**。
    #[serde(rename = "HealthyThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthy_threshold: Option<i32>,
    /// 健康检查的域名。
    #[serde(rename = "HealthCheckDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_domain: Option<String>,
    /// 健康检查连续失败多少次后，将后端服务器的健康检查状态由**成功**判定为**失败**。
    #[serde(rename = "UnhealthyThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unhealthy_threshold: Option<i32>,
    /// 是否开启连接优雅中断。取值：
    #[serde(rename = "ConnectionDrain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_drain: Option<String>,
    /// 健康检查方式。
    #[serde(rename = "HealthCheckMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_method: Option<String>,
    /// 是否开启健康检查。取值：
    #[serde(rename = "HealthCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<String>,
    /// 是否支持通过Proxy Protocol协议携带客户端源地址到后端服务器。取值：
    #[serde(rename = "ProxyProtocolV2Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_protocol_v2_enabled: Option<String>,
}

impl DescribeLoadBalancerListenersResponseListenersItemTCPListenerConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.health_check_http_code {
            params.push(("HealthCheckHttpCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.connection_drain_timeout {
            params.push(("ConnectionDrainTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.persistence_timeout {
            params.push(("PersistenceTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_interval {
            params.push(("HealthCheckInterval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_uri {
            params.push(("HealthCheckURI".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_connect_port {
            params.push(("HealthCheckConnectPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.established_timeout {
            params.push(("EstablishedTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_type {
            params.push(("HealthCheckType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_connect_timeout {
            params.push(("HealthCheckConnectTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.master_slave_server_group_id {
            params.push(("MasterSlaveServerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.healthy_threshold {
            params.push(("HealthyThreshold".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_domain {
            params.push(("HealthCheckDomain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.unhealthy_threshold {
            params.push(("UnhealthyThreshold".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.connection_drain {
            params.push(("ConnectionDrain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_method {
            params.push(("HealthCheckMethod".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check {
            params.push(("HealthCheck".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.proxy_protocol_v2_enabled {
            params.push(("ProxyProtocolV2Enabled".to_string(), v.to_string()));
        }
        params
    }
}

/// UDP监听配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeLoadBalancerListenersResponseListenersItemUDPListenerConfig {
    /// 设置连接优雅中断超时时间。单位：秒。
    #[serde(rename = "ConnectionDrainTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_drain_timeout: Option<i32>,
    /// 健康检查的时间间隔，单位为秒。
    #[serde(rename = "HealthCheckInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_interval: Option<i32>,
    /// UDP监听健康检查的响应串。
    #[serde(rename = "HealthCheckExp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_exp: Option<String>,
    /// 健康检查的端口。
    #[serde(rename = "HealthCheckConnectPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_connect_port: Option<i32>,
    /// 健康检查响应超时时间。
    #[serde(rename = "HealthCheckConnectTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_connect_timeout: Option<i32>,
    /// 绑定的主备服务器组ID。
    #[serde(rename = "MasterSlaveServerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_slave_server_group_id: Option<String>,
    /// 健康检查阈值。
    #[serde(rename = "HealthyThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthy_threshold: Option<i32>,
    /// 不健康检查阈值。
    #[serde(rename = "UnhealthyThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unhealthy_threshold: Option<i32>,
    /// 是否开启连接优雅中断。取值：
    #[serde(rename = "ConnectionDrain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_drain: Option<String>,
    /// UDP监听健康检查的请求串。
    #[serde(rename = "HealthCheckReq")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_req: Option<String>,
    /// 是否开启健康检查。取值：
    #[serde(rename = "HealthCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<String>,
    /// 是否支持通过Proxy Protocol协议携带客户端源地址到后端服务器。取值：
    #[serde(rename = "ProxyProtocolV2Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_protocol_v2_enabled: Option<String>,
}

impl DescribeLoadBalancerListenersResponseListenersItemUDPListenerConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.connection_drain_timeout {
            params.push(("ConnectionDrainTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_interval {
            params.push(("HealthCheckInterval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_exp {
            params.push(("HealthCheckExp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_connect_port {
            params.push(("HealthCheckConnectPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_connect_timeout {
            params.push(("HealthCheckConnectTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.master_slave_server_group_id {
            params.push(("MasterSlaveServerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.healthy_threshold {
            params.push(("HealthyThreshold".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.unhealthy_threshold {
            params.push(("UnhealthyThreshold".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.connection_drain {
            params.push(("ConnectionDrain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_req {
            params.push(("HealthCheckReq".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check {
            params.push(("HealthCheck".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.proxy_protocol_v2_enabled {
            params.push(("ProxyProtocolV2Enabled".to_string(), v.to_string()));
        }
        params
    }
}

/// 标签列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeLoadBalancerListenersResponseListenersItemTagsItem {
    /// 资源的标签值。
    #[serde(rename = "TagValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
    /// 资源的标签键。
    #[serde(rename = "TagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
}

impl DescribeLoadBalancerListenersResponseListenersItemTagsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.tag_value {
            params.push(("TagValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_key {
            params.push(("TagKey".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回传统型负载均衡的监听列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeLoadBalancerListenersResponseListenersItem {
    /// 访问控制类型。取值：
    #[serde(rename = "AclType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_type: Option<String>,
    /// 监听的状态。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 监听绑定的虚拟服务器组ID。
    #[serde(rename = "VServerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_server_group_id: Option<String>,
    /// 传统型负载均衡监听协议。
    #[serde(rename = "ListenerProtocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_protocol: Option<String>,
    /// 传统型负载均衡实例的ID。
    #[serde(rename = "LoadBalancerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_id: Option<String>,
    /// 监听端口。
    #[serde(rename = "ListenerPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_port: Option<i32>,
    /// 访问控制策略组ID。
    #[serde(rename = "AclId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_id: Option<String>,
    /// 调度算法。取值：
    #[serde(rename = "Scheduler")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduler: Option<String>,
    /// 监听的带宽峰值。单位：Mbps。
    #[serde(rename = "Bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<i32>,
    /// 监听描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 是否开启访问控制功能。取值：
    #[serde(rename = "AclStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_status: Option<String>,
    /// 后端服务器的端口。
    #[serde(rename = "BackendServerPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_server_port: Option<i32>,
    /// 访问控制策略组ID列表。
    #[serde(rename = "AclIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_ids: Option<Vec<String>>,
    /// HTTP监听配置。
    #[serde(rename = "HTTPListenerConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_listener_config: Option<DescribeLoadBalancerListenersResponseListenersItemHTTPListenerConfig>,
    /// HTTPS监听配置。
    #[serde(rename = "HTTPSListenerConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub https_listener_config: Option<DescribeLoadBalancerListenersResponseListenersItemHTTPSListenerConfig>,
    /// TCP监听配置。
    #[serde(rename = "TCPListenerConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_listener_config: Option<DescribeLoadBalancerListenersResponseListenersItemTCPListenerConfig>,
    /// UDP监听配置。
    #[serde(rename = "UDPListenerConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp_listener_config: Option<DescribeLoadBalancerListenersResponseListenersItemUDPListenerConfig>,
    /// 标签列表。
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<DescribeLoadBalancerListenersResponseListenersItemTagsItem>>,
}

impl DescribeLoadBalancerListenersResponseListenersItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.acl_type {
            params.push(("AclType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_server_group_id {
            params.push(("VServerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.listener_protocol {
            params.push(("ListenerProtocol".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.load_balancer_id {
            params.push(("LoadBalancerId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.listener_port {
            params.push(("ListenerPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.acl_id {
            params.push(("AclId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scheduler {
            params.push(("Scheduler".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bandwidth {
            params.push(("Bandwidth".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.acl_status {
            params.push(("AclStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backend_server_port {
            params.push(("BackendServerPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.acl_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("AclIds.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.http_listener_config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("HTTPListenerConfig.{}", k), v2));
            }
        }
        if let Some(ref v) = self.https_listener_config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("HTTPSListenerConfig.{}", k), v2));
            }
        }
        if let Some(ref v) = self.tcp_listener_config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("TCPListenerConfig.{}", k), v2));
            }
        }
        if let Some(ref v) = self.udp_listener_config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("UDPListenerConfig.{}", k), v2));
            }
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

/// 标签列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateLoadBalancerTCPListenerRequestTagItem {
    /// 监听的标签键。N的取值范围：**1**~**20**。一旦传入该值，则不允许为空字符串。最多支持64个字符，不能以`aliyun`或`acs:`开头，不能包含`http://`或`https://`。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 监听的标签值。N的取值范围：**1**~**20**。一旦传入该值，可以为空字符串。最多支持128个字符，不能以`acs:`开头，不能包含`http://`或者`https://`。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreateLoadBalancerTCPListenerRequestTagItem {
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
pub struct DescribeLoadBalancerTCPListenerAttributeResponseAclIds {
    /// 访问控制策略组ID列表。
    #[serde(rename = "AclId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_id: Option<Vec<String>>,
}

impl DescribeLoadBalancerTCPListenerAttributeResponseAclIds {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.acl_id {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("AclId.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 标签列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeLoadBalancerTCPListenerAttributeResponseTagsTagItem {
    /// 监听的标签键。N的取值范围：**1**~**20**。一旦传入该值，则不允许为空字符串。最多支持128个字符，不能以`aliyun`或`acs:`开头，不能包含`http://`或`https:...
    #[serde(rename = "TagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    /// 监听的标签值。N的取值范围：**1**~**20**。一旦传入该值，可以为空字符串。最多支持128个字符，不能以`acs:`开头，不能包含`http://`或者`https://`。
    #[serde(rename = "TagValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
}

impl DescribeLoadBalancerTCPListenerAttributeResponseTagsTagItem {
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
pub struct DescribeLoadBalancerTCPListenerAttributeResponseTags {
    /// 标签列表。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<DescribeLoadBalancerTCPListenerAttributeResponseTagsTagItem>>,
}

impl DescribeLoadBalancerTCPListenerAttributeResponseTags {
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

/// 监听的标签集合。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateLoadBalancerUDPListenerRequestTagItem {
    /// 监听的标签键。N的取值范围：**1**~**20**。一旦传入该值，则不允许为空字符串。最多支持128个字符，不能以`aliyun`或`acs:`开头，不能包含`http://`或`https:...
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 监听的标签值。N的取值范围：**1**~**20**。一旦传入该值，可以为空字符串。最多支持128个字符，不能以`acs:`开头，不能包含`http://`或者`https://`。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreateLoadBalancerUDPListenerRequestTagItem {
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
pub struct DescribeLoadBalancerUDPListenerAttributeResponseAclIds {
    /// 访问控制策略组ID。
    #[serde(rename = "AclId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_id: Option<Vec<String>>,
}

impl DescribeLoadBalancerUDPListenerAttributeResponseAclIds {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.acl_id {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("AclId.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 标签列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeLoadBalancerUDPListenerAttributeResponseTagsTagItem {
    /// 监听的标签键。N的取值范围：**1**~**20**。一旦传入该值，则不允许为空字符串。最多支持128个字符，不能以`aliyun`或`acs:`开头，不能包含`http://`或`https:...
    #[serde(rename = "TagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    /// 监听的标签值。N的取值范围：**1**~**20**。一旦传入该值，可以为空字符串。最多支持128个字符，不能以`acs:`开头，不能包含`http://`或者`https://`。
    #[serde(rename = "TagValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
}

impl DescribeLoadBalancerUDPListenerAttributeResponseTagsTagItem {
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
pub struct DescribeLoadBalancerUDPListenerAttributeResponseTags {
    /// 标签列表。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<DescribeLoadBalancerUDPListenerAttributeResponseTagsTagItem>>,
}

impl DescribeLoadBalancerUDPListenerAttributeResponseTags {
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

/// 标签列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateLoadBalancerHTTPListenerRequestTagItem {
    /// 监听的标签键。N的取值范围：**1**~**20**。一旦传入该值，则不允许为空字符串。最多支持128个字符，不能以`aliyun`或`acs:`开头，不能包含`http://`或`https:...
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 监听的标签值。N的取值范围：**1**~**20**。一旦传入该值，可以为空字符串。最多支持128个字符，不能以`acs:`开头，不能包含`http://`或者`https://`。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreateLoadBalancerHTTPListenerRequestTagItem {
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
pub struct DescribeLoadBalancerHTTPListenerAttributeResponseAclIds {
    /// 访问控制策略组ID。
    #[serde(rename = "AclId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_id: Option<Vec<String>>,
}

impl DescribeLoadBalancerHTTPListenerAttributeResponseAclIds {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.acl_id {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("AclId.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeLoadBalancerHTTPListenerAttributeResponseRulesRuleItem {
    /// 转发规则的目标服务器组ID。
    #[serde(rename = "VServerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_server_group_id: Option<String>,
    /// 访问路径。
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 转发规则名称。
    #[serde(rename = "RuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    /// 转发规则ID。
    #[serde(rename = "RuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
}

impl DescribeLoadBalancerHTTPListenerAttributeResponseRulesRuleItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.v_server_group_id {
            params.push(("VServerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.url {
            params.push(("Url".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rule_name {
            params.push(("RuleName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rule_id {
            params.push(("RuleId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeLoadBalancerHTTPListenerAttributeResponseRules {
    /// 转发规则描述。
    #[serde(rename = "Rule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<Vec<DescribeLoadBalancerHTTPListenerAttributeResponseRulesRuleItem>>,
}

impl DescribeLoadBalancerHTTPListenerAttributeResponseRules {
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

/// 标签列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeLoadBalancerHTTPListenerAttributeResponseTagsTagItem {
    /// 监听的标签键。N的取值范围：**1**~**20**。一旦传入该值，则不允许为空字符串。最多支持64个字符，不能以`aliyun`或`acs:`开头，不能包含`http://`或`https://`。
    #[serde(rename = "TagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    /// 监听的标签值。N的取值范围：**1**~**20**。一旦传入该值，可以为空字符串。最多支持128个字符，不能以`acs:`开头，不能包含`http://`或者`https://`。
    #[serde(rename = "TagValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
}

impl DescribeLoadBalancerHTTPListenerAttributeResponseTagsTagItem {
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
pub struct DescribeLoadBalancerHTTPListenerAttributeResponseTags {
    /// 标签列表。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<DescribeLoadBalancerHTTPListenerAttributeResponseTagsTagItem>>,
}

impl DescribeLoadBalancerHTTPListenerAttributeResponseTags {
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

/// 标签列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateLoadBalancerHTTPSListenerRequestTagItem {
    /// 监听的标签键。N的取值范围：**1**~**20**。一旦传入该值，则不允许为空字符串。最多支持64个字符，不能以`aliyun`或`acs:`开头，不能包含`http://`或`https://`。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 监听的标签值。N的取值范围：**1**~**20**。一旦传入该值，可以为空字符串。最多支持128个字符，不能以`acs:`开头，不能包含`http://`或者`https://`。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreateLoadBalancerHTTPSListenerRequestTagItem {
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
pub struct DescribeLoadBalancerHTTPSListenerAttributeResponseAclIds {
    /// 监听绑定的访问策略组ID。
    #[serde(rename = "AclId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_id: Option<Vec<String>>,
}

impl DescribeLoadBalancerHTTPSListenerAttributeResponseAclIds {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.acl_id {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("AclId.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 监听下的转发规则列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeLoadBalancerHTTPSListenerAttributeResponseRulesRuleItem {
    /// 转发规则的目标服务器组ID。
    #[serde(rename = "VServerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_server_group_id: Option<String>,
    /// 访问路径。
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 转发规则名称。
    #[serde(rename = "RuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    /// 转发规则ID。
    #[serde(rename = "RuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
}

impl DescribeLoadBalancerHTTPSListenerAttributeResponseRulesRuleItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.v_server_group_id {
            params.push(("VServerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.url {
            params.push(("Url".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rule_name {
            params.push(("RuleName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rule_id {
            params.push(("RuleId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeLoadBalancerHTTPSListenerAttributeResponseRules {
    /// 监听下的转发规则列表。
    #[serde(rename = "Rule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<Vec<DescribeLoadBalancerHTTPSListenerAttributeResponseRulesRuleItem>>,
}

impl DescribeLoadBalancerHTTPSListenerAttributeResponseRules {
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

/// 域名扩展列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeLoadBalancerHTTPSListenerAttributeResponseDomainExtensionsDomainExtensionItem {
    /// 与域名对应的证书ID。
    #[serde(rename = "ServerCertificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificate_id: Option<String>,
    /// 域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 域名扩展ID。
    #[serde(rename = "DomainExtensionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_extension_id: Option<String>,
}

impl DescribeLoadBalancerHTTPSListenerAttributeResponseDomainExtensionsDomainExtensionItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.server_certificate_id {
            params.push(("ServerCertificateId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain_extension_id {
            params.push(("DomainExtensionId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeLoadBalancerHTTPSListenerAttributeResponseDomainExtensions {
    /// 域名扩展列表。
    #[serde(rename = "DomainExtension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_extension: Option<Vec<DescribeLoadBalancerHTTPSListenerAttributeResponseDomainExtensionsDomainExtensionItem>>,
}

impl DescribeLoadBalancerHTTPSListenerAttributeResponseDomainExtensions {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain_extension {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("DomainExtension.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 标签列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeLoadBalancerHTTPSListenerAttributeResponseTagsTagItem {
    /// 监听的标签键。N的取值范围：**1**~**20**。一旦传入该值，则不允许为空字符串。最多支持64个字符，不能以`aliyun`或`acs:`开头，不能包含`http://`或`https://`。
    #[serde(rename = "TagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    /// 监听的标签值。N的取值范围：**1**~**20**。一旦传入该值，可以为空字符串。最多支持128个字符，不能以`acs:`开头，不能包含`http://`或者`https://`。
    #[serde(rename = "TagValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
}

impl DescribeLoadBalancerHTTPSListenerAttributeResponseTagsTagItem {
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
pub struct DescribeLoadBalancerHTTPSListenerAttributeResponseTags {
    /// 标签列表。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<DescribeLoadBalancerHTTPSListenerAttributeResponseTagsTagItem>>,
}

impl DescribeLoadBalancerHTTPSListenerAttributeResponseTags {
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

/// 转发规则列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateRulesResponseRulesRuleItem {
    /// 转发规则名称。
    #[serde(rename = "RuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    /// 转发规则ID。
    #[serde(rename = "RuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
}

impl CreateRulesResponseRulesRuleItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.rule_name {
            params.push(("RuleName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rule_id {
            params.push(("RuleId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateRulesResponseRules {
    /// 转发规则列表。
    #[serde(rename = "Rule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<Vec<CreateRulesResponseRulesRuleItem>>,
}

impl CreateRulesResponseRules {
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
pub struct DescribeRulesResponseRulesRuleItem {
    /// 健康检查正常的HTTP状态码，多个状态码用半角逗号（,）分隔。默认值为**http_2xx**。
    #[serde(rename = "HealthCheckHttpCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_http_code: Option<String>,
    /// 转发规则绑定的目标虚拟服务器组ID。
    #[serde(rename = "VServerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_server_group_id: Option<String>,
    /// 转发规则绑定的请求域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 服务器上配置的Cookie。
    #[serde(rename = "Cookie")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie: Option<String>,
    /// 健康检查的时间间隔。单位：秒。
    #[serde(rename = "HealthCheckInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_interval: Option<i32>,
    /// 转发规则绑定的请求路径。
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 用于健康检查的URI。
    #[serde(rename = "HealthCheckURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_uri: Option<String>,
    /// Cookie的处理方式。取值：
    #[serde(rename = "StickySessionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticky_session_type: Option<String>,
    /// 转发规则名称，长度为1~80个字符，只能使用字母、数字、短划线（-）、正斜线（/）、半角句号（.）和下划线（_）这些字符。
    #[serde(rename = "RuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    /// 转发规则ID。
    #[serde(rename = "RuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    /// 健康检查的后端服务器的端口。
    #[serde(rename = "HealthCheckConnectPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_connect_port: Option<i32>,
    /// 调度算法。取值：
    #[serde(rename = "Scheduler")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduler: Option<String>,
    /// 接收来自运行状况检查的响应需要等待的时间。如果后端ECS在指定的时间内没有正确响应，则判定为健康检查失败。单位：秒。
    #[serde(rename = "HealthCheckTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_timeout: Option<i32>,
    /// 转发规则是否从监听上继承健康检查、会话保持和调度算法配置。取值：
    #[serde(rename = "ListenerSync")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_sync: Option<String>,
    /// 健康检查连续成功多少次后，将后端服务器的健康检查状态由**失败**判定为**成功**。
    #[serde(rename = "HealthyThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthy_threshold: Option<i32>,
    /// Cookie超时时间。单位：秒。取值范围：**1~86400**。
    #[serde(rename = "CookieTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie_timeout: Option<i32>,
    /// 用于健康检查的域名，取值：
    #[serde(rename = "HealthCheckDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_domain: Option<String>,
    /// 健康检查连续失败多少次后，将后端服务器的健康检查状态由**success**判定为**fail**。
    #[serde(rename = "UnhealthyThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unhealthy_threshold: Option<i32>,
    /// 是否开启会话保持。取值：
    #[serde(rename = "StickySession")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticky_session: Option<String>,
    /// 是否开启健康检查。取值：
    #[serde(rename = "HealthCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<String>,
}

impl DescribeRulesResponseRulesRuleItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.health_check_http_code {
            params.push(("HealthCheckHttpCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_server_group_id {
            params.push(("VServerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cookie {
            params.push(("Cookie".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_interval {
            params.push(("HealthCheckInterval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.url {
            params.push(("Url".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_uri {
            params.push(("HealthCheckURI".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sticky_session_type {
            params.push(("StickySessionType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rule_name {
            params.push(("RuleName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rule_id {
            params.push(("RuleId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_connect_port {
            params.push(("HealthCheckConnectPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scheduler {
            params.push(("Scheduler".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_timeout {
            params.push(("HealthCheckTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.listener_sync {
            params.push(("ListenerSync".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.healthy_threshold {
            params.push(("HealthyThreshold".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cookie_timeout {
            params.push(("CookieTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_domain {
            params.push(("HealthCheckDomain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.unhealthy_threshold {
            params.push(("UnhealthyThreshold".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sticky_session {
            params.push(("StickySession".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check {
            params.push(("HealthCheck".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRulesResponseRules {
    /// 转发规则列表。
    #[serde(rename = "Rule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<Vec<DescribeRulesResponseRulesRuleItem>>,
}

impl DescribeRulesResponseRules {
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
pub struct AddBackendServersResponseBackendServersBackendServerItem {
    /// 后端服务器类型。取值：
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 后端服务器的权重。
    #[serde(rename = "Weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<String>,
    /// 后端服务器描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// ECS、ENI、或者ECI实例ID。
    #[serde(rename = "ServerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
}

impl AddBackendServersResponseBackendServersBackendServerItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.weight {
            params.push(("Weight".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.server_id {
            params.push(("ServerId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddBackendServersResponseBackendServers {
    /// 后端服务器列表。
    #[serde(rename = "BackendServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_server: Option<Vec<AddBackendServersResponseBackendServersBackendServerItem>>,
}

impl AddBackendServersResponseBackendServers {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.backend_server {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("BackendServer.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 后端服务器列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeHealthStatusResponseBackendServersBackendServerItem {
    /// 负载均衡实例前端使用的协议。
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// 后端服务器的健康状况。
    #[serde(rename = "ServerHealthStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_health_status: Option<String>,
    /// 负载均衡实例前端使用的端口。
    #[serde(rename = "ListenerPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_port: Option<i32>,
    /// 后端服务器IP地址。
    #[serde(rename = "ServerIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_ip: Option<String>,
    /// 负载均衡实例后端使用的端口。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// 后端服务器实例ID。
    #[serde(rename = "ServerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
}

impl DescribeHealthStatusResponseBackendServersBackendServerItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.protocol {
            params.push(("Protocol".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.server_health_status {
            params.push(("ServerHealthStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.listener_port {
            params.push(("ListenerPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.server_ip {
            params.push(("ServerIp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.server_id {
            params.push(("ServerId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeHealthStatusResponseBackendServers {
    /// 后端服务器列表。
    #[serde(rename = "BackendServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_server: Option<Vec<DescribeHealthStatusResponseBackendServersBackendServerItem>>,
}

impl DescribeHealthStatusResponseBackendServers {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.backend_server {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("BackendServer.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 后端服务器列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemoveBackendServersResponseBackendServersBackendServerItem {
    /// 后端服务器类型，取值：
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 后端服务器的权重，范围为**0~100**。
    #[serde(rename = "Weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    /// 后端服务器组描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 后端服务器的实例ID。
    #[serde(rename = "ServerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
}

impl RemoveBackendServersResponseBackendServersBackendServerItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.weight {
            params.push(("Weight".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.server_id {
            params.push(("ServerId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemoveBackendServersResponseBackendServers {
    /// 后端服务器列表。
    #[serde(rename = "BackendServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_server: Option<Vec<RemoveBackendServersResponseBackendServersBackendServerItem>>,
}

impl RemoveBackendServersResponseBackendServers {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.backend_server {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("BackendServer.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 后端服务器列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetBackendServersResponseBackendServersBackendServerItem {
    /// 后端服务器类型，取值：
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 后端服务器的权重。
    #[serde(rename = "Weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<String>,
    /// 后端服务器描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 后端服务器ID。
    #[serde(rename = "ServerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
}

impl SetBackendServersResponseBackendServersBackendServerItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.weight {
            params.push(("Weight".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.server_id {
            params.push(("ServerId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetBackendServersResponseBackendServers {
    /// 后端服务器列表。
    #[serde(rename = "BackendServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_server: Option<Vec<SetBackendServersResponseBackendServersBackendServerItem>>,
}

impl SetBackendServersResponseBackendServers {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.backend_server {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("BackendServer.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 标签列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateVServerGroupRequestTagItem {
    /// 资源的标签键。N的取值范围：**1~20**。一旦输入该值，则不允许为空字符串。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 资源的标签值。N的取值范围：**1~20**。一旦输入该值，可以为空字符串。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreateVServerGroupRequestTagItem {
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

/// 后端服务器列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateVServerGroupResponseBackendServersBackendServerItem {
    /// 后端服务器类型，取值：
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 后端服务器的权重。
    #[serde(rename = "Weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    /// 后端服务器组描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 后端服务器使用的端口。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// ECS实例ID或ENI实例ID。
    #[serde(rename = "ServerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
}

impl CreateVServerGroupResponseBackendServersBackendServerItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.weight {
            params.push(("Weight".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.server_id {
            params.push(("ServerId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateVServerGroupResponseBackendServers {
    /// 后端服务器列表。
    #[serde(rename = "BackendServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_server: Option<Vec<CreateVServerGroupResponseBackendServersBackendServerItem>>,
}

impl CreateVServerGroupResponseBackendServers {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.backend_server {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("BackendServer.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 后端服务器列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetVServerGroupAttributeResponseBackendServersBackendServerItem {
    /// 后端服务器类型，取值：
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 后端服务器的权重。
    #[serde(rename = "Weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    /// 后端服务器组描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 后端服务器使用的端口。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// 后端服务器实例ID。
    #[serde(rename = "ServerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
}

impl SetVServerGroupAttributeResponseBackendServersBackendServerItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.weight {
            params.push(("Weight".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.server_id {
            params.push(("ServerId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetVServerGroupAttributeResponseBackendServers {
    /// 后端服务器列表。
    #[serde(rename = "BackendServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_server: Option<Vec<SetVServerGroupAttributeResponseBackendServersBackendServerItem>>,
}

impl SetVServerGroupAttributeResponseBackendServers {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.backend_server {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("BackendServer.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 资源标签。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeVServerGroupsRequestTagItem {
    /// 资源的标签键。N的取值范围：**1~20**。一旦输入该值，则不允许为空字符串。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 资源的标签值。N的取值范围：**1~20**。一旦输入该值，可以为空字符串。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DescribeVServerGroupsRequestTagItem {
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

/// 监听列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeVServerGroupsResponseVServerGroupsVServerGroupItemAssociatedObjectsListenersListenerItem {
    /// 监听端口。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// 监听协议。取值：**tcp**、**udp**、**http**或**https**。
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

impl DescribeVServerGroupsResponseVServerGroupsVServerGroupItemAssociatedObjectsListenersListenerItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.protocol {
            params.push(("Protocol".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeVServerGroupsResponseVServerGroupsVServerGroupItemAssociatedObjectsListeners {
    /// 监听列表。
    #[serde(rename = "Listener")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener: Option<Vec<DescribeVServerGroupsResponseVServerGroupsVServerGroupItemAssociatedObjectsListenersListenerItem>>,
}

impl DescribeVServerGroupsResponseVServerGroupsVServerGroupItemAssociatedObjectsListeners {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.listener {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Listener.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 转发规则列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeVServerGroupsResponseVServerGroupsVServerGroupItemAssociatedObjectsRulesRuleItem {
    /// 访问路径。
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 请求域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 转发规则名称。
    #[serde(rename = "RuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    /// 转发规则ID。
    #[serde(rename = "RuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
}

impl DescribeVServerGroupsResponseVServerGroupsVServerGroupItemAssociatedObjectsRulesRuleItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.url {
            params.push(("Url".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rule_name {
            params.push(("RuleName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rule_id {
            params.push(("RuleId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeVServerGroupsResponseVServerGroupsVServerGroupItemAssociatedObjectsRules {
    /// 转发规则列表。
    #[serde(rename = "Rule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<Vec<DescribeVServerGroupsResponseVServerGroupsVServerGroupItemAssociatedObjectsRulesRuleItem>>,
}

impl DescribeVServerGroupsResponseVServerGroupsVServerGroupItemAssociatedObjectsRules {
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

/// 关联信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeVServerGroupsResponseVServerGroupsVServerGroupItemAssociatedObjects {
    #[serde(rename = "Listeners")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listeners: Option<DescribeVServerGroupsResponseVServerGroupsVServerGroupItemAssociatedObjectsListeners>,
    #[serde(rename = "Rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<DescribeVServerGroupsResponseVServerGroupsVServerGroupItemAssociatedObjectsRules>,
}

impl DescribeVServerGroupsResponseVServerGroupsVServerGroupItemAssociatedObjects {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.listeners {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Listeners.{}", k), v2));
            }
        }
        if let Some(ref v) = self.rules {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Rules.{}", k), v2));
            }
        }
        params
    }
}

/// 标签列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeVServerGroupsResponseVServerGroupsVServerGroupItemTagsTagItem {
    /// 资源的标签键。
    #[serde(rename = "TagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    /// 资源的标签值。
    #[serde(rename = "TagValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
}

impl DescribeVServerGroupsResponseVServerGroupsVServerGroupItemTagsTagItem {
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
pub struct DescribeVServerGroupsResponseVServerGroupsVServerGroupItemTags {
    /// 标签列表。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<DescribeVServerGroupsResponseVServerGroupsVServerGroupItemTagsTagItem>>,
}

impl DescribeVServerGroupsResponseVServerGroupsVServerGroupItemTags {
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

/// 后端服务器列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeVServerGroupsResponseVServerGroupsVServerGroupItem {
    /// 服务器组ID。
    #[serde(rename = "VServerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_server_group_id: Option<String>,
    /// 服务器组名称。
    #[serde(rename = "VServerGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_server_group_name: Option<String>,
    /// 关联信息。
    #[serde(rename = "AssociatedObjects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_objects: Option<DescribeVServerGroupsResponseVServerGroupsVServerGroupItemAssociatedObjects>,
    /// 服务器个数。
    #[serde(rename = "ServerCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_count: Option<i64>,
    /// 传统型负载均衡实例的创建时间，格式：`YYYY-MM-DDThh:mm:ssZ`。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<DescribeVServerGroupsResponseVServerGroupsVServerGroupItemTags>,
}

impl DescribeVServerGroupsResponseVServerGroupsVServerGroupItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.v_server_group_id {
            params.push(("VServerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_server_group_name {
            params.push(("VServerGroupName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.associated_objects {
            for (k, v2) in v.to_query_params() {
                params.push((format!("AssociatedObjects.{}", k), v2));
            }
        }
        if let Some(ref v) = self.server_count {
            params.push(("ServerCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Tags.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeVServerGroupsResponseVServerGroups {
    /// 后端服务器列表。
    #[serde(rename = "VServerGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_server_group: Option<Vec<DescribeVServerGroupsResponseVServerGroupsVServerGroupItem>>,
}

impl DescribeVServerGroupsResponseVServerGroups {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.v_server_group {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("VServerGroup.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 后端服务器列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeVServerGroupAttributeResponseBackendServersBackendServerItem {
    /// 后端服务器类型，取值：
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 后端服务器的权重。
    #[serde(rename = "Weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    /// 服务器组描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 后端服务器实例IP。
    #[serde(rename = "ServerIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_ip: Option<String>,
    /// 后端服务器使用的端口。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// 后端服务器实例ID。
    #[serde(rename = "ServerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
}

impl DescribeVServerGroupAttributeResponseBackendServersBackendServerItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.weight {
            params.push(("Weight".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.server_ip {
            params.push(("ServerIp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.server_id {
            params.push(("ServerId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeVServerGroupAttributeResponseBackendServers {
    /// 后端服务器列表。
    #[serde(rename = "BackendServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_server: Option<Vec<DescribeVServerGroupAttributeResponseBackendServersBackendServerItem>>,
}

impl DescribeVServerGroupAttributeResponseBackendServers {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.backend_server {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("BackendServer.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 服务器上配置的标签。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeVServerGroupAttributeResponseTagsTagItem {
    /// 实例的标签键。N的取值范围：**1**~**20**。一旦输入该值，则不允许为空字符串。
    #[serde(rename = "TagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    /// 实例的标签值。N的取值范围：**1**~**20**。一旦输入该值，可以为空字符串。
    #[serde(rename = "TagValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
}

impl DescribeVServerGroupAttributeResponseTagsTagItem {
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
pub struct DescribeVServerGroupAttributeResponseTags {
    /// 服务器上配置的标签。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<DescribeVServerGroupAttributeResponseTagsTagItem>>,
}

impl DescribeVServerGroupAttributeResponseTags {
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

/// 后端服务器列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddVServerGroupBackendServersResponseBackendServersBackendServerItem {
    /// 后端服务器类型，取值：
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 后端服务器的权重。
    #[serde(rename = "Weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    /// 后端服务器组描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 后端服务器使用的端口。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// ECS实例ID或者ENI的实例ID。
    #[serde(rename = "ServerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
}

impl AddVServerGroupBackendServersResponseBackendServersBackendServerItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.weight {
            params.push(("Weight".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.server_id {
            params.push(("ServerId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddVServerGroupBackendServersResponseBackendServers {
    /// 后端服务器列表。
    #[serde(rename = "BackendServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_server: Option<Vec<AddVServerGroupBackendServersResponseBackendServersBackendServerItem>>,
}

impl AddVServerGroupBackendServersResponseBackendServers {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.backend_server {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("BackendServer.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 后端服务器列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModifyVServerGroupBackendServersResponseBackendServersBackendServerItem {
    /// 后端服务器类型，取值：
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 后端服务器的权重。
    #[serde(rename = "Weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    /// 后端服务器描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 后端服务器使用的端口。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// ECS实例ID或ENI的实例ID。
    #[serde(rename = "ServerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
}

impl ModifyVServerGroupBackendServersResponseBackendServersBackendServerItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.weight {
            params.push(("Weight".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.server_id {
            params.push(("ServerId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModifyVServerGroupBackendServersResponseBackendServers {
    /// 后端服务器列表。
    #[serde(rename = "BackendServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_server: Option<Vec<ModifyVServerGroupBackendServersResponseBackendServersBackendServerItem>>,
}

impl ModifyVServerGroupBackendServersResponseBackendServers {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.backend_server {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("BackendServer.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 后端服务器列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemoveVServerGroupBackendServersResponseBackendServersBackendServerItem {
    /// 后端服务器类型，取值：
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 后端服务器的权重。
    #[serde(rename = "Weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    /// 后端服务器使用的端口。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// 后端服务器实例ID。
    #[serde(rename = "ServerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
}

impl RemoveVServerGroupBackendServersResponseBackendServersBackendServerItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.weight {
            params.push(("Weight".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.server_id {
            params.push(("ServerId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemoveVServerGroupBackendServersResponseBackendServers {
    /// 后端服务器列表。
    #[serde(rename = "BackendServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_server: Option<Vec<RemoveVServerGroupBackendServersResponseBackendServersBackendServerItem>>,
}

impl RemoveVServerGroupBackendServersResponseBackendServers {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.backend_server {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("BackendServer.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 标签列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateMasterSlaveServerGroupRequestTagItem {
    /// 资源标签键。N的取值范围：**1**~**20**。一旦输入该值，则不允许为空字符串。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 资源的标签值。N的取值范围：**1~20**。一旦输入该值，可以为空字符串。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreateMasterSlaveServerGroupRequestTagItem {
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

/// 主备服务器组列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateMasterSlaveServerGroupResponseMasterSlaveBackendServersMasterSlaveBackendServerItem {
    /// 后端服务器类型，取值：
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 后端服务器的权重。
    #[serde(rename = "Weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    /// 主备服务器组描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 后端服务器使用的端口。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// 要添加的后端服务器实例ID。
    #[serde(rename = "ServerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    /// 服务器类型。
    #[serde(rename = "ServerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_type: Option<String>,
}

impl CreateMasterSlaveServerGroupResponseMasterSlaveBackendServersMasterSlaveBackendServerItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.weight {
            params.push(("Weight".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.server_id {
            params.push(("ServerId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.server_type {
            params.push(("ServerType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateMasterSlaveServerGroupResponseMasterSlaveBackendServers {
    /// 主备服务器组列表。
    #[serde(rename = "MasterSlaveBackendServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_slave_backend_server: Option<Vec<CreateMasterSlaveServerGroupResponseMasterSlaveBackendServersMasterSlaveBackendServerItem>>,
}

impl CreateMasterSlaveServerGroupResponseMasterSlaveBackendServers {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.master_slave_backend_server {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("MasterSlaveBackendServer.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 关联的标签类型。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeMasterSlaveServerGroupAttributeResponseTagsTagItem {
    /// 实例的标签键。N的取值范围：**1**~**20**。一旦输入该值，则不允许为空字符串。
    #[serde(rename = "TagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    /// 实例的标签值。N的取值范围：**1**~**20**。一旦输入该值，可以为空字符串。
    #[serde(rename = "TagValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
}

impl DescribeMasterSlaveServerGroupAttributeResponseTagsTagItem {
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
pub struct DescribeMasterSlaveServerGroupAttributeResponseTags {
    /// 关联的标签类型。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<DescribeMasterSlaveServerGroupAttributeResponseTagsTagItem>>,
}

impl DescribeMasterSlaveServerGroupAttributeResponseTags {
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

/// 主备服务器组列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeMasterSlaveServerGroupAttributeResponseMasterSlaveBackendServersMasterSlaveBackendServerItem {
    /// 后端服务器类型，取值：
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 后端服务器的权重。
    #[serde(rename = "Weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    /// 主备服务器组描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 后端服务器使用的端口。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// 后端服务器实例ID。
    #[serde(rename = "ServerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    /// 服务器类型，取值：**Master**或**Slave**。
    #[serde(rename = "ServerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_type: Option<String>,
}

impl DescribeMasterSlaveServerGroupAttributeResponseMasterSlaveBackendServersMasterSlaveBackendServerItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.weight {
            params.push(("Weight".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.server_id {
            params.push(("ServerId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.server_type {
            params.push(("ServerType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeMasterSlaveServerGroupAttributeResponseMasterSlaveBackendServers {
    /// 主备服务器组列表。
    #[serde(rename = "MasterSlaveBackendServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_slave_backend_server: Option<Vec<DescribeMasterSlaveServerGroupAttributeResponseMasterSlaveBackendServersMasterSlaveBackendServerItem>>,
}

impl DescribeMasterSlaveServerGroupAttributeResponseMasterSlaveBackendServers {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.master_slave_backend_server {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("MasterSlaveBackendServer.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 资源标签。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeMasterSlaveServerGroupsRequestTagItem {
    /// 资源的标签键。N的取值范围：**1~20**。一旦输入该值，则不允许为空字符串。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 资源的标签值。N的取值范围：**1~20**。一旦输入该值，可以为空字符串。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DescribeMasterSlaveServerGroupsRequestTagItem {
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

/// 监听列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeMasterSlaveServerGroupsResponseMasterSlaveServerGroupsMasterSlaveServerGroupItemAssociatedObjectsListenersListenerItem {
    /// 监听端口。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// 监听协议。
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

impl DescribeMasterSlaveServerGroupsResponseMasterSlaveServerGroupsMasterSlaveServerGroupItemAssociatedObjectsListenersListenerItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.protocol {
            params.push(("Protocol".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeMasterSlaveServerGroupsResponseMasterSlaveServerGroupsMasterSlaveServerGroupItemAssociatedObjectsListeners {
    /// 监听列表。
    #[serde(rename = "Listener")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener: Option<Vec<DescribeMasterSlaveServerGroupsResponseMasterSlaveServerGroupsMasterSlaveServerGroupItemAssociatedObjectsListenersListenerItem>>,
}

impl DescribeMasterSlaveServerGroupsResponseMasterSlaveServerGroupsMasterSlaveServerGroupItemAssociatedObjectsListeners {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.listener {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Listener.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 关联信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeMasterSlaveServerGroupsResponseMasterSlaveServerGroupsMasterSlaveServerGroupItemAssociatedObjects {
    #[serde(rename = "Listeners")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listeners: Option<DescribeMasterSlaveServerGroupsResponseMasterSlaveServerGroupsMasterSlaveServerGroupItemAssociatedObjectsListeners>,
}

impl DescribeMasterSlaveServerGroupsResponseMasterSlaveServerGroupsMasterSlaveServerGroupItemAssociatedObjects {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.listeners {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Listeners.{}", k), v2));
            }
        }
        params
    }
}

/// 标签列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeMasterSlaveServerGroupsResponseMasterSlaveServerGroupsMasterSlaveServerGroupItemTagsTagItem {
    /// 资源的标签键。
    #[serde(rename = "TagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    /// 资源的标签值。
    #[serde(rename = "TagValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
}

impl DescribeMasterSlaveServerGroupsResponseMasterSlaveServerGroupsMasterSlaveServerGroupItemTagsTagItem {
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
pub struct DescribeMasterSlaveServerGroupsResponseMasterSlaveServerGroupsMasterSlaveServerGroupItemTags {
    /// 标签列表。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<DescribeMasterSlaveServerGroupsResponseMasterSlaveServerGroupsMasterSlaveServerGroupItemTagsTagItem>>,
}

impl DescribeMasterSlaveServerGroupsResponseMasterSlaveServerGroupsMasterSlaveServerGroupItemTags {
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

/// 主备服务器组列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeMasterSlaveServerGroupsResponseMasterSlaveServerGroupsMasterSlaveServerGroupItem {
    /// 主备服务器组ID。
    #[serde(rename = "MasterSlaveServerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_slave_server_group_id: Option<String>,
    /// 主备服务器组的名称。
    #[serde(rename = "MasterSlaveServerGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_slave_server_group_name: Option<String>,
    /// 关联信息。
    #[serde(rename = "AssociatedObjects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_objects: Option<DescribeMasterSlaveServerGroupsResponseMasterSlaveServerGroupsMasterSlaveServerGroupItemAssociatedObjects>,
    /// 传统型负载均衡实例创建时间，格式为：`YYYY-MM-DDThh:mm:ssZ`。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<DescribeMasterSlaveServerGroupsResponseMasterSlaveServerGroupsMasterSlaveServerGroupItemTags>,
}

impl DescribeMasterSlaveServerGroupsResponseMasterSlaveServerGroupsMasterSlaveServerGroupItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.master_slave_server_group_id {
            params.push(("MasterSlaveServerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.master_slave_server_group_name {
            params.push(("MasterSlaveServerGroupName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.associated_objects {
            for (k, v2) in v.to_query_params() {
                params.push((format!("AssociatedObjects.{}", k), v2));
            }
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Tags.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeMasterSlaveServerGroupsResponseMasterSlaveServerGroups {
    /// 主备服务器组列表。
    #[serde(rename = "MasterSlaveServerGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_slave_server_group: Option<Vec<DescribeMasterSlaveServerGroupsResponseMasterSlaveServerGroupsMasterSlaveServerGroupItem>>,
}

impl DescribeMasterSlaveServerGroupsResponseMasterSlaveServerGroups {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.master_slave_server_group {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("MasterSlaveServerGroup.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 标签列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCACertificatesRequestTagItem {
    /// 资源的标签键。N的取值范围：**1~20**。一旦输入该值，则不允许为空字符串。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 资源的标签值。N的取值范围：**1~20**。一旦输入该值，可以为空字符串。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DescribeCACertificatesRequestTagItem {
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

/// 标签。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCACertificatesResponseCACertificatesCACertificateItemTagsTagItem {
    /// 资源的标签值。
    #[serde(rename = "TagValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
    /// 资源的标签键。
    #[serde(rename = "TagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
}

impl DescribeCACertificatesResponseCACertificatesCACertificateItemTagsTagItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.tag_value {
            params.push(("TagValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_key {
            params.push(("TagKey".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCACertificatesResponseCACertificatesCACertificateItemTags {
    /// 标签。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<DescribeCACertificatesResponseCACertificatesCACertificateItemTagsTagItem>>,
}

impl DescribeCACertificatesResponseCACertificatesCACertificateItemTags {
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
pub struct DescribeCACertificatesResponseCACertificatesCACertificateItem {
    /// CA证书创建的时间戳。
    #[serde(rename = "CreateTimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time_stamp: Option<i64>,
    /// CA证书的过期时间。格式为：`YYYY-MM-DDThh:mm:ssZ`。
    #[serde(rename = "ExpireTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<String>,
    /// CA证书的创建时间。格式为：`YYYY-MM-DDThh:mm:ssZ`。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// CA证书过期的时间戳。
    #[serde(rename = "ExpireTimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time_stamp: Option<i64>,
    /// CA证书ID。
    #[serde(rename = "CACertificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_certificate_id: Option<String>,
    /// CA证书所属地域。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// CA证书的指纹。
    #[serde(rename = "Fingerprint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// CA证书的域名。
    #[serde(rename = "CommonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_name: Option<String>,
    /// CA证书名称。
    #[serde(rename = "CACertificateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_certificate_name: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<DescribeCACertificatesResponseCACertificatesCACertificateItemTags>,
}

impl DescribeCACertificatesResponseCACertificatesCACertificateItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.create_time_stamp {
            params.push(("CreateTimeStamp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.expire_time {
            params.push(("ExpireTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.expire_time_stamp {
            params.push(("ExpireTimeStamp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ca_certificate_id {
            params.push(("CACertificateId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.fingerprint {
            params.push(("Fingerprint".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.common_name {
            params.push(("CommonName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ca_certificate_name {
            params.push(("CACertificateName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Tags.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCACertificatesResponseCACertificates {
    /// CA证书信息。
    #[serde(rename = "CACertificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_certificate: Option<Vec<DescribeCACertificatesResponseCACertificatesCACertificateItem>>,
}

impl DescribeCACertificatesResponseCACertificates {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.ca_certificate {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("CACertificate.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 标签列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeServerCertificatesRequestTagItem {
    /// 资源的标签键。N的取值范围：**1~20**。一旦输入该值，则不允许为空字符串。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 资源的标签值。N的取值范围：**1~20**。一旦输入该值，可以为空字符串。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DescribeServerCertificatesRequestTagItem {
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

/// 资源标签。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeServerCertificatesResponseServerCertificatesServerCertificateItemTagsTagItem {
    /// 资源的标签值。
    #[serde(rename = "TagValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
    /// 资源的标签键。
    #[serde(rename = "TagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
}

impl DescribeServerCertificatesResponseServerCertificatesServerCertificateItemTagsTagItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.tag_value {
            params.push(("TagValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_key {
            params.push(("TagKey".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeServerCertificatesResponseServerCertificatesServerCertificateItemTags {
    /// 资源标签。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<DescribeServerCertificatesResponseServerCertificatesServerCertificateItemTagsTagItem>>,
}

impl DescribeServerCertificatesResponseServerCertificatesServerCertificateItemTags {
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
pub struct DescribeServerCertificatesResponseServerCertificatesServerCertificateItemSubjectAlternativeNames {
    /// 数组格式，返回证书的备用域名列表，对应证书的`Subject Alternative Name`字段。
    #[serde(rename = "SubjectAlternativeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_alternative_name: Option<Vec<String>>,
}

impl DescribeServerCertificatesResponseServerCertificatesServerCertificateItemSubjectAlternativeNames {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.subject_alternative_name {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("SubjectAlternativeName.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeServerCertificatesResponseServerCertificatesServerCertificateItem {
    /// 服务器证书上传的时间戳。
    #[serde(rename = "CreateTimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time_stamp: Option<i64>,
    /// 阿里云证书服务的服务器证书名称。
    #[serde(rename = "AliCloudCertificateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ali_cloud_certificate_name: Option<String>,
    /// 过期时间。
    #[serde(rename = "ExpireTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<String>,
    /// 服务器证书上传的时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 服务器证书ID。
    #[serde(rename = "ServerCertificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificate_id: Option<String>,
    /// 过期时间戳。
    #[serde(rename = "ExpireTimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time_stamp: Option<i64>,
    /// 服务器证书的地域。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 服务器证书名称。
    #[serde(rename = "ServerCertificateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificate_name: Option<String>,
    /// 服务器证书的指纹。
    #[serde(rename = "Fingerprint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    /// 域名，对应证书的`CommonName`字段。
    #[serde(rename = "CommonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_name: Option<String>,
    /// 企业资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 是否是阿里云证书服务中的证书：
    #[serde(rename = "IsAliCloudCertificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_ali_cloud_certificate: Option<i32>,
    /// 阿里云证书服务的服务器证书ID。
    #[serde(rename = "AliCloudCertificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ali_cloud_certificate_id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<DescribeServerCertificatesResponseServerCertificatesServerCertificateItemTags>,
    #[serde(rename = "SubjectAlternativeNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_alternative_names: Option<DescribeServerCertificatesResponseServerCertificatesServerCertificateItemSubjectAlternativeNames>,
}

impl DescribeServerCertificatesResponseServerCertificatesServerCertificateItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.create_time_stamp {
            params.push(("CreateTimeStamp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ali_cloud_certificate_name {
            params.push(("AliCloudCertificateName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.expire_time {
            params.push(("ExpireTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.server_certificate_id {
            params.push(("ServerCertificateId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.expire_time_stamp {
            params.push(("ExpireTimeStamp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.server_certificate_name {
            params.push(("ServerCertificateName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.fingerprint {
            params.push(("Fingerprint".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.common_name {
            params.push(("CommonName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_ali_cloud_certificate {
            params.push(("IsAliCloudCertificate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ali_cloud_certificate_id {
            params.push(("AliCloudCertificateId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Tags.{}", k), v2));
            }
        }
        if let Some(ref v) = self.subject_alternative_names {
            for (k, v2) in v.to_query_params() {
                params.push((format!("SubjectAlternativeNames.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeServerCertificatesResponseServerCertificates {
    /// 服务器证书列表。
    #[serde(rename = "ServerCertificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificate: Option<Vec<DescribeServerCertificatesResponseServerCertificatesServerCertificateItem>>,
}

impl DescribeServerCertificatesResponseServerCertificates {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.server_certificate {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("ServerCertificate.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 标签列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UploadCACertificateRequestTagItem {
    /// 监听的标签键。N的取值范围：**1**~**20**。一旦传入该值，则不允许为空字符串。最多支持128个字符，不能以`aliyun`或`acs:`开头，不能包含`http://`或`https:...
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 实例的标签值。N的取值范围：**1~20**。一旦输入该值，可以为空字符串。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl UploadCACertificateRequestTagItem {
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
pub struct UploadServerCertificateRequestTagItem {
    /// 资源的标签键。N的取值范围：**1~20**。一旦输入该值，则不允许为空字符串。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 资源的标签值。N的取值范围：**1~20**。一旦输入该值，可以为空字符串。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl UploadServerCertificateRequestTagItem {
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
pub struct UploadServerCertificateResponseSubjectAlternativeNames {
    /// 返回证书的备用域名列表。
    #[serde(rename = "SubjectAlternativeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_alternative_name: Option<Vec<String>>,
}

impl UploadServerCertificateResponseSubjectAlternativeNames {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.subject_alternative_name {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("SubjectAlternativeName.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 扩展域名列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainExtensionsResponseDomainExtensionsDomainExtensionItem {
    /// 域名使用的证书ID。
    #[serde(rename = "ServerCertificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificate_id: Option<String>,
    /// 域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 扩展域名ID。
    #[serde(rename = "DomainExtensionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_extension_id: Option<String>,
}

impl DescribeDomainExtensionsResponseDomainExtensionsDomainExtensionItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.server_certificate_id {
            params.push(("ServerCertificateId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain_extension_id {
            params.push(("DomainExtensionId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainExtensionsResponseDomainExtensions {
    /// 扩展域名列表。
    #[serde(rename = "DomainExtension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_extension: Option<Vec<DescribeDomainExtensionsResponseDomainExtensionsDomainExtensionItem>>,
}

impl DescribeDomainExtensionsResponseDomainExtensions {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain_extension {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("DomainExtension.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListTLSCipherPoliciesResponseTLSCipherPoliciesItemRelateListenersItem {
    /// 监听端口。取值：**1**~**65535**。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// 监听协议。取值：
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// 负载均衡实例ID。
    #[serde(rename = "LoadBalancerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_id: Option<String>,
}

impl ListTLSCipherPoliciesResponseTLSCipherPoliciesItemRelateListenersItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.protocol {
            params.push(("Protocol".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.load_balancer_id {
            params.push(("LoadBalancerId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListTLSCipherPoliciesResponseTLSCipherPoliciesItem {
    /// TLS策略实例状态。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// TLS策略实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// TLS策略名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 创建时间的时间戳。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 关联的监听。
    #[serde(rename = "RelateListeners")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relate_listeners: Option<Vec<ListTLSCipherPoliciesResponseTLSCipherPoliciesItemRelateListenersItem>>,
    #[serde(rename = "TLSVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_versions: Option<Vec<String>>,
    #[serde(rename = "Ciphers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ciphers: Option<Vec<String>>,
}

impl ListTLSCipherPoliciesResponseTLSCipherPoliciesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.relate_listeners {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("RelateListeners.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.tls_versions {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("TLSVersions.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.ciphers {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Ciphers.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 标签列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateAccessControlListRequestTagItem {
    /// 监听的标签键。N的取值范围：**1**~**20**。一旦传入该值，则不允许为空字符串。最多支持128个字符，不能以`aliyun`或`acs:`开头，不能包含`http://`或`https:...
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 监听的标签值。N的取值范围：**1**~**20**。一旦传入该值，可以为空字符串。最多支持128个字符，不能以`acs:`开头，不能包含`http://`或者`https://`。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreateAccessControlListRequestTagItem {
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

/// ACL的标签集合。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAccessControlListAttributeResponseTagsTagItem {
    /// ACL的标签键。
    #[serde(rename = "TagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    /// ACL的标签值。
    #[serde(rename = "TagValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
}

impl DescribeAccessControlListAttributeResponseTagsTagItem {
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
pub struct DescribeAccessControlListAttributeResponseTags {
    /// ACL的标签集合。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<DescribeAccessControlListAttributeResponseTagsTagItem>>,
}

impl DescribeAccessControlListAttributeResponseTags {
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
pub struct DescribeAccessControlListAttributeResponseAclEntrysAclEntryItem {
    /// 访问控制条目备注。
    #[serde(rename = "AclEntryComment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_entry_comment: Option<String>,
    /// 访问控制条目IP。
    #[serde(rename = "AclEntryIP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_entry_ip: Option<String>,
}

impl DescribeAccessControlListAttributeResponseAclEntrysAclEntryItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.acl_entry_comment {
            params.push(("AclEntryComment".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.acl_entry_ip {
            params.push(("AclEntryIP".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAccessControlListAttributeResponseAclEntrys {
    /// 访问控制策略组的信息列表。
    #[serde(rename = "AclEntry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_entry: Option<Vec<DescribeAccessControlListAttributeResponseAclEntrysAclEntryItem>>,
}

impl DescribeAccessControlListAttributeResponseAclEntrys {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.acl_entry {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("AclEntry.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 该访问控制策略组已绑定的监听列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAccessControlListAttributeResponseRelatedListenersRelatedListenerItem {
    /// 绑定的监听的前端端口。
    #[serde(rename = "ListenerPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_port: Option<i32>,
    /// 访问控制的类型：
    #[serde(rename = "AclType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_type: Option<String>,
    /// 绑定的监听的协议类型。
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// 传统型负载均衡实例的ID。
    #[serde(rename = "LoadBalancerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_id: Option<String>,
}

impl DescribeAccessControlListAttributeResponseRelatedListenersRelatedListenerItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.listener_port {
            params.push(("ListenerPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.acl_type {
            params.push(("AclType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.protocol {
            params.push(("Protocol".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.load_balancer_id {
            params.push(("LoadBalancerId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAccessControlListAttributeResponseRelatedListeners {
    /// 该访问控制策略组已绑定的监听列表。
    #[serde(rename = "RelatedListener")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_listener: Option<Vec<DescribeAccessControlListAttributeResponseRelatedListenersRelatedListenerItem>>,
}

impl DescribeAccessControlListAttributeResponseRelatedListeners {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.related_listener {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("RelatedListener.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 标签列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAccessControlListsRequestTagItem {
    /// 资源的标签键。N的取值范围：**1~20**。一旦输入该值，则不允许为空字符串。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 资源的标签值。N的取值范围：**1~20**。一旦输入该值，可以为空字符串。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DescribeAccessControlListsRequestTagItem {
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

/// 访问控制策略组标签列表。取值：是一个Json string，其结构是一个JsonList。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAccessControlListsResponseAclsAclItemTagsTagItem {
    /// 资源的标签值。
    #[serde(rename = "TagValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
    /// 资源的标签键。
    #[serde(rename = "TagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
}

impl DescribeAccessControlListsResponseAclsAclItemTagsTagItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.tag_value {
            params.push(("TagValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_key {
            params.push(("TagKey".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAccessControlListsResponseAclsAclItemTags {
    /// 访问控制策略组标签列表。取值：是一个Json string，其结构是一个JsonList。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<DescribeAccessControlListsResponseAclsAclItemTagsTagItem>>,
}

impl DescribeAccessControlListsResponseAclsAclItemTags {
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

/// 查询到的访问控制策略组列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAccessControlListsResponseAclsAclItem {
    /// 访问控制策略组ID。
    #[serde(rename = "AclId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_id: Option<String>,
    /// 关联的负载均衡实例的IP地址类型。
    #[serde(rename = "AddressIPVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_ip_version: Option<String>,
    /// 访问控制策略组名称。
    #[serde(rename = "AclName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_name: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<DescribeAccessControlListsResponseAclsAclItemTags>,
    /// 传统型负载均衡实例的创建时间，格式为：`YYYY-MM-DDThh:mm:ssZ`。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
}

impl DescribeAccessControlListsResponseAclsAclItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.acl_id {
            params.push(("AclId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.address_ip_version {
            params.push(("AddressIPVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.acl_name {
            params.push(("AclName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Tags.{}", k), v2));
            }
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAccessControlListsResponseAcls {
    /// 查询到的访问控制策略组列表。
    #[serde(rename = "Acl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl: Option<Vec<DescribeAccessControlListsResponseAclsAclItem>>,
}

impl DescribeAccessControlListsResponseAcls {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.acl {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Acl.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeTagsResponseTagSetsTagSetItem {
    /// 标签Value。
    #[serde(rename = "TagValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
    /// 该标签绑定的实例总数。
    #[serde(rename = "InstanceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_count: Option<i32>,
    /// 标签Key。
    #[serde(rename = "TagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
}

impl DescribeTagsResponseTagSetsTagSetItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.tag_value {
            params.push(("TagValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_count {
            params.push(("InstanceCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_key {
            params.push(("TagKey".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeTagsResponseTagSets {
    /// Tag列表。
    #[serde(rename = "TagSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_set: Option<Vec<DescribeTagsResponseTagSetsTagSetItem>>,
}

impl DescribeTagsResponseTagSets {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.tag_set {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("TagSet.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 标签列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListTagResourcesRequestTagItem {
    /// 资源的标签键。最多支持添加20个标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 资源的标签值。最多支持添加20个标签值。一旦传入该值，不可以为空字符串。
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

/// 绑定标签的资源信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListTagResourcesResponseTagResourcesTagResourceItem {
    /// 标签值。
    #[serde(rename = "TagValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
    /// 资源类型。
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
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
    /// 绑定标签的资源信息。
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

/// 标签列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TagResourcesRequestTagItem {
    /// 资源的标签键。最多支持20个标签键。一旦传入该值，则不允许为空字符串。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 资源的标签值。最多支持20个标签值。一旦传入该值，可以为空字符串。
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

/// 访问日志配置信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAccessLogsDownloadAttributeResponseLogsDownloadAttributesLogsDownloadAttributeItem {
    /// 日志服务LogProject的名称。
    #[serde(rename = "LogProject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_project: Option<String>,
    /// 日志服务LogStore的名称。
    #[serde(rename = "LogStore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_store: Option<String>,
    /// 传统型负载均衡实例的ID。
    #[serde(rename = "LoadBalancerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_id: Option<String>,
    /// 传统型负载均衡实例所在的地域ID。
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// 访问日志类型。仅取值**layer7**，表示七层访问日志。
    #[serde(rename = "LogType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_type: Option<String>,
}

impl DescribeAccessLogsDownloadAttributeResponseLogsDownloadAttributesLogsDownloadAttributeItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.log_project {
            params.push(("LogProject".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.log_store {
            params.push(("LogStore".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.load_balancer_id {
            params.push(("LoadBalancerId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region {
            params.push(("Region".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.log_type {
            params.push(("LogType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAccessLogsDownloadAttributeResponseLogsDownloadAttributes {
    /// 访问日志配置信息。
    #[serde(rename = "LogsDownloadAttribute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs_download_attribute: Option<Vec<DescribeAccessLogsDownloadAttributeResponseLogsDownloadAttributesLogsDownloadAttributeItem>>,
}

impl DescribeAccessLogsDownloadAttributeResponseLogsDownloadAttributes {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.logs_download_attribute {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("LogsDownloadAttribute.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// DescribeAvailableResource 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeAvailableResourceRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 网络类型。
    #[serde(rename = "AddressType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_type: Option<String>,
    /// IP地址类型。
    #[serde(rename = "AddressIPVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_ip_version: Option<String>,
}

impl DescribeAvailableResourceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.address_type {
            params.push(("AddressType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.address_ip_version {
            params.push(("AddressIPVersion".to_string(), v.to_string()));
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
    #[serde(rename = "AvailableResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_resources: Option<DescribeAvailableResourceResponseAvailableResources>,
}

/// DescribeRegions 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeRegionsRequest {
    /// 支持的语言。取值：
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// 地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
}

impl DescribeRegionsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.accept_language {
            params.push(("AcceptLanguage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
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
    #[serde(rename = "Regions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<DescribeRegionsResponseRegions>,
}

/// DescribeZones 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeZonesRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
}

impl DescribeZonesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
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

/// CreateLoadBalancer 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateLoadBalancerRequest {
    /// 传统型负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 传统型负载均衡实例的网络类型。取值：
    #[serde(rename = "AddressType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_type: Option<String>,
    /// 公网类型实例的付费方式。取值：
    #[serde(rename = "InternetChargeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internet_charge_type: Option<String>,
    /// 实例的带宽峰值，单位：Mbps。
    #[serde(rename = "Bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<i32>,
    /// 客户端Token，用于保证请求的幂等性。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// 传统型负载均衡实例的名称。
    #[serde(rename = "LoadBalancerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_name: Option<String>,
    /// 传统型负载均衡实例的所属的VPC ID。
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 专有网络实例的所属的交换机ID。
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 传统型负载均衡实例的主可用区ID。
    #[serde(rename = "MasterZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_zone_id: Option<String>,
    /// 传统型负载均衡实例的备可用区ID。
    #[serde(rename = "SlaveZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slave_zone_id: Option<String>,
    /// 传统型负载均衡实例的规格。取值：
    #[serde(rename = "LoadBalancerSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_spec: Option<String>,
    /// 企业资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 实例的计费类型，取值：
    #[serde(rename = "PayType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_type: Option<String>,
    /// 预付费公网实例的计费周期，取值：
    #[serde(rename = "PricingCycle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_cycle: Option<String>,
    /// 预付费公网实例的购买时长，取值：
    #[serde(rename = "Duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    /// 是否是自动支付预付费公网实例的账单。取值：
    #[serde(rename = "AutoPay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_pay: Option<bool>,
    /// 传统型负载均衡实例的IP版本，取值：**ipv4**或**ipv6**。
    #[serde(rename = "AddressIPVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_ip_version: Option<String>,
    /// 指定实例的私网IP地址，该地址必须包含在交换机的目标网段下。
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// 标签列表。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<CreateLoadBalancerRequestTagItem>>,
    /// 是否开启实例删除保护。取值：
    #[serde(rename = "DeleteProtection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_protection: Option<String>,
    /// 传统型负载均衡修改保护状态：
    #[serde(rename = "ModificationProtectionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modification_protection_status: Option<String>,
    /// 设置修改保护状态的原因，长度为1～80个字符，必须以字母或中文开头，支持数字、半角句号（.）、下划线（_）和短划线（-）。
    #[serde(rename = "ModificationProtectionReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modification_protection_reason: Option<String>,
    /// <props="china">
    #[serde(rename = "InstanceChargeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_charge_type: Option<String>,
}

impl CreateLoadBalancerRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.address_type {
            params.push(("AddressType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.internet_charge_type {
            params.push(("InternetChargeType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bandwidth {
            params.push(("Bandwidth".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.load_balancer_name {
            params.push(("LoadBalancerName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_id {
            params.push(("VpcId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("VSwitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.master_zone_id {
            params.push(("MasterZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.slave_zone_id {
            params.push(("SlaveZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.load_balancer_spec {
            params.push(("LoadBalancerSpec".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.pay_type {
            params.push(("PayType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.pricing_cycle {
            params.push(("PricingCycle".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.duration {
            params.push(("Duration".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_pay {
            params.push(("AutoPay".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.address_ip_version {
            params.push(("AddressIPVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.address {
            params.push(("Address".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tag.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.delete_protection {
            params.push(("DeleteProtection".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.modification_protection_status {
            params.push(("ModificationProtectionStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.modification_protection_reason {
            params.push(("ModificationProtectionReason".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_charge_type {
            params.push(("InstanceChargeType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateLoadBalancerResponse {
    /// 传统型负载均衡实例的所属专有网络的ID。
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 传统型负载均衡实例的IP地址类型。
    #[serde(rename = "AddressIPVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_ip_version: Option<String>,
    /// 传统型负载均衡实例的所属交换机的ID。
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 传统型负载均衡实例的名称。
    #[serde(rename = "LoadBalancerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_name: Option<String>,
    /// 传统型负载均衡实例的ID。
    #[serde(rename = "LoadBalancerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_id: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 分配的负载均衡实例的IP地址。
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// 传统型负载均衡实例的网络类型，取值：
    #[serde(rename = "NetworkType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    /// 预付费实例的订单ID。
    #[serde(rename = "OrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i64>,
}

/// DeleteLoadBalancer 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteLoadBalancerRequest {
    /// 负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 负载均衡实例的ID。
    #[serde(rename = "LoadBalancerId")]
    pub load_balancer_id: String,
}

impl DeleteLoadBalancerRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params.push(("LoadBalancerId".to_string(), self.load_balancer_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteLoadBalancerResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyLoadBalancerInstanceSpec 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyLoadBalancerInstanceSpecRequest {
    /// 负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 负载均衡实例ID。
    #[serde(rename = "LoadBalancerId")]
    pub load_balancer_id: String,
    /// 负载均衡实例的规格。取值：
    #[serde(rename = "LoadBalancerSpec")]
    pub load_balancer_spec: String,
    /// 是否自动付费。取值：
    #[serde(rename = "AutoPay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_pay: Option<bool>,
}

impl ModifyLoadBalancerInstanceSpecRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params.push(("LoadBalancerId".to_string(), self.load_balancer_id.to_string()));
        params.push(("LoadBalancerSpec".to_string(), self.load_balancer_spec.to_string()));
        if let Some(ref v) = self.auto_pay {
            params.push(("AutoPay".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyLoadBalancerInstanceSpecResponse {
    /// 预付费实例的订单ID。
    #[serde(rename = "OrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i64>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyLoadBalancerInternetSpec 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyLoadBalancerInternetSpecRequest {
    /// 传统型负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 负载均衡实例的ID。
    #[serde(rename = "LoadBalancerId")]
    pub load_balancer_id: String,
    /// 公网类型实例的付费方式。取值：
    #[serde(rename = "InternetChargeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internet_charge_type: Option<String>,
    /// 按固定带宽计费方式的公网类型实例的带宽峰值。单位：Mbps。
    #[serde(rename = "Bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<i32>,
    /// 是否是自动支付预付费公网实例的账单。取值：
    #[serde(rename = "AutoPay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_pay: Option<bool>,
}

impl ModifyLoadBalancerInternetSpecRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params.push(("LoadBalancerId".to_string(), self.load_balancer_id.to_string()));
        if let Some(ref v) = self.internet_charge_type {
            params.push(("InternetChargeType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bandwidth {
            params.push(("Bandwidth".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_pay {
            params.push(("AutoPay".to_string(), v.to_string()));
        }
        params
    }
}

/// 修改公网负载均衡实例的计费方式。
#[derive(Debug, Clone, Deserialize)]
pub struct ModifyLoadBalancerInternetSpecResponse {
    /// 预付费实例的订单ID。
    #[serde(rename = "OrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i64>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyLoadBalancerPayType 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyLoadBalancerPayTypeRequest {
    /// 负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 负载均衡实例的ID。
    #[serde(rename = "LoadBalancerId")]
    pub load_balancer_id: String,
    /// 负载均衡实例付费模式，取值：
    #[serde(rename = "PayType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_type: Option<String>,
    /// 计费周期。
    #[serde(rename = "PricingCycle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_cycle: Option<String>,
    /// 计费时长。
    #[serde(rename = "Duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    /// 是否自动付费。取值：
    #[serde(rename = "AutoPay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_pay: Option<bool>,
}

impl ModifyLoadBalancerPayTypeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("LoadBalancerId".to_string(), self.load_balancer_id.to_string()));
        if let Some(ref v) = self.pay_type {
            params.push(("PayType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.pricing_cycle {
            params.push(("PricingCycle".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.duration {
            params.push(("Duration".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_pay {
            params.push(("AutoPay".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyLoadBalancerPayTypeResponse {
    /// 预付费实例的订单ID。
    #[serde(rename = "OrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i64>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// SetLoadBalancerDeleteProtection 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetLoadBalancerDeleteProtectionRequest {
    /// 传统型负载均衡实例的地域。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 传统型负载均衡实例的ID。
    #[serde(rename = "LoadBalancerId")]
    pub load_balancer_id: String,
    /// 负载均衡删除保护状态。取值：
    #[serde(rename = "DeleteProtection")]
    pub delete_protection: String,
}

impl SetLoadBalancerDeleteProtectionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("LoadBalancerId".to_string(), self.load_balancer_id.to_string()));
        params.push(("DeleteProtection".to_string(), self.delete_protection.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetLoadBalancerDeleteProtectionResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// SetLoadBalancerModificationProtection 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetLoadBalancerModificationProtectionRequest {
    /// 传统型负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 传统型负载均衡实例ID。
    #[serde(rename = "LoadBalancerId")]
    pub load_balancer_id: String,
    /// 传统型负载均衡修改保护状态：
    #[serde(rename = "ModificationProtectionStatus")]
    pub modification_protection_status: String,
    /// 设置修改保护状态的原因，长度为1~80个字符，必须以字母或中文开头，支持数字、半角句号（.）、下划线（_）和短划线（-）。
    #[serde(rename = "ModificationProtectionReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modification_protection_reason: Option<String>,
}

impl SetLoadBalancerModificationProtectionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("LoadBalancerId".to_string(), self.load_balancer_id.to_string()));
        params.push(("ModificationProtectionStatus".to_string(), self.modification_protection_status.to_string()));
        if let Some(ref v) = self.modification_protection_reason {
            params.push(("ModificationProtectionReason".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetLoadBalancerModificationProtectionResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// SetLoadBalancerName 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetLoadBalancerNameRequest {
    /// 传统型负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 传统型负载均衡实例ID。
    #[serde(rename = "LoadBalancerId")]
    pub load_balancer_id: String,
    /// 传统型负载均衡实例的新名称。
    #[serde(rename = "LoadBalancerName")]
    pub load_balancer_name: String,
}

impl SetLoadBalancerNameRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params.push(("LoadBalancerId".to_string(), self.load_balancer_id.to_string()));
        params.push(("LoadBalancerName".to_string(), self.load_balancer_name.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetLoadBalancerNameResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// SetLoadBalancerStatus 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetLoadBalancerStatusRequest {
    /// 负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 传统型负载均衡实例ID。
    #[serde(rename = "LoadBalancerId")]
    pub load_balancer_id: String,
    /// 传统型负载均衡实例状态。取值：**active**或**inactive**。
    #[serde(rename = "LoadBalancerStatus")]
    pub load_balancer_status: String,
}

impl SetLoadBalancerStatusRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params.push(("LoadBalancerId".to_string(), self.load_balancer_id.to_string()));
        params.push(("LoadBalancerStatus".to_string(), self.load_balancer_status.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetLoadBalancerStatusResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyLoadBalancerInstanceChargeType 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyLoadBalancerInstanceChargeTypeRequest {
    /// 传统型负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 传统型负载均衡实例ID。
    #[serde(rename = "LoadBalancerId")]
    pub load_balancer_id: String,
    /// 变更后的公网计费方式。
    #[serde(rename = "InternetChargeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internet_charge_type: Option<String>,
    /// 变更后的实例计费方式。
    #[serde(rename = "InstanceChargeType")]
    pub instance_charge_type: String,
    /// 传统型负载均衡实例的规格。
    #[serde(rename = "LoadBalancerSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_spec: Option<String>,
    /// 按带宽计费的公网型实例的带宽峰值。
    #[serde(rename = "Bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<i32>,
}

impl ModifyLoadBalancerInstanceChargeTypeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("LoadBalancerId".to_string(), self.load_balancer_id.to_string()));
        if let Some(ref v) = self.internet_charge_type {
            params.push(("InternetChargeType".to_string(), v.to_string()));
        }
        params.push(("InstanceChargeType".to_string(), self.instance_charge_type.to_string()));
        if let Some(ref v) = self.load_balancer_spec {
            params.push(("LoadBalancerSpec".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bandwidth {
            params.push(("Bandwidth".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyLoadBalancerInstanceChargeTypeResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeLoadBalancerAttribute 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeLoadBalancerAttributeRequest {
    /// 传统型负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 传统型负载均衡实例ID。
    #[serde(rename = "LoadBalancerId")]
    pub load_balancer_id: String,
}

impl DescribeLoadBalancerAttributeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params.push(("LoadBalancerId".to_string(), self.load_balancer_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeLoadBalancerAttributeResponse {
    /// 传统型负载均衡私网实例的专有网络ID。
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<DescribeLoadBalancerAttributeResponseTags>,
    /// 传统型负载均衡实例创建时间戳。
    #[serde(rename = "CreateTimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time_stamp: Option<i64>,
    /// 传统型负载均衡实例的创建时间。格式为：`YYYY-MM-DDThh:mm:ssZ`。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 传统型负载均衡实例的ID。
    #[serde(rename = "LoadBalancerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_id: Option<String>,
    /// 传统型负载均衡实例付费类型。取值：
    #[serde(rename = "PayType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_type: Option<String>,
    /// 传统型负载均衡实例的地址类型。
    #[serde(rename = "AddressType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_type: Option<String>,
    /// 传统型负载均衡实例的网络类型。
    #[serde(rename = "NetworkType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    /// IP版本。取值：**ipv4**或**ipv6**。
    #[serde(rename = "AddressIPVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_ip_version: Option<String>,
    /// 自动续费周期，取值：**Year**或**Month**（默认值）。
    #[serde(rename = "RenewalCycUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_cyc_unit: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 按带宽计费的公网型实例的带宽峰值。
    #[serde(rename = "Bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<i32>,
    /// 传统型负载均衡实例的名称。
    #[serde(rename = "LoadBalancerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_name: Option<String>,
    /// 传统型负载均衡实例的服务地址。
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// 传统型负载均衡实例的备可用区ID。
    #[serde(rename = "SlaveZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slave_zone_id: Option<String>,
    /// 传统型负载均衡实例结束时间戳。
    #[serde(rename = "EndTimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time_stamp: Option<i64>,
    /// 传统型负载均衡实例的主可用区ID。
    #[serde(rename = "MasterZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_zone_id: Option<String>,
    /// 传统型负载均衡实例的性能规格。
    #[serde(rename = "LoadBalancerSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_spec: Option<String>,
    /// 释放时间的时间戳。
    #[serde(rename = "AutoReleaseTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_release_time: Option<i64>,
    /// 设置修改保护状态的原因，长度为1~80个字符，必须以字母或中文开头，支持数字、半角句号（.）、下划线（_）和短划线（-）。
    #[serde(rename = "ModificationProtectionReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modification_protection_reason: Option<String>,
    /// 传统型负载均衡实例所在地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 传统型负载均衡修改保护状态：
    #[serde(rename = "ModificationProtectionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modification_protection_status: Option<String>,
    /// 传统型负载均衡实例结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 私网实例的交换机ID。
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 传统型负载均衡实例状态：
    #[serde(rename = "LoadBalancerStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_status: Option<String>,
    /// 企业资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 公网类型实例付费方式。取值：
    #[serde(rename = "InternetChargeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internet_charge_type: Option<String>,
    /// 是否开启实例删除保护。
    #[serde(rename = "DeleteProtection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_protection: Option<String>,
    /// 传统型负载均衡实例所属的地域别名。
    #[serde(rename = "RegionIdAlias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id_alias: Option<String>,
    /// 续费状态，取值：
    #[serde(rename = "RenewalStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_status: Option<String>,
    /// 自动续费时长，仅在**RenewalStatus**为**AutoRenewal**时有效。
    #[serde(rename = "RenewalDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_duration: Option<i32>,
    #[serde(rename = "ListenerPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_ports: Option<DescribeLoadBalancerAttributeResponseListenerPorts>,
    #[serde(rename = "ListenerPortsAndProtocal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_ports_and_protocal: Option<DescribeLoadBalancerAttributeResponseListenerPortsAndProtocal>,
    #[serde(rename = "ListenerPortsAndProtocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_ports_and_protocol: Option<DescribeLoadBalancerAttributeResponseListenerPortsAndProtocol>,
    #[serde(rename = "BackendServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_servers: Option<DescribeLoadBalancerAttributeResponseBackendServers>,
    /// 实例计费方式。取值：
    #[serde(rename = "InstanceChargeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_charge_type: Option<String>,
}

/// DescribeLoadBalancers 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeLoadBalancersRequest {
    /// 传统型负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 添加的后端服务器ID。
    #[serde(rename = "ServerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    /// IP版本，可以设置为**ipv4**或者**ipv6**。
    #[serde(rename = "AddressIPVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_ip_version: Option<String>,
    /// 实例状态。取值：
    #[serde(rename = "LoadBalancerStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_status: Option<String>,
    /// 传统型负载均衡实例ID。
    #[serde(rename = "LoadBalancerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_id: Option<String>,
    /// 传统型负载均衡实例名称。
    #[serde(rename = "LoadBalancerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_name: Option<String>,
    /// 添加的后端服务器的内网地址。
    #[serde(rename = "ServerIntranetAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_intranet_address: Option<String>,
    /// 传统型负载均衡实例的网络类型。取值：
    #[serde(rename = "AddressType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_type: Option<String>,
    /// 公网计费方式。取值：
    #[serde(rename = "InternetChargeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internet_charge_type: Option<String>,
    /// 传统型负载均衡实例所属的VPC ID。
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 传统型负载均衡实例所属的交换机ID。
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 私网实例的网络类型。取值：
    #[serde(rename = "NetworkType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    /// 传统型负载均衡实例的服务地址。
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// 负载均衡实例的主可用区ID。
    #[serde(rename = "MasterZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_zone_id: Option<String>,
    /// 传统型负载均衡实例的备可用区ID。
    #[serde(rename = "SlaveZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slave_zone_id: Option<String>,
    /// 传统型负载均衡实例绑定的标签列表，其结构是一个JSON dictionary，包含标签键和标签值。
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    /// 传统型负载均衡实例付费模式。取值：
    #[serde(rename = "PayType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_type: Option<String>,
    /// 企业资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 分页查询时的页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 分页查询时设置的每页行数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 标签列表。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<DescribeLoadBalancersRequestTagItem>>,
}

impl DescribeLoadBalancersRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.server_id {
            params.push(("ServerId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.address_ip_version {
            params.push(("AddressIPVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.load_balancer_status {
            params.push(("LoadBalancerStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.load_balancer_id {
            params.push(("LoadBalancerId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.load_balancer_name {
            params.push(("LoadBalancerName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.server_intranet_address {
            params.push(("ServerIntranetAddress".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.address_type {
            params.push(("AddressType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.internet_charge_type {
            params.push(("InternetChargeType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_id {
            params.push(("VpcId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("VSwitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.network_type {
            params.push(("NetworkType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.address {
            params.push(("Address".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.master_zone_id {
            params.push(("MasterZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.slave_zone_id {
            params.push(("SlaveZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            params.push(("Tags".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.pay_type {
            params.push(("PayType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
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
pub struct DescribeLoadBalancersResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 实例列表页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 当前分页的行数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 根据过滤条件得到的实例总个数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    #[serde(rename = "LoadBalancers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancers: Option<DescribeLoadBalancersResponseLoadBalancers>,
}

/// DeleteLoadBalancerListener 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteLoadBalancerListenerRequest {
    /// 传统型负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 传统型负载均衡实例的ID。
    #[serde(rename = "LoadBalancerId")]
    pub load_balancer_id: String,
    /// 传统型负载均衡实例前端使用的端口。
    #[serde(rename = "ListenerPort")]
    pub listener_port: i32,
    /// 传统型负载均衡实例前端使用的协议。
    #[serde(rename = "ListenerProtocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_protocol: Option<String>,
}

impl DeleteLoadBalancerListenerRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params.push(("LoadBalancerId".to_string(), self.load_balancer_id.to_string()));
        params.push(("ListenerPort".to_string(), self.listener_port.to_string()));
        if let Some(ref v) = self.listener_protocol {
            params.push(("ListenerProtocol".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteLoadBalancerListenerResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// StartLoadBalancerListener 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct StartLoadBalancerListenerRequest {
    /// 负载均衡实例的地域。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 负载均衡实例的ID。
    #[serde(rename = "LoadBalancerId")]
    pub load_balancer_id: String,
    /// 负载均衡实例前端使用的端口。
    #[serde(rename = "ListenerPort")]
    pub listener_port: i32,
    /// 负载均衡实例前端使用的协议。
    #[serde(rename = "ListenerProtocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_protocol: Option<String>,
}

impl StartLoadBalancerListenerRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params.push(("LoadBalancerId".to_string(), self.load_balancer_id.to_string()));
        params.push(("ListenerPort".to_string(), self.listener_port.to_string()));
        if let Some(ref v) = self.listener_protocol {
            params.push(("ListenerProtocol".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct StartLoadBalancerListenerResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// StopLoadBalancerListener 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct StopLoadBalancerListenerRequest {
    /// 负载均衡实例的地域。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 负载均衡实例的ID。
    #[serde(rename = "LoadBalancerId")]
    pub load_balancer_id: String,
    /// 负载均衡实例前端使用的端口。
    #[serde(rename = "ListenerPort")]
    pub listener_port: i32,
    /// 负载均衡实例前端使用的协议。
    #[serde(rename = "ListenerProtocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_protocol: Option<String>,
}

impl StopLoadBalancerListenerRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params.push(("LoadBalancerId".to_string(), self.load_balancer_id.to_string()));
        params.push(("ListenerPort".to_string(), self.listener_port.to_string()));
        if let Some(ref v) = self.listener_protocol {
            params.push(("ListenerProtocol".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct StopLoadBalancerListenerResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeLoadBalancerListeners 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeLoadBalancerListenersRequest {
    /// 传统型负载均衡实例所在的地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 是否拥有下一次查询的令牌（Token）。取值：
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 分批次查询时每次显示的条目数。
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// 传统型负载均衡监听协议。取值：
    #[serde(rename = "ListenerProtocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_protocol: Option<String>,
    /// 传统型负载均衡实例的ID列表，最多支持10个实例ID。
    #[serde(rename = "LoadBalancerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_id: Option<Vec<String>>,
    /// 标签列表。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<DescribeLoadBalancerListenersRequestTagItem>>,
    /// CLB实例使用的监听端口。
    #[serde(rename = "ListenerPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_port: Option<i32>,
    /// 自定义监听名称。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl DescribeLoadBalancerListenersRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.next_token {
            params.push(("NextToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_results {
            params.push(("MaxResults".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.listener_protocol {
            params.push(("ListenerProtocol".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.load_balancer_id {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("LoadBalancerId.{}", i + 1), item.to_string()));
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
        if let Some(ref v) = self.listener_port {
            params.push(("ListenerPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeLoadBalancerListenersResponse {
    /// 是否拥有下一次查询的令牌（Token）。取值：
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 本次请求条件下的数据总量。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 分批次查询时每次显示的条目数。
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// 返回传统型负载均衡的监听列表。
    #[serde(rename = "Listeners")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listeners: Option<Vec<DescribeLoadBalancerListenersResponseListenersItem>>,
}

/// CreateLoadBalancerTCPListener 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateLoadBalancerTCPListenerRequest {
    /// 传统型负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 传统型负载均衡实例的ID。
    #[serde(rename = "LoadBalancerId")]
    pub load_balancer_id: String,
    /// 传统型负载均衡实例前端使用的端口。
    #[serde(rename = "ListenerPort")]
    pub listener_port: i32,
    /// 传统型负载均衡实例后端使用的端口。
    #[serde(rename = "BackendServerPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_server_port: Option<i32>,
    /// 标签列表。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<CreateLoadBalancerTCPListenerRequestTagItem>>,
    /// 监听的带宽峰值，单位：Mbps。取值：
    #[serde(rename = "Bandwidth")]
    pub bandwidth: i32,
    /// 调度算法。取值：
    #[serde(rename = "Scheduler")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduler: Option<String>,
    /// 会话保持的超时时间。单位：秒。
    #[serde(rename = "PersistenceTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistence_timeout: Option<i32>,
    /// 连接超时时间。单位：秒。
    #[serde(rename = "EstablishedTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub established_timeout: Option<i32>,
    /// 健康检查连续成功多少次后，将后端服务器的健康检查状态由**fail**判定为**success**。
    #[serde(rename = "HealthyThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthy_threshold: Option<i32>,
    /// 健康检查连续失败多少次后，将后端服务器的健康检查状态由**success**判定为**fail**。
    #[serde(rename = "UnhealthyThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unhealthy_threshold: Option<i32>,
    /// 每次健康检查响应的最大超时时间。单位：秒。
    #[serde(rename = "HealthCheckConnectTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_connect_timeout: Option<i32>,
    /// 健康检查使用的端口。
    #[serde(rename = "HealthCheckConnectPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_connect_port: Option<i32>,
    /// 健康检查的时间间隔。单位：秒。
    #[serde(rename = "healthCheckInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_interval: Option<i32>,
    /// 用于健康检查的域名。取值：
    #[serde(rename = "HealthCheckDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_domain: Option<String>,
    /// 用于健康检查的URI。长度限制为1~80，只能使用字母、数字、短划线（-）、正斜线（/）、半角句号（.）、百分号（%）、井号（#）和and（&）这些字符。 URI不能只为正斜线（/），但必须以正...
    #[serde(rename = "HealthCheckURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_uri: Option<String>,
    /// 健康检查正常的HTTP状态码，多个状态码用半角逗号（,）分割。取值：
    #[serde(rename = "HealthCheckHttpCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_http_code: Option<String>,
    /// 健康检查类型。取值：
    #[serde(rename = "HealthCheckType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_type: Option<String>,
    /// 虚拟服务器组ID。
    #[serde(rename = "VServerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_server_group_id: Option<String>,
    /// 主备服务器组ID。
    #[serde(rename = "MasterSlaveServerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_slave_server_group_id: Option<String>,
    /// 监听绑定的访问策略组ID。
    #[serde(rename = "AclId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_id: Option<String>,
    /// 访问控制类型：
    #[serde(rename = "AclType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_type: Option<String>,
    /// 是否开启访问控制功能。取值：
    #[serde(rename = "AclStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_status: Option<String>,
    /// 自定义监听名称。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 是否开启连接优雅中断。取值：
    #[serde(rename = "ConnectionDrain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_drain: Option<String>,
    /// 设置连接优雅中断超时时间。单位：秒。
    #[serde(rename = "ConnectionDrainTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_drain_timeout: Option<i32>,
    /// 是否支持通过Proxy Protocol协议携带客户端源地址到后端服务器。取值：
    #[serde(rename = "ProxyProtocolV2Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_protocol_v2_enabled: Option<bool>,
    /// 是否开启健康检查。取值：
    #[serde(rename = "HealthCheckSwitch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_switch: Option<String>,
}

impl CreateLoadBalancerTCPListenerRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params.push(("LoadBalancerId".to_string(), self.load_balancer_id.to_string()));
        params.push(("ListenerPort".to_string(), self.listener_port.to_string()));
        if let Some(ref v) = self.backend_server_port {
            params.push(("BackendServerPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tag.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params.push(("Bandwidth".to_string(), self.bandwidth.to_string()));
        if let Some(ref v) = self.scheduler {
            params.push(("Scheduler".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.persistence_timeout {
            params.push(("PersistenceTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.established_timeout {
            params.push(("EstablishedTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.healthy_threshold {
            params.push(("HealthyThreshold".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.unhealthy_threshold {
            params.push(("UnhealthyThreshold".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_connect_timeout {
            params.push(("HealthCheckConnectTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_connect_port {
            params.push(("HealthCheckConnectPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_interval {
            params.push(("healthCheckInterval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_domain {
            params.push(("HealthCheckDomain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_uri {
            params.push(("HealthCheckURI".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_http_code {
            params.push(("HealthCheckHttpCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_type {
            params.push(("HealthCheckType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_server_group_id {
            params.push(("VServerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.master_slave_server_group_id {
            params.push(("MasterSlaveServerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.acl_id {
            params.push(("AclId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.acl_type {
            params.push(("AclType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.acl_status {
            params.push(("AclStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.connection_drain {
            params.push(("ConnectionDrain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.connection_drain_timeout {
            params.push(("ConnectionDrainTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.proxy_protocol_v2_enabled {
            params.push(("ProxyProtocolV2Enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_switch {
            params.push(("HealthCheckSwitch".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateLoadBalancerTCPListenerResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// SetLoadBalancerTCPListenerAttribute 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetLoadBalancerTCPListenerAttributeRequest {
    /// 传统型负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 传统型负载均衡实例ID。
    #[serde(rename = "LoadBalancerId")]
    pub load_balancer_id: String,
    /// 传统型负载均衡实例前端使用的端口。
    #[serde(rename = "ListenerPort")]
    pub listener_port: i32,
    /// 监听的带宽峰值。单位：Mbps。取值：**-1**或**1**~**5120**。
    #[serde(rename = "Bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<i32>,
    /// 调度算法。取值：
    #[serde(rename = "Scheduler")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduler: Option<String>,
    /// 会话保持的超时时间。单位：秒，取值范围：**0**~**3600**。
    #[serde(rename = "PersistenceTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistence_timeout: Option<i32>,
    /// 连接超时时间。单位：秒。取值范围：**10**~**900**。
    #[serde(rename = "EstablishedTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub established_timeout: Option<i32>,
    /// 健康检查连续成功多少次后，将后端服务器的健康检查状态由**失败**判定为**成功**。
    #[serde(rename = "HealthyThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthy_threshold: Option<i32>,
    /// 健康检查连续失败多少次后，将后端服务器的健康检查状态由**成功**判定为**失败**。
    #[serde(rename = "UnhealthyThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unhealthy_threshold: Option<i32>,
    /// 接收来自运行状况检查的响应需要等待的时间。单位：秒，取值范围：**1**~**300**。
    #[serde(rename = "HealthCheckConnectTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_connect_timeout: Option<i32>,
    /// 健康检查使用的端口。取值范围：**1**~**65535**。
    #[serde(rename = "HealthCheckConnectPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_connect_port: Option<i32>,
    /// 健康检查的时间间隔。单位：秒。
    #[serde(rename = "HealthCheckInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_interval: Option<i32>,
    /// 用于健康检查的域名。当TCP监听需要使用HTTP健康检查时可配置此参数，如不配置则按TCP健康检查。
    #[serde(rename = "HealthCheckDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_domain: Option<String>,
    /// 用于健康检查的URI。长度为1~80个字符，支持字母、数字和短划线（-）、正斜线（/）、半角句号（.）、百分号（%）、半角问号（?）、井号（#）和and（&）这些字符。URI不能只为正斜线（/）...
    #[serde(rename = "HealthCheckURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_uri: Option<String>,
    /// 健康检查正常的HTTP状态码，多个状态码用半角逗号（,）分割。
    #[serde(rename = "HealthCheckHttpCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_http_code: Option<String>,
    /// 健康检查类型。取值：**tcp**或**http**。
    #[serde(rename = "HealthCheckType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_type: Option<String>,
    /// 是否开启负载均衡的攻击防护功能SynProxy。取值：
    #[serde(rename = "SynProxy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub syn_proxy: Option<String>,
    /// 是否使用虚拟服务器组。取值：
    #[serde(rename = "VServerGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_server_group: Option<String>,
    /// 虚拟服务器组ID。
    #[serde(rename = "VServerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_server_group_id: Option<String>,
    /// 主备服务器组ID。
    #[serde(rename = "MasterSlaveServerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_slave_server_group_id: Option<String>,
    /// 是否使用主备服务器组。取值：
    #[serde(rename = "MasterSlaveServerGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_slave_server_group: Option<String>,
    /// 监听绑定的访问策略组ID。
    #[serde(rename = "AclId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_id: Option<String>,
    /// 访问控制类型。取值：
    #[serde(rename = "AclType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_type: Option<String>,
    /// 是否开启访问控制功能。取值：
    #[serde(rename = "AclStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_status: Option<String>,
    /// 自定义监听名称。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 是否开启连接优雅中断。取值：
    #[serde(rename = "ConnectionDrain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_drain: Option<String>,
    /// 设置连接优雅中断超时时间。当**ConnectionDrain**取值为**on**时，该选项必选。单位：秒。
    #[serde(rename = "ConnectionDrainTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_drain_timeout: Option<i32>,
    /// 是否支持通过Proxy Protocol协议携带客户端源地址到后端服务器。取值：
    #[serde(rename = "ProxyProtocolV2Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_protocol_v2_enabled: Option<bool>,
    /// 是否开启健康检查。取值：
    #[serde(rename = "HealthCheckSwitch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_switch: Option<String>,
}

impl SetLoadBalancerTCPListenerAttributeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params.push(("LoadBalancerId".to_string(), self.load_balancer_id.to_string()));
        params.push(("ListenerPort".to_string(), self.listener_port.to_string()));
        if let Some(ref v) = self.bandwidth {
            params.push(("Bandwidth".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scheduler {
            params.push(("Scheduler".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.persistence_timeout {
            params.push(("PersistenceTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.established_timeout {
            params.push(("EstablishedTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.healthy_threshold {
            params.push(("HealthyThreshold".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.unhealthy_threshold {
            params.push(("UnhealthyThreshold".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_connect_timeout {
            params.push(("HealthCheckConnectTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_connect_port {
            params.push(("HealthCheckConnectPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_interval {
            params.push(("HealthCheckInterval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_domain {
            params.push(("HealthCheckDomain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_uri {
            params.push(("HealthCheckURI".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_http_code {
            params.push(("HealthCheckHttpCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_type {
            params.push(("HealthCheckType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.syn_proxy {
            params.push(("SynProxy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_server_group {
            params.push(("VServerGroup".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_server_group_id {
            params.push(("VServerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.master_slave_server_group_id {
            params.push(("MasterSlaveServerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.master_slave_server_group {
            params.push(("MasterSlaveServerGroup".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.acl_id {
            params.push(("AclId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.acl_type {
            params.push(("AclType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.acl_status {
            params.push(("AclStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.connection_drain {
            params.push(("ConnectionDrain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.connection_drain_timeout {
            params.push(("ConnectionDrainTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.proxy_protocol_v2_enabled {
            params.push(("ProxyProtocolV2Enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_switch {
            params.push(("HealthCheckSwitch".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetLoadBalancerTCPListenerAttributeResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeLoadBalancerTCPListenerAttribute 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeLoadBalancerTCPListenerAttributeRequest {
    /// 传统型负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 传统型负载均衡实例ID。
    #[serde(rename = "LoadBalancerId")]
    pub load_balancer_id: String,
    /// 传统型负载均衡实例前端使用的端口。
    #[serde(rename = "ListenerPort")]
    pub listener_port: i32,
}

impl DescribeLoadBalancerTCPListenerAttributeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params.push(("LoadBalancerId".to_string(), self.load_balancer_id.to_string()));
        params.push(("ListenerPort".to_string(), self.listener_port.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeLoadBalancerTCPListenerAttributeResponse {
    /// 绑定的服务器组ID。
    #[serde(rename = "VServerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_server_group_id: Option<String>,
    /// 当前监听的状态。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 访问控制类型：
    #[serde(rename = "AclType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_type: Option<String>,
    /// 设置连接优雅中断超时时间。当**ConnectionDrain**取值为**on**时，返回该值。
    #[serde(rename = "ConnectionDrainTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_drain_timeout: Option<i32>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 健康检查使用的端口。取值：**1**~**65535**。不设置此参数时，表示使用后端服务端口（BackendServerPort）。
    #[serde(rename = "HealthCheckConnectPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_connect_port: Option<i32>,
    /// 自定义监听名称。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 监听的带宽峰值，单位Mbps。取值：
    #[serde(rename = "Bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<i32>,
    /// TCP协议监听的健康检查方式。
    #[serde(rename = "HealthCheckType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_type: Option<String>,
    /// 绑定的主备服务器组ID。
    #[serde(rename = "MasterSlaveServerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_slave_server_group_id: Option<String>,
    /// 传统型负载均衡实例后端使用的端口。
    #[serde(rename = "BackendServerPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_server_port: Option<i32>,
    /// 是否开启访问控制功能。取值：
    #[serde(rename = "AclStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_status: Option<String>,
    /// 用于健康检查的域名。取值：
    #[serde(rename = "HealthCheckDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_domain: Option<String>,
    /// 不健康检查阈值。健康检查连续失败多少次后，将后端服务器的健康检查状态由**成功**判定为**失败**。取值：**2**~**10**。
    #[serde(rename = "UnhealthyThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unhealthy_threshold: Option<i32>,
    /// 健康检查正常的HTTP状态码。
    #[serde(rename = "HealthCheckHttpCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_http_code: Option<String>,
    /// 是否支持通过Proxy Protocol协议携带客户端源地址到后端服务器。取值：
    #[serde(rename = "ProxyProtocolV2Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_protocol_v2_enabled: Option<bool>,
    /// 会话保持的超时时间。
    #[serde(rename = "PersistenceTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistence_timeout: Option<i32>,
    /// 传统型负载均衡实例前端使用的端口。
    #[serde(rename = "ListenerPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_port: Option<i32>,
    /// 健康检查的时间间隔，取值：**1**~**50**秒。
    #[serde(rename = "HealthCheckInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_interval: Option<i32>,
    /// 用于健康检查的URL。长度限制为1~80个字符，只能使用字母、数字和短划线（-）、正斜线（/）、半角句号（.）、百分号（%）、半角问号（?）、井号（#）和and（&amp;）这些字符。 URL不...
    #[serde(rename = "HealthCheckURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_uri: Option<String>,
    /// 监听绑定的访问策略组ID。
    #[serde(rename = "AclId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_id: Option<String>,
    /// 是否开启负载均衡的攻击防护功能SynProxy。
    #[serde(rename = "SynProxy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub syn_proxy: Option<String>,
    /// 调度算法。取值：
    #[serde(rename = "Scheduler")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduler: Option<String>,
    /// 连接超时时间。
    #[serde(rename = "EstablishedTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub established_timeout: Option<i32>,
    /// 超时时间。
    #[serde(rename = "HealthCheckConnectTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_connect_timeout: Option<i32>,
    /// 健康检查阈值。健康检查连续成功多少次后，将后端服务器的健康检查状态由**失败**判定为**成功**。取值：**2**~**10**。
    #[serde(rename = "HealthyThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthy_threshold: Option<i32>,
    /// 是否开启连接优雅中断。当**ConnectionDrain**取值为**on**时，返回该值。取值：
    #[serde(rename = "ConnectionDrain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_drain: Option<String>,
    /// 健康检查方式。
    #[serde(rename = "HealthCheckMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_method: Option<String>,
    /// 是否开启健康检查。取值：
    #[serde(rename = "HealthCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<String>,
    #[serde(rename = "AclIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_ids: Option<DescribeLoadBalancerTCPListenerAttributeResponseAclIds>,
    /// 传统型负载均衡实例ID。
    #[serde(rename = "LoadBalancerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<DescribeLoadBalancerTCPListenerAttributeResponseTags>,
}

/// CreateLoadBalancerUDPListener 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateLoadBalancerUDPListenerRequest {
    /// 传统型负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 监听的标签集合。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<CreateLoadBalancerUDPListenerRequestTagItem>>,
    /// 传统型负载均衡实例的ID。
    #[serde(rename = "LoadBalancerId")]
    pub load_balancer_id: String,
    /// 传统型负载均衡实例前端使用的端口。
    #[serde(rename = "ListenerPort")]
    pub listener_port: i32,
    /// 传统型负载均衡实例后端使用的端口。
    #[serde(rename = "BackendServerPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_server_port: Option<i32>,
    /// 监听的带宽峰值。单位：Mbps。取值：
    #[serde(rename = "Bandwidth")]
    pub bandwidth: i32,
    /// 调度算法。取值：
    #[serde(rename = "Scheduler")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduler: Option<String>,
    /// 健康检查连续成功多少次后，将后端服务器的健康检查状态由**fail**判定为**success**。
    #[serde(rename = "HealthyThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthy_threshold: Option<i32>,
    /// 健康检查连续失败多少次后，将后端服务器的健康检查状态由**success**判定为**fail**。
    #[serde(rename = "UnhealthyThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unhealthy_threshold: Option<i32>,
    /// 接收来自运行状况检查的响应需要等待的时间。
    #[serde(rename = "HealthCheckConnectTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_connect_timeout: Option<i32>,
    /// 健康检查使用的端口。
    #[serde(rename = "HealthCheckConnectPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_connect_port: Option<i32>,
    /// 健康检查的时间间隔。单位：秒。
    #[serde(rename = "healthCheckInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_interval: Option<i32>,
    /// UDP监听健康检查的请求字符串，只允许包含字母、数字，最大长度限制为64个字符。
    #[serde(rename = "healthCheckReq")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_req: Option<String>,
    /// UDP监听健康检查的响应字符串，只允许包含字母、数字，最大长度限制为64个字符。
    #[serde(rename = "healthCheckExp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_exp: Option<String>,
    /// 虚拟服务器组ID。
    #[serde(rename = "VServerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_server_group_id: Option<String>,
    /// 主备服务器组ID。
    #[serde(rename = "MasterSlaveServerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_slave_server_group_id: Option<String>,
    /// 监听绑定的访问策略组ID。
    #[serde(rename = "AclId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_id: Option<String>,
    /// 访问控制类型，取值：
    #[serde(rename = "AclType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_type: Option<String>,
    /// 是否开启访问控制功能。取值：
    #[serde(rename = "AclStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_status: Option<String>,
    /// 自定义监听名称。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 是否支持通过Proxy Protocol协议携带客户端源地址到后端服务器。取值：
    #[serde(rename = "ProxyProtocolV2Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_protocol_v2_enabled: Option<bool>,
    /// 是否开启健康检查。取值：
    #[serde(rename = "HealthCheckSwitch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_switch: Option<String>,
}

impl CreateLoadBalancerUDPListenerRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tag.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params.push(("LoadBalancerId".to_string(), self.load_balancer_id.to_string()));
        params.push(("ListenerPort".to_string(), self.listener_port.to_string()));
        if let Some(ref v) = self.backend_server_port {
            params.push(("BackendServerPort".to_string(), v.to_string()));
        }
        params.push(("Bandwidth".to_string(), self.bandwidth.to_string()));
        if let Some(ref v) = self.scheduler {
            params.push(("Scheduler".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.healthy_threshold {
            params.push(("HealthyThreshold".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.unhealthy_threshold {
            params.push(("UnhealthyThreshold".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_connect_timeout {
            params.push(("HealthCheckConnectTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_connect_port {
            params.push(("HealthCheckConnectPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_interval {
            params.push(("healthCheckInterval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_req {
            params.push(("healthCheckReq".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_exp {
            params.push(("healthCheckExp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_server_group_id {
            params.push(("VServerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.master_slave_server_group_id {
            params.push(("MasterSlaveServerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.acl_id {
            params.push(("AclId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.acl_type {
            params.push(("AclType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.acl_status {
            params.push(("AclStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.proxy_protocol_v2_enabled {
            params.push(("ProxyProtocolV2Enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_switch {
            params.push(("HealthCheckSwitch".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateLoadBalancerUDPListenerResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// SetLoadBalancerUDPListenerAttribute 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetLoadBalancerUDPListenerAttributeRequest {
    /// 传统型负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 传统型负载均衡实例的ID。
    #[serde(rename = "LoadBalancerId")]
    pub load_balancer_id: String,
    /// 传统型负载均衡实例前端使用的端口。
    #[serde(rename = "ListenerPort")]
    pub listener_port: i32,
    /// 监听的带宽峰值，单位：Mbps。取值：
    #[serde(rename = "Bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<i32>,
    /// 调度算法。取值：
    #[serde(rename = "Scheduler")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduler: Option<String>,
    /// 健康检查连续成功多少次后，将后端服务器的健康检查状态由**fail**判定为**success**。
    #[serde(rename = "HealthyThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthy_threshold: Option<i32>,
    /// 健康检查连续失败多少次后，将后端服务器的健康检查状态由**success**判定为**fail**。
    #[serde(rename = "UnhealthyThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unhealthy_threshold: Option<i32>,
    /// 接收来自运行状况检查的响应需要等待的时间。如果后端ECS在指定的时间内没有正确响应，则判定为健康检查失败。单位：秒。
    #[serde(rename = "HealthCheckConnectTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_connect_timeout: Option<i32>,
    /// 健康检查使用的端口。
    #[serde(rename = "HealthCheckConnectPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_connect_port: Option<i32>,
    /// 健康检查的时间间隔，单位：秒。
    #[serde(rename = "HealthCheckInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_interval: Option<i32>,
    /// UDP监听健康检查的请求串，只允许包含字母、数字，最大长度限制为64个字符。
    #[serde(rename = "healthCheckReq")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_req: Option<String>,
    /// UDP监听健康检查的响应串，只允许包含字母、数字，最大长度限制为64个字符。
    #[serde(rename = "healthCheckExp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_exp: Option<String>,
    /// 是否使用虚拟服务器组。取值：
    #[serde(rename = "VServerGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_server_group: Option<String>,
    /// 虚拟服务器组ID。
    #[serde(rename = "VServerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_server_group_id: Option<String>,
    /// 主备服务器组ID。
    #[serde(rename = "MasterSlaveServerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_slave_server_group_id: Option<String>,
    /// 是否使用主备服务器组。取值：
    #[serde(rename = "MasterSlaveServerGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_slave_server_group: Option<String>,
    /// 监听绑定的访问策略组ID。
    #[serde(rename = "AclId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_id: Option<String>,
    /// 访问控制类型。取值：
    #[serde(rename = "AclType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_type: Option<String>,
    /// 是否开启访问控制功能。取值：
    #[serde(rename = "AclStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_status: Option<String>,
    /// 自定义监听名称。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 是否支持通过Proxy Protocol协议携带客户端源地址到后端服务器。取值：
    #[serde(rename = "ProxyProtocolV2Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_protocol_v2_enabled: Option<bool>,
    /// 是否开启健康检查。取值：
    #[serde(rename = "HealthCheckSwitch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_switch: Option<String>,
}

impl SetLoadBalancerUDPListenerAttributeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params.push(("LoadBalancerId".to_string(), self.load_balancer_id.to_string()));
        params.push(("ListenerPort".to_string(), self.listener_port.to_string()));
        if let Some(ref v) = self.bandwidth {
            params.push(("Bandwidth".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scheduler {
            params.push(("Scheduler".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.healthy_threshold {
            params.push(("HealthyThreshold".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.unhealthy_threshold {
            params.push(("UnhealthyThreshold".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_connect_timeout {
            params.push(("HealthCheckConnectTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_connect_port {
            params.push(("HealthCheckConnectPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_interval {
            params.push(("HealthCheckInterval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_req {
            params.push(("healthCheckReq".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_exp {
            params.push(("healthCheckExp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_server_group {
            params.push(("VServerGroup".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_server_group_id {
            params.push(("VServerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.master_slave_server_group_id {
            params.push(("MasterSlaveServerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.master_slave_server_group {
            params.push(("MasterSlaveServerGroup".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.acl_id {
            params.push(("AclId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.acl_type {
            params.push(("AclType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.acl_status {
            params.push(("AclStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.proxy_protocol_v2_enabled {
            params.push(("ProxyProtocolV2Enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_switch {
            params.push(("HealthCheckSwitch".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetLoadBalancerUDPListenerAttributeResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeLoadBalancerUDPListenerAttribute 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeLoadBalancerUDPListenerAttributeRequest {
    /// 负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 传统型负载均衡实例ID。
    #[serde(rename = "LoadBalancerId")]
    pub load_balancer_id: String,
    /// 传统型负载均衡实例前端使用的端口。
    #[serde(rename = "ListenerPort")]
    pub listener_port: i32,
}

impl DescribeLoadBalancerUDPListenerAttributeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params.push(("LoadBalancerId".to_string(), self.load_balancer_id.to_string()));
        params.push(("ListenerPort".to_string(), self.listener_port.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeLoadBalancerUDPListenerAttributeResponse {
    /// 绑定的虚拟服务器组ID。
    #[serde(rename = "VServerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_server_group_id: Option<String>,
    /// 当前监听的状态。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 访问控制类型：
    #[serde(rename = "AclType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_type: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 健康检查的端口。取值：**1**~**65535**。不设置此参数时，表示使用后端服务端口（BackendServerPort）。
    #[serde(rename = "HealthCheckConnectPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_connect_port: Option<i32>,
    /// 自定义监听名称。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 监听的带宽峰值，单位Mbps。取值：
    #[serde(rename = "Bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<i32>,
    /// 绑定的主备服务器组ID。
    #[serde(rename = "MasterSlaveServerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_slave_server_group_id: Option<String>,
    /// 传统型负载均衡实例后端使用的端口。
    #[serde(rename = "BackendServerPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_server_port: Option<i32>,
    /// 是否开启访问控制功能。取值：**on**或**off**（默认值）。
    #[serde(rename = "AclStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_status: Option<String>,
    /// 不健康检查阈值。健康检查连续失败多少次后，将后端服务器的健康检查状态由**成功**判定为**失败**。取值：**2**~**10**。
    #[serde(rename = "UnhealthyThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unhealthy_threshold: Option<i32>,
    /// 是否支持通过Proxy Protocol协议携带客户端源地址到后端服务器。取值：
    #[serde(rename = "ProxyProtocolV2Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_protocol_v2_enabled: Option<bool>,
    /// 传统型负载均衡实例前端使用的端口。
    #[serde(rename = "ListenerPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_port: Option<i32>,
    /// 健康检查的时间间隔，取值：**1**~**50**秒。
    #[serde(rename = "HealthCheckInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_interval: Option<i32>,
    /// UDP监听健康检查的响应串。只允许包含字母、数字，最大长度限制为64个字符。
    #[serde(rename = "HealthCheckExp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_exp: Option<String>,
    /// 访问控制策略组ID。
    #[serde(rename = "AclId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_id: Option<String>,
    /// 调度算法，取值：
    #[serde(rename = "Scheduler")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduler: Option<String>,
    /// 接收来自运行状况检查的响应需要等待的时间。如果后端ECS在指定的时间内没有正确响应，则判定为健康检查失败。取值：**1**~**300**秒。
    #[serde(rename = "HealthCheckConnectTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_connect_timeout: Option<i32>,
    /// 健康检查阈值。健康检查连续成功多少次后，将后端服务器的健康检查状态由**失败**判定为**成功**。取值：**2**~**10**。
    #[serde(rename = "HealthyThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthy_threshold: Option<i32>,
    /// UDP监听健康检查的请求串。只允许包含字母、数字，最大长度限制为64个字符。
    #[serde(rename = "HealthCheckReq")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_req: Option<String>,
    /// 是否开启健康检查。取值：
    #[serde(rename = "HealthCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<String>,
    #[serde(rename = "AclIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_ids: Option<DescribeLoadBalancerUDPListenerAttributeResponseAclIds>,
    /// 传统型负载均衡实例ID。
    #[serde(rename = "LoadBalancerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<DescribeLoadBalancerUDPListenerAttributeResponseTags>,
}

/// CreateLoadBalancerHTTPListener 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateLoadBalancerHTTPListenerRequest {
    /// 传统型负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 传统型负载均衡实例ID。
    #[serde(rename = "LoadBalancerId")]
    pub load_balancer_id: String,
    /// 监听的带宽峰值，单位：Mbps。取值：
    #[serde(rename = "Bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<i32>,
    /// 实例前端使用的端口。
    #[serde(rename = "ListenerPort")]
    pub listener_port: i32,
    /// 实例后端使用的端口。
    #[serde(rename = "BackendServerPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_server_port: Option<i32>,
    /// 是否开启通过`X-Forwarded-For`头字段获取来访客户端IP。取值：
    #[serde(rename = "XForwardedFor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for: Option<String>,
    /// 调度算法。取值：
    #[serde(rename = "Scheduler")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduler: Option<String>,
    /// 是否开启会话保持。取值：
    #[serde(rename = "StickySession")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticky_session: Option<String>,
    /// Cookie处理方式。取值：
    #[serde(rename = "StickySessionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticky_session_type: Option<String>,
    /// 标签列表。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<CreateLoadBalancerHTTPListenerRequestTagItem>>,
    /// Cookie超时时间。单位：秒。
    #[serde(rename = "CookieTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie_timeout: Option<i32>,
    /// 服务器上配置的Cookie。
    #[serde(rename = "Cookie")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie: Option<String>,
    /// 是否开启健康检查。取值：
    #[serde(rename = "HealthCheck")]
    pub health_check: String,
    /// 监听HTTP类型健康检查的健康检查方法。取值：**head**或**get**。
    #[serde(rename = "HealthCheckMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_method: Option<String>,
    /// 用于健康检查的域名，取值：
    #[serde(rename = "HealthCheckDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_domain: Option<String>,
    /// 用于健康检查的URI。
    #[serde(rename = "HealthCheckURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_uri: Option<String>,
    /// 健康检查连续成功多少次后，将后端服务器的健康检查状态由**失败**判定为**成功**。
    #[serde(rename = "HealthyThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthy_threshold: Option<i32>,
    /// 健康检查连续失败多少次后，将后端服务器的健康检查状态由**成功**判定为**失败**。
    #[serde(rename = "UnhealthyThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unhealthy_threshold: Option<i32>,
    /// 接收来自运行状况检查的响应需要等待的时间。如果后端ECS在指定的时间内没有正确响应，则判定为健康检查失败。单位：秒。
    #[serde(rename = "HealthCheckTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_timeout: Option<i32>,
    /// 健康检查的后端服务器的端口。
    #[serde(rename = "HealthCheckConnectPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_connect_port: Option<i32>,
    /// 健康检查的时间间隔。单位：秒。
    #[serde(rename = "HealthCheckInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_interval: Option<i32>,
    /// 健康检查正常的HTTP状态码，多个状态码用逗号分隔。
    #[serde(rename = "HealthCheckHttpCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_http_code: Option<String>,
    /// 虚拟服务器组ID。
    #[serde(rename = "VServerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_server_group_id: Option<String>,
    /// 是否通过`SLB-IP`头字段获取客户端请求的VIP（Virtual IP address）。取值：
    #[serde(rename = "XForwardedFor_SLBIP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_slbip: Option<String>,
    /// 是否通过`SLB-ID`头字段获取负载均衡实例ID。取值：
    #[serde(rename = "XForwardedFor_SLBID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_slbid: Option<String>,
    /// 是否通过`X-Forwarded-Proto`头字段获取负载均衡实例的监听协议。取值：
    #[serde(rename = "XForwardedFor_proto")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_proto: Option<String>,
    /// 是否开启`Gzip`压缩，对特定文件类型进行压缩。取值：
    #[serde(rename = "Gzip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gzip: Option<String>,
    /// 监听绑定的访问策略组ID。
    #[serde(rename = "AclId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_id: Option<String>,
    /// 访问控制类型。取值：
    #[serde(rename = "AclType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_type: Option<String>,
    /// 是否开启访问控制功能。取值：
    #[serde(rename = "AclStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_status: Option<String>,
    /// 自定义监听名称。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 是否开启HTTP至HTTPS的转发。取值：
    #[serde(rename = "ListenerForward")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_forward: Option<String>,
    /// HTTP至HTTPS的监听转发端口。
    #[serde(rename = "ForwardPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_port: Option<i32>,
    /// 指定连接空闲超时时间。单位：秒。
    #[serde(rename = "IdleTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_timeout: Option<i32>,
    /// 指定请求超时时间。单位：秒。
    #[serde(rename = "RequestTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_timeout: Option<i32>,
    /// 是否通过`XForwardedFor_SLBPORT`头字段获取负载均衡实例的监听端口。取值：
    #[serde(rename = "XForwardedFor_SLBPORT")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_slbport: Option<String>,
    /// 是否通过`XForwardedFor_ClientSrcPort`头字段获取访问负载均衡实例客户端的端口。取值：
    #[serde(rename = "XForwardedFor_ClientSrcPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_client_src_port: Option<String>,
}

impl CreateLoadBalancerHTTPListenerRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params.push(("LoadBalancerId".to_string(), self.load_balancer_id.to_string()));
        if let Some(ref v) = self.bandwidth {
            params.push(("Bandwidth".to_string(), v.to_string()));
        }
        params.push(("ListenerPort".to_string(), self.listener_port.to_string()));
        if let Some(ref v) = self.backend_server_port {
            params.push(("BackendServerPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.x_forwarded_for {
            params.push(("XForwardedFor".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scheduler {
            params.push(("Scheduler".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sticky_session {
            params.push(("StickySession".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sticky_session_type {
            params.push(("StickySessionType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tag.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.cookie_timeout {
            params.push(("CookieTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cookie {
            params.push(("Cookie".to_string(), v.to_string()));
        }
        params.push(("HealthCheck".to_string(), self.health_check.to_string()));
        if let Some(ref v) = self.health_check_method {
            params.push(("HealthCheckMethod".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_domain {
            params.push(("HealthCheckDomain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_uri {
            params.push(("HealthCheckURI".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.healthy_threshold {
            params.push(("HealthyThreshold".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.unhealthy_threshold {
            params.push(("UnhealthyThreshold".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_timeout {
            params.push(("HealthCheckTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_connect_port {
            params.push(("HealthCheckConnectPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_interval {
            params.push(("HealthCheckInterval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_http_code {
            params.push(("HealthCheckHttpCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_server_group_id {
            params.push(("VServerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.x_forwarded_for_slbip {
            params.push(("XForwardedFor_SLBIP".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.x_forwarded_for_slbid {
            params.push(("XForwardedFor_SLBID".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.x_forwarded_for_proto {
            params.push(("XForwardedFor_proto".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.gzip {
            params.push(("Gzip".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.acl_id {
            params.push(("AclId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.acl_type {
            params.push(("AclType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.acl_status {
            params.push(("AclStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.listener_forward {
            params.push(("ListenerForward".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.forward_port {
            params.push(("ForwardPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.idle_timeout {
            params.push(("IdleTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.request_timeout {
            params.push(("RequestTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.x_forwarded_for_slbport {
            params.push(("XForwardedFor_SLBPORT".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.x_forwarded_for_client_src_port {
            params.push(("XForwardedFor_ClientSrcPort".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateLoadBalancerHTTPListenerResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// SetLoadBalancerHTTPListenerAttribute 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetLoadBalancerHTTPListenerAttributeRequest {
    /// 负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 负载均衡实例的ID。
    #[serde(rename = "LoadBalancerId")]
    pub load_balancer_id: String,
    /// 负载均衡实例前端使用的端口。
    #[serde(rename = "ListenerPort")]
    pub listener_port: i32,
    /// 监听的带宽峰值。单位：Mbps。取值：
    #[serde(rename = "Bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<i32>,
    /// 是否通过`X-Forwarded-For`头字段获取客户端请求的真实IP。取值：
    #[serde(rename = "XForwardedFor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for: Option<String>,
    /// 调度算法。取值：
    #[serde(rename = "Scheduler")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduler: Option<String>,
    /// 是否开启会话保持。取值：
    #[serde(rename = "StickySession")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticky_session: Option<String>,
    /// Cookie的处理方式。取值：
    #[serde(rename = "StickySessionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticky_session_type: Option<String>,
    /// Cookie超时时间。
    #[serde(rename = "CookieTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie_timeout: Option<i32>,
    /// 服务器上配置的Cookie。
    #[serde(rename = "Cookie")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie: Option<String>,
    /// 是否开启健康检查。取值：
    #[serde(rename = "HealthCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<String>,
    /// 监听HTTP类型健康检查的健康检查方法。取值：**head**或**get**。
    #[serde(rename = "HealthCheckMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_method: Option<String>,
    /// 用于健康检查的域名，取值：
    #[serde(rename = "HealthCheckDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_domain: Option<String>,
    /// 用于健康检查的URI。
    #[serde(rename = "HealthCheckURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_uri: Option<String>,
    /// 健康检查连续成功多少次后，将后端服务器的健康检查状态由**fail**判定为**success**。
    #[serde(rename = "HealthyThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthy_threshold: Option<i32>,
    /// 健康检查连续失败多少次后，将后端服务器的健康检查状态由**success**判定为**fail**。
    #[serde(rename = "UnhealthyThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unhealthy_threshold: Option<i32>,
    /// 接收来自运行状况检查的响应需要等待的时间。如果后端ECS在指定的时间内没有正确响应，则判定为健康检查失败。在**HealthCheck**值为**on**时才会有效。
    #[serde(rename = "HealthCheckTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_timeout: Option<i32>,
    /// 健康检查的时间间隔。
    #[serde(rename = "HealthCheckInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_interval: Option<i32>,
    /// 健康检查使用的端口。
    #[serde(rename = "HealthCheckConnectPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_connect_port: Option<i32>,
    /// 健康检查正常的HTTP状态码，多个状态码用半角逗号（,）分割。
    #[serde(rename = "HealthCheckHttpCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_http_code: Option<String>,
    /// 是否使用虚拟服务器组。取值：
    #[serde(rename = "VServerGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_server_group: Option<String>,
    /// 虚拟服务器组ID。
    #[serde(rename = "VServerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_server_group_id: Option<String>,
    /// 是否通过`SLB-IP`头字段获取客户端请求的VIP（Virtual IP address）。取值：
    #[serde(rename = "XForwardedFor_SLBIP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_slbip: Option<String>,
    /// 是否通过`SLB-ID`头字段获取负载均衡实例ID。取值：
    #[serde(rename = "XForwardedFor_SLBID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_slbid: Option<String>,
    /// 是否通过`X-Forwarded-Proto`头字段获取负载均衡实例的监听协议。取值：
    #[serde(rename = "XForwardedFor_proto")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_proto: Option<String>,
    /// 是否开启`Gzip`压缩，对特定文件类型进行压缩。取值：
    #[serde(rename = "Gzip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gzip: Option<String>,
    /// 监听绑定的访问策略组ID。
    #[serde(rename = "AclId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_id: Option<String>,
    /// 访问控制类型：
    #[serde(rename = "AclType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_type: Option<String>,
    /// 是否开启访问控制功能。取值：
    #[serde(rename = "AclStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_status: Option<String>,
    /// 指定连接空闲超时时间，取值范围为**1~60**秒，默认值为**15**秒。
    #[serde(rename = "IdleTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_timeout: Option<i32>,
    /// 指定请求超时时间，取值范围为**1~180**秒，默认值为**60**秒。
    #[serde(rename = "RequestTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_timeout: Option<i32>,
    /// 设置监听的描述信息。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 是否通过`XForwardedFor_SLBPORT`头字段获取负载均衡实例的监听端口。取值：
    #[serde(rename = "XForwardedFor_SLBPORT")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_slbport: Option<String>,
    /// 是否通过`XForwardedFor_ClientSrcPort`头字段获取访问负载均衡实例客户端的端口。取值：
    #[serde(rename = "XForwardedFor_ClientSrcPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_client_src_port: Option<String>,
}

impl SetLoadBalancerHTTPListenerAttributeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params.push(("LoadBalancerId".to_string(), self.load_balancer_id.to_string()));
        params.push(("ListenerPort".to_string(), self.listener_port.to_string()));
        if let Some(ref v) = self.bandwidth {
            params.push(("Bandwidth".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.x_forwarded_for {
            params.push(("XForwardedFor".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scheduler {
            params.push(("Scheduler".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sticky_session {
            params.push(("StickySession".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sticky_session_type {
            params.push(("StickySessionType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cookie_timeout {
            params.push(("CookieTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cookie {
            params.push(("Cookie".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check {
            params.push(("HealthCheck".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_method {
            params.push(("HealthCheckMethod".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_domain {
            params.push(("HealthCheckDomain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_uri {
            params.push(("HealthCheckURI".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.healthy_threshold {
            params.push(("HealthyThreshold".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.unhealthy_threshold {
            params.push(("UnhealthyThreshold".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_timeout {
            params.push(("HealthCheckTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_interval {
            params.push(("HealthCheckInterval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_connect_port {
            params.push(("HealthCheckConnectPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_http_code {
            params.push(("HealthCheckHttpCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_server_group {
            params.push(("VServerGroup".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_server_group_id {
            params.push(("VServerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.x_forwarded_for_slbip {
            params.push(("XForwardedFor_SLBIP".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.x_forwarded_for_slbid {
            params.push(("XForwardedFor_SLBID".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.x_forwarded_for_proto {
            params.push(("XForwardedFor_proto".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.gzip {
            params.push(("Gzip".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.acl_id {
            params.push(("AclId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.acl_type {
            params.push(("AclType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.acl_status {
            params.push(("AclStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.idle_timeout {
            params.push(("IdleTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.request_timeout {
            params.push(("RequestTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.x_forwarded_for_slbport {
            params.push(("XForwardedFor_SLBPORT".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.x_forwarded_for_client_src_port {
            params.push(("XForwardedFor_ClientSrcPort".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetLoadBalancerHTTPListenerAttributeResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeLoadBalancerHTTPListenerAttribute 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeLoadBalancerHTTPListenerAttributeRequest {
    /// 传统型负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 传统型负载均衡实例的ID。
    #[serde(rename = "LoadBalancerId")]
    pub load_balancer_id: String,
    /// 实例前端使用的端口。
    #[serde(rename = "ListenerPort")]
    pub listener_port: i32,
}

impl DescribeLoadBalancerHTTPListenerAttributeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params.push(("LoadBalancerId".to_string(), self.load_balancer_id.to_string()));
        params.push(("ListenerPort".to_string(), self.listener_port.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeLoadBalancerHTTPListenerAttributeResponse {
    /// 访问控制类型：
    #[serde(rename = "AclType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_type: Option<String>,
    /// 绑定的服务器组ID。
    #[serde(rename = "VServerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_server_group_id: Option<String>,
    /// 当前监听的状态。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 是否通过`XForwardedFor_ClientSrcPort`头字段获取访问负载均衡实例客户端的端口。取值：
    #[serde(rename = "XForwardedFor_ClientSrcPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_client_src_port: Option<String>,
    /// 服务器上配置的Cookie。
    #[serde(rename = "Cookie")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie: Option<String>,
    /// 是否开启`Gzip`压缩，对特定文件类型进行压缩。取值：
    #[serde(rename = "Gzip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gzip: Option<String>,
    /// 健康检查的端口。
    #[serde(rename = "HealthCheckConnectPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_connect_port: Option<i32>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 自定义监听名称。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 监听的带宽峰值。单位：Mbps。
    #[serde(rename = "Bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<i32>,
    /// 每次健康检查响应的超时时间，单位：秒。
    #[serde(rename = "HealthCheckTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_timeout: Option<i32>,
    /// 是否开启访问控制功能。取值：
    #[serde(rename = "AclStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_status: Option<String>,
    /// 实例后端使用的端口。
    #[serde(rename = "BackendServerPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_server_port: Option<i32>,
    /// Cookie超时时间。单位：秒。
    #[serde(rename = "CookieTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie_timeout: Option<i32>,
    /// 用于健康检查的域名。
    #[serde(rename = "HealthCheckDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_domain: Option<String>,
    /// 不健康检查阈值。
    #[serde(rename = "UnhealthyThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unhealthy_threshold: Option<i32>,
    /// HTTP至HTTPS的监听转发端口。
    #[serde(rename = "ForwardPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_port: Option<i32>,
    /// 是否通过`SLB-ID`头字段获取负载均衡实例ID。取值：
    #[serde(rename = "XForwardedFor_SLBID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_slbid: Option<String>,
    /// 是否开启安全状态。取值：
    #[serde(rename = "SecurityStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_status: Option<String>,
    /// 健康检查正常的HTTP状态码。
    #[serde(rename = "HealthCheckHttpCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_http_code: Option<String>,
    /// 是否开启HTTP至HTTPS的监听转发。取值：
    #[serde(rename = "ListenerForward")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_forward: Option<String>,
    /// 是否开启通过`X-Forwarded-For`头字段的方式获取来访者真实IP。取值：
    #[serde(rename = "XForwardedFor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for: Option<String>,
    /// 指定连接空闲超时时间，单位：秒。
    #[serde(rename = "IdleTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_timeout: Option<i32>,
    /// 指定请求超时时间。单位：秒。
    #[serde(rename = "RequestTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_timeout: Option<i32>,
    /// 实例前端使用的端口。
    #[serde(rename = "ListenerPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_port: Option<i32>,
    /// 健康检查的时间间隔，单位：秒。
    #[serde(rename = "HealthCheckInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_interval: Option<i32>,
    /// 是否通过`XForwardedFor_SLBPORT`头字段获取负载均衡实例的监听端口。取值：
    #[serde(rename = "XForwardedFor_SLBPORT")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_slbport: Option<String>,
    /// 用于健康检查的URI。
    #[serde(rename = "HealthCheckURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_uri: Option<String>,
    /// Cookie的处理方式。
    #[serde(rename = "StickySessionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticky_session_type: Option<String>,
    /// 监听绑定的访问策略组ID。
    #[serde(rename = "AclId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_id: Option<String>,
    /// 调度算法。取值：
    #[serde(rename = "Scheduler")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduler: Option<String>,
    /// 健康检查阈值。
    #[serde(rename = "HealthyThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthy_threshold: Option<i32>,
    /// 是否通过`X-Forwarded-Proto`头字段获取负载均衡实例的监听协议。取值：
    #[serde(rename = "XForwardedFor_proto")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_proto: Option<String>,
    /// 是否通过`SLB-IP`头字段获取客户端请求的VIP（Virtual IP address）。取值：
    #[serde(rename = "XForwardedFor_SLBIP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_slbip: Option<String>,
    /// 是否开启会话保持。取值：
    #[serde(rename = "StickySession")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticky_session: Option<String>,
    /// 监听HTTP类型健康检查的健康检查方法。取值：**head**或**get**。
    #[serde(rename = "HealthCheckMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_method: Option<String>,
    /// 是否开启健康检查。取值：
    #[serde(rename = "HealthCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<String>,
    #[serde(rename = "AclIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_ids: Option<DescribeLoadBalancerHTTPListenerAttributeResponseAclIds>,
    #[serde(rename = "Rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<DescribeLoadBalancerHTTPListenerAttributeResponseRules>,
    /// 传统型负载均衡实例ID。
    #[serde(rename = "LoadBalancerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<DescribeLoadBalancerHTTPListenerAttributeResponseTags>,
}

/// CreateLoadBalancerHTTPSListener 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateLoadBalancerHTTPSListenerRequest {
    /// 传统型负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 传统型负载均衡实例的ID。
    #[serde(rename = "LoadBalancerId")]
    pub load_balancer_id: String,
    /// 监听的带宽峰值。单位：Mbps。
    #[serde(rename = "Bandwidth")]
    pub bandwidth: i32,
    /// 实例前端使用的端口。
    #[serde(rename = "ListenerPort")]
    pub listener_port: i32,
    /// 实例后端使用的端口，取值范围：**1**~**65535**。
    #[serde(rename = "BackendServerPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_server_port: Option<i32>,
    /// 标签列表。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<CreateLoadBalancerHTTPSListenerRequestTagItem>>,
    /// 是否通过`X-Forwarded-For`获取来访者客户端IP。取值：
    #[serde(rename = "XForwardedFor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for: Option<String>,
    /// 调度算法。取值：
    #[serde(rename = "Scheduler")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduler: Option<String>,
    /// 是否开启会话保持。取值：
    #[serde(rename = "StickySession")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticky_session: Option<String>,
    /// Cookie的处理方式。取值：**insert**或**server**。
    #[serde(rename = "StickySessionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticky_session_type: Option<String>,
    /// Cookie超时时间。单位：秒。
    #[serde(rename = "CookieTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie_timeout: Option<i32>,
    /// 服务器上配置的Cookie。
    #[serde(rename = "Cookie")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie: Option<String>,
    /// 是否开启健康检查。取值：
    #[serde(rename = "HealthCheck")]
    pub health_check: String,
    /// 监听HTTP类型健康检查的健康检查方法。取值：**head**或**get**。
    #[serde(rename = "HealthCheckMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_method: Option<String>,
    /// 用于健康检查的域名。取值：
    #[serde(rename = "HealthCheckDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_domain: Option<String>,
    /// 用于健康检查的URI。
    #[serde(rename = "HealthCheckURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_uri: Option<String>,
    /// 健康检查连续成功多少次后，将后端服务器的健康检查状态由**fail**判定为**success**。
    #[serde(rename = "HealthyThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthy_threshold: Option<i32>,
    /// 健康检查连续失败多少次后，将后端服务器的健康检查状态由**success**判定为**fail**。
    #[serde(rename = "UnhealthyThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unhealthy_threshold: Option<i32>,
    /// 接收来自运行状况检查的响应需要等待的时间。如果后端ECS在指定的时间内没有正确响应，则判定为健康检查失败。单位：秒。取值：**1**~**300**。
    #[serde(rename = "HealthCheckTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_timeout: Option<i32>,
    /// 健康检查使用的端口。
    #[serde(rename = "HealthCheckConnectPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_connect_port: Option<i32>,
    /// 健康检查的时间间隔。单位：秒。
    #[serde(rename = "HealthCheckInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_interval: Option<i32>,
    /// 健康检查正常的HTTP状态码，多个状态码用半角逗号（,）分割。
    #[serde(rename = "HealthCheckHttpCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_http_code: Option<String>,
    /// 服务器证书的ID。
    #[serde(rename = "ServerCertificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificate_id: Option<String>,
    /// 服务器组ID。
    #[serde(rename = "VServerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_server_group_id: Option<String>,
    /// CA证书ID。
    #[serde(rename = "CACertificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_certificate_id: Option<String>,
    /// 是否通过`SLB-IP`头字段获取来访者的VIP（Virtual IP address）。取值：
    #[serde(rename = "XForwardedFor_SLBIP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_slbip: Option<String>,
    /// 是否通过`SLB-ID`头字段获取SLB实例ID。取值：
    #[serde(rename = "XForwardedFor_SLBID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_slbid: Option<String>,
    /// 是否通过`X-Forwarded-Proto`头字段获取SLB的监听协议。取值：
    #[serde(rename = "XForwardedFor_proto")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_proto: Option<String>,
    /// 是否开启`Gzip`压缩，对特定文件类型进行压缩。取值：
    #[serde(rename = "Gzip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gzip: Option<String>,
    /// 监听绑定的访问策略组ID。
    #[serde(rename = "AclId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_id: Option<String>,
    /// 访问控制类型。取值：
    #[serde(rename = "AclType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_type: Option<String>,
    /// 是否开启访问控制功能。取值：
    #[serde(rename = "AclStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_status: Option<String>,
    /// 自定义监听名称。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 指定连接空闲超时时间，取值范围为**1~60**，默认值为**15**。单位：秒。
    #[serde(rename = "IdleTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_timeout: Option<i32>,
    /// 指定请求超时时间，取值范围为**1~180**，默认值为**60**。单位：秒。
    #[serde(rename = "RequestTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_timeout: Option<i32>,
    /// 是否开启HTTP2特性。取值：
    #[serde(rename = "EnableHttp2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_http2: Option<String>,
    /// 安全策略包含HTTPS可选的TLS协议版本和配套的加密算法套件。
    #[serde(rename = "TLSCipherPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_cipher_policy: Option<String>,
    /// 是否通过`XForwardedFor_SLBPORT`头字段获取负载均衡实例的监听端口。取值：
    #[serde(rename = "XForwardedFor_SLBPORT")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_slbport: Option<String>,
    /// 是否通过`XForwardedFor_ClientSrcPort`头字段获取访问负载均衡实例客户端的端口。取值：
    #[serde(rename = "XForwardedFor_ClientSrcPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_client_src_port: Option<String>,
}

impl CreateLoadBalancerHTTPSListenerRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params.push(("LoadBalancerId".to_string(), self.load_balancer_id.to_string()));
        params.push(("Bandwidth".to_string(), self.bandwidth.to_string()));
        params.push(("ListenerPort".to_string(), self.listener_port.to_string()));
        if let Some(ref v) = self.backend_server_port {
            params.push(("BackendServerPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tag.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.x_forwarded_for {
            params.push(("XForwardedFor".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scheduler {
            params.push(("Scheduler".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sticky_session {
            params.push(("StickySession".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sticky_session_type {
            params.push(("StickySessionType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cookie_timeout {
            params.push(("CookieTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cookie {
            params.push(("Cookie".to_string(), v.to_string()));
        }
        params.push(("HealthCheck".to_string(), self.health_check.to_string()));
        if let Some(ref v) = self.health_check_method {
            params.push(("HealthCheckMethod".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_domain {
            params.push(("HealthCheckDomain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_uri {
            params.push(("HealthCheckURI".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.healthy_threshold {
            params.push(("HealthyThreshold".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.unhealthy_threshold {
            params.push(("UnhealthyThreshold".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_timeout {
            params.push(("HealthCheckTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_connect_port {
            params.push(("HealthCheckConnectPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_interval {
            params.push(("HealthCheckInterval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_http_code {
            params.push(("HealthCheckHttpCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.server_certificate_id {
            params.push(("ServerCertificateId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_server_group_id {
            params.push(("VServerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ca_certificate_id {
            params.push(("CACertificateId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.x_forwarded_for_slbip {
            params.push(("XForwardedFor_SLBIP".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.x_forwarded_for_slbid {
            params.push(("XForwardedFor_SLBID".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.x_forwarded_for_proto {
            params.push(("XForwardedFor_proto".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.gzip {
            params.push(("Gzip".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.acl_id {
            params.push(("AclId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.acl_type {
            params.push(("AclType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.acl_status {
            params.push(("AclStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.idle_timeout {
            params.push(("IdleTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.request_timeout {
            params.push(("RequestTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_http2 {
            params.push(("EnableHttp2".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tls_cipher_policy {
            params.push(("TLSCipherPolicy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.x_forwarded_for_slbport {
            params.push(("XForwardedFor_SLBPORT".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.x_forwarded_for_client_src_port {
            params.push(("XForwardedFor_ClientSrcPort".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateLoadBalancerHTTPSListenerResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// SetLoadBalancerHTTPSListenerAttribute 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetLoadBalancerHTTPSListenerAttributeRequest {
    /// 传统型负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 传统型负载均衡实例的ID。
    #[serde(rename = "LoadBalancerId")]
    pub load_balancer_id: String,
    /// 传统型负载均衡实例前端使用的端口。
    #[serde(rename = "ListenerPort")]
    pub listener_port: i32,
    /// 监听的带宽峰值。单位：Mbps。
    #[serde(rename = "Bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<i32>,
    /// 是否开启通过`X-Forwarded-For`头字段获取来访者真实IP。取值：
    #[serde(rename = "XForwardedFor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for: Option<String>,
    /// 调度算法。取值：
    #[serde(rename = "Scheduler")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduler: Option<String>,
    /// 是否开启会话保持。取值：
    #[serde(rename = "StickySession")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticky_session: Option<String>,
    /// cookie的处理方式。取值：
    #[serde(rename = "StickySessionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticky_session_type: Option<String>,
    /// Cookie超时时间。
    #[serde(rename = "CookieTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie_timeout: Option<i32>,
    /// 服务器上配置的Cookie。
    #[serde(rename = "Cookie")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie: Option<String>,
    /// 是否开启健康检查。取值：
    #[serde(rename = "HealthCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<String>,
    /// 监听HTTP类型健康检查的健康检查方法。取值：**head**或**get**。
    #[serde(rename = "HealthCheckMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_method: Option<String>,
    /// 用于健康检查的域名，取值：
    #[serde(rename = "HealthCheckDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_domain: Option<String>,
    /// 用于健康检查的URI。
    #[serde(rename = "HealthCheckURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_uri: Option<String>,
    /// 健康检查连续成功多少次后，将后端服务器的健康检查状态由**fail**判定为**success**。
    #[serde(rename = "HealthyThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthy_threshold: Option<i32>,
    /// 健康检查连续失败多少次后，将后端服务器的健康检查状态由**success**判定为**fail**。
    #[serde(rename = "UnhealthyThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unhealthy_threshold: Option<i32>,
    /// 接收来自运行状况检查的响应需要等待的时间。如果后端ECS在指定的时间内没有正确响应，则判定为健康检查失败。单位：秒。取值：**1**~**300**。
    #[serde(rename = "HealthCheckTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_timeout: Option<i32>,
    /// 健康检查的时间间隔。单位：秒。
    #[serde(rename = "HealthCheckInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_interval: Option<i32>,
    /// 健康检查使用的端口。
    #[serde(rename = "HealthCheckConnectPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_connect_port: Option<i32>,
    /// 健康检查正常的HTTP状态码，多个状态码用半角逗号（,）分割。
    #[serde(rename = "HealthCheckHttpCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_http_code: Option<String>,
    /// 服务器证书的ID。
    #[serde(rename = "ServerCertificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificate_id: Option<String>,
    /// CA证书ID。
    #[serde(rename = "CACertificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_certificate_id: Option<String>,
    /// 是否使用服务器组。取值：
    #[serde(rename = "VServerGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_server_group: Option<String>,
    /// 虚拟服务器组ID。
    #[serde(rename = "VServerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_server_group_id: Option<String>,
    /// 是否通过`SLB-IP`头字段获取客户端请求的VIP（Virtual IP address）。取值：
    #[serde(rename = "XForwardedFor_SLBIP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_slbip: Option<String>,
    /// 是否通过`SLB-ID`头字段获取负载均衡实例ID。取值：
    #[serde(rename = "XForwardedFor_SLBID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_slbid: Option<String>,
    /// 是否通过`X-Forwarded-Proto`头字段获取负载均衡实例的监听协议。取值：
    #[serde(rename = "XForwardedFor_proto")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_proto: Option<String>,
    /// 是否开启`Gzip`压缩，对特定文件类型进行压缩。取值：
    #[serde(rename = "Gzip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gzip: Option<String>,
    /// 监听绑定的访问策略组ID。
    #[serde(rename = "AclId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_id: Option<String>,
    /// 访问控制类型：
    #[serde(rename = "AclType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_type: Option<String>,
    /// 是否开启访问控制功能。取值：
    #[serde(rename = "AclStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_status: Option<String>,
    /// 指定连接空闲超时时间，取值范围为**1~60**秒，默认值为**15**秒。
    #[serde(rename = "IdleTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_timeout: Option<i32>,
    /// 指定请求超时时间，取值范围为**1~180**秒，默认值为**60**秒。
    #[serde(rename = "RequestTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_timeout: Option<i32>,
    /// 是否开启`HTTP 2.0`特性。取值：
    #[serde(rename = "EnableHttp2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_http2: Option<String>,
    /// 安全策略包含HTTPS可选的TLS协议版本和配套的加密算法套件。
    #[serde(rename = "TLSCipherPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_cipher_policy: Option<String>,
    /// 自定义监听名称。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 是否通过`XForwardedFor_SLBPORT`头字段获取负载均衡实例的监听端口。取值：
    #[serde(rename = "XForwardedFor_SLBPORT")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_slbport: Option<String>,
    /// 是否通过`XForwardedFor_ClientSrcPort`头字段获取访问负载均衡实例客户端的端口。取值：
    #[serde(rename = "XForwardedFor_ClientSrcPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_client_src_port: Option<String>,
}

impl SetLoadBalancerHTTPSListenerAttributeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params.push(("LoadBalancerId".to_string(), self.load_balancer_id.to_string()));
        params.push(("ListenerPort".to_string(), self.listener_port.to_string()));
        if let Some(ref v) = self.bandwidth {
            params.push(("Bandwidth".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.x_forwarded_for {
            params.push(("XForwardedFor".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scheduler {
            params.push(("Scheduler".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sticky_session {
            params.push(("StickySession".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sticky_session_type {
            params.push(("StickySessionType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cookie_timeout {
            params.push(("CookieTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cookie {
            params.push(("Cookie".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check {
            params.push(("HealthCheck".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_method {
            params.push(("HealthCheckMethod".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_domain {
            params.push(("HealthCheckDomain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_uri {
            params.push(("HealthCheckURI".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.healthy_threshold {
            params.push(("HealthyThreshold".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.unhealthy_threshold {
            params.push(("UnhealthyThreshold".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_timeout {
            params.push(("HealthCheckTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_interval {
            params.push(("HealthCheckInterval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_connect_port {
            params.push(("HealthCheckConnectPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_http_code {
            params.push(("HealthCheckHttpCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.server_certificate_id {
            params.push(("ServerCertificateId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ca_certificate_id {
            params.push(("CACertificateId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_server_group {
            params.push(("VServerGroup".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_server_group_id {
            params.push(("VServerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.x_forwarded_for_slbip {
            params.push(("XForwardedFor_SLBIP".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.x_forwarded_for_slbid {
            params.push(("XForwardedFor_SLBID".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.x_forwarded_for_proto {
            params.push(("XForwardedFor_proto".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.gzip {
            params.push(("Gzip".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.acl_id {
            params.push(("AclId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.acl_type {
            params.push(("AclType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.acl_status {
            params.push(("AclStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.idle_timeout {
            params.push(("IdleTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.request_timeout {
            params.push(("RequestTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_http2 {
            params.push(("EnableHttp2".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tls_cipher_policy {
            params.push(("TLSCipherPolicy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.x_forwarded_for_slbport {
            params.push(("XForwardedFor_SLBPORT".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.x_forwarded_for_client_src_port {
            params.push(("XForwardedFor_ClientSrcPort".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetLoadBalancerHTTPSListenerAttributeResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeLoadBalancerHTTPSListenerAttribute 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeLoadBalancerHTTPSListenerAttributeRequest {
    /// 传统型负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 传统型负载均衡实例的ID。
    #[serde(rename = "LoadBalancerId")]
    pub load_balancer_id: String,
    /// 实例前端使用的端口。
    #[serde(rename = "ListenerPort")]
    pub listener_port: i32,
}

impl DescribeLoadBalancerHTTPSListenerAttributeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params.push(("LoadBalancerId".to_string(), self.load_balancer_id.to_string()));
        params.push(("ListenerPort".to_string(), self.listener_port.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeLoadBalancerHTTPSListenerAttributeResponse {
    /// 访问控制类型：
    #[serde(rename = "AclType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_type: Option<String>,
    /// 是否通过`XForwardedFor_ClientCertClientVerify`头字段获取对访问负载均衡实例客户端证书的校验结果。取值：
    #[serde(rename = "XForwardedFor_ClientCertClientVerify")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_client_cert_client_verify: Option<String>,
    /// CA证书ID。
    #[serde(rename = "CACertificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_certificate_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 健康检查的端口。
    #[serde(rename = "HealthCheckConnectPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_connect_port: Option<i32>,
    /// 实例后端使用的端口。
    #[serde(rename = "BackendServerPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_server_port: Option<i32>,
    /// Cookie超时时间。
    #[serde(rename = "CookieTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie_timeout: Option<i32>,
    /// 用于健康检查的域名。
    #[serde(rename = "HealthCheckDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_domain: Option<String>,
    /// 是否通过`X-Forwarded-For`的方式获取来访者真实IP。取值：
    #[serde(rename = "XForwardedFor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for: Option<String>,
    /// 是否通过`XForwardedFor_ClientCertFingerprint`头字段获取访问负载均衡实例客户端证书的指纹。取值：
    #[serde(rename = "XForwardedFor_ClientCertFingerprint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_client_cert_fingerprint: Option<String>,
    /// 指定连接空闲超时时间，取值范围为**1**~**60**，默认值为**15**。单位：秒。
    #[serde(rename = "IdleTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_timeout: Option<i32>,
    /// 实例前端使用的端口。
    #[serde(rename = "ListenerPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_port: Option<i32>,
    /// 用于健康检查的URI。
    #[serde(rename = "HealthCheckURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_uri: Option<String>,
    /// 是否通过`XForwardedFor_SLBPORT`头字段获取负载均衡实例的监听端口。取值：
    #[serde(rename = "XForwardedFor_SLBPORT")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_slbport: Option<String>,
    /// Cookie的处理方式。
    #[serde(rename = "StickySessionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticky_session_type: Option<String>,
    /// 调度算法。取值：**wrr**或**rr**。
    #[serde(rename = "Scheduler")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduler: Option<String>,
    /// 是否通过`X-Forwarded-Proto`头字段获取负载均衡实例的监听协议。取值：
    #[serde(rename = "XForwardedFor_proto")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_proto: Option<String>,
    /// 监听HTTP类型健康检查的健康检查方法。取值：**head**或**get**。
    #[serde(rename = "HealthCheckMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_method: Option<String>,
    /// 只有性能保障型实例才可以指定TLSCipherPolicy参数，每种policy定义了一种安全策略。
    #[serde(rename = "TLSCipherPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_cipher_policy: Option<String>,
    /// 当前监听的状态。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 绑定的服务器组ID。
    #[serde(rename = "VServerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_server_group_id: Option<String>,
    /// 是否通过`XForwardedFor_ClientSrcPort`头字段获取访问负载均衡实例客户端的端口。取值：
    #[serde(rename = "XForwardedFor_ClientSrcPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_client_src_port: Option<String>,
    /// 服务器上配置的Cookie。
    #[serde(rename = "Cookie")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie: Option<String>,
    /// 是否开启`Gzip`压缩。取值：
    #[serde(rename = "Gzip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gzip: Option<String>,
    /// 是否开启`HTTP2`特性。取值：
    #[serde(rename = "EnableHttp2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_http2: Option<String>,
    /// 监听的带宽峰值。单位：Mbps。
    #[serde(rename = "Bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<i32>,
    /// 自定义监听名称。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 每次健康检查响应的最大超时间。单位：秒。
    #[serde(rename = "HealthCheckTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_timeout: Option<i32>,
    /// 是否开启访问控制功能。取值：
    #[serde(rename = "AclStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_status: Option<String>,
    /// 不健康检查阈值。
    #[serde(rename = "UnhealthyThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unhealthy_threshold: Option<i32>,
    /// 是否通过`SLB-ID`头字段获取负载均衡实例ID。取值：
    #[serde(rename = "XForwardedFor_SLBID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_slbid: Option<String>,
    /// 是否通过`XForwardedFor_ClientCertSubjectDN`头字段获取访问负载均衡实例客户端证书的所有者信息。取值：
    #[serde(rename = "XForwardedFor_ClientCertSubjectDN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_client_cert_subject_dn: Option<String>,
    /// 是否处于安全状态。取值：
    #[serde(rename = "SecurityStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_status: Option<String>,
    /// 健康检查正常的HTTP状态码。
    #[serde(rename = "HealthCheckHttpCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_http_code: Option<String>,
    /// 指定请求超时时间，取值范围为**1**~**180**，默认值为**60**。单位：秒。
    #[serde(rename = "RequestTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_timeout: Option<i32>,
    /// 健康检查的时间间隔。单位：秒。
    #[serde(rename = "HealthCheckInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_interval: Option<i32>,
    /// 服务器证书的ID。
    #[serde(rename = "ServerCertificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificate_id: Option<String>,
    /// 监听绑定的访问策略组ID。
    #[serde(rename = "AclId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_id: Option<String>,
    /// 是否通过`XForwardedFor_ClientCertIssuerDN`头字段获取访问负载均衡实例客户端证书的发行者信息。取值：
    #[serde(rename = "XForwardedFor_ClientCertIssuerDN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_client_cert_issuer_dn: Option<String>,
    /// 健康检查阈值。
    #[serde(rename = "HealthyThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthy_threshold: Option<i32>,
    /// 是否通过`SLB-IP`头字段获取客户端请求的VIP（Virtual IP address）。取值：
    #[serde(rename = "XForwardedFor_SLBIP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for_slbip: Option<String>,
    /// 是否开启会话保持。取值：
    #[serde(rename = "StickySession")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticky_session: Option<String>,
    /// 是否开启健康检查。取值：
    #[serde(rename = "HealthCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<String>,
    #[serde(rename = "AclIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_ids: Option<DescribeLoadBalancerHTTPSListenerAttributeResponseAclIds>,
    #[serde(rename = "Rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<DescribeLoadBalancerHTTPSListenerAttributeResponseRules>,
    #[serde(rename = "DomainExtensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_extensions: Option<DescribeLoadBalancerHTTPSListenerAttributeResponseDomainExtensions>,
    /// 传统型负载均衡实例ID。
    #[serde(rename = "LoadBalancerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<DescribeLoadBalancerHTTPSListenerAttributeResponseTags>,
}

/// CreateRules 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateRulesRequest {
    /// 负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 负载均衡实例ID。
    #[serde(rename = "LoadBalancerId")]
    pub load_balancer_id: String,
    /// 负载均衡实例前端使用的监听端口。
    #[serde(rename = "ListenerPort")]
    pub listener_port: i32,
    /// 负载均衡实例前端使用的协议。
    #[serde(rename = "ListenerProtocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_protocol: Option<String>,
    /// 要添加的转发规则。一次请求中，最多可添加10条转发规则。每条转发规则包含以下参数：
    #[serde(rename = "RuleList")]
    pub rule_list: String,
}

impl CreateRulesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("LoadBalancerId".to_string(), self.load_balancer_id.to_string()));
        params.push(("ListenerPort".to_string(), self.listener_port.to_string()));
        if let Some(ref v) = self.listener_protocol {
            params.push(("ListenerProtocol".to_string(), v.to_string()));
        }
        params.push(("RuleList".to_string(), self.rule_list.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateRulesResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<CreateRulesResponseRules>,
}

/// SetRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetRuleRequest {
    /// 负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 转发规则ID。
    #[serde(rename = "RuleId")]
    pub rule_id: String,
    /// 转发规则的目标服务器组ID。
    #[serde(rename = "VServerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_server_group_id: Option<String>,
    /// 转发规则名称，长度限制为1~40个字符，支持中文、字母、数字、短划线（-）、正斜线（/）、半角句号（.）和下划线（_）这些字符。
    #[serde(rename = "RuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    /// 是否继承监听的健康检查、会话保持和调度算法配置。取值：
    #[serde(rename = "ListenerSync")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_sync: Option<String>,
    /// 调度算法。取值：
    #[serde(rename = "Scheduler")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduler: Option<String>,
    /// 是否开启会话保持，取值：
    #[serde(rename = "StickySession")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticky_session: Option<String>,
    /// Cookie的处理方式。取值：
    #[serde(rename = "StickySessionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticky_session_type: Option<String>,
    /// Cookie超时时间。单位：秒。取值范围：**1**~**86400**。
    #[serde(rename = "CookieTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie_timeout: Option<i32>,
    /// 服务器上配置的Cookie。
    #[serde(rename = "Cookie")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie: Option<String>,
    /// 是否开启健康检查。取值：
    #[serde(rename = "HealthCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<String>,
    /// 用于健康检查的域名，取值：
    #[serde(rename = "HealthCheckDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_domain: Option<String>,
    /// 用于健康检查的URI。
    #[serde(rename = "HealthCheckURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_uri: Option<String>,
    /// 健康检查连续成功多少次后，将后端服务器的健康检查状态由**失败**判定为**成功**。
    #[serde(rename = "HealthyThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthy_threshold: Option<i32>,
    /// 健康检查连续失败多少次后，将后端服务器的健康检查状态由**成功**判定为**失败**。
    #[serde(rename = "UnhealthyThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unhealthy_threshold: Option<i32>,
    /// 接收来自运行状况检查的响应需要等待的时间。如果后端ECS在指定的时间内没有正确响应，则判定为健康检查失败。单位：秒。取值范围：**1**~**300**。
    #[serde(rename = "HealthCheckTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_timeout: Option<i32>,
    /// 健康检查的时间间隔。单位：秒。取值范围：**1**~**50**。
    #[serde(rename = "HealthCheckInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_interval: Option<i32>,
    /// 健康检查使用的端口。取值范围：**1**~**65535**。
    #[serde(rename = "HealthCheckConnectPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_connect_port: Option<i32>,
    /// 健康检查正常的HTTP状态码，多个状态码用半角逗号（,）分割。
    #[serde(rename = "HealthCheckHttpCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_http_code: Option<String>,
}

impl SetRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("RuleId".to_string(), self.rule_id.to_string()));
        if let Some(ref v) = self.v_server_group_id {
            params.push(("VServerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rule_name {
            params.push(("RuleName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.listener_sync {
            params.push(("ListenerSync".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scheduler {
            params.push(("Scheduler".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sticky_session {
            params.push(("StickySession".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sticky_session_type {
            params.push(("StickySessionType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cookie_timeout {
            params.push(("CookieTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cookie {
            params.push(("Cookie".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check {
            params.push(("HealthCheck".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_domain {
            params.push(("HealthCheckDomain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_uri {
            params.push(("HealthCheckURI".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.healthy_threshold {
            params.push(("HealthyThreshold".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.unhealthy_threshold {
            params.push(("UnhealthyThreshold".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_timeout {
            params.push(("HealthCheckTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_interval {
            params.push(("HealthCheckInterval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_connect_port {
            params.push(("HealthCheckConnectPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check_http_code {
            params.push(("HealthCheckHttpCode".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetRuleResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteRules 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteRulesRequest {
    /// 负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 要删除的转发规则列表。
    #[serde(rename = "RuleIds")]
    pub rule_ids: String,
}

impl DeleteRulesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("RuleIds".to_string(), self.rule_ids.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteRulesResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeRuleAttribute 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeRuleAttributeRequest {
    /// 负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 转发规则ID。
    #[serde(rename = "RuleId")]
    pub rule_id: String,
}

impl DescribeRuleAttributeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("RuleId".to_string(), self.rule_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeRuleAttributeResponse {
    /// 转发规则关联的服务器组ID。
    #[serde(rename = "VServerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_server_group_id: Option<String>,
    /// 服务器上配置的Cookie。
    #[serde(rename = "Cookie")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie: Option<String>,
    /// 负载均衡实例ID。
    #[serde(rename = "LoadBalancerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_id: Option<String>,
    /// 转发规则ID。
    #[serde(rename = "RuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 健康检查的后端服务器的端口。
    #[serde(rename = "HealthCheckConnectPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_connect_port: Option<i32>,
    /// 接收来自运行状况检查的响应需要等待的时间。如果后端ECS在指定的时间内没有正确响应，则判定为健康检查失败。
    #[serde(rename = "HealthCheckTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_timeout: Option<i32>,
    /// Cookie超时时间。
    #[serde(rename = "CookieTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie_timeout: Option<i32>,
    /// 用于健康检查的域名，取值：
    #[serde(rename = "HealthCheckDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_domain: Option<String>,
    /// 健康检查连续失败多少次后，将后端服务器的健康检查状态由**success**判定为**fail**。
    #[serde(rename = "UnhealthyThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unhealthy_threshold: Option<i32>,
    /// 健康检查正常的HTTP状态码，多个状态码用逗号分隔。默认值为**http_2xx**。
    #[serde(rename = "HealthCheckHttpCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_http_code: Option<String>,
    /// 转发规则域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 负载均衡实例前端使用的监听端口。
    #[serde(rename = "ListenerPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_port: Option<String>,
    /// 转发规则路径。
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 健康检查的时间间隔。
    #[serde(rename = "HealthCheckInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_interval: Option<i32>,
    /// 用于健康检查的URI。
    #[serde(rename = "HealthCheckURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_uri: Option<String>,
    /// 转发规则名称。
    #[serde(rename = "RuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    /// Cookie的处理方式。取值：
    #[serde(rename = "StickySessionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticky_session_type: Option<String>,
    /// 调度算法。取值：
    #[serde(rename = "Scheduler")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduler: Option<String>,
    /// 转发规则是否从监听上继承健康检查、会话保持和调度算法配置。
    #[serde(rename = "ListenerSync")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_sync: Option<String>,
    /// 健康检查连续成功多少次后，将后端服务器的健康检查状态由**fail**判定为**success**。
    #[serde(rename = "HealthyThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthy_threshold: Option<i32>,
    /// 是否开启会话保持。取值：
    #[serde(rename = "StickySession")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticky_session: Option<String>,
    /// 是否开启健康检查。取值：
    #[serde(rename = "HealthCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<String>,
}

/// DescribeRules 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeRulesRequest {
    /// 负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 负载均衡实例ID。
    #[serde(rename = "LoadBalancerId")]
    pub load_balancer_id: String,
    /// 负载均衡实例前端使用的协议。取值：
    #[serde(rename = "ListenerProtocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_protocol: Option<String>,
    /// 负载均衡实例前端使用的监听端口。
    #[serde(rename = "ListenerPort")]
    pub listener_port: i32,
}

impl DescribeRulesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("LoadBalancerId".to_string(), self.load_balancer_id.to_string()));
        if let Some(ref v) = self.listener_protocol {
            params.push(("ListenerProtocol".to_string(), v.to_string()));
        }
        params.push(("ListenerPort".to_string(), self.listener_port.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeRulesResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<DescribeRulesResponseRules>,
}

/// AddBackendServers 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct AddBackendServersRequest {
    /// 负载均衡实例所属地域的ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 负载均衡实例ID。
    #[serde(rename = "LoadBalancerId")]
    pub load_balancer_id: String,
    /// 要添加的后端服务器列表，包含以下参数：
    #[serde(rename = "BackendServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_servers: Option<String>,
}

impl AddBackendServersRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params.push(("LoadBalancerId".to_string(), self.load_balancer_id.to_string()));
        if let Some(ref v) = self.backend_servers {
            params.push(("BackendServers".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AddBackendServersResponse {
    /// 负载均衡实例ID。
    #[serde(rename = "LoadBalancerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "BackendServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_servers: Option<AddBackendServersResponseBackendServers>,
}

/// DescribeHealthStatus 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeHealthStatusRequest {
    /// 负载均衡实例 ID。
    #[serde(rename = "LoadBalancerId")]
    pub load_balancer_id: String,
    /// 负载均衡实例前端使用的端口。
    #[serde(rename = "ListenerPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_port: Option<i32>,
    /// 负载均衡实例前端使用的协议。
    #[serde(rename = "ListenerProtocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_protocol: Option<String>,
    /// 负载均衡实例的地域 ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
}

impl DescribeHealthStatusRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("LoadBalancerId".to_string(), self.load_balancer_id.to_string()));
        if let Some(ref v) = self.listener_port {
            params.push(("ListenerPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.listener_protocol {
            params.push(("ListenerProtocol".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeHealthStatusResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "BackendServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_servers: Option<DescribeHealthStatusResponseBackendServers>,
}

/// RemoveBackendServers 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct RemoveBackendServersRequest {
    /// 负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 负载均衡实例ID。
    #[serde(rename = "LoadBalancerId")]
    pub load_balancer_id: String,
    /// 要移除的后端服务器。
    #[serde(rename = "BackendServers")]
    pub backend_servers: String,
}

impl RemoveBackendServersRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params.push(("LoadBalancerId".to_string(), self.load_balancer_id.to_string()));
        params.push(("BackendServers".to_string(), self.backend_servers.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RemoveBackendServersResponse {
    /// 负载均衡实例ID。
    #[serde(rename = "LoadBalancerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "BackendServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_servers: Option<RemoveBackendServersResponseBackendServers>,
}

/// SetBackendServers 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetBackendServersRequest {
    /// 传统型负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 传统型负载均衡实例ID。
    #[serde(rename = "LoadBalancerId")]
    pub load_balancer_id: String,
    /// 要添加的后端服务器列表，包含以下参数：
    #[serde(rename = "BackendServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_servers: Option<String>,
}

impl SetBackendServersRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params.push(("LoadBalancerId".to_string(), self.load_balancer_id.to_string()));
        if let Some(ref v) = self.backend_servers {
            params.push(("BackendServers".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetBackendServersResponse {
    /// 传统型负载均衡实例ID。
    #[serde(rename = "LoadBalancerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "BackendServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_servers: Option<SetBackendServersResponseBackendServers>,
}

/// CreateVServerGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateVServerGroupRequest {
    /// 标签列表。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<CreateVServerGroupRequestTagItem>>,
    /// 传统型负载均衡地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 传统型负载均衡实例ID。
    #[serde(rename = "LoadBalancerId")]
    pub load_balancer_id: String,
    /// 虚拟服务器组名称。
    #[serde(rename = "VServerGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_server_group_name: Option<String>,
    /// 要添加的后端服务器列表，包含以下参数：
    #[serde(rename = "BackendServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_servers: Option<String>,
}

impl CreateVServerGroupRequest {
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
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("LoadBalancerId".to_string(), self.load_balancer_id.to_string()));
        if let Some(ref v) = self.v_server_group_name {
            params.push(("VServerGroupName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backend_servers {
            params.push(("BackendServers".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateVServerGroupResponse {
    /// 后端服务器组ID。
    #[serde(rename = "VServerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_server_group_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "BackendServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_servers: Option<CreateVServerGroupResponseBackendServers>,
}

/// DeleteVServerGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteVServerGroupRequest {
    /// 负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 虚拟服务器组ID。
    #[serde(rename = "VServerGroupId")]
    pub v_server_group_id: String,
}

impl DeleteVServerGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("VServerGroupId".to_string(), self.v_server_group_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteVServerGroupResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// SetVServerGroupAttribute 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetVServerGroupAttributeRequest {
    /// 负载均衡地域ID，不可更改。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 虚拟服务器组ID，不可更改。
    #[serde(rename = "VServerGroupId")]
    pub v_server_group_id: String,
    /// 虚拟服务器组名称，可自定义更改。
    #[serde(rename = "VServerGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_server_group_name: Option<String>,
    /// 后端服务器列表，该接口只能用于修改后端服务器的权重和虚拟服务器组名称。
    #[serde(rename = "BackendServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_servers: Option<String>,
}

impl SetVServerGroupAttributeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("VServerGroupId".to_string(), self.v_server_group_id.to_string()));
        if let Some(ref v) = self.v_server_group_name {
            params.push(("VServerGroupName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backend_servers {
            params.push(("BackendServers".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetVServerGroupAttributeResponse {
    /// 服务器组ID。
    #[serde(rename = "VServerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_server_group_id: Option<String>,
    /// 虚拟服务器组名称。
    #[serde(rename = "VServerGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_server_group_name: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "BackendServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_servers: Option<SetVServerGroupAttributeResponseBackendServers>,
}

/// DescribeVServerGroups 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeVServerGroupsRequest {
    /// 传统型负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 传统型负载均衡实例ID。
    #[serde(rename = "LoadBalancerId")]
    pub load_balancer_id: String,
    /// 是否返回关联的转发规则信息。取值：
    #[serde(rename = "IncludeRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_rule: Option<bool>,
    /// 是否返回关联的监听信息。取值：
    #[serde(rename = "IncludeListener")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_listener: Option<bool>,
    /// 资源标签。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<DescribeVServerGroupsRequestTagItem>>,
    /// 虚拟服务器组名称。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl DescribeVServerGroupsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("LoadBalancerId".to_string(), self.load_balancer_id.to_string()));
        if let Some(ref v) = self.include_rule {
            params.push(("IncludeRule".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.include_listener {
            params.push(("IncludeListener".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tag.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeVServerGroupsResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "VServerGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_server_groups: Option<DescribeVServerGroupsResponseVServerGroups>,
}

/// DescribeVServerGroupAttribute 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeVServerGroupAttributeRequest {
    /// 传统型负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 虚拟服务器组ID。
    #[serde(rename = "VServerGroupId")]
    pub v_server_group_id: String,
}

impl DescribeVServerGroupAttributeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("VServerGroupId".to_string(), self.v_server_group_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeVServerGroupAttributeResponse {
    /// 虚拟服务器组ID。
    #[serde(rename = "VServerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_server_group_id: Option<String>,
    /// 虚拟服务器组的名称。
    #[serde(rename = "VServerGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_server_group_name: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 传统型负载均衡实例ID。
    #[serde(rename = "LoadBalancerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_id: Option<String>,
    #[serde(rename = "BackendServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_servers: Option<DescribeVServerGroupAttributeResponseBackendServers>,
    /// 传统型负载均衡实例的创建时间，格式：`YYYY-MM-DDThh:mm:ssZ`。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<DescribeVServerGroupAttributeResponseTags>,
}

/// AddVServerGroupBackendServers 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct AddVServerGroupBackendServersRequest {
    /// 负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 虚拟服务器组ID。
    #[serde(rename = "VServerGroupId")]
    pub v_server_group_id: String,
    /// 要添加的后端服务器列表，包含以下参数：
    #[serde(rename = "BackendServers")]
    pub backend_servers: String,
}

impl AddVServerGroupBackendServersRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("VServerGroupId".to_string(), self.v_server_group_id.to_string()));
        params.push(("BackendServers".to_string(), self.backend_servers.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AddVServerGroupBackendServersResponse {
    /// 虚拟服务器组ID。
    #[serde(rename = "VServerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_server_group_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "BackendServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_servers: Option<AddVServerGroupBackendServersResponseBackendServers>,
}

/// ModifyVServerGroupBackendServers 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyVServerGroupBackendServersRequest {
    /// 负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 服务器组ID。
    #[serde(rename = "VServerGroupId")]
    pub v_server_group_id: String,
    /// 要被替换的后端服务器列表，包含以下参数：
    #[serde(rename = "OldBackendServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_backend_servers: Option<String>,
    /// 新的后端服务器列表，包含以下参数：
    #[serde(rename = "NewBackendServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_backend_servers: Option<String>,
}

impl ModifyVServerGroupBackendServersRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("VServerGroupId".to_string(), self.v_server_group_id.to_string()));
        if let Some(ref v) = self.old_backend_servers {
            params.push(("OldBackendServers".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.new_backend_servers {
            params.push(("NewBackendServers".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyVServerGroupBackendServersResponse {
    /// 服务器组ID。
    #[serde(rename = "VServerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_server_group_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "BackendServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_servers: Option<ModifyVServerGroupBackendServersResponseBackendServers>,
}

/// RemoveVServerGroupBackendServers 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct RemoveVServerGroupBackendServersRequest {
    /// 负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 虚拟服务器组ID。
    #[serde(rename = "VServerGroupId")]
    pub v_server_group_id: String,
    /// 后端服务器列表，包含以下参数：
    #[serde(rename = "BackendServers")]
    pub backend_servers: String,
}

impl RemoveVServerGroupBackendServersRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("VServerGroupId".to_string(), self.v_server_group_id.to_string()));
        params.push(("BackendServers".to_string(), self.backend_servers.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RemoveVServerGroupBackendServersResponse {
    /// 虚拟服务器组ID。
    #[serde(rename = "VServerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_server_group_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "BackendServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_servers: Option<RemoveVServerGroupBackendServersResponseBackendServers>,
}

/// CreateMasterSlaveServerGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateMasterSlaveServerGroupRequest {
    /// 传统型负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 传统型负载均衡实例ID。
    #[serde(rename = "LoadBalancerId")]
    pub load_balancer_id: String,
    /// 主备服务器组名称。
    #[serde(rename = "MasterSlaveServerGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_slave_server_group_name: Option<String>,
    /// 主备服务器组列表。一个主备服务器组只能包含2个后端服务器。
    #[serde(rename = "MasterSlaveBackendServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_slave_backend_servers: Option<String>,
    /// 标签列表。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<CreateMasterSlaveServerGroupRequestTagItem>>,
}

impl CreateMasterSlaveServerGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("LoadBalancerId".to_string(), self.load_balancer_id.to_string()));
        if let Some(ref v) = self.master_slave_server_group_name {
            params.push(("MasterSlaveServerGroupName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.master_slave_backend_servers {
            params.push(("MasterSlaveBackendServers".to_string(), v.to_string()));
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
pub struct CreateMasterSlaveServerGroupResponse {
    /// 主备服务器组ID。
    #[serde(rename = "MasterSlaveServerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_slave_server_group_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "MasterSlaveBackendServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_slave_backend_servers: Option<CreateMasterSlaveServerGroupResponseMasterSlaveBackendServers>,
}

/// DeleteMasterSlaveServerGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteMasterSlaveServerGroupRequest {
    /// 负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 主备服务器组ID。
    #[serde(rename = "MasterSlaveServerGroupId")]
    pub master_slave_server_group_id: String,
}

impl DeleteMasterSlaveServerGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("MasterSlaveServerGroupId".to_string(), self.master_slave_server_group_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteMasterSlaveServerGroupResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeMasterSlaveServerGroupAttribute 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeMasterSlaveServerGroupAttributeRequest {
    /// 传统型负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 主备服务器组ID。
    #[serde(rename = "MasterSlaveServerGroupId")]
    pub master_slave_server_group_id: String,
}

impl DescribeMasterSlaveServerGroupAttributeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("MasterSlaveServerGroupId".to_string(), self.master_slave_server_group_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeMasterSlaveServerGroupAttributeResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<DescribeMasterSlaveServerGroupAttributeResponseTags>,
    /// 关联的传统型负载均衡实例ID。
    #[serde(rename = "LoadBalancerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_id: Option<String>,
    /// 主备服务器组的名称。
    #[serde(rename = "MasterSlaveServerGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_slave_server_group_name: Option<String>,
    /// 主备服务器组ID。
    #[serde(rename = "MasterSlaveServerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_slave_server_group_id: Option<String>,
    #[serde(rename = "MasterSlaveBackendServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_slave_backend_servers: Option<DescribeMasterSlaveServerGroupAttributeResponseMasterSlaveBackendServers>,
    /// 传统型负载均衡实例创建时间，格式为：`YYYY-MM-DDThh:mm:ssZ`。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
}

/// DescribeMasterSlaveServerGroups 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeMasterSlaveServerGroupsRequest {
    /// 传统型负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 传统型负载均衡实例ID。
    #[serde(rename = "LoadBalancerId")]
    pub load_balancer_id: String,
    /// 是否返回关联的监听信息，取值：
    #[serde(rename = "IncludeListener")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_listener: Option<bool>,
    /// 资源标签。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<DescribeMasterSlaveServerGroupsRequestTagItem>>,
    /// 主备服务器组名称。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl DescribeMasterSlaveServerGroupsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("LoadBalancerId".to_string(), self.load_balancer_id.to_string()));
        if let Some(ref v) = self.include_listener {
            params.push(("IncludeListener".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tag.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeMasterSlaveServerGroupsResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "MasterSlaveServerGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_slave_server_groups: Option<DescribeMasterSlaveServerGroupsResponseMasterSlaveServerGroups>,
}

/// DeleteCACertificate 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteCACertificateRequest {
    /// CA证书的地域。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// CA证书ID。
    #[serde(rename = "CACertificateId")]
    pub ca_certificate_id: String,
}

impl DeleteCACertificateRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("CACertificateId".to_string(), self.ca_certificate_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteCACertificateResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteServerCertificate 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteServerCertificateRequest {
    /// 负载均衡实例的地域。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 服务器证书ID。
    #[serde(rename = "ServerCertificateId")]
    pub server_certificate_id: String,
}

impl DeleteServerCertificateRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params.push(("ServerCertificateId".to_string(), self.server_certificate_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteServerCertificateResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// SetCACertificateName 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetCACertificateNameRequest {
    /// CA证书的地域。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// CA证书ID。
    #[serde(rename = "CACertificateId")]
    pub ca_certificate_id: String,
    /// CA证书名称。
    #[serde(rename = "CACertificateName")]
    pub ca_certificate_name: String,
}

impl SetCACertificateNameRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("CACertificateId".to_string(), self.ca_certificate_id.to_string()));
        params.push(("CACertificateName".to_string(), self.ca_certificate_name.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetCACertificateNameResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// SetServerCertificateName 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetServerCertificateNameRequest {
    /// 传统型负载均衡实例的地域。您可以通过调用[DescribeRegions](~~2401682~~) 接口查询地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 服务器证书ID。
    #[serde(rename = "ServerCertificateId")]
    pub server_certificate_id: String,
    /// 要上传的非阿里云签发的服务器证书的名称。长度限制为1~80个字符，允许包含中文、字母、数字、短划线（-）、正斜线（/）、半角句号（.）、下划线（_）和星号（*）。
    #[serde(rename = "ServerCertificateName")]
    pub server_certificate_name: String,
}

impl SetServerCertificateNameRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("ServerCertificateId".to_string(), self.server_certificate_id.to_string()));
        params.push(("ServerCertificateName".to_string(), self.server_certificate_name.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetServerCertificateNameResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeCACertificates 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCACertificatesRequest {
    /// CA证书的地域。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// CA证书ID。
    #[serde(rename = "CACertificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_certificate_id: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 标签列表。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<DescribeCACertificatesRequestTagItem>>,
}

impl DescribeCACertificatesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.ca_certificate_id {
            params.push(("CACertificateId".to_string(), v.to_string()));
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
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeCACertificatesResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "CACertificates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_certificates: Option<DescribeCACertificatesResponseCACertificates>,
}

/// DescribeServerCertificates 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeServerCertificatesRequest {
    /// 传统型负载均衡实例的地域。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 服务器证书ID。
    #[serde(rename = "ServerCertificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificate_id: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 标签列表。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<DescribeServerCertificatesRequestTagItem>>,
}

impl DescribeServerCertificatesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.server_certificate_id {
            params.push(("ServerCertificateId".to_string(), v.to_string()));
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
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeServerCertificatesResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "ServerCertificates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificates: Option<DescribeServerCertificatesResponseServerCertificates>,
}

/// UploadCACertificate 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UploadCACertificateRequest {
    /// 标签列表。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<UploadCACertificateRequestTagItem>>,
    /// CA证书的地域。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 要上传CA证书的内容。
    #[serde(rename = "CACertificate")]
    pub ca_certificate: String,
    /// CA证书名称。
    #[serde(rename = "CACertificateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_certificate_name: Option<String>,
    /// 企业资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl UploadCACertificateRequest {
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
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("CACertificate".to_string(), self.ca_certificate.to_string()));
        if let Some(ref v) = self.ca_certificate_name {
            params.push(("CACertificateName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UploadCACertificateResponse {
    /// CA证书上传的时间戳。
    #[serde(rename = "CreateTimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time_stamp: Option<i64>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// CA证书的过期时间。
    #[serde(rename = "ExpireTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<String>,
    /// CA证书的指纹。
    #[serde(rename = "Fingerprint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    /// CA证书上传的时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// CA证书的域名。
    #[serde(rename = "CommonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_name: Option<String>,
    /// 企业资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// CA证书的名称。
    #[serde(rename = "CACertificateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_certificate_name: Option<String>,
    /// CA证书的过期时间戳。
    #[serde(rename = "ExpireTimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time_stamp: Option<i64>,
    /// CA证书ID。
    #[serde(rename = "CACertificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_certificate_id: Option<String>,
}

/// UploadServerCertificate 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UploadServerCertificateRequest {
    /// 标签列表。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<UploadServerCertificateRequestTagItem>>,
    /// 服务器证书部署的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 阿里云签发证书ID。
    #[serde(rename = "AliCloudCertificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ali_cloud_certificate_id: Option<String>,
    /// 阿里云签发证书名称。
    #[serde(rename = "AliCloudCertificateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ali_cloud_certificate_name: Option<String>,
    /// 阿里云签发证书所属的地域ID。
    #[serde(rename = "AliCloudCertificateRegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ali_cloud_certificate_region_id: Option<String>,
    /// 需要上传的公钥证书。
    #[serde(rename = "ServerCertificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificate: Option<String>,
    /// 需要上传的私钥。
    #[serde(rename = "PrivateKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    /// 要上传的非阿里云签发的服务器证书的名称。长度限制为1~80个字符，允许包含中文、字母、数字、短划线（-）、正斜线（/）、半角句号（.）、下划线（_）和星号（*）。
    #[serde(rename = "ServerCertificateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificate_name: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl UploadServerCertificateRequest {
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
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.ali_cloud_certificate_id {
            params.push(("AliCloudCertificateId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ali_cloud_certificate_name {
            params.push(("AliCloudCertificateName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ali_cloud_certificate_region_id {
            params.push(("AliCloudCertificateRegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.server_certificate {
            params.push(("ServerCertificate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.private_key {
            params.push(("PrivateKey".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.server_certificate_name {
            params.push(("ServerCertificateName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回数据结构体。
#[derive(Debug, Clone, Deserialize)]
pub struct UploadServerCertificateResponse {
    /// 阿里云签发证书名称。
    #[serde(rename = "AliCloudCertificateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ali_cloud_certificate_name: Option<String>,
    /// 证书创建的时间戳。
    #[serde(rename = "CreateTimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time_stamp: Option<i64>,
    /// 证书过期时间。
    #[serde(rename = "ExpireTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<String>,
    /// 证书创建时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 服务器证书ID。
    #[serde(rename = "ServerCertificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificate_id: Option<String>,
    /// 证书过期的时间戳。
    #[serde(rename = "ExpireTimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time_stamp: Option<i64>,
    /// 服务器证书部署的地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 服务器证书的指纹。
    #[serde(rename = "Fingerprint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    /// 非阿里云签发的服务器证书的名称。
    #[serde(rename = "ServerCertificateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificate_name: Option<String>,
    /// 域名，对应证书的`Common Name`字段。
    #[serde(rename = "CommonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_name: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 阿里云签发证书ID。
    #[serde(rename = "AliCloudCertificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ali_cloud_certificate_id: Option<String>,
    /// 是否为阿里云证书服务中的证书。
    #[serde(rename = "IsAliCloudCertificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_ali_cloud_certificate: Option<i32>,
    #[serde(rename = "SubjectAlternativeNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_alternative_names: Option<UploadServerCertificateResponseSubjectAlternativeNames>,
}

/// CreateDomainExtension 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateDomainExtensionRequest {
    /// 负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 负载均衡实例的ID。
    #[serde(rename = "LoadBalancerId")]
    pub load_balancer_id: String,
    /// 负载均衡实例HTTPS监听的前端端口。
    #[serde(rename = "ListenerPort")]
    pub listener_port: i32,
    /// 域名。
    #[serde(rename = "Domain")]
    pub domain: String,
    /// 与域名对应的证书ID。
    #[serde(rename = "ServerCertificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificate_id: Option<String>,
}

impl CreateDomainExtensionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("LoadBalancerId".to_string(), self.load_balancer_id.to_string()));
        params.push(("ListenerPort".to_string(), self.listener_port.to_string()));
        params.push(("Domain".to_string(), self.domain.to_string()));
        if let Some(ref v) = self.server_certificate_id {
            params.push(("ServerCertificateId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateDomainExtensionResponse {
    /// 负载均衡实例前端使用的端口。
    #[serde(rename = "ListenerPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_port: Option<i32>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 创建的扩展域名ID。
    #[serde(rename = "DomainExtensionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_extension_id: Option<String>,
}

/// SetDomainExtensionAttribute 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetDomainExtensionAttributeRequest {
    /// 负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 需要修改的扩展域名ID。
    #[serde(rename = "DomainExtensionId")]
    pub domain_extension_id: String,
    /// 新的证书ID。
    #[serde(rename = "ServerCertificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificate_id: Option<String>,
}

impl SetDomainExtensionAttributeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("DomainExtensionId".to_string(), self.domain_extension_id.to_string()));
        if let Some(ref v) = self.server_certificate_id {
            params.push(("ServerCertificateId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetDomainExtensionAttributeResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteDomainExtension 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteDomainExtensionRequest {
    /// 负载均衡实例的所属地域。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 要删除的扩展域名ID。
    #[serde(rename = "DomainExtensionId")]
    pub domain_extension_id: String,
}

impl DeleteDomainExtensionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("DomainExtensionId".to_string(), self.domain_extension_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteDomainExtensionResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeDomainExtensionAttribute 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainExtensionAttributeRequest {
    /// 负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 扩展域名ID。
    #[serde(rename = "DomainExtensionId")]
    pub domain_extension_id: String,
}

impl DescribeDomainExtensionAttributeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("DomainExtensionId".to_string(), self.domain_extension_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainExtensionAttributeResponse {
    /// 域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 负载均衡实例ID。
    #[serde(rename = "LoadBalancerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_id: Option<String>,
    /// 负载均衡实例HTTPS监听的前端端口，取值：**1**~**65535**。
    #[serde(rename = "ListenerPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_port: Option<i32>,
    /// 域名使用的服务器证书ID。
    #[serde(rename = "ServerCertificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificate_id: Option<String>,
    /// 扩展域名ID。
    #[serde(rename = "DomainExtensionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_extension_id: Option<String>,
}

/// DescribeDomainExtensions 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainExtensionsRequest {
    /// 负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 负载均衡实例的ID。
    #[serde(rename = "LoadBalancerId")]
    pub load_balancer_id: String,
    /// 负载均衡实例HTTPS监听的前端端口，取值：**1-65535**。
    #[serde(rename = "ListenerPort")]
    pub listener_port: i32,
    /// 扩展域名ID。
    #[serde(rename = "DomainExtensionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_extension_id: Option<String>,
}

impl DescribeDomainExtensionsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("LoadBalancerId".to_string(), self.load_balancer_id.to_string()));
        params.push(("ListenerPort".to_string(), self.listener_port.to_string()));
        if let Some(ref v) = self.domain_extension_id {
            params.push(("DomainExtensionId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainExtensionsResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "DomainExtensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_extensions: Option<DescribeDomainExtensionsResponseDomainExtensions>,
}

/// CreateTLSCipherPolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateTLSCipherPolicyRequest {
    /// 负载均衡实例所在的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// TLS 策略名称。长度为 2~200 个英文或中文字符，必须以大小字母或中文开头，可包含数字、半角句号（.）、下划线（_）和短划线（-）。
    #[serde(rename = "Name")]
    pub name: String,
    /// 支持的加密套件列表，具体依赖TLSVersion值。最多支持添加32个加密套件。
    #[serde(rename = "Ciphers")]
    pub ciphers: Vec<String>,
    /// 支持的TLS协议版本。取值：**TLSv1.0**、**TLSv1.1**、**TLSv1.2**和**TLSv1.3**。最多支持添加4个TLS协议版本。
    #[serde(rename = "TLSVersions")]
    pub tls_versions: Vec<String>,
}

impl CreateTLSCipherPolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("Name".to_string(), self.name.to_string()));
        for (i, item) in self.ciphers.iter().enumerate() {
            params.push((format!("Ciphers.{}", i + 1), item.to_string()));
        }
        for (i, item) in self.tls_versions.iter().enumerate() {
            params.push((format!("TLSVersions.{}", i + 1), item.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateTLSCipherPolicyResponse {
    /// 策略ID。
    #[serde(rename = "TLSCipherPolicyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_cipher_policy_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteTLSCipherPolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteTLSCipherPolicyRequest {
    /// 负载均衡实例所在的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// TLS策略实例ID。
    #[serde(rename = "TLSCipherPolicyId")]
    pub tls_cipher_policy_id: String,
}

impl DeleteTLSCipherPolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("TLSCipherPolicyId".to_string(), self.tls_cipher_policy_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteTLSCipherPolicyResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// SetTLSCipherPolicyAttribute 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetTLSCipherPolicyAttributeRequest {
    /// 负载均衡实例地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// TLS策略ID。
    #[serde(rename = "TLSCipherPolicyId")]
    pub tls_cipher_policy_id: String,
    /// TLS策略名称。长度为2~200个英文或中文字符，必须以大小字母或中文开头，可包含数字、半角句号（.）、下划线（_）和短划线（-）。
    #[serde(rename = "Name")]
    pub name: String,
    /// 支持的TLS协议版本。取值：**TLSv1.0**、**TLSv1.1**、**TLSv1.2**和**TLSv1.3**。
    #[serde(rename = "TLSVersions")]
    pub tls_versions: Vec<String>,
    /// 支持的加密套件，具体依赖TLSVersions的取值。
    #[serde(rename = "Ciphers")]
    pub ciphers: Vec<String>,
}

impl SetTLSCipherPolicyAttributeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("TLSCipherPolicyId".to_string(), self.tls_cipher_policy_id.to_string()));
        params.push(("Name".to_string(), self.name.to_string()));
        for (i, item) in self.tls_versions.iter().enumerate() {
            params.push((format!("TLSVersions.{}", i + 1), item.to_string()));
        }
        for (i, item) in self.ciphers.iter().enumerate() {
            params.push((format!("Ciphers.{}", i + 1), item.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetTLSCipherPolicyAttributeResponse {
    /// 异步任务ID。
    #[serde(rename = "TaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListTLSCipherPolicies 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListTLSCipherPoliciesRequest {
    /// 负载均衡实例地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// TLS策略ID。
    #[serde(rename = "TLSCipherPolicyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_cipher_policy_id: Option<String>,
    /// TLS策略名称。长度为2~200个英文或中文字符，必须以大小写字母或中文开头，可包含数字、半角句号（.）、下划线（_）和短划线（-）。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 是否返回关联的监听信息。取值：
    #[serde(rename = "IncludeListener")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_listener: Option<bool>,
    /// 是否拥有下一次查询的令牌（Token）。取值：
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 本次读取的最大TLS策略数，取值：**1**~**100**。设置为空时，默认值为**20**。
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

impl ListTLSCipherPoliciesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.tls_cipher_policy_id {
            params.push(("TLSCipherPolicyId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.include_listener {
            params.push(("IncludeListener".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.next_token {
            params.push(("NextToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_items {
            params.push(("MaxItems".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListTLSCipherPoliciesResponse {
    /// 是否拥有下一次查询的令牌（Token）。取值：
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 本次请求条件下的TLS策略总数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 是否结束。取值：
    #[serde(rename = "IsTruncated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    /// TLS策略列表。
    #[serde(rename = "TLSCipherPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_cipher_policies: Option<Vec<ListTLSCipherPoliciesResponseTLSCipherPoliciesItem>>,
}

/// CreateAccessControlList 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateAccessControlListRequest {
    /// 访问控制策略组的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 访问控制策略组名称，长度限制为1~80个字符，只支持中文、字母、数字和半角句号（.）、短划线（-）、正斜线（/）和下划线（_）。访问控制策略组名称必须为地域内唯一。
    #[serde(rename = "AclName")]
    pub acl_name: String,
    /// IP版本，可以设置为**ipv4**或者**ipv6**。
    #[serde(rename = "AddressIPVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_ip_version: Option<String>,
    /// 访问控制策略组所在的资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 标签列表。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<CreateAccessControlListRequestTagItem>>,
}

impl CreateAccessControlListRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("AclName".to_string(), self.acl_name.to_string()));
        if let Some(ref v) = self.address_ip_version {
            params.push(("AddressIPVersion".to_string(), v.to_string()));
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
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateAccessControlListResponse {
    /// 访问控制策略组ID。
    #[serde(rename = "AclId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// AddAccessControlListEntry 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct AddAccessControlListEntryRequest {
    /// 访问控制策略组的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 访问控制策略组ID。
    #[serde(rename = "AclId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_id: Option<String>,
    /// 设置访问控制策略组。取值：
    #[serde(rename = "AclEntrys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_entrys: Option<String>,
}

impl AddAccessControlListEntryRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.acl_id {
            params.push(("AclId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.acl_entrys {
            params.push(("AclEntrys".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AddAccessControlListEntryResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteAccessControlList 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteAccessControlListRequest {
    /// 访问控制策略组的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 访问控制策略组ID。
    #[serde(rename = "AclId")]
    pub acl_id: String,
}

impl DeleteAccessControlListRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("AclId".to_string(), self.acl_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteAccessControlListResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// SetAccessControlListAttribute 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetAccessControlListAttributeRequest {
    /// 访问控制策略组的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 访问控制策略组ID。
    #[serde(rename = "AclId")]
    pub acl_id: String,
    /// 访问控制策略组名称。
    #[serde(rename = "AclName")]
    pub acl_name: String,
}

impl SetAccessControlListAttributeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("AclId".to_string(), self.acl_id.to_string()));
        params.push(("AclName".to_string(), self.acl_name.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetAccessControlListAttributeResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeAccessControlListAttribute 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeAccessControlListAttributeRequest {
    /// 访问控制策略组的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 要查询的访问控制策略组ID。
    #[serde(rename = "AclId")]
    pub acl_id: String,
    /// 访问控制策略组的条目的备注信息。
    #[serde(rename = "AclEntryComment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_entry_comment: Option<String>,
    /// 分页查询页码。
    #[serde(rename = "Page")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    /// 分页查询时每页的行数。最大值：**50**。默认值：**10**。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl DescribeAccessControlListAttributeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("AclId".to_string(), self.acl_id.to_string()));
        if let Some(ref v) = self.acl_entry_comment {
            params.push(("AclEntryComment".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page {
            params.push(("Page".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeAccessControlListAttributeResponse {
    /// 访问控制策略组ID。
    #[serde(rename = "AclId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<DescribeAccessControlListAttributeResponseTags>,
    /// 关联的实例的IP类型。
    #[serde(rename = "AddressIPVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_ip_version: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 企业资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 访问控制策略组名称。
    #[serde(rename = "AclName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_name: Option<String>,
    #[serde(rename = "AclEntrys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_entrys: Option<DescribeAccessControlListAttributeResponseAclEntrys>,
    #[serde(rename = "RelatedListeners")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_listeners: Option<DescribeAccessControlListAttributeResponseRelatedListeners>,
    /// 访问控制策略的创建时间，格式为：`YYYY-MM-DDThh:mm:ssZ`。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 访问控制条目总数。
    #[serde(rename = "TotalAclEntry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_acl_entry: Option<i32>,
}

/// DescribeAccessControlLists 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeAccessControlListsRequest {
    /// 访问控制策略组的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 访问控制策略组名称。访问控制策略组名称。长度限制为1~80个字符，只支持中文、字母、数字和半角句号（.）、短划线（-）、正斜线（/）和下划线（_）。访问控制策略组名称必须为地域内唯一。访问控制策...
    #[serde(rename = "AclName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_name: Option<String>,
    /// 访问控制策略组绑定的实例的IP类型。取值：
    #[serde(rename = "AddressIPVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_ip_version: Option<String>,
    /// 分页查询时每页的行数，最大值：**50**，默认值：**10**。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 列表的页码，默认值：**1**。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 企业资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 标签列表。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<DescribeAccessControlListsRequestTagItem>>,
}

impl DescribeAccessControlListsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.acl_name {
            params.push(("AclName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.address_ip_version {
            params.push(("AddressIPVersion".to_string(), v.to_string()));
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
pub struct DescribeAccessControlListsResponse {
    /// 列表的页码，起始值**1**，默认值：**1**。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 分页查询时每页的行数，最大值：**50**，默认值：**10**。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 已创建的访问控制组策略组个数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 当前页展示的访问控制策略组个数。
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "Acls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acls: Option<DescribeAccessControlListsResponseAcls>,
}

/// RemoveAccessControlListEntry 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct RemoveAccessControlListEntryRequest {
    /// 访问控制策略组的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 访问控制策略组ID。
    #[serde(rename = "AclId")]
    pub acl_id: String,
    /// 设置访问控制策略组。取值：
    #[serde(rename = "AclEntrys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_entrys: Option<String>,
}

impl RemoveAccessControlListEntryRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("AclId".to_string(), self.acl_id.to_string()));
        if let Some(ref v) = self.acl_entrys {
            params.push(("AclEntrys".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RemoveAccessControlListEntryResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// SetListenerAccessControlStatus 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetListenerAccessControlStatusRequest {
    /// 负载均衡实例的地域。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 负载均衡实例的ID。
    #[serde(rename = "LoadBalancerId")]
    pub load_balancer_id: String,
    /// 负载均衡实例前端使用的端口。
    #[serde(rename = "ListenerPort")]
    pub listener_port: i32,
    /// 是否开启访问控制。取值：
    #[serde(rename = "AccessControlStatus")]
    pub access_control_status: String,
    /// 负载均衡实例前端使用的协议。
    #[serde(rename = "ListenerProtocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_protocol: Option<String>,
}

impl SetListenerAccessControlStatusRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params.push(("LoadBalancerId".to_string(), self.load_balancer_id.to_string()));
        params.push(("ListenerPort".to_string(), self.listener_port.to_string()));
        params.push(("AccessControlStatus".to_string(), self.access_control_status.to_string()));
        if let Some(ref v) = self.listener_protocol {
            params.push(("ListenerProtocol".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetListenerAccessControlStatusResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// RemoveListenerWhiteListItem 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct RemoveListenerWhiteListItemRequest {
    /// 负载均衡实例的地域。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 负载均衡实例的ID。
    #[serde(rename = "LoadBalancerId")]
    pub load_balancer_id: String,
    /// 监听端口。
    #[serde(rename = "ListenerPort")]
    pub listener_port: i32,
    /// 访问控制列表。支持输入IP地址或IP地址段（CIDR block形式），多个IP地址或地址段用逗号（,）分隔。
    #[serde(rename = "SourceItems")]
    pub source_items: String,
    /// 负载均衡实例前端使用的协议。
    #[serde(rename = "ListenerProtocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_protocol: Option<String>,
}

impl RemoveListenerWhiteListItemRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params.push(("LoadBalancerId".to_string(), self.load_balancer_id.to_string()));
        params.push(("ListenerPort".to_string(), self.listener_port.to_string()));
        params.push(("SourceItems".to_string(), self.source_items.to_string()));
        if let Some(ref v) = self.listener_protocol {
            params.push(("ListenerProtocol".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RemoveListenerWhiteListItemResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// AddListenerWhiteListItem 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct AddListenerWhiteListItemRequest {
    /// 负载均衡实例所在的地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 负载均衡实例的ID。
    #[serde(rename = "LoadBalancerId")]
    pub load_balancer_id: String,
    /// 负载均衡实例前端使用的端口。
    #[serde(rename = "ListenerPort")]
    pub listener_port: i32,
    /// 访问控制列表。
    #[serde(rename = "SourceItems")]
    pub source_items: String,
    /// 负载均衡实例前端使用的协议。
    #[serde(rename = "ListenerProtocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_protocol: Option<String>,
}

impl AddListenerWhiteListItemRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params.push(("LoadBalancerId".to_string(), self.load_balancer_id.to_string()));
        params.push(("ListenerPort".to_string(), self.listener_port.to_string()));
        params.push(("SourceItems".to_string(), self.source_items.to_string()));
        if let Some(ref v) = self.listener_protocol {
            params.push(("ListenerProtocol".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AddListenerWhiteListItemResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeListenerAccessControlAttribute 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeListenerAccessControlAttributeRequest {
    /// 负载均衡实例的地域。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 负载均衡实例的ID。
    #[serde(rename = "LoadBalancerId")]
    pub load_balancer_id: String,
    /// 负载均衡实例前端使用的端口。
    #[serde(rename = "ListenerPort")]
    pub listener_port: i32,
    /// 负载均衡实例前端使用的协议。
    #[serde(rename = "ListenerProtocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_protocol: Option<String>,
}

impl DescribeListenerAccessControlAttributeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params.push(("LoadBalancerId".to_string(), self.load_balancer_id.to_string()));
        params.push(("ListenerPort".to_string(), self.listener_port.to_string()));
        if let Some(ref v) = self.listener_protocol {
            params.push(("ListenerProtocol".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeListenerAccessControlAttributeResponse {
    /// 访问控制列表。
    #[serde(rename = "SourceItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_items: Option<String>,
    /// 是否开启访问控制。取值：
    #[serde(rename = "AccessControlStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control_status: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// AddTags 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct AddTagsRequest {
    /// 负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 负载均衡实例ID。
    #[serde(rename = "LoadBalancerId")]
    pub load_balancer_id: String,
    /// 需要添加的Tag列表。
    #[serde(rename = "Tags")]
    pub tags: String,
}

impl AddTagsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("LoadBalancerId".to_string(), self.load_balancer_id.to_string()));
        params.push(("Tags".to_string(), self.tags.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AddTagsResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeTags 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeTagsRequest {
    /// 负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 负载均衡实例ID。
    #[serde(rename = "LoadBalancerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_id: Option<String>,
    /// 要查询的标签列表。
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    /// 是否为DistinctKey。
    #[serde(rename = "DistinctKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distinct_key: Option<bool>,
    /// 单页结果数量，接口默认50，最大100。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 实例列表页码，起始值1，默认值1。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
}

impl DescribeTagsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.load_balancer_id {
            params.push(("LoadBalancerId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            params.push(("Tags".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.distinct_key {
            params.push(("DistinctKey".to_string(), v.to_string()));
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
pub struct DescribeTagsResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 默认50，最大100。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 实例列表页码，起始值1，默认值1。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 根据过滤条件得到的实例总个数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    #[serde(rename = "TagSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_sets: Option<DescribeTagsResponseTagSets>,
}

/// ListTagResources 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListTagResourcesRequest {
    /// 传统型负载均衡实例所在的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 资源类型定义，取值：
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// 是否拥有下一次查询的令牌（Token）。取值：
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 资源ID。最多支持添加20个资源。
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<Vec<String>>,
    /// 标签列表。
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

#[derive(Debug, Clone, Deserialize)]
pub struct ListTagResourcesResponse {
    /// 是否拥有下一次查询的令牌（Token）。取值：
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
    /// 负载均衡实例所在的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 资源类型定义，取值：
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// 资源ID，最多支持20个资源ID。
    #[serde(rename = "ResourceId")]
    pub resource_id: Vec<String>,
    /// 标签列表。
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
    /// 传统型负载均衡实例所在的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 资源类型定义，取值：
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// 是否全部删除，只针对**TagKey.N**为空时有效。
    #[serde(rename = "All")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
    /// 资源ID，最多支持20个资源ID。
    #[serde(rename = "ResourceId")]
    pub resource_id: Vec<String>,
    /// 资源的标签键。最多支持20个标签键。一旦传入该值，则不允许为空字符串。
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

/// RemoveTags 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct RemoveTagsRequest {
    /// 负载均衡实例所属地域的ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 负载均衡实例ID。
    #[serde(rename = "LoadBalancerId")]
    pub load_balancer_id: String,
    /// 需要解绑的Tag列表。
    #[serde(rename = "Tags")]
    pub tags: String,
}

impl RemoveTagsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("LoadBalancerId".to_string(), self.load_balancer_id.to_string()));
        params.push(("Tags".to_string(), self.tags.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RemoveTagsResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteAccessLogsDownloadAttribute 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteAccessLogsDownloadAttributeRequest {
    /// 传统型负载均衡实例所在的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 传统型负载均衡实例绑定的标签列表，其结构是一个JSON dictionary，包含标签键和标签值。
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    /// 传统型负载均衡实例的ID。
    #[serde(rename = "LoadBalancerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_id: Option<String>,
    /// 访问日志转发规则。组成参数：
    #[serde(rename = "LogsDownloadAttributes")]
    pub logs_download_attributes: String,
}

impl DeleteAccessLogsDownloadAttributeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.tags {
            params.push(("Tags".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.load_balancer_id {
            params.push(("LoadBalancerId".to_string(), v.to_string()));
        }
        params.push(("LogsDownloadAttributes".to_string(), self.logs_download_attributes.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteAccessLogsDownloadAttributeResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// SetAccessLogsDownloadAttribute 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetAccessLogsDownloadAttributeRequest {
    /// 传统型负载均衡实例所在的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 传统型负载均衡实例绑定的标签列表，其结构是一个JSON dictionary，包含标签键和标签值。
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    /// 传统型负载均衡实例的ID。
    #[serde(rename = "LoadBalancerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_id: Option<String>,
    /// 访问日志转发规则。组成参数：
    #[serde(rename = "LogsDownloadAttributes")]
    pub logs_download_attributes: String,
}

impl SetAccessLogsDownloadAttributeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.tags {
            params.push(("Tags".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.load_balancer_id {
            params.push(("LoadBalancerId".to_string(), v.to_string()));
        }
        params.push(("LogsDownloadAttributes".to_string(), self.logs_download_attributes.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetAccessLogsDownloadAttributeResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeAccessLogsDownloadAttribute 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeAccessLogsDownloadAttributeRequest {
    /// 传统型负载均衡实例所在的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 传统型负载均衡实例绑定的标签列表，其结构是一个JSON dictionary，包含标签键和标签值。
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    /// 传统型负载均衡实例的ID。
    #[serde(rename = "LoadBalancerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_id: Option<String>,
    /// 访问日志类型。仅取值**layer7**，表示七层访问日志。
    #[serde(rename = "LogType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_type: Option<String>,
    /// 列表的页码。默认值：**1**。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 分页查询时每页的行数。最大值：**50**。默认值：**10**。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl DescribeAccessLogsDownloadAttributeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.tags {
            params.push(("Tags".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.load_balancer_id {
            params.push(("LoadBalancerId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.log_type {
            params.push(("LogType".to_string(), v.to_string()));
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
pub struct DescribeAccessLogsDownloadAttributeResponse {
    /// 分页查询时每页的行数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 列表的页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 列表条目数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    #[serde(rename = "LogsDownloadAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs_download_attributes: Option<DescribeAccessLogsDownloadAttributeResponseLogsDownloadAttributes>,
}

/// EnableHighDefinationMonitor 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct EnableHighDefinationMonitorRequest {
    /// 日志标签。其结构是一个JSON dictionary，包含标签键和标签值。
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    /// 传统型负载均衡的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 日志服务LogProject的名称。长度为4~63个字符，支持数字、小写字母，可包含短划线（-），开头和结尾必须为数字或字母。
    #[serde(rename = "LogProject")]
    pub log_project: String,
    /// 日志服务LogStore的名称。长度为2~64个字符，支持数字、小写字母，可包含短划线（-）和下划线（_），开头和结尾必须为数字或字母。
    #[serde(rename = "LogStore")]
    pub log_store: String,
}

impl EnableHighDefinationMonitorRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.tags {
            params.push(("Tags".to_string(), v.to_string()));
        }
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("LogProject".to_string(), self.log_project.to_string()));
        params.push(("LogStore".to_string(), self.log_store.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnableHighDefinationMonitorResponse {
    /// 本次调用是否成功。取值：
    #[serde(rename = "Success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyHighDefinationMonitor 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyHighDefinationMonitorRequest {
    /// 传统型负载均衡实例所在的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 修改后的日志服务LogProject的名称。日志服务LogProject的名称。长度为4~63个字符，支持数字、小写字母，可包含短划线（-），开头和结尾必须为数字或字母。
    #[serde(rename = "LogProject")]
    pub log_project: String,
    /// 修改后的日志服务LogStore的名称。长度为2~64个字符，支持数字、小写字母，可包含短划线（-）和下划线（_），开头和结尾必须为数字或字母。
    #[serde(rename = "LogStore")]
    pub log_store: String,
}

impl ModifyHighDefinationMonitorRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("LogProject".to_string(), self.log_project.to_string()));
        params.push(("LogStore".to_string(), self.log_store.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyHighDefinationMonitorResponse {
    /// 本次调用是否成功。取值：
    #[serde(rename = "Success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeHighDefinationMonitor 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeHighDefinationMonitorRequest {
    /// 日志标签。其结构是一个JSON dictionary，包含标签键和标签值。
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    /// 需要查询秒级监控信息的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
}

impl DescribeHighDefinationMonitorRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.tags {
            params.push(("Tags".to_string(), v.to_string()));
        }
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeHighDefinationMonitorResponse {
    /// 日志服务LogProject的名称。
    #[serde(rename = "LogProject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_project: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 日志服务LogStore的名称。
    #[serde(rename = "LogStore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_store: Option<String>,
    /// 本次调用是否成功。取值：
    #[serde(rename = "Success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<String>,
}

/// MoveResourceGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct MoveResourceGroupRequest {
    /// 负载均衡实例的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 需要修改资源组的目标资源的ID。
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// 资源类型定义，取值：
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// 新资源组ID。
    #[serde(rename = "NewResourceGroupId")]
    pub new_resource_group_id: String,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 阿里云颁发给用户的访问服务所用的密钥ID。
    #[serde(rename = "access_key_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
}

impl MoveResourceGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("ResourceId".to_string(), self.resource_id.to_string()));
        params.push(("ResourceType".to_string(), self.resource_type.to_string()));
        params.push(("NewResourceGroupId".to_string(), self.new_resource_group_id.to_string()));
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.access_key_id {
            params.push(("access_key_id".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct MoveResourceGroupResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
