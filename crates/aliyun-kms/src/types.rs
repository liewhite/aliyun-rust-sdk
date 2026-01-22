//! 类型定义 - 自动生成，请勿手动修改

#![allow(unused_mut)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRegionsResponseRegionsRegionItem {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
}

impl DescribeRegionsResponseRegionsRegionItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRegionsResponseRegions {
    /// 地域。
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

/// KMS实例列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListKmsInstancesResponseKmsInstancesKmsInstanceItem {
    /// KMS实例的ARN。
    #[serde(rename = "KmsInstanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_instance_arn: Option<String>,
    /// KMS实例的ID。
    #[serde(rename = "KmsInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_instance_id: Option<String>,
}

impl ListKmsInstancesResponseKmsInstancesKmsInstanceItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.kms_instance_arn {
            params.push(("KmsInstanceArn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.kms_instance_id {
            params.push(("KmsInstanceId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListKmsInstancesResponseKmsInstances {
    /// KMS实例列表。
    #[serde(rename = "KmsInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_instance: Option<Vec<ListKmsInstancesResponseKmsInstancesKmsInstanceItem>>,
}

impl ListKmsInstancesResponseKmsInstances {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.kms_instance {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("KmsInstance.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 配置的VPC列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetKmsInstanceResponseKmsInstanceBindVpcsBindVpcItem {
    /// VPC所属的地域。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// VPC的ID。
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// VPC所属的阿里云账号。
    #[serde(rename = "VpcOwnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_owner_id: Option<String>,
    /// VPC下的交换机。
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
}

impl GetKmsInstanceResponseKmsInstanceBindVpcsBindVpcItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_id {
            params.push(("VpcId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_owner_id {
            params.push(("VpcOwnerId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("VSwitchId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetKmsInstanceResponseKmsInstanceBindVpcs {
    /// 配置的VPC列表。
    #[serde(rename = "BindVpc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_vpc: Option<Vec<GetKmsInstanceResponseKmsInstanceBindVpcsBindVpcItem>>,
}

impl GetKmsInstanceResponseKmsInstanceBindVpcs {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.bind_vpc {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("BindVpc.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// KMS实例详情。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetKmsInstanceResponseKmsInstance {
    /// KMS实例的ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// KMS实例的名称。
    #[serde(rename = "InstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    /// KMS实例的状态。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// KMS实例创建的时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// KMS实例的计算性能。
    #[serde(rename = "Spec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<i64>,
    /// KMS实例支持创建的密钥数量。
    #[serde(rename = "KeyNum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_num: Option<i64>,
    /// KMS实例支持创建的凭据数量。
    #[serde(rename = "SecretNum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_num: Option<String>,
    /// KMS实例的访问管理总量。
    #[serde(rename = "VpcNum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_num: Option<i64>,
    /// KMS实例绑定的VPC。
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// KMS实例绑定的可用区。
    #[serde(rename = "ZoneIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_ids: Option<Vec<String>>,
    /// KMS实例绑定的VPC中的交换机。
    #[serde(rename = "VswitchIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vswitch_ids: Option<Vec<String>>,
    /// KMS实例的到期时间。
    #[serde(rename = "EndDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// KMS实例启用的时间。
    #[serde(rename = "StartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// KMS实例的CA证书的内容。
    #[serde(rename = "CaCertificateChainPem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_certificate_chain_pem: Option<String>,
    #[serde(rename = "BindVpcs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_vpcs: Option<GetKmsInstanceResponseKmsInstanceBindVpcs>,
    /// 实例的付费类型，取值：
    #[serde(rename = "ChargeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_type: Option<String>,
    /// KMS实例的版本。
    #[serde(rename = "ProductVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_version: Option<String>,
    #[serde(rename = "SaleStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sale_status: Option<String>,
    #[serde(rename = "Log")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log: Option<i64>,
    #[serde(rename = "LogStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_storage: Option<i64>,
    #[serde(rename = "ProductType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<String>,
}

impl GetKmsInstanceResponseKmsInstance {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_name {
            params.push(("InstanceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.spec {
            params.push(("Spec".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key_num {
            params.push(("KeyNum".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.secret_num {
            params.push(("SecretNum".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_num {
            params.push(("VpcNum".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_id {
            params.push(("VpcId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.zone_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("ZoneIds.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.vswitch_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("VswitchIds.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.end_date {
            params.push(("EndDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_date {
            params.push(("StartDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ca_certificate_chain_pem {
            params.push(("CaCertificateChainPem".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bind_vpcs {
            for (k, v2) in v.to_query_params() {
                params.push((format!("BindVpcs.{}", k), v2));
            }
        }
        if let Some(ref v) = self.charge_type {
            params.push(("ChargeType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.product_version {
            params.push(("ProductVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sale_status {
            params.push(("SaleStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.log {
            params.push(("Log".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.log_storage {
            params.push(("LogStorage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.product_type {
            params.push(("ProductType".to_string(), v.to_string()));
        }
        params
    }
}

/// 密钥的元数据。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateKeyResponseKeyMetadata {
    /// 密钥的全局唯一标识符。
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// 密钥下一次轮转的时间。
    #[serde(rename = "NextRotationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_rotation_date: Option<String>,
    /// 密钥的状态。
    #[serde(rename = "KeyState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_state: Option<String>,
    /// 密钥自动轮转的周期。单位为秒，格式为整数值后加上字符s。例如：7天的轮转周期为604800s。
    #[serde(rename = "RotationInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_interval: Option<String>,
    /// 密钥的ARN。
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// 密钥的创建者。
    #[serde(rename = "Creator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    /// 最近一次轮转的时间（UTC）。
    #[serde(rename = "LastRotationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_rotation_date: Option<String>,
    /// 密钥的预计删除时间。 更多信息，请参见[ScheduleKeyDeletion](~~601417~~)。
    #[serde(rename = "DeleteDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_date: Option<String>,
    /// 密钥的当前主版本标识符。
    #[serde(rename = "PrimaryKeyVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_key_version: Option<String>,
    /// 密钥的描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 密钥的规格。
    #[serde(rename = "KeySpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_spec: Option<String>,
    /// 密钥材料来源。
    #[serde(rename = "Origin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    /// 密钥材料的过期时间（UTC）。
    #[serde(rename = "MaterialExpireTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material_expire_time: Option<String>,
    /// 是否开启了密钥自动轮转，取值：
    #[serde(rename = "AutomaticRotation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_rotation: Option<String>,
    /// 密钥的保护级别。
    #[serde(rename = "ProtectionLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection_level: Option<String>,
    /// 密钥的用途。
    #[serde(rename = "KeyUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_usage: Option<String>,
    /// 密钥创建的日期和时间（UTC）。
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// KMS实例的实例ID。
    #[serde(rename = "DKMSInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkms_instance_id: Option<String>,
}

impl CreateKeyResponseKeyMetadata {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key_id {
            params.push(("KeyId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.next_rotation_date {
            params.push(("NextRotationDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key_state {
            params.push(("KeyState".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rotation_interval {
            params.push(("RotationInterval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.arn {
            params.push(("Arn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.creator {
            params.push(("Creator".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.last_rotation_date {
            params.push(("LastRotationDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.delete_date {
            params.push(("DeleteDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.primary_key_version {
            params.push(("PrimaryKeyVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key_spec {
            params.push(("KeySpec".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.origin {
            params.push(("Origin".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.material_expire_time {
            params.push(("MaterialExpireTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.automatic_rotation {
            params.push(("AutomaticRotation".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.protection_level {
            params.push(("ProtectionLevel".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key_usage {
            params.push(("KeyUsage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.creation_date {
            params.push(("CreationDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dkms_instance_id {
            params.push(("DKMSInstanceId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListKeysResponseKeysKeyItem {
    /// 主密钥的全局唯一标识符。
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// 主密钥的ARN。
    #[serde(rename = "KeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_arn: Option<String>,
}

impl ListKeysResponseKeysKeyItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key_id {
            params.push(("KeyId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key_arn {
            params.push(("KeyArn".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListKeysResponseKeys {
    /// 主密钥。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<Vec<ListKeysResponseKeysKeyItem>>,
}

impl ListKeysResponseKeys {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Key.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// CMK的元数据。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeKeyResponseKeyMetadata {
    /// 是否开启删除保护，取值：
    #[serde(rename = "DeletionProtection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<String>,
    /// 密钥ID。如果请求中的KeyId参数使用的是密钥别名、密钥ARN，在响应中也会返回密钥ID。
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// 下一次轮转的时间。
    #[serde(rename = "NextRotationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_rotation_date: Option<String>,
    /// CMK的状态。
    #[serde(rename = "KeyState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_state: Option<String>,
    /// 密钥自动轮转的周期。
    #[serde(rename = "RotationInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_interval: Option<String>,
    /// CMK的资源名称（ARN）。
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// 创建CMK的阿里云账号。
    #[serde(rename = "Creator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    /// 最近一次轮转的时间（UTC）。如果是新创建的密钥，则为初始密钥版本生成时间。
    #[serde(rename = "LastRotationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_rotation_date: Option<String>,
    /// CMK的预计删除时间（UTC）。
    #[serde(rename = "DeleteDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_date: Option<String>,
    /// 对称类型CMK当前的主版本标识符。
    #[serde(rename = "PrimaryKeyVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_key_version: Option<String>,
    /// CMK的描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// CMK的类型。
    #[serde(rename = "KeySpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_spec: Option<String>,
    /// CMK的密钥材料来源。
    #[serde(rename = "Origin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    /// 密钥材料的过期时间（UTC）。当该值为空时，表示密钥材料不会过期。
    #[serde(rename = "MaterialExpireTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material_expire_time: Option<String>,
    /// 删除保护描述。
    #[serde(rename = "DeletionProtectionDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection_description: Option<String>,
    /// 是否开启自动轮转，取值：
    #[serde(rename = "AutomaticRotation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_rotation: Option<String>,
    /// CMK的保护级别。
    #[serde(rename = "ProtectionLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection_level: Option<String>,
    /// CMK的用途。
    #[serde(rename = "KeyUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_usage: Option<String>,
    /// CMK的创建时间（UTC）。
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// KMS实例的实例ID。
    #[serde(rename = "DKMSInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkms_instance_id: Option<String>,
}

impl DescribeKeyResponseKeyMetadata {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.deletion_protection {
            params.push(("DeletionProtection".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key_id {
            params.push(("KeyId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.next_rotation_date {
            params.push(("NextRotationDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key_state {
            params.push(("KeyState".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rotation_interval {
            params.push(("RotationInterval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.arn {
            params.push(("Arn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.creator {
            params.push(("Creator".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.last_rotation_date {
            params.push(("LastRotationDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.delete_date {
            params.push(("DeleteDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.primary_key_version {
            params.push(("PrimaryKeyVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key_spec {
            params.push(("KeySpec".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.origin {
            params.push(("Origin".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.material_expire_time {
            params.push(("MaterialExpireTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.deletion_protection_description {
            params.push(("DeletionProtectionDescription".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.automatic_rotation {
            params.push(("AutomaticRotation".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.protection_level {
            params.push(("ProtectionLevel".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key_usage {
            params.push(("KeyUsage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.creation_date {
            params.push(("CreationDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dkms_instance_id {
            params.push(("DKMSInstanceId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListAliasesResponseAliasesAliasItem {
    /// 别名对应的主密钥（CMK）。
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// 别名的ARN。
    #[serde(rename = "AliasArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_arn: Option<String>,
    /// 别名的唯一标识符。
    #[serde(rename = "AliasName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_name: Option<String>,
}

impl ListAliasesResponseAliasesAliasItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key_id {
            params.push(("KeyId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.alias_arn {
            params.push(("AliasArn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.alias_name {
            params.push(("AliasName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListAliasesResponseAliases {
    /// 用户别名。
    #[serde(rename = "Alias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<ListAliasesResponseAliasesAliasItem>>,
}

impl ListAliasesResponseAliases {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.alias {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Alias.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListAliasesByKeyIdResponseAliasesAliasItem {
    /// 密钥ID。如果请求中的KeyId参数使用的是密钥ARN，在响应中也会返回密钥ID。
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// 别名的ARN。
    #[serde(rename = "AliasArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_arn: Option<String>,
    /// 别名的唯一标识符。
    #[serde(rename = "AliasName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_name: Option<String>,
}

impl ListAliasesByKeyIdResponseAliasesAliasItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key_id {
            params.push(("KeyId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.alias_arn {
            params.push(("AliasArn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.alias_name {
            params.push(("AliasName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListAliasesByKeyIdResponseAliases {
    /// 别名。
    #[serde(rename = "Alias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<ListAliasesByKeyIdResponseAliasesAliasItem>>,
}

impl ListAliasesByKeyIdResponseAliases {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.alias {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Alias.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 密钥版本的元数据。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeKeyVersionResponseKeyVersion {
    /// 密钥ID。如果请求中的KeyId参数使用的是密钥别名、密钥ARN，在响应中也会返回密钥ID。
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// 密钥版本的全局唯一标识符。
    #[serde(rename = "KeyVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_version_id: Option<String>,
    /// 创建密钥版本时的日期和时间（UTC时间）。
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
}

impl DescribeKeyVersionResponseKeyVersion {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key_id {
            params.push(("KeyId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key_version_id {
            params.push(("KeyVersionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.creation_date {
            params.push(("CreationDate".to_string(), v.to_string()));
        }
        params
    }
}

/// 密钥版本的元数据。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateKeyVersionResponseKeyVersion {
    /// 密钥ID。
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// 密钥版本ID。
    #[serde(rename = "KeyVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_version_id: Option<String>,
    /// 创建密钥版本的时间（UTC时间）。
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
}

impl CreateKeyVersionResponseKeyVersion {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key_id {
            params.push(("KeyId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key_version_id {
            params.push(("KeyVersionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.creation_date {
            params.push(("CreationDate".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回的密钥版本数组。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListKeyVersionsResponseKeyVersionsKeyVersionItem {
    /// 密钥ID。如果请求中的KeyId参数使用的是密钥别名、密钥ARN，在响应中也会返回密钥ID。
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// 密钥版本的全局唯一标识符。
    #[serde(rename = "KeyVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_version_id: Option<String>,
    /// 创建密钥版本时的日期和时间（UTC时间）。
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
}

impl ListKeyVersionsResponseKeyVersionsKeyVersionItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key_id {
            params.push(("KeyId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key_version_id {
            params.push(("KeyVersionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.creation_date {
            params.push(("CreationDate".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListKeyVersionsResponseKeyVersions {
    /// 返回的密钥版本数组。
    #[serde(rename = "KeyVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_version: Option<Vec<ListKeyVersionsResponseKeyVersionsKeyVersionItem>>,
}

impl ListKeyVersionsResponseKeyVersions {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key_version {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("KeyVersion.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 凭据的资源标签。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListSecretsResponseSecretListSecretItemTagsTagItem {
    /// 标签值。
    #[serde(rename = "TagValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
    /// 标签键。
    #[serde(rename = "TagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
}

impl ListSecretsResponseSecretListSecretItemTagsTagItem {
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
pub struct ListSecretsResponseSecretListSecretItemTags {
    /// 凭据的资源标签。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<ListSecretsResponseSecretListSecretItemTagsTagItem>>,
}

impl ListSecretsResponseSecretListSecretItemTags {
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
pub struct ListSecretsResponseSecretListSecretItem {
    /// 凭据名称。
    #[serde(rename = "SecretName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<String>,
    /// 更新时间。
    #[serde(rename = "UpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 凭据类型。取值：
    #[serde(rename = "SecretType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_type: Option<String>,
    /// 计划删除时间。
    #[serde(rename = "PlannedDeleteTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planned_delete_time: Option<String>,
    /// 创建时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<ListSecretsResponseSecretListSecretItemTags>,
    #[serde(rename = "OwingService")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owing_service: Option<String>,
}

impl ListSecretsResponseSecretListSecretItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.secret_name {
            params.push(("SecretName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_time {
            params.push(("UpdateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.secret_type {
            params.push(("SecretType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.planned_delete_time {
            params.push(("PlannedDeleteTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Tags.{}", k), v2));
            }
        }
        if let Some(ref v) = self.owing_service {
            params.push(("OwingService".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListSecretsResponseSecretList {
    /// 凭据列表。
    #[serde(rename = "Secret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<Vec<ListSecretsResponseSecretListSecretItem>>,
}

impl ListSecretsResponseSecretList {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.secret {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Secret.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeSecretResponseTagsTagItem {
    /// 标签值。
    #[serde(rename = "TagValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
    /// 标签键。
    #[serde(rename = "TagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
}

impl DescribeSecretResponseTagsTagItem {
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
pub struct DescribeSecretResponseTags {
    /// 凭据的资源标签。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<DescribeSecretResponseTagsTagItem>>,
}

impl DescribeSecretResponseTags {
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
pub struct GetSecretValueResponseVersionStages {
    /// 凭据版本的状态标记。
    #[serde(rename = "VersionStage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_stage: Option<Vec<String>>,
}

impl GetSecretValueResponseVersionStages {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.version_stage {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("VersionStage.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListSecretVersionIdsResponseVersionIdsVersionIdItemVersionStages {
    /// 版本的状态标记。
    #[serde(rename = "VersionStage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_stage: Option<Vec<String>>,
}

impl ListSecretVersionIdsResponseVersionIdsVersionIdItemVersionStages {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.version_stage {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("VersionStage.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 凭据的版本信息列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListSecretVersionIdsResponseVersionIdsVersionIdItem {
    /// 版本号。
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    /// 版本的创建时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(rename = "VersionStages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_stages: Option<ListSecretVersionIdsResponseVersionIdsVersionIdItemVersionStages>,
}

impl ListSecretVersionIdsResponseVersionIdsVersionIdItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.version_id {
            params.push(("VersionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version_stages {
            for (k, v2) in v.to_query_params() {
                params.push((format!("VersionStages.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListSecretVersionIdsResponseVersionIds {
    /// 凭据的版本信息列表。
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<Vec<ListSecretVersionIdsResponseVersionIdsVersionIdItem>>,
}

impl ListSecretVersionIdsResponseVersionIds {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.version_id {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("VersionId.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PutSecretValueResponseVersionStages {
    /// 凭据的版本状态。
    #[serde(rename = "VersionStage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_stage: Option<Vec<String>>,
}

impl PutSecretValueResponseVersionStages {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.version_stage {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("VersionStage.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 要查询的标签列表。N的取值范围1~20。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListTagResourcesRequestTagItem {
    /// 标签键。每个标签由一个键值对（Key:Value）组成，包含标签键（Key）、标签值（Value）。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。每个标签由一个键值对（Key:Value）组成，包含标签键（Key）、标签值（Value）。
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

/// 资源的标签列表。
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
    /// 资源的标签列表。
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

/// 标签列表，最多可以输入20个标签。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TagResourcesRequestTagItem {
    /// 标签键。每个标签由一个键值对（Key:Value）组成，包含标签键（Key）、标签值（Value）。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。每个标签由一个键值对（Key:Value）组成，包含标签键（Key）、标签值（Value）。
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListResourceTagsResponseTagsTagItem {
    /// 全局唯一标识符。
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// 标签值。
    #[serde(rename = "TagValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
    /// 标签键。
    #[serde(rename = "TagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
}

impl ListResourceTagsResponseTagsTagItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key_id {
            params.push(("KeyId".to_string(), v.to_string()));
        }
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
pub struct ListResourceTagsResponseTags {
    /// 标签。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<ListResourceTagsResponseTagsTagItem>>,
}

impl ListResourceTagsResponseTags {
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

/// 网络控制规则列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListNetworkRulesResponseNetworkRulesNetworkRuleItem {
    /// 网络类型。取值仅支持Private，即自建应用仅支持通过私网VPC访问KMS实例。
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 网络控制规则名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ListNetworkRulesResponseNetworkRulesNetworkRuleItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListNetworkRulesResponseNetworkRules {
    /// 网络控制规则列表。
    #[serde(rename = "NetworkRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_rule: Option<Vec<ListNetworkRulesResponseNetworkRulesNetworkRuleItem>>,
}

impl ListNetworkRulesResponseNetworkRules {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.network_rule {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("NetworkRule.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 权限策略列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListPoliciesResponsePoliciesPolicyItem {
    /// 权限策略名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ListPoliciesResponsePoliciesPolicyItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListPoliciesResponsePolicies {
    /// 权限策略列表。
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<Vec<ListPoliciesResponsePoliciesPolicyItem>>,
}

impl ListPoliciesResponsePolicies {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.policy {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Policy.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 应用接入点列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListApplicationAccessPointsResponseApplicationAccessPointsApplicationAccessPointItem {
    /// 应用接入点名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 认证方式。
    #[serde(rename = "AuthenticationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_method: Option<String>,
}

impl ListApplicationAccessPointsResponseApplicationAccessPointsApplicationAccessPointItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.authentication_method {
            params.push(("AuthenticationMethod".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListApplicationAccessPointsResponseApplicationAccessPoints {
    /// 应用接入点列表。
    #[serde(rename = "ApplicationAccessPoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_access_point: Option<Vec<ListApplicationAccessPointsResponseApplicationAccessPointsApplicationAccessPointItem>>,
}

impl ListApplicationAccessPointsResponseApplicationAccessPoints {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.application_access_point {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("ApplicationAccessPoint.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// ClientKey列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListClientKeysResponseClientKeysItem {
    /// ClientKey的ID。
    #[serde(rename = "ClientKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_key_id: Option<String>,
    /// ClientKey的创建时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// ClientKey的公钥内容。
    #[serde(rename = "PublicKeyData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key_data: Option<String>,
    /// ClientKey的私钥算法。
    #[serde(rename = "KeyAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_algorithm: Option<String>,
    /// ClientKey的有效期起始时间。
    #[serde(rename = "NotBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_before: Option<String>,
    /// ClientKey的有效期结束时间。
    #[serde(rename = "NotAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_after: Option<String>,
    /// ClientKey由谁生成。
    #[serde(rename = "KeyOrigin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_origin: Option<String>,
    /// 绑定的应用接入点名称。
    #[serde(rename = "AapName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aap_name: Option<String>,
}

impl ListClientKeysResponseClientKeysItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.client_key_id {
            params.push(("ClientKeyId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.public_key_data {
            params.push(("PublicKeyData".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key_algorithm {
            params.push(("KeyAlgorithm".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.not_before {
            params.push(("NotBefore".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.not_after {
            params.push(("NotAfter".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key_origin {
            params.push(("KeyOrigin".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.aap_name {
            params.push(("AapName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetKmsInstanceQuotaInfosResponseKmsInstanceQuotaInfosItem {
    /// 配额。
    #[serde(rename = "ResourceQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_quota: Option<i64>,
    /// 资源类型
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// 已使用的额度
    #[serde(rename = "UsedQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_quantity: Option<i64>,
}

impl GetKmsInstanceQuotaInfosResponseKmsInstanceQuotaInfosItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_quota {
            params.push(("ResourceQuota".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_type {
            params.push(("ResourceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.used_quantity {
            params.push(("UsedQuantity".to_string(), v.to_string()));
        }
        params
    }
}

/// DescribeRegions 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeRegionsRequest {
}

impl DescribeRegionsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
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

/// DescribeAccountKmsStatus 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeAccountKmsStatusRequest {
}

impl DescribeAccountKmsStatusRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeAccountKmsStatusResponse {
    /// 当前阿里云账号的密钥管理服务状态，取值：
    #[serde(rename = "AccountStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_status: Option<String>,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// OpenKmsService 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct OpenKmsServiceRequest {
}

impl OpenKmsServiceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct OpenKmsServiceResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListKmsInstances 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListKmsInstancesRequest {
    /// 分页查询时，设置当前页面的页码。默认值为1。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 分页查询时，设置每页包含网络控制规则的数量。取值范围：1~100，默认值为20。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<String>,
}

impl ListKmsInstancesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.filters {
            params.push(("Filters".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ListKmsInstancesResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "KmsInstances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_instances: Option<ListKmsInstancesResponseKmsInstances>,
    /// KMS实例的总数量。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// 分页查询时，当前页面的页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    /// 分页查询时，每页包含KMS实例的数量。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

/// ConnectKmsInstance 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ConnectKmsInstanceRequest {
    /// 要启用的KMS实例的ID。
    #[serde(rename = "KmsInstanceId")]
    pub kms_instance_id: String,
    /// 为KMS实例设置专有网络VPC ID。
    #[serde(rename = "VpcId")]
    pub vpc_id: String,
    /// 为KMS实例设置两个可用区。通过双可用区负载均衡，提高服务可用性与容灾能力。
    #[serde(rename = "ZoneIds")]
    pub zone_ids: String,
    /// 设置双可用区下的一个交换机，并且该交换机至少有1个可用IP。
    #[serde(rename = "VSwitchIds")]
    pub v_switch_ids: String,
    /// KMS实例提供商。目前取值仅支持Aliyun。
    #[serde(rename = "KMProvider")]
    pub km_provider: String,
}

impl ConnectKmsInstanceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("KmsInstanceId".to_string(), self.kms_instance_id.to_string()));
        params.push(("VpcId".to_string(), self.vpc_id.to_string()));
        params.push(("ZoneIds".to_string(), self.zone_ids.to_string()));
        params.push(("VSwitchIds".to_string(), self.v_switch_ids.to_string()));
        params.push(("KMProvider".to_string(), self.km_provider.to_string()));
        params
    }
}

/// 响应消息。
#[derive(Debug, Clone, Deserialize)]
pub struct ConnectKmsInstanceResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// GetKmsInstance 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetKmsInstanceRequest {
    /// 要查询的KMS实例的ID。
    #[serde(rename = "KmsInstanceId")]
    pub kms_instance_id: String,
}

impl GetKmsInstanceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("KmsInstanceId".to_string(), self.kms_instance_id.to_string()));
        params
    }
}

/// KMS实例详情。
#[derive(Debug, Clone, Deserialize)]
pub struct GetKmsInstanceResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// KMS实例详情。
    #[serde(rename = "KmsInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_instance: Option<GetKmsInstanceResponseKmsInstance>,
}

/// UpdateKmsInstanceBindVpc 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateKmsInstanceBindVpcRequest {
    /// KMS实例的ID。
    #[serde(rename = "KmsInstanceId")]
    pub kms_instance_id: String,
    /// VPC配置，每个VPC包含如下内容：
    #[serde(rename = "BindVpcs")]
    pub bind_vpcs: String,
}

impl UpdateKmsInstanceBindVpcRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("KmsInstanceId".to_string(), self.kms_instance_id.to_string()));
        params.push(("BindVpcs".to_string(), self.bind_vpcs.to_string()));
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateKmsInstanceBindVpcResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ReleaseKmsInstance 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ReleaseKmsInstanceRequest {
    /// KMS实例ID。
    #[serde(rename = "KmsInstanceId")]
    pub kms_instance_id: String,
    /// KMS实例未备份的场景下是否强制释放。
    #[serde(rename = "ForceDeleteWithoutBackup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_delete_without_backup: Option<String>,
}

impl ReleaseKmsInstanceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("KmsInstanceId".to_string(), self.kms_instance_id.to_string()));
        if let Some(ref v) = self.force_delete_without_backup {
            params.push(("ForceDeleteWithoutBackup".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回数据。
#[derive(Debug, Clone, Deserialize)]
pub struct ReleaseKmsInstanceResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// GetDefaultKmsInstance 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetDefaultKmsInstanceRequest {
}

impl GetDefaultKmsInstanceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 响应消息。
#[derive(Debug, Clone, Deserialize)]
pub struct GetDefaultKmsInstanceResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 默认KMS实例的实例ID。
    #[serde(rename = "DefaultKmsInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_kms_instance_id: Option<String>,
}

/// CreateKey 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateKeyRequest {
    /// 密钥的描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 密钥的用途。取值：
    #[serde(rename = "KeyUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_usage: Option<String>,
    /// 密钥材料来源。取值：
    #[serde(rename = "Origin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    /// 您无需输入本参数，KMS会为您的密钥设置合适的保护级别。
    #[serde(rename = "ProtectionLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection_level: Option<String>,
    /// 是否开启密钥自动轮转。取值：
    #[serde(rename = "EnableAutomaticRotation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_automatic_rotation: Option<bool>,
    /// 自动轮转的时间周期。格式为integer\[unit]，其中integer表示时间长度，unit表示时间单位。合法的unit单位为：d（天）、h（小时）、m（分钟）、s（秒）。7d或者60480...
    #[serde(rename = "RotationInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_interval: Option<String>,
    /// 密钥规格，不同密钥管理类型的取值不同。关于密钥规格、遵循的标准、密钥算法的详细介绍，请参见[密钥管理类型和密钥规格](~~480161~~)。
    #[serde(rename = "KeySpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_spec: Option<String>,
    /// KMS实例的实例ID。
    #[serde(rename = "DKMSInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkms_instance_id: Option<String>,
    /// 为密钥绑定标签。每个标签由一个键值对（Key:Value）组成，包含标签键（Key）、标签值（Value）。
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    /// 密钥策略的具体内容，JSON格式。最大长度为32768个字节。
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    /// 密钥存储位置，仅当DKMSInstanceId输入的是KMS硬件密钥管理实例时生效。取值：
    #[serde(rename = "KeyStorageMechanism")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_storage_mechanism: Option<String>,
}

impl CreateKeyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key_usage {
            params.push(("KeyUsage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.origin {
            params.push(("Origin".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.protection_level {
            params.push(("ProtectionLevel".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_automatic_rotation {
            params.push(("EnableAutomaticRotation".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rotation_interval {
            params.push(("RotationInterval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key_spec {
            params.push(("KeySpec".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dkms_instance_id {
            params.push(("DKMSInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            params.push(("Tags".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy {
            params.push(("Policy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key_storage_mechanism {
            params.push(("KeyStorageMechanism".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateKeyResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 密钥的元数据。
    #[serde(rename = "KeyMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_metadata: Option<CreateKeyResponseKeyMetadata>,
}

/// ListKeys 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListKeysRequest {
    /// 当前页数。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 每页返回值的个数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 主密钥过滤器。由Key-Values键值对组成，长度为0~10。
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<String>,
}

impl ListKeysRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.filters {
            params.push(("Filters".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListKeysResponse {
    /// 当前页数。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 每页返回值的个数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 主密钥的总数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    #[serde(rename = "Keys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<ListKeysResponseKeys>,
}

/// DescribeKey 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeKeyRequest {
    /// 密钥的ID，也可以指定为密钥别名或密钥资源名称（ARN）。关于别名的详细介绍，请参见[管理密钥别名](~~480655~~)。
    #[serde(rename = "KeyId")]
    pub key_id: String,
}

impl DescribeKeyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("KeyId".to_string(), self.key_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeKeyResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// CMK的元数据。
    #[serde(rename = "KeyMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_metadata: Option<DescribeKeyResponseKeyMetadata>,
}

/// UpdateKeyDescription 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateKeyDescriptionRequest {
    /// 密钥ID。主密钥的全局唯一标识符。
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// 主密钥的描述性信息。通常用于描述主密钥的用途，例如主密钥保护的数据类型、可使用主密钥的应用等。
    #[serde(rename = "Description")]
    pub description: String,
}

impl UpdateKeyDescriptionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("KeyId".to_string(), self.key_id.to_string()));
        params.push(("Description".to_string(), self.description.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateKeyDescriptionResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// EnableKey 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct EnableKeyRequest {
    /// 密钥ID。主密钥的全局唯一标识符。
    #[serde(rename = "KeyId")]
    pub key_id: String,
}

impl EnableKeyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("KeyId".to_string(), self.key_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnableKeyResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DisableKey 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DisableKeyRequest {
    /// 密钥ID。主密钥的全局唯一标识符。
    #[serde(rename = "KeyId")]
    pub key_id: String,
}

impl DisableKeyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("KeyId".to_string(), self.key_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DisableKeyResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// GetPublicKey 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetPublicKeyRequest {
    /// 主密钥（CMK）的全局唯一标识符。该参数也可以被指定为CMK绑定的别名，详情见[别名使用说明](~~68522~~)。
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// 密钥版本的全局唯一标识符。
    #[serde(rename = "KeyVersionId")]
    pub key_version_id: String,
    /// 是否开启DryRun模式。
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<String>,
}

impl GetPublicKeyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("KeyId".to_string(), self.key_id.to_string()));
        params.push(("KeyVersionId".to_string(), self.key_version_id.to_string()));
        if let Some(ref v) = self.dry_run {
            params.push(("DryRun".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetPublicKeyResponse {
    /// 对明文数据进行加密的主密钥版本号。
    #[serde(rename = "KeyVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_version_id: Option<String>,
    /// CMK的全局唯一标识符。
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// 随机的访问ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// PEM格式的公钥。
    #[serde(rename = "PublicKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
}

/// CreateAlias 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateAliasRequest {
    /// CMK的全局唯一标识符。
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// CMK的别名名称。
    #[serde(rename = "AliasName")]
    pub alias_name: String,
}

impl CreateAliasRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("KeyId".to_string(), self.key_id.to_string()));
        params.push(("AliasName".to_string(), self.alias_name.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateAliasResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListAliases 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListAliasesRequest {
    /// 当前页数。取值范围：大于0的整数。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 每页返回的结果个数。取值范围：0~100。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListAliasesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
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
pub struct ListAliasesResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 当前页数。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 每页的返回结果个数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 返回的别名总数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    #[serde(rename = "Aliases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<ListAliasesResponseAliases>,
}

/// ListAliasesByKeyId 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListAliasesByKeyIdRequest {
    /// 密钥的ID或密钥资源名称（ARN）。
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// 当前页数。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 每页返回的结果个数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListAliasesByKeyIdRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("KeyId".to_string(), self.key_id.to_string()));
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
pub struct ListAliasesByKeyIdResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 当前页数。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 每页的返回结果个数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 返回的密钥总数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    #[serde(rename = "Aliases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<ListAliasesByKeyIdResponseAliases>,
}

/// DeleteAlias 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteAliasRequest {
    /// 要操作的别名。
    #[serde(rename = "AliasName")]
    pub alias_name: String,
}

impl DeleteAliasRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("AliasName".to_string(), self.alias_name.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteAliasResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// UpdateAlias 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateAliasRequest {
    /// 新的密钥ID。主密钥的全局唯一标识符。
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// 要操作的别名。
    #[serde(rename = "AliasName")]
    pub alias_name: String,
}

impl UpdateAliasRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("KeyId".to_string(), self.key_id.to_string()));
        params.push(("AliasName".to_string(), self.alias_name.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateAliasResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// GetParametersForImport 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetParametersForImportRequest {
    /// 主密钥的全局唯一标识符。
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// 用于加密密钥材料的算法。
    #[serde(rename = "WrappingAlgorithm")]
    pub wrapping_algorithm: String,
    /// 用于加密密钥材料的公钥类型。
    #[serde(rename = "WrappingKeySpec")]
    pub wrapping_key_spec: String,
}

impl GetParametersForImportRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("KeyId".to_string(), self.key_id.to_string()));
        params.push(("WrappingAlgorithm".to_string(), self.wrapping_algorithm.to_string()));
        params.push(("WrappingKeySpec".to_string(), self.wrapping_key_spec.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetParametersForImportResponse {
    /// 主密钥全局唯一标识符。
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// 导入令牌。
    #[serde(rename = "ImportToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_token: Option<String>,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 导入令牌的过期时间。
    #[serde(rename = "TokenExpireTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_expire_time: Option<String>,
    /// 用于加密密钥材料的公钥。
    #[serde(rename = "PublicKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
}

/// ImportKeyMaterial 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ImportKeyMaterialRequest {
    /// 待导入的主密钥ID。
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// 使用**GetParametersForImport**返回的公钥加密并用base64编码后的密钥材料。
    #[serde(rename = "EncryptedKeyMaterial")]
    pub encrypted_key_material: String,
    /// 通过调用**GetParametersForImport**获得的导入令牌。
    #[serde(rename = "ImportToken")]
    pub import_token: String,
    /// 密钥材料过期时间。
    #[serde(rename = "KeyMaterialExpireUnix")]
    pub key_material_expire_unix: i64,
}

impl ImportKeyMaterialRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("KeyId".to_string(), self.key_id.to_string()));
        params.push(("EncryptedKeyMaterial".to_string(), self.encrypted_key_material.to_string()));
        params.push(("ImportToken".to_string(), self.import_token.to_string()));
        params.push(("KeyMaterialExpireUnix".to_string(), self.key_material_expire_unix.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ImportKeyMaterialResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteKeyMaterial 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteKeyMaterialRequest {
    /// 密钥ID。主密钥（CMK）的全局唯一标识符。
    #[serde(rename = "KeyId")]
    pub key_id: String,
}

impl DeleteKeyMaterialRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("KeyId".to_string(), self.key_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteKeyMaterialResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ScheduleKeyDeletion 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ScheduleKeyDeletionRequest {
    /// 密钥ID。CMK全局唯一标识符。
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// 密钥预删除周期。在这段时间内，您可以撤销删除处于待删除状态的密钥；预删除时间过后无法撤销删除。
    #[serde(rename = "PendingWindowInDays")]
    pub pending_window_in_days: i32,
}

impl ScheduleKeyDeletionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("KeyId".to_string(), self.key_id.to_string()));
        params.push(("PendingWindowInDays".to_string(), self.pending_window_in_days.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ScheduleKeyDeletionResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CancelKeyDeletion 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CancelKeyDeletionRequest {
    /// 主密钥的全局唯一标识符。
    #[serde(rename = "KeyId")]
    pub key_id: String,
}

impl CancelKeyDeletionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("KeyId".to_string(), self.key_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CancelKeyDeletionResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// SetDeletionProtection 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetDeletionProtectionRequest {
    /// 待设置删除保护的CMK ARN。
    #[serde(rename = "ProtectedResourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protected_resource_arn: Option<String>,
    /// 是否开启删除保护，取值：
    #[serde(rename = "EnableDeletionProtection")]
    pub enable_deletion_protection: bool,
    /// 删除保护描述。
    #[serde(rename = "DeletionProtectionDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection_description: Option<String>,
    /// 密钥的ID。
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
}

impl SetDeletionProtectionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.protected_resource_arn {
            params.push(("ProtectedResourceArn".to_string(), v.to_string()));
        }
        params.push(("EnableDeletionProtection".to_string(), self.enable_deletion_protection.to_string()));
        if let Some(ref v) = self.deletion_protection_description {
            params.push(("DeletionProtectionDescription".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key_id {
            params.push(("KeyId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetDeletionProtectionResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// UpdateRotationPolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateRotationPolicyRequest {
    /// 密钥ID，即密钥的全局唯一标识符。
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// 是否开启周期性自动轮转。取值：
    #[serde(rename = "EnableAutomaticRotation")]
    pub enable_automatic_rotation: bool,
    /// 自动轮转的时间周期。格式为integer\[unit]，其中integer表示时间长度，unit表示时间单位。合法的unit单位为：d（天）、h（小时）、m（分钟）、s（秒）。7d或者60480...
    #[serde(rename = "RotationInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_interval: Option<String>,
}

impl UpdateRotationPolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("KeyId".to_string(), self.key_id.to_string()));
        params.push(("EnableAutomaticRotation".to_string(), self.enable_automatic_rotation.to_string()));
        if let Some(ref v) = self.rotation_interval {
            params.push(("RotationInterval".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateRotationPolicyResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeKeyVersion 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeKeyVersionRequest {
    /// 密钥的ID，也可以指定为密钥别名或密钥资源名称（ARN）。关于别名的详细介绍，请参见[管理密钥别名](~~480655~~)。
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// 密钥版本的全局唯一标识符。
    #[serde(rename = "KeyVersionId")]
    pub key_version_id: String,
}

impl DescribeKeyVersionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("KeyId".to_string(), self.key_id.to_string()));
        params.push(("KeyVersionId".to_string(), self.key_version_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeKeyVersionResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 密钥版本的元数据。
    #[serde(rename = "KeyVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_version: Option<DescribeKeyVersionResponseKeyVersion>,
}

/// CreateKeyVersion 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateKeyVersionRequest {
    /// 密钥ID，即密钥的全局唯一标识符。
    #[serde(rename = "KeyId")]
    pub key_id: String,
}

impl CreateKeyVersionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("KeyId".to_string(), self.key_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateKeyVersionResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 密钥版本的元数据。
    #[serde(rename = "KeyVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_version: Option<CreateKeyVersionResponseKeyVersion>,
}

/// ListKeyVersions 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListKeyVersionsRequest {
    /// 密钥的ID，也可以指定为密钥别名或密钥资源名称（ARN）。关于别名的详细介绍，请参见[管理密钥别名](~~480655~~)。
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// 当前页数。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 每页返回的结果个数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListKeyVersionsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("KeyId".to_string(), self.key_id.to_string()));
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
pub struct ListKeyVersionsResponse {
    /// 每页的返回结果个数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 当前页数。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 返回的密钥版本总数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    #[serde(rename = "KeyVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_versions: Option<ListKeyVersionsResponseKeyVersions>,
}

/// SetKeyPolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetKeyPolicyRequest {
    /// 密钥ID或密钥资源名称（ARN）。
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// 密钥策略名称。仅支持固定取值default。
    #[serde(rename = "PolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    /// 密钥策略的具体内容，JSON格式。最大长度为32768个字节。
    #[serde(rename = "Policy")]
    pub policy: String,
}

impl SetKeyPolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("KeyId".to_string(), self.key_id.to_string()));
        if let Some(ref v) = self.policy_name {
            params.push(("PolicyName".to_string(), v.to_string()));
        }
        params.push(("Policy".to_string(), self.policy.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetKeyPolicyResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// GetKeyPolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetKeyPolicyRequest {
    /// 密钥ID或密钥资源名称（ARN）。
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// 密钥策略名称。仅支持固定值default。
    #[serde(rename = "PolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}

impl GetKeyPolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("KeyId".to_string(), self.key_id.to_string()));
        if let Some(ref v) = self.policy_name {
            params.push(("PolicyName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetKeyPolicyResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 密钥策略。
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

/// GenerateDataKey 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GenerateDataKeyRequest {
    /// 密钥的ID，也可以指定为密钥别名或密钥资源名称（ARN）。关于别名的详细介绍，请参见[管理密钥别名](~~480655~~)。
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// 指定生成的数据密钥的长度，取值：
    #[serde(rename = "KeySpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_spec: Option<String>,
    /// 指定生成的数据密钥的长度，单位为字节。
    #[serde(rename = "NumberOfBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_bytes: Option<i32>,
    /// key/value对的JSON字符串。
    #[serde(rename = "EncryptionContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_context: Option<serde_json::Value>,
    /// 是否开启DryRun模式。
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<String>,
}

impl GenerateDataKeyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("KeyId".to_string(), self.key_id.to_string()));
        if let Some(ref v) = self.key_spec {
            params.push(("KeySpec".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.number_of_bytes {
            params.push(("NumberOfBytes".to_string(), v.to_string()));
        }
        // 跳过: EncryptionContext (serde_json::Value)
        if let Some(ref v) = self.dry_run {
            params.push(("DryRun".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GenerateDataKeyResponse {
    /// 密钥版本ID。主密钥版本的全局唯一标识符。
    #[serde(rename = "KeyVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_version_id: Option<String>,
    /// 密钥ID。如果请求中的KeyId参数使用的是密钥别名、密钥ARN，在响应中也会返回密钥ID。
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// 数据密钥被指定密钥的主版本加密后的密文。
    #[serde(rename = "CiphertextBlob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ciphertext_blob: Option<String>,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 数据密钥的明文经过Base64编码后的值。
    #[serde(rename = "Plaintext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plaintext: Option<String>,
}

/// GenerateAndExportDataKey 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GenerateAndExportDataKeyRequest {
    /// 密钥的ID，也可以指定为密钥别名或密钥资源名称（ARN）。关于别名的详细介绍，请参见[管理密钥别名](~~480655~~)。
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// 指定生成的数据密钥的长度，取值：
    #[serde(rename = "KeySpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_spec: Option<String>,
    /// 指定生成的数据密钥的长度。
    #[serde(rename = "NumberOfBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_bytes: Option<i32>,
    /// key/value对的JSON字符串，如果指定了该参数，则在解密或者使用其他密钥转加密时需要提供同样的参数，详情请参见[EncryptionContext说明](~~42975~~)。
    #[serde(rename = "EncryptionContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_context: Option<serde_json::Value>,
    /// Base64编码的公钥。
    #[serde(rename = "PublicKeyBlob")]
    pub public_key_blob: String,
    /// PublicKeyBlob密钥的类型。密钥类型详情，请参见[非对称密钥简介](~~148147~~)。
    #[serde(rename = "WrappingKeySpec")]
    pub wrapping_key_spec: String,
    /// 使用PublicKeyBlob所指定的公钥，加密（Wrap）数据密钥时的加密算法。算法详情，请参见[AsymmetricDecrypt](~~148130~~)。
    #[serde(rename = "WrappingAlgorithm")]
    pub wrapping_algorithm: String,
    /// 是否开启DryRun模式。
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<String>,
}

impl GenerateAndExportDataKeyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("KeyId".to_string(), self.key_id.to_string()));
        if let Some(ref v) = self.key_spec {
            params.push(("KeySpec".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.number_of_bytes {
            params.push(("NumberOfBytes".to_string(), v.to_string()));
        }
        // 跳过: EncryptionContext (serde_json::Value)
        params.push(("PublicKeyBlob".to_string(), self.public_key_blob.to_string()));
        params.push(("WrappingKeySpec".to_string(), self.wrapping_key_spec.to_string()));
        params.push(("WrappingAlgorithm".to_string(), self.wrapping_algorithm.to_string()));
        if let Some(ref v) = self.dry_run {
            params.push(("DryRun".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GenerateAndExportDataKeyResponse {
    /// 用于加密明文的密钥版本标识符。是指定KMS密钥的主版本。
    #[serde(rename = "KeyVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_version_id: Option<String>,
    /// 密钥ID。如果请求中的KeyId参数使用的是密钥别名、密钥ARN，在响应中也会返回密钥ID。
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// 数据密钥被指定KMS密钥的主版本加密后的密文。
    #[serde(rename = "CiphertextBlob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ciphertext_blob: Option<String>,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 公钥加密保护导出的数据密钥。
    #[serde(rename = "ExportedDataKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exported_data_key: Option<String>,
}

/// Encrypt 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct EncryptRequest {
    /// 密钥的ID，也可以指定为密钥别名或密钥资源名称（ARN）。关于别名的详细介绍，请参见[管理密钥别名](~~480655~~)。
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// 待加密明文（必须经过Base64编码）。
    #[serde(rename = "Plaintext")]
    pub plaintext: String,
    /// key/value的JSON字符串。如果指定了该参数，则在调用Decrypt时需要提供同样的参数，详情请参见[EncryptionContext说明](~~42975~~)。
    #[serde(rename = "EncryptionContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_context: Option<serde_json::Value>,
    /// 是否开启DryRun模式。
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<String>,
}

impl EncryptRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("KeyId".to_string(), self.key_id.to_string()));
        params.push(("Plaintext".to_string(), self.plaintext.to_string()));
        // 跳过: EncryptionContext (serde_json::Value)
        if let Some(ref v) = self.dry_run {
            params.push(("DryRun".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct EncryptResponse {
    /// 用于加密明文的密钥版本标志符。是指定密钥的主版本。
    #[serde(rename = "KeyVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_version_id: Option<String>,
    /// 密钥ID。如果请求中的KeyId参数使用的是密钥别名、密钥ARN，在响应中会返回密钥ID。
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// 数据被指定密钥的主版本加密后的密文。
    #[serde(rename = "CiphertextBlob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ciphertext_blob: Option<String>,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// Decrypt 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DecryptRequest {
    /// 待解密的密文。
    #[serde(rename = "CiphertextBlob")]
    pub ciphertext_blob: String,
    /// key/value的JSON字符串。
    #[serde(rename = "EncryptionContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_context: Option<serde_json::Value>,
    /// 是否开启DryRun模式。
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<String>,
}

impl DecryptRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("CiphertextBlob".to_string(), self.ciphertext_blob.to_string()));
        // 跳过: EncryptionContext (serde_json::Value)
        if let Some(ref v) = self.dry_run {
            params.push(("DryRun".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DecryptResponse {
    /// 主密钥下用于解密密文的密钥版本标识符。
    #[serde(rename = "KeyVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_version_id: Option<String>,
    /// 解密密文使用的主密钥ID。
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 解密后的明文。
    #[serde(rename = "Plaintext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plaintext: Option<String>,
}

/// ReEncrypt 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ReEncryptRequest {
    /// 待转加密的密文。
    #[serde(rename = "CiphertextBlob")]
    pub ciphertext_blob: String,
    /// 解密密文时使用的主密钥ID。
    #[serde(rename = "SourceKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_key_id: Option<String>,
    /// 用于解密密文的密钥版本标识符。
    #[serde(rename = "SourceKeyVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_key_version_id: Option<String>,
    /// CiphertextBlob是公钥加密结果时，指定公钥加密的算法。算法详情，请参见[AsymmetricDecrypt](~~148130~~)。
    #[serde(rename = "SourceEncryptionAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_encryption_algorithm: Option<String>,
    /// key/value的JSON字符串。如果在[Encrypt](~~28949~~)、[GenerateDataKey](~~28948~~)、[GenerateDataKeyWithoutPla...
    #[serde(rename = "SourceEncryptionContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_encryption_context: Option<serde_json::Value>,
    /// 对密文解密后再次加密时使用的对称主密钥ID。
    #[serde(rename = "DestinationKeyId")]
    pub destination_key_id: String,
    /// key/value的JSON字符串，用于目标主密钥加密时的加密上下文。
    #[serde(rename = "DestinationEncryptionContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_encryption_context: Option<serde_json::Value>,
    /// 是否开启DryRun模式。
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<String>,
}

impl ReEncryptRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("CiphertextBlob".to_string(), self.ciphertext_blob.to_string()));
        if let Some(ref v) = self.source_key_id {
            params.push(("SourceKeyId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source_key_version_id {
            params.push(("SourceKeyVersionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source_encryption_algorithm {
            params.push(("SourceEncryptionAlgorithm".to_string(), v.to_string()));
        }
        // 跳过: SourceEncryptionContext (serde_json::Value)
        params.push(("DestinationKeyId".to_string(), self.destination_key_id.to_string()));
        // 跳过: DestinationEncryptionContext (serde_json::Value)
        if let Some(ref v) = self.dry_run {
            params.push(("DryRun".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReEncryptResponse {
    /// 解密密文使用的主密钥ID。
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// 主密钥下用于解密密文的密钥版本标识符。
    #[serde(rename = "KeyVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_version_id: Option<String>,
    /// 使用指定的主密钥进行再次加密得到的密文。
    #[serde(rename = "CiphertextBlob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ciphertext_blob: Option<String>,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ExportDataKey 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ExportDataKeyRequest {
    /// 主密钥（CMK）加密的数据密钥的密文。
    #[serde(rename = "CiphertextBlob")]
    pub ciphertext_blob: String,
    /// key/value的JSON字符串。EncryptionContext是使用CMK加密数据密钥时传入的加密上下文，详情请参见[EncryptionContext说明](~~42975~~)。
    #[serde(rename = "EncryptionContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_context: Option<serde_json::Value>,
    /// Base64格式的公钥。
    #[serde(rename = "PublicKeyBlob")]
    pub public_key_blob: String,
    /// PublicKeyBlob的密钥类型。密钥类型详情，请参见[非对称密钥简介](~~148147~~)。
    #[serde(rename = "WrappingKeySpec")]
    pub wrapping_key_spec: String,
    /// 使用PublicKeyBlob所指定的公钥，加密（Wrap）数据密钥时的加密算法。算法详情，请参见[AsymmetricDecrypt](~~148130~~)。
    #[serde(rename = "WrappingAlgorithm")]
    pub wrapping_algorithm: String,
    /// 是否开启DryRun模式。
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<String>,
}

impl ExportDataKeyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("CiphertextBlob".to_string(), self.ciphertext_blob.to_string()));
        // 跳过: EncryptionContext (serde_json::Value)
        params.push(("PublicKeyBlob".to_string(), self.public_key_blob.to_string()));
        params.push(("WrappingKeySpec".to_string(), self.wrapping_key_spec.to_string()));
        params.push(("WrappingAlgorithm".to_string(), self.wrapping_algorithm.to_string()));
        if let Some(ref v) = self.dry_run {
            params.push(("DryRun".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExportDataKeyResponse {
    /// 用于解密传入的数据密钥密文的密钥版本标识符。
    #[serde(rename = "KeyVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_version_id: Option<String>,
    /// 解密传入的数据密钥密文使用的主密钥ID。
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 公钥加密保护导出的数据密钥。
    #[serde(rename = "ExportedDataKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exported_data_key: Option<String>,
}

/// GenerateDataKeyWithoutPlaintext 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GenerateDataKeyWithoutPlaintextRequest {
    /// 主密钥（CMK）的全局唯一标识符。该参数也可以被指定为CMK绑定的别名，详情请参见别名使用说明。
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// 指定生成的数据密钥的长度，取值：
    #[serde(rename = "KeySpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_spec: Option<String>,
    /// 指定生成的数据密钥的长度。
    #[serde(rename = "NumberOfBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_bytes: Option<i32>,
    /// key/value对的JSON字符串，如果指定了该参数，则在调用Decrypt 时需要提供同样的参数，详情请参见[EncryptionContext说明](~~42975~~)。
    #[serde(rename = "EncryptionContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_context: Option<serde_json::Value>,
    /// 是否开启DryRun模式。
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<String>,
}

impl GenerateDataKeyWithoutPlaintextRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("KeyId".to_string(), self.key_id.to_string()));
        if let Some(ref v) = self.key_spec {
            params.push(("KeySpec".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.number_of_bytes {
            params.push(("NumberOfBytes".to_string(), v.to_string()));
        }
        // 跳过: EncryptionContext (serde_json::Value)
        if let Some(ref v) = self.dry_run {
            params.push(("DryRun".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GenerateDataKeyWithoutPlaintextResponse {
    /// 用于加密明文的密钥版本标志符。是指定CMK的主版本。
    #[serde(rename = "KeyVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_version_id: Option<String>,
    /// CMK的全局唯一标识符。
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// 数据密钥被指定CMK的主版本加密后的密文。
    #[serde(rename = "CiphertextBlob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ciphertext_blob: Option<String>,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// AsymmetricSign 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct AsymmetricSignRequest {
    /// 主密钥（CMK）的全局唯一标识符。
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// 密钥版本ID。密钥版本的全局唯一标识符。
    #[serde(rename = "KeyVersionId")]
    pub key_version_id: String,
    /// 签名算法。
    #[serde(rename = "Algorithm")]
    pub algorithm: String,
    /// 使用Algorithm中对应的哈希算法，对原始消息生成的摘要。
    #[serde(rename = "Digest")]
    pub digest: String,
    /// 是否开启DryRun模式。
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<String>,
}

impl AsymmetricSignRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("KeyId".to_string(), self.key_id.to_string()));
        params.push(("KeyVersionId".to_string(), self.key_version_id.to_string()));
        params.push(("Algorithm".to_string(), self.algorithm.to_string()));
        params.push(("Digest".to_string(), self.digest.to_string()));
        if let Some(ref v) = self.dry_run {
            params.push(("DryRun".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AsymmetricSignResponse {
    /// 密钥版本ID。密钥版本的全局唯一标识符。
    #[serde(rename = "KeyVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_version_id: Option<String>,
    /// 主密钥的全局唯一标识符。
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// 计算出来的签名。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// AsymmetricVerify 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct AsymmetricVerifyRequest {
    /// 主密钥（CMK）的全局唯一标识符。
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// 密钥版本ID。密钥版本的全局唯一标识符。
    #[serde(rename = "KeyVersionId")]
    pub key_version_id: String,
    /// 签名算法。
    #[serde(rename = "Algorithm")]
    pub algorithm: String,
    /// 使用**Algorithm**中对应的哈希算法，对原始消息生成的摘要。
    #[serde(rename = "Digest")]
    pub digest: String,
    /// 待验证的签名值。
    #[serde(rename = "Value")]
    pub value: String,
    /// 是否开启DryRun模式。
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<String>,
}

impl AsymmetricVerifyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("KeyId".to_string(), self.key_id.to_string()));
        params.push(("KeyVersionId".to_string(), self.key_version_id.to_string()));
        params.push(("Algorithm".to_string(), self.algorithm.to_string()));
        params.push(("Digest".to_string(), self.digest.to_string()));
        params.push(("Value".to_string(), self.value.to_string()));
        if let Some(ref v) = self.dry_run {
            params.push(("DryRun".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AsymmetricVerifyResponse {
    /// 对明文数据进行加密的主密钥版本号。
    #[serde(rename = "KeyVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_version_id: Option<String>,
    /// 主密钥的全局唯一标识符。
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// 签名验证是否通过。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// AsymmetricEncrypt 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct AsymmetricEncryptRequest {
    /// 要加密的明文，使用Base64编码。
    #[serde(rename = "Plaintext")]
    pub plaintext: String,
    /// 密钥的ID，也可以指定为密钥别名或密钥资源名称（ARN）。关于别名的详细介绍，请参见[管理密钥别名](~~480655~~)。
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// 密钥版本ID。密钥版本的全局唯一标识符。
    #[serde(rename = "KeyVersionId")]
    pub key_version_id: String,
    /// 加密算法。
    #[serde(rename = "Algorithm")]
    pub algorithm: String,
    /// 是否开启DryRun模式。
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<String>,
}

impl AsymmetricEncryptRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Plaintext".to_string(), self.plaintext.to_string()));
        params.push(("KeyId".to_string(), self.key_id.to_string()));
        params.push(("KeyVersionId".to_string(), self.key_version_id.to_string()));
        params.push(("Algorithm".to_string(), self.algorithm.to_string()));
        if let Some(ref v) = self.dry_run {
            params.push(("DryRun".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AsymmetricEncryptResponse {
    /// 对明文数据进行加密的主密钥版本号。
    #[serde(rename = "KeyVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_version_id: Option<String>,
    /// 密钥ID。如果请求中的KeyId参数使用的是密钥别名、密钥ARN，在响应中也会返回密钥ID。
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// 加密后的密文，使用Base64编码。
    #[serde(rename = "CiphertextBlob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ciphertext_blob: Option<String>,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// AsymmetricDecrypt 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct AsymmetricDecryptRequest {
    /// 解密密文，使用Base64编码。
    #[serde(rename = "CiphertextBlob")]
    pub ciphertext_blob: String,
    /// 密钥的ID，也可以指定为密钥别名或密钥资源名称（ARN）。关于别名的详细介绍，请参见[管理密钥别名](~~480655~~)。
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// 密钥版本ID。密钥版本的全局唯一标识符。
    #[serde(rename = "KeyVersionId")]
    pub key_version_id: String,
    /// 解密算法。
    #[serde(rename = "Algorithm")]
    pub algorithm: String,
    /// 是否开启DryRun模式。
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<String>,
}

impl AsymmetricDecryptRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("CiphertextBlob".to_string(), self.ciphertext_blob.to_string()));
        params.push(("KeyId".to_string(), self.key_id.to_string()));
        params.push(("KeyVersionId".to_string(), self.key_version_id.to_string()));
        params.push(("Algorithm".to_string(), self.algorithm.to_string()));
        if let Some(ref v) = self.dry_run {
            params.push(("DryRun".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AsymmetricDecryptResponse {
    /// 对明文数据进行加密的主密钥版本号。
    #[serde(rename = "KeyVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_version_id: Option<String>,
    /// 密钥ID。如果请求中的KeyId参数使用的是密钥别名、密钥ARN，在响应中也会返回密钥ID。
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 解密后的明文，使用Base64编码。
    #[serde(rename = "Plaintext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plaintext: Option<String>,
}

/// CreateSecret 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateSecretRequest {
    /// 凭据名称。  凭据名称在当前地域下唯一。
    #[serde(rename = "SecretName")]
    pub secret_name: String,
    /// 初始版本的版本号，版本号在该凭据内唯一。
    #[serde(rename = "VersionId")]
    pub version_id: String,
    /// 用于加密凭据值的密钥ID。
    #[serde(rename = "EncryptionKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_id: Option<String>,
    /// 凭据值。长度不超过30720字节（30KB）。KMS使用指定的密钥对其加密后，存入初始版本中。
    #[serde(rename = "SecretData")]
    pub secret_data: String,
    /// 凭据值类型。取值：
    #[serde(rename = "SecretDataType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_data_type: Option<String>,
    /// 凭据的描述信息。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 凭据的标签。每个标签由一个键值对（Key:Value）组成，包含标签键（Key）、标签值（Value）。
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    /// 凭据类型。取值：
    #[serde(rename = "SecretType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_type: Option<String>,
    /// 凭据的拓展配置，用于指定特定凭据类型的属性。长度不超过1024个字符。
    #[serde(rename = "ExtendedConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_config: Option<serde_json::Value>,
    /// 是否开启自动轮转，取值：
    #[serde(rename = "EnableAutomaticRotation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_automatic_rotation: Option<bool>,
    /// 自动轮转的周期。取值范围：6小时~8,760小时（365天）。
    #[serde(rename = "RotationInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_interval: Option<String>,
    /// KMS实例的实例ID。
    #[serde(rename = "DKMSInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkms_instance_id: Option<String>,
    /// 凭据策略的具体内容，JSON格式。最大长度为32768个字节。
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

impl CreateSecretRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("SecretName".to_string(), self.secret_name.to_string()));
        params.push(("VersionId".to_string(), self.version_id.to_string()));
        if let Some(ref v) = self.encryption_key_id {
            params.push(("EncryptionKeyId".to_string(), v.to_string()));
        }
        params.push(("SecretData".to_string(), self.secret_data.to_string()));
        if let Some(ref v) = self.secret_data_type {
            params.push(("SecretDataType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            params.push(("Tags".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.secret_type {
            params.push(("SecretType".to_string(), v.to_string()));
        }
        // 跳过: ExtendedConfig (serde_json::Value)
        if let Some(ref v) = self.enable_automatic_rotation {
            params.push(("EnableAutomaticRotation".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rotation_interval {
            params.push(("RotationInterval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dkms_instance_id {
            params.push(("DKMSInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy {
            params.push(("Policy".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateSecretResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否开启自动轮转。取值：
    #[serde(rename = "AutomaticRotation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_rotation: Option<String>,
    /// 凭据名称。
    #[serde(rename = "SecretName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<String>,
    /// 凭据版本号。
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    /// 下一次轮转的时间。
    #[serde(rename = "NextRotationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_rotation_date: Option<String>,
    /// 凭据类型。取值：
    #[serde(rename = "SecretType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_type: Option<String>,
    /// 凭据自动轮转的周期。
    #[serde(rename = "RotationInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_interval: Option<String>,
    /// 阿里云资源名称。
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// 凭据的拓展配置。
    #[serde(rename = "ExtendedConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_config: Option<String>,
    /// KMS实例的实例ID。
    #[serde(rename = "DKMSInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkms_instance_id: Option<String>,
}

/// DeleteSecret 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteSecretRequest {
    /// 凭据名称或凭据资源名称（ARN）。
    #[serde(rename = "SecretName")]
    pub secret_name: String,
    /// 是否立即删除凭据，且不允许恢复。
    #[serde(rename = "ForceDeleteWithoutRecovery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_delete_without_recovery: Option<String>,
    /// 计划删除凭据时，该参数用于指定删除窗口，窗口期内可以恢复凭据。取值：7天~30天。
    #[serde(rename = "RecoveryWindowInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_window_in_days: Option<String>,
}

impl DeleteSecretRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("SecretName".to_string(), self.secret_name.to_string()));
        if let Some(ref v) = self.force_delete_without_recovery {
            params.push(("ForceDeleteWithoutRecovery".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.recovery_window_in_days {
            params.push(("RecoveryWindowInDays".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteSecretResponse {
    /// 凭据名称。
    #[serde(rename = "SecretName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<String>,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 凭据的删除时间，为时间戳格式。
    #[serde(rename = "PlannedDeleteTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planned_delete_time: Option<String>,
}

/// UpdateSecret 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateSecretRequest {
    /// 凭据名称或凭据资源名称（ARN）。
    #[serde(rename = "SecretName")]
    pub secret_name: String,
    /// 凭据的描述信息。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 拓展配置中的自定义数据。
    #[serde(rename = "ExtendedConfig.CustomData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_config_custom_data: Option<serde_json::Value>,
}

impl UpdateSecretRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("SecretName".to_string(), self.secret_name.to_string()));
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        // 跳过: ExtendedConfig.CustomData (serde_json::Value)
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateSecretResponse {
    /// 凭据名称。
    #[serde(rename = "SecretName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<String>,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// UpdateSecretVersionStage 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateSecretVersionStageRequest {
    /// 凭据名称或凭据资源名称（ARN）。
    #[serde(rename = "SecretName")]
    pub secret_name: String,
    /// 凭据的版本状态。
    #[serde(rename = "VersionStage")]
    pub version_stage: String,
    /// 凭据版本的版本号。表示将入参VersionStage指定的版本状态从该版本号移除。
    #[serde(rename = "RemoveFromVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_from_version: Option<String>,
    /// 凭据版本的版本号。表示将入参VersionStage指定的版本状态绑定到该版本号。
    #[serde(rename = "MoveToVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub move_to_version: Option<String>,
}

impl UpdateSecretVersionStageRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("SecretName".to_string(), self.secret_name.to_string()));
        params.push(("VersionStage".to_string(), self.version_stage.to_string()));
        if let Some(ref v) = self.remove_from_version {
            params.push(("RemoveFromVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.move_to_version {
            params.push(("MoveToVersion".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateSecretVersionStageResponse {
    /// 凭据名称。
    #[serde(rename = "SecretName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<String>,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// UpdateSecretRotationPolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateSecretRotationPolicyRequest {
    /// 凭据名称或凭据资源名称（ARN）。
    #[serde(rename = "SecretName")]
    pub secret_name: String,
    /// 是否开启自动轮转，取值：
    #[serde(rename = "EnableAutomaticRotation")]
    pub enable_automatic_rotation: bool,
    /// 自动轮转的周期。取值范围：168小时（7天）~8,760小时（365天）。
    #[serde(rename = "RotationInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_interval: Option<String>,
}

impl UpdateSecretRotationPolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("SecretName".to_string(), self.secret_name.to_string()));
        params.push(("EnableAutomaticRotation".to_string(), self.enable_automatic_rotation.to_string()));
        if let Some(ref v) = self.rotation_interval {
            params.push(("RotationInterval".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateSecretRotationPolicyResponse {
    /// 凭据名称。
    #[serde(rename = "SecretName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<String>,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListSecrets 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListSecretsRequest {
    /// 返回值中是否包含凭据的资源标签。取值：
    #[serde(rename = "FetchTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fetch_tags: Option<String>,
    /// 当前页数。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 每页返回值的个数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 查询符合指定条件的凭据。
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<String>,
}

impl ListSecretsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.fetch_tags {
            params.push(("FetchTags".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.filters {
            params.push(("Filters".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListSecretsResponse {
    /// 当前页数。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 每页返回值的个数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 凭据列表中的凭据个数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    #[serde(rename = "SecretList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_list: Option<ListSecretsResponseSecretList>,
}

/// DescribeSecret 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeSecretRequest {
    /// 凭据名称或凭据资源名称（ARN）。
    #[serde(rename = "SecretName")]
    pub secret_name: String,
    /// 是否在返回参数中包含凭据的资源标签。取值：
    #[serde(rename = "FetchTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fetch_tags: Option<String>,
}

impl DescribeSecretRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("SecretName".to_string(), self.secret_name.to_string()));
        if let Some(ref v) = self.fetch_tags {
            params.push(("FetchTags".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeSecretResponse {
    /// 凭据的更新时间。
    #[serde(rename = "UpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 创建凭据的时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 下一次轮转的时间。
    #[serde(rename = "NextRotationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_rotation_date: Option<String>,
    /// 加密凭据值的KMS密钥的标识符。
    #[serde(rename = "EncryptionKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_id: Option<String>,
    /// 凭据自动轮转的周期。
    #[serde(rename = "RotationInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_interval: Option<String>,
    /// 凭据的资源名称（ARN）。
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// 凭据的拓展配置。
    #[serde(rename = "ExtendedConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_config: Option<String>,
    /// 最近一次轮转的时间。
    #[serde(rename = "LastRotationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_rotation_date: Option<String>,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 凭据的描述信息。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 凭据名称。
    #[serde(rename = "SecretName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<String>,
    /// 是否开启自动轮转。取值：
    #[serde(rename = "AutomaticRotation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_rotation: Option<String>,
    /// 凭据类型。取值：
    #[serde(rename = "SecretType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_type: Option<String>,
    /// 计划删除时间。
    #[serde(rename = "PlannedDeleteTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planned_delete_time: Option<String>,
    /// KMS实例的实例ID。
    #[serde(rename = "DKMSInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkms_instance_id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<DescribeSecretResponseTags>,
    #[serde(rename = "OwingService")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owing_service: Option<String>,
}

/// GetSecretValue 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetSecretValueRequest {
    /// 凭据名称或凭据资源名称（ARN）。
    #[serde(rename = "SecretName")]
    pub secret_name: String,
    /// 版本状态。默认值：ACSCurrent。
    #[serde(rename = "VersionStage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_stage: Option<String>,
    /// 版本号。
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    /// 是否获取凭据的拓展配置。取值：
    #[serde(rename = "FetchExtendedConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fetch_extended_config: Option<bool>,
    /// 是否开启DryRun模式。
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<String>,
}

impl GetSecretValueRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("SecretName".to_string(), self.secret_name.to_string()));
        if let Some(ref v) = self.version_stage {
            params.push(("VersionStage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version_id {
            params.push(("VersionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.fetch_extended_config {
            params.push(("FetchExtendedConfig".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dry_run {
            params.push(("DryRun".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetSecretValueResponse {
    /// 凭据值类型。取值：
    #[serde(rename = "SecretDataType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_data_type: Option<String>,
    /// 创建凭据的时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 凭据的版本号。
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    /// 下一次轮转的时间。
    #[serde(rename = "NextRotationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_rotation_date: Option<String>,
    /// 凭据值。KMS将存储的密文凭据值进行解密后返回该参数。
    #[serde(rename = "SecretData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_data: Option<String>,
    /// 凭据自动轮转的周期。
    #[serde(rename = "RotationInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_interval: Option<String>,
    /// 凭据的拓展配置。
    #[serde(rename = "ExtendedConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_config: Option<String>,
    /// 最近一次轮转的时间。
    #[serde(rename = "LastRotationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_rotation_date: Option<String>,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 凭据名称。
    #[serde(rename = "SecretName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<String>,
    /// 是否开启了自动轮转。取值：
    #[serde(rename = "AutomaticRotation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_rotation: Option<String>,
    /// 凭据类型。取值：
    #[serde(rename = "SecretType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_type: Option<String>,
    #[serde(rename = "VersionStages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_stages: Option<GetSecretValueResponseVersionStages>,
}

/// ListSecretVersionIds 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListSecretVersionIdsRequest {
    /// 凭据名称或凭据资源名称（ARN）。
    #[serde(rename = "SecretName")]
    pub secret_name: String,
    /// 返回值中是否包含没有版本状态的凭据版本。
    #[serde(rename = "IncludeDeprecated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_deprecated: Option<String>,
    /// 分页查询时，设置当前页面的页码。默认值：1。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 分页查询时，设置每页大小。默认值：20。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListSecretVersionIdsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("SecretName".to_string(), self.secret_name.to_string()));
        if let Some(ref v) = self.include_deprecated {
            params.push(("IncludeDeprecated".to_string(), v.to_string()));
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
pub struct ListSecretVersionIdsResponse {
    /// 凭据名称。
    #[serde(rename = "SecretName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<String>,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 当前页大小。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 当前页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 列表项总个数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    #[serde(rename = "VersionIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_ids: Option<ListSecretVersionIdsResponseVersionIds>,
}

/// GetRandomPassword 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetRandomPasswordRequest {
    /// 生成口令的字节数。
    #[serde(rename = "PasswordLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_length: Option<String>,
    /// 生成口令时排除的字符。
    #[serde(rename = "ExcludeCharacters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_characters: Option<String>,
    /// 生成口令时是否排除小写字母。
    #[serde(rename = "ExcludeLowercase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_lowercase: Option<String>,
    /// 生成口令时是否排除大写字母。
    #[serde(rename = "ExcludeUppercase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_uppercase: Option<String>,
    /// 生成口令时是否排除数字。
    #[serde(rename = "ExcludeNumbers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_numbers: Option<String>,
    /// 生成口令时是否排除特殊字符。
    #[serde(rename = "ExcludePunctuation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_punctuation: Option<String>,
    /// 生成口令时是否上述每种类型都包含。
    #[serde(rename = "RequireEachIncludedType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_each_included_type: Option<String>,
}

impl GetRandomPasswordRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.password_length {
            params.push(("PasswordLength".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.exclude_characters {
            params.push(("ExcludeCharacters".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.exclude_lowercase {
            params.push(("ExcludeLowercase".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.exclude_uppercase {
            params.push(("ExcludeUppercase".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.exclude_numbers {
            params.push(("ExcludeNumbers".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.exclude_punctuation {
            params.push(("ExcludePunctuation".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.require_each_included_type {
            params.push(("RequireEachIncludedType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetRandomPasswordResponse {
    /// 随机口令。
    #[serde(rename = "RandomPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_password: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// PutSecretValue 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutSecretValueRequest {
    /// 凭据的版本号，在该凭据内唯一。
    #[serde(rename = "VersionId")]
    pub version_id: String,
    /// 凭据名称或凭据资源名称（ARN）。
    #[serde(rename = "SecretName")]
    pub secret_name: String,
    /// 凭据值。加密后存入指定的新版本中。
    #[serde(rename = "SecretData")]
    pub secret_data: String,
    /// 凭据值类型。取值：
    #[serde(rename = "SecretDataType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_data_type: Option<String>,
    /// 凭据版本在存入时需要被同时标记的版本状态。如果您不指定此参数，KMS默认为新版本标记ACSCurrent。
    #[serde(rename = "VersionStages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_stages: Option<String>,
}

impl PutSecretValueRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("VersionId".to_string(), self.version_id.to_string()));
        params.push(("SecretName".to_string(), self.secret_name.to_string()));
        params.push(("SecretData".to_string(), self.secret_data.to_string()));
        if let Some(ref v) = self.secret_data_type {
            params.push(("SecretDataType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version_stages {
            params.push(("VersionStages".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutSecretValueResponse {
    /// 凭据名称。
    #[serde(rename = "SecretName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<String>,
    /// 凭据的版本号。
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "VersionStages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_stages: Option<PutSecretValueResponseVersionStages>,
}

/// RestoreSecret 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct RestoreSecretRequest {
    /// 凭据名称或凭据资源名称（ARN）。
    #[serde(rename = "SecretName")]
    pub secret_name: String,
}

impl RestoreSecretRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("SecretName".to_string(), self.secret_name.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RestoreSecretResponse {
    /// 凭据名称。
    #[serde(rename = "SecretName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<String>,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// RotateSecret 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct RotateSecretRequest {
    /// 凭据名称或凭据资源名称（ARN）。
    #[serde(rename = "SecretName")]
    pub secret_name: String,
    /// 轮转后的新凭据版本的版本号。
    #[serde(rename = "VersionId")]
    pub version_id: String,
}

impl RotateSecretRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("SecretName".to_string(), self.secret_name.to_string()));
        params.push(("VersionId".to_string(), self.version_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RotateSecretResponse {
    /// 轮转后的新凭据版本的版本号。
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    /// 凭据名称。
    #[serde(rename = "SecretName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<String>,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 凭据ARN。
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

/// SetSecretPolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetSecretPolicyRequest {
    /// 凭据名称或凭据资源名称（ARN）。
    #[serde(rename = "SecretName")]
    pub secret_name: String,
    /// 凭据策略名称。仅支持固定取值default。
    #[serde(rename = "PolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    /// 凭据策略的具体内容，JSON格式。最大长度为32768个字节。
    #[serde(rename = "Policy")]
    pub policy: String,
}

impl SetSecretPolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("SecretName".to_string(), self.secret_name.to_string()));
        if let Some(ref v) = self.policy_name {
            params.push(("PolicyName".to_string(), v.to_string()));
        }
        params.push(("Policy".to_string(), self.policy.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetSecretPolicyResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// GetSecretPolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetSecretPolicyRequest {
    /// 凭据名称或凭据资源名称（ARN）。
    #[serde(rename = "SecretName")]
    pub secret_name: String,
    /// 凭据策略名称。仅支持固定值default。
    #[serde(rename = "PolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}

impl GetSecretPolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("SecretName".to_string(), self.secret_name.to_string()));
        if let Some(ref v) = self.policy_name {
            params.push(("PolicyName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetSecretPolicyResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 凭据策略。
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

/// UntagResources 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UntagResourcesRequest {
    /// 资源所属的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 要解绑标签的资源类型。取值：
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// 是否为资源解绑所有标签。取值：
    #[serde(rename = "All")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
    /// 要解绑标签的资源ID列表，最多可以输入50个资源ID。
    #[serde(rename = "ResourceId")]
    pub resource_id: Vec<String>,
    /// 要解绑的标签的标签键，最多可以输入20个标签键。
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
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListTagResources 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListTagResourcesRequest {
    /// 资源所属的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 下一个查询开始的Token。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 要查询标签的资源类型。取值：
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// 要查询标签的资源ID列表，最多可以输入50个资源ID。
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<Vec<String>>,
    /// 要查询的标签列表。N的取值范围1~20。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<ListTagResourcesRequestTagItem>>,
}

impl ListTagResourcesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.next_token {
            params.push(("NextToken".to_string(), v.to_string()));
        }
        params.push(("ResourceType".to_string(), self.resource_type.to_string()));
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
    /// 本次调用返回的Token，根据取值判断是否具备下一个查询开始的Token。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
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
    /// 资源所属的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 要绑定标签的资源类型。取值：
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// 要绑定标签的资源ID列表，最多可以输入50个资源ID。
    #[serde(rename = "ResourceId")]
    pub resource_id: Vec<String>,
    /// 标签列表，最多可以输入20个标签。
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

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct TagResourcesResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListResourceTags 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListResourceTagsRequest {
    /// 全局唯一标识符。
    #[serde(rename = "KeyId")]
    pub key_id: String,
}

impl ListResourceTagsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("KeyId".to_string(), self.key_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListResourceTagsResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<ListResourceTagsResponseTags>,
}

/// TagResource 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct TagResourceRequest {
    /// 密钥的ID，也可以指定为密钥别名或密钥资源名称（ARN）。关于别名的详细介绍，请参见[管理密钥别名](~~480655~~)。
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// 一个或多个标签。格式为Tag对象数组。
    #[serde(rename = "Tags")]
    pub tags: String,
    /// 凭据名称。
    #[serde(rename = "SecretName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<String>,
    /// 证书ID。
    #[serde(rename = "CertificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_id: Option<String>,
}

impl TagResourceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key_id {
            params.push(("KeyId".to_string(), v.to_string()));
        }
        params.push(("Tags".to_string(), self.tags.to_string()));
        if let Some(ref v) = self.secret_name {
            params.push(("SecretName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.certificate_id {
            params.push(("CertificateId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct TagResourceResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// UntagResource 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UntagResourceRequest {
    /// 密钥ID。主密钥（CMK）的全局唯一标识符。
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// 一个或多个标签键，多个标签键用半角逗号（,）间隔。
    #[serde(rename = "TagKeys")]
    pub tag_keys: String,
    /// 凭据名称。
    #[serde(rename = "SecretName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<String>,
    /// 证书ID。
    #[serde(rename = "CertificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_id: Option<String>,
}

impl UntagResourceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key_id {
            params.push(("KeyId".to_string(), v.to_string()));
        }
        params.push(("TagKeys".to_string(), self.tag_keys.to_string()));
        if let Some(ref v) = self.secret_name {
            params.push(("SecretName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.certificate_id {
            params.push(("CertificateId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UntagResourceResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreateNetworkRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateNetworkRuleRequest {
    /// 网络控制规则名称。
    #[serde(rename = "Name")]
    pub name: String,
    /// 网络类型。
    #[serde(rename = "Type")]
    pub r#type: String,
    /// 描述信息。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 私网IP地址或者私网网段，各个IP地址间用半角逗号（,）分隔。
    #[serde(rename = "SourcePrivateIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_private_ip: Option<String>,
}

impl CreateNetworkRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Name".to_string(), self.name.to_string()));
        params.push(("Type".to_string(), self.r#type.to_string()));
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source_private_ip {
            params.push(("SourcePrivateIp".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateNetworkRuleResponse {
    /// 网络类型。
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 描述信息。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 私网IP地址或者私网网段。
    #[serde(rename = "SourcePrivateIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_private_ip: Option<String>,
    /// 网络控制规则的名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 网络控制规则的ARN。
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

/// ListNetworkRules 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListNetworkRulesRequest {
    /// 分页查询时，设置当前页面的页码。默认值为1。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 分页查询时，设置每页包含网络控制规则的数量。取值范围：1~100，默认值为20。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListNetworkRulesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
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
pub struct ListNetworkRulesResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 分页查询时，当前页面的页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 分页查询时，每页包含网络控制规则的数量。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 网络控制规则的总数量。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    #[serde(rename = "NetworkRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_rules: Option<ListNetworkRulesResponseNetworkRules>,
}

/// DescribeNetworkRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeNetworkRuleRequest {
    /// 要查询的网络控制规则名称。
    #[serde(rename = "Name")]
    pub name: String,
}

impl DescribeNetworkRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Name".to_string(), self.name.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeNetworkRuleResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 网络控制规则的ARN。
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// 网络类型。取值仅支持Private，即仅支持私网IP。
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 描述信息。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 私网IP地址或者私网网段。
    #[serde(rename = "SourcePrivateIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_private_ip: Option<String>,
}

/// UpdateNetworkRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateNetworkRuleRequest {
    /// 要更新的网络控制规则名称。
    #[serde(rename = "Name")]
    pub name: String,
    /// 更新后的描述信息。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 更新后的私网IP地址或者私网网段，各个IP地址间用半角逗号（,）分隔。
    #[serde(rename = "SourcePrivateIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_private_ip: Option<String>,
}

impl UpdateNetworkRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Name".to_string(), self.name.to_string()));
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source_private_ip {
            params.push(("SourcePrivateIp".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateNetworkRuleResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteNetworkRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteNetworkRuleRequest {
    /// 要删除的网络控制规则的名称。
    #[serde(rename = "Name")]
    pub name: String,
}

impl DeleteNetworkRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Name".to_string(), self.name.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteNetworkRuleResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreatePolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreatePolicyRequest {
    /// 权限策略名称。
    #[serde(rename = "Name")]
    pub name: String,
    /// 描述信息。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 权限策略的作用域。即要访问的KMS实例。
    #[serde(rename = "KmsInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_instance: Option<String>,
    /// 权限策略支持的操作。取值：
    #[serde(rename = "Permissions")]
    pub permissions: String,
    /// 允许访问的密钥和凭据。
    #[serde(rename = "Resources")]
    pub resources: String,
    /// 网络控制规则名称。
    #[serde(rename = "AccessControlRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control_rules: Option<String>,
}

impl CreatePolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Name".to_string(), self.name.to_string()));
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.kms_instance {
            params.push(("KmsInstance".to_string(), v.to_string()));
        }
        params.push(("Permissions".to_string(), self.permissions.to_string()));
        params.push(("Resources".to_string(), self.resources.to_string()));
        if let Some(ref v) = self.access_control_rules {
            params.push(("AccessControlRules".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreatePolicyResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 权限策略的ARN。
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// 权限策略名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 描述信息。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 权限策略的作用域。
    #[serde(rename = "KmsInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_instance: Option<String>,
    /// 权限策略支持的操作。
    #[serde(rename = "Permissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<String>,
    /// 允许访问的密钥和凭据。
    #[serde(rename = "Resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<String>,
    /// 网络控制规则名称。
    #[serde(rename = "AccessControlRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control_rules: Option<String>,
}

/// ListPolicies 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListPoliciesRequest {
    /// 分页查询时，设置当前页面的页码。默认值为1。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 分页查询时，设置每页包含权限策略的数量。取值范围：1~100，默认值为20。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListPoliciesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
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
pub struct ListPoliciesResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 分页查询时，当前页面的页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 分页查询时，每页包含权限策略的数量。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 权限策略的总数量。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    #[serde(rename = "Policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<ListPoliciesResponsePolicies>,
}

/// DescribePolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribePolicyRequest {
    /// 要查询的权限策略名称。
    #[serde(rename = "Name")]
    pub name: String,
}

impl DescribePolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Name".to_string(), self.name.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribePolicyResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 权限策略的ARN。
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// 权限策略名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 描述信息。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 权限策略的作用域。
    #[serde(rename = "KmsInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_instance: Option<String>,
    /// 权限策略支持的操作。
    #[serde(rename = "Permissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    /// 允许访问的密钥和凭据。
    #[serde(rename = "Resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<String>>,
    /// 绑定的网络控制规则。
    #[serde(rename = "AccessControlRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control_rules: Option<String>,
}

/// UpdatePolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdatePolicyRequest {
    /// 要更新的权限策略名称。
    #[serde(rename = "Name")]
    pub name: String,
    /// 更改后的权限策略支持的操作。取值：
    #[serde(rename = "Permissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<String>,
    /// 更改后的允许访问的密钥和凭据。
    #[serde(rename = "Resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<String>,
    /// 网络控制规则。
    #[serde(rename = "AccessControlRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control_rules: Option<String>,
    /// 描述信息。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl UpdatePolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Name".to_string(), self.name.to_string()));
        if let Some(ref v) = self.permissions {
            params.push(("Permissions".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resources {
            params.push(("Resources".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.access_control_rules {
            params.push(("AccessControlRules".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdatePolicyResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeletePolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeletePolicyRequest {
    /// 要删除的权限策略名称。
    #[serde(rename = "Name")]
    pub name: String,
}

impl DeletePolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Name".to_string(), self.name.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeletePolicyResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreateApplicationAccessPoint 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateApplicationAccessPointRequest {
    /// 应用接入点名称。
    #[serde(rename = "Name")]
    pub name: String,
    /// 描述信息。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 认证方式。目前仅支持ClientKey。
    #[serde(rename = "AuthenticationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_method: Option<String>,
    /// 绑定的权限策略。
    #[serde(rename = "Policies")]
    pub policies: String,
}

impl CreateApplicationAccessPointRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Name".to_string(), self.name.to_string()));
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.authentication_method {
            params.push(("AuthenticationMethod".to_string(), v.to_string()));
        }
        params.push(("Policies".to_string(), self.policies.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateApplicationAccessPointResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 描述信息。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 绑定的权限策略。
    #[serde(rename = "Policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<String>,
    /// 应用接入点名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 应用接入点的ARN。
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// 认证方式。
    #[serde(rename = "AuthenticationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_method: Option<String>,
}

/// ListApplicationAccessPoints 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListApplicationAccessPointsRequest {
    /// 分页查询时，设置当前页面的页码。默认值为1。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 分页查询时，设置每页包含应用接入点的数量。取值范围：1~100，默认值为20。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListApplicationAccessPointsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
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
pub struct ListApplicationAccessPointsResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 分页查询时，当前页面的页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 分页查询时，每页包含应用接入点的数量。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 应用接入点的总数量。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    #[serde(rename = "ApplicationAccessPoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_access_points: Option<ListApplicationAccessPointsResponseApplicationAccessPoints>,
}

/// DescribeApplicationAccessPoint 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeApplicationAccessPointRequest {
    /// 要查询的应用接入点名称。
    #[serde(rename = "Name")]
    pub name: String,
}

impl DescribeApplicationAccessPointRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Name".to_string(), self.name.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeApplicationAccessPointResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 应用接入点的ARN。
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// 应用接入点名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 描述信息。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 认证方式。
    #[serde(rename = "AuthenticationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_method: Option<String>,
    /// 绑定的权限策略。
    #[serde(rename = "Policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<String>,
}

/// UpdateApplicationAccessPoint 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateApplicationAccessPointRequest {
    /// 待更新的应用接入点名称。
    #[serde(rename = "Name")]
    pub name: String,
    /// 描述信息。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 更新绑定的权限策略。
    #[serde(rename = "Policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<String>,
}

impl UpdateApplicationAccessPointRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Name".to_string(), self.name.to_string()));
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policies {
            params.push(("Policies".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateApplicationAccessPointResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteApplicationAccessPoint 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteApplicationAccessPointRequest {
    /// 要删除的应用接入点的名称。
    #[serde(rename = "Name")]
    pub name: String,
}

impl DeleteApplicationAccessPointRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Name".to_string(), self.name.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteApplicationAccessPointResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreateClientKey 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateClientKeyRequest {
    /// 绑定的应用接入点名称。
    #[serde(rename = "AapName")]
    pub aap_name: String,
    /// Client Key加密口令。
    #[serde(rename = "Password")]
    pub password: String,
    /// ClientKey的有效期结束时间。
    #[serde(rename = "NotAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_after: Option<String>,
    /// ClientKey的有效期起始时间。
    #[serde(rename = "NotBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_before: Option<String>,
}

impl CreateClientKeyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("AapName".to_string(), self.aap_name.to_string()));
        params.push(("Password".to_string(), self.password.to_string()));
        if let Some(ref v) = self.not_after {
            params.push(("NotAfter".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.not_before {
            params.push(("NotBefore".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateClientKeyResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// ClientKey的ID。
    #[serde(rename = "ClientKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_key_id: Option<String>,
    /// 加密ClientKey私钥内容的算法。目前仅支持RSA_2048。
    #[serde(rename = "KeyAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_algorithm: Option<String>,
    /// ClientKey的私钥内容。
    #[serde(rename = "PrivateKeyData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key_data: Option<String>,
    /// ClientKey的有效期起始时间。
    #[serde(rename = "NotBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_before: Option<String>,
    /// ClientKey的有效期结束时间。
    #[serde(rename = "NotAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_after: Option<String>,
}

/// ListClientKeys 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListClientKeysRequest {
    /// 应用接入点名称。
    #[serde(rename = "AapName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aap_name: Option<String>,
}

impl ListClientKeysRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.aap_name {
            params.push(("AapName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListClientKeysResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// ClientKey列表。
    #[serde(rename = "ClientKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_keys: Option<Vec<ListClientKeysResponseClientKeysItem>>,
}

/// GetClientKey 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetClientKeyRequest {
    /// ClientKey的ID。
    #[serde(rename = "ClientKeyId")]
    pub client_key_id: String,
}

impl GetClientKeyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("ClientKeyId".to_string(), self.client_key_id.to_string()));
        params
    }
}

/// ClientKey详情。
#[derive(Debug, Clone, Deserialize)]
pub struct GetClientKeyResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// ClientKey的ID。
    #[serde(rename = "ClientKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_key_id: Option<String>,
    /// ClientKey的创建时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// ClientKey的私钥算法。
    #[serde(rename = "KeyAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_algorithm: Option<String>,
    /// ClientKey由谁生成。
    #[serde(rename = "KeyOrigin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_origin: Option<String>,
    /// ClientKey的公钥内容。
    #[serde(rename = "PublicKeyData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key_data: Option<String>,
    /// ClientKey的有效期结束时间。
    #[serde(rename = "NotAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_after: Option<String>,
    /// ClientKey的有效期起始时间。
    #[serde(rename = "NotBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_before: Option<String>,
    /// 应用接入点名称。
    #[serde(rename = "AapName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aap_name: Option<String>,
}

/// DeleteClientKey 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteClientKeyRequest {
    /// ClientKey的ID。
    #[serde(rename = "ClientKeyId")]
    pub client_key_id: String,
}

impl DeleteClientKeyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("ClientKeyId".to_string(), self.client_key_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteClientKeyResponse {
    /// 本次调用请求ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// GetKmsInstanceQuotaInfos 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetKmsInstanceQuotaInfosRequest {
    /// 要查询的KMS实例的ID。
    #[serde(rename = "KmsInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_instance_id: Option<String>,
    /// 资源类型。取值：
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

impl GetKmsInstanceQuotaInfosRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.kms_instance_id {
            params.push(("KmsInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_type {
            params.push(("ResourceType".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct GetKmsInstanceQuotaInfosResponse {
    /// Id of the request
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// KMS实例ID。
    #[serde(rename = "KmsInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_instance_id: Option<String>,
    /// 实例配额信息数组
    #[serde(rename = "KmsInstanceQuotaInfos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_instance_quota_infos: Option<Vec<GetKmsInstanceQuotaInfosResponseKmsInstanceQuotaInfosItem>>,
}
