//! 类型定义 - 自动生成，请勿手动修改

#![allow(unused_mut)]

use serde::{Deserialize, Serialize};

/// RAM用户信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateUserResponseUser {
    /// RAM用户的显示名称。
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// RAM用户的电子邮箱。
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// RAM用户的手机号码。
    #[serde(rename = "MobilePhone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_phone: Option<String>,
    /// RAM用户的唯一标识。
    #[serde(rename = "UserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 备注。
    #[serde(rename = "Comments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// RAM用户的创建时间（UTC时间）。
    #[serde(rename = "CreateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    /// RAM用户的名称。
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

impl CreateUserResponseUser {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.display_name {
            params.push(("DisplayName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.email {
            params.push(("Email".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.mobile_phone {
            params.push(("MobilePhone".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_id {
            params.push(("UserId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.comments {
            params.push(("Comments".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_date {
            params.push(("CreateDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_name {
            params.push(("UserName".to_string(), v.to_string()));
        }
        params
    }
}

/// RAM用户信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetUserResponseUser {
    /// 显示名称。
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// RAM用户的电子邮箱。
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// RAM用户的更新时间（UTC时间）。
    #[serde(rename = "UpdateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_date: Option<String>,
    /// RAM用户的手机号码。
    #[serde(rename = "MobilePhone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_phone: Option<String>,
    /// RAM用户的唯一标识。
    #[serde(rename = "UserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 备注。
    #[serde(rename = "Comments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// 上次使用密码登录时间（UTC时间）。
    #[serde(rename = "LastLoginDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_login_date: Option<String>,
    /// RAM用户的创建时间（UTC时间）。
    #[serde(rename = "CreateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    /// RAM用户的名称。
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

impl GetUserResponseUser {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.display_name {
            params.push(("DisplayName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.email {
            params.push(("Email".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_date {
            params.push(("UpdateDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.mobile_phone {
            params.push(("MobilePhone".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_id {
            params.push(("UserId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.comments {
            params.push(("Comments".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.last_login_date {
            params.push(("LastLoginDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_date {
            params.push(("CreateDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_name {
            params.push(("UserName".to_string(), v.to_string()));
        }
        params
    }
}

/// RAM用户信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateUserResponseUser {
    /// RAM用户的显示名称。
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// RAM用户的电子邮箱。
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// RAM用户的更新时间（UTC时间）。
    #[serde(rename = "UpdateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_date: Option<String>,
    /// RAM用户的手机号码。
    #[serde(rename = "MobilePhone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_phone: Option<String>,
    /// RAM用户的唯一标识。
    #[serde(rename = "UserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 备注。
    #[serde(rename = "Comments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// RAM用户的创建时间（UTC时间）。
    #[serde(rename = "CreateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    /// RAM用户的名称。
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

impl UpdateUserResponseUser {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.display_name {
            params.push(("DisplayName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.email {
            params.push(("Email".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_date {
            params.push(("UpdateDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.mobile_phone {
            params.push(("MobilePhone".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_id {
            params.push(("UserId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.comments {
            params.push(("Comments".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_date {
            params.push(("CreateDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_name {
            params.push(("UserName".to_string(), v.to_string()));
        }
        params
    }
}

/// RAM用户信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListUsersResponseUsersUserItem {
    /// RAM用户的显示名称。
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// RAM用户邮箱。
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// RAM用户的更新时间（UTC时间）。
    #[serde(rename = "UpdateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_date: Option<String>,
    /// RAM用户手机号码。
    #[serde(rename = "MobilePhone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_phone: Option<String>,
    /// RAM用户ID。
    #[serde(rename = "UserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 备注。
    #[serde(rename = "Comments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// RAM用户的创建时间（UTC时间）。
    #[serde(rename = "CreateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    /// RAM用户的登录名称。
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

impl ListUsersResponseUsersUserItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.display_name {
            params.push(("DisplayName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.email {
            params.push(("Email".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_date {
            params.push(("UpdateDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.mobile_phone {
            params.push(("MobilePhone".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_id {
            params.push(("UserId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.comments {
            params.push(("Comments".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_date {
            params.push(("CreateDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_name {
            params.push(("UserName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListUsersResponseUsers {
    /// RAM用户列表。
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<Vec<ListUsersResponseUsersUserItem>>,
}

impl ListUsersResponseUsers {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.user {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("User.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 登录配置信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateLoginProfileResponseLoginProfile {
    /// 要求下次登录时重设密码。
    #[serde(rename = "PasswordResetRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_reset_required: Option<bool>,
    /// 创建时间。
    #[serde(rename = "CreateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    /// 用户名。
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// 要求必须绑定多因素认证设备。
    #[serde(rename = "MFABindRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_bind_required: Option<bool>,
}

impl CreateLoginProfileResponseLoginProfile {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.password_reset_required {
            params.push(("PasswordResetRequired".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_date {
            params.push(("CreateDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_name {
            params.push(("UserName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.mfa_bind_required {
            params.push(("MFABindRequired".to_string(), v.to_string()));
        }
        params
    }
}

/// 登录配置信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetLoginProfileResponseLoginProfile {
    /// 要求下次登录时重设密码。
    #[serde(rename = "PasswordResetRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_reset_required: Option<bool>,
    /// 创建时间。
    #[serde(rename = "CreateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    /// 用户名。
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// 要求必须绑定多因素认证设备。
    #[serde(rename = "MFABindRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_bind_required: Option<bool>,
}

impl GetLoginProfileResponseLoginProfile {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.password_reset_required {
            params.push(("PasswordResetRequired".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_date {
            params.push(("CreateDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_name {
            params.push(("UserName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.mfa_bind_required {
            params.push(("MFABindRequired".to_string(), v.to_string()));
        }
        params
    }
}

/// 访问密钥。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateAccessKeyResponseAccessKey {
    /// 状态，激活或禁用。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 访问密钥。
    #[serde(rename = "AccessKeySecret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_secret: Option<String>,
    /// 创建时间。
    #[serde(rename = "CreateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    /// 访问密钥标识。
    #[serde(rename = "AccessKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
}

impl CreateAccessKeyResponseAccessKey {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.access_key_secret {
            params.push(("AccessKeySecret".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_date {
            params.push(("CreateDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.access_key_id {
            params.push(("AccessKeyId".to_string(), v.to_string()));
        }
        params
    }
}

/// 用户信息集合。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListAccessKeysResponseAccessKeysAccessKeyItem {
    /// 状态，激活或禁用。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 访问密钥标识。
    #[serde(rename = "AccessKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    /// 创建时间。
    #[serde(rename = "CreateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
}

impl ListAccessKeysResponseAccessKeysAccessKeyItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.access_key_id {
            params.push(("AccessKeyId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_date {
            params.push(("CreateDate".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListAccessKeysResponseAccessKeys {
    /// 用户信息集合。
    #[serde(rename = "AccessKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key: Option<Vec<ListAccessKeysResponseAccessKeysAccessKeyItem>>,
}

impl ListAccessKeysResponseAccessKeys {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.access_key {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("AccessKey.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 访问密钥的最后使用信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetAccessKeyLastUsedResponseAccessKeyLastUsed {
    /// 最后使用时间。
    #[serde(rename = "LastUsedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_used_date: Option<String>,
}

impl GetAccessKeyLastUsedResponseAccessKeyLastUsed {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.last_used_date {
            params.push(("LastUsedDate".to_string(), v.to_string()));
        }
        params
    }
}

/// 多因素认证设备。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateVirtualMFADeviceResponseVirtualMFADevice {
    /// 设备序列号。
    #[serde(rename = "SerialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// 密钥二维码PNG，使用Base64编码。
    #[serde(rename = "QRCodePNG")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qr_code_png: Option<String>,
    /// 多因素认证设备密钥。
    #[serde(rename = "Base32StringSeed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base32_string_seed: Option<String>,
}

impl CreateVirtualMFADeviceResponseVirtualMFADevice {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.serial_number {
            params.push(("SerialNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.qr_code_png {
            params.push(("QRCodePNG".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.base32_string_seed {
            params.push(("Base32StringSeed".to_string(), v.to_string()));
        }
        params
    }
}

/// MFA设备信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetUserMFAInfoResponseMFADevice {
    /// 设备序列号。
    #[serde(rename = "SerialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// 多因素认证设备类型。取值：
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl GetUserMFAInfoResponseMFADevice {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.serial_number {
            params.push(("SerialNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        params
    }
}

/// 绑定用户的基本信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListVirtualMFADevicesResponseVirtualMFADevicesVirtualMFADeviceItemUser {
    /// 显示名称。
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// 用户唯一标识。
    #[serde(rename = "UserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户名称。
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

impl ListVirtualMFADevicesResponseVirtualMFADevicesVirtualMFADeviceItemUser {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.display_name {
            params.push(("DisplayName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_id {
            params.push(("UserId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_name {
            params.push(("UserName".to_string(), v.to_string()));
        }
        params
    }
}

/// 多因素认证设备列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListVirtualMFADevicesResponseVirtualMFADevicesVirtualMFADeviceItem {
    /// 多因素认证设备序列号。
    #[serde(rename = "SerialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// 绑定用户的基本信息。
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<ListVirtualMFADevicesResponseVirtualMFADevicesVirtualMFADeviceItemUser>,
    /// 激活日期。
    #[serde(rename = "ActivateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activate_date: Option<String>,
}

impl ListVirtualMFADevicesResponseVirtualMFADevicesVirtualMFADeviceItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.serial_number {
            params.push(("SerialNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user {
            for (k, v2) in v.to_query_params() {
                params.push((format!("User.{}", k), v2));
            }
        }
        if let Some(ref v) = self.activate_date {
            params.push(("ActivateDate".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListVirtualMFADevicesResponseVirtualMFADevices {
    /// 多因素认证设备列表。
    #[serde(rename = "VirtualMFADevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_mfa_device: Option<Vec<ListVirtualMFADevicesResponseVirtualMFADevicesVirtualMFADeviceItem>>,
}

impl ListVirtualMFADevicesResponseVirtualMFADevices {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.virtual_mfa_device {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("VirtualMFADevice.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 多因素认证设备信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnbindMFADeviceResponseMFADevice {
    /// 设备序列号。
    #[serde(rename = "SerialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
}

impl UnbindMFADeviceResponseMFADevice {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.serial_number {
            params.push(("SerialNumber".to_string(), v.to_string()));
        }
        params
    }
}

/// 用户组信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateGroupResponseGroup {
    /// 用户组ID。
    #[serde(rename = "GroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// 用户组名称。
    #[serde(rename = "GroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// 备注信息。
    #[serde(rename = "Comments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// 创建时间。
    #[serde(rename = "CreateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
}

impl CreateGroupResponseGroup {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.group_id {
            params.push(("GroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.group_name {
            params.push(("GroupName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.comments {
            params.push(("Comments".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_date {
            params.push(("CreateDate".to_string(), v.to_string()));
        }
        params
    }
}

/// 用户组信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetGroupResponseGroup {
    /// 用户组ID。
    #[serde(rename = "GroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// 更新时间。
    #[serde(rename = "UpdateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_date: Option<String>,
    /// 用户组名称。
    #[serde(rename = "GroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// 备注信息。
    #[serde(rename = "Comments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// 创建时间。
    #[serde(rename = "CreateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
}

impl GetGroupResponseGroup {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.group_id {
            params.push(("GroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_date {
            params.push(("UpdateDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.group_name {
            params.push(("GroupName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.comments {
            params.push(("Comments".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_date {
            params.push(("CreateDate".to_string(), v.to_string()));
        }
        params
    }
}

/// 用户组信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateGroupResponseGroup {
    /// 用户组ID。
    #[serde(rename = "GroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// 更新时间。
    #[serde(rename = "UpdateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_date: Option<String>,
    /// 用户组名称。
    #[serde(rename = "GroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// 备注信息。
    #[serde(rename = "Comments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// 创建时间。
    #[serde(rename = "CreateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
}

impl UpdateGroupResponseGroup {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.group_id {
            params.push(("GroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_date {
            params.push(("UpdateDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.group_name {
            params.push(("GroupName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.comments {
            params.push(("Comments".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_date {
            params.push(("CreateDate".to_string(), v.to_string()));
        }
        params
    }
}

/// 用户组信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListGroupsResponseGroupsGroupItem {
    /// 用户组ID。
    #[serde(rename = "GroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// 更新时间。
    #[serde(rename = "UpdateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_date: Option<String>,
    /// 用户组名称。
    #[serde(rename = "GroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// 备注信息。
    #[serde(rename = "Comments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// 创建时间。
    #[serde(rename = "CreateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
}

impl ListGroupsResponseGroupsGroupItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.group_id {
            params.push(("GroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_date {
            params.push(("UpdateDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.group_name {
            params.push(("GroupName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.comments {
            params.push(("Comments".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_date {
            params.push(("CreateDate".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListGroupsResponseGroups {
    /// 用户组信息。
    #[serde(rename = "Group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Vec<ListGroupsResponseGroupsGroupItem>>,
}

impl ListGroupsResponseGroups {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.group {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Group.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 用户组信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListGroupsForUserResponseGroupsGroupItem {
    /// 用户组ID。
    #[serde(rename = "GroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// 用户组名称。
    #[serde(rename = "GroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// 备注信息。
    #[serde(rename = "Comments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// 加入时间。
    #[serde(rename = "JoinDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_date: Option<String>,
}

impl ListGroupsForUserResponseGroupsGroupItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.group_id {
            params.push(("GroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.group_name {
            params.push(("GroupName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.comments {
            params.push(("Comments".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.join_date {
            params.push(("JoinDate".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListGroupsForUserResponseGroups {
    /// 用户组信息。
    #[serde(rename = "Group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Vec<ListGroupsForUserResponseGroupsGroupItem>>,
}

impl ListGroupsForUserResponseGroups {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.group {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Group.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 用户信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListUsersForGroupResponseUsersUserItem {
    /// 显示名称。
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// 加入日期。
    #[serde(rename = "JoinDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_date: Option<String>,
    /// 用户名称。
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

impl ListUsersForGroupResponseUsersUserItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.display_name {
            params.push(("DisplayName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.join_date {
            params.push(("JoinDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_name {
            params.push(("UserName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListUsersForGroupResponseUsers {
    /// 用户信息。
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<Vec<ListUsersForGroupResponseUsersUserItem>>,
}

impl ListUsersForGroupResponseUsers {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.user {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("User.{}", i + 1);
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
pub struct CreateRoleRequestTagItem {
    /// 标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreateRoleRequestTagItem {
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

/// RAM角色信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateRoleResponseRole {
    /// RAM角色的信任策略。
    #[serde(rename = "AssumeRolePolicyDocument")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assume_role_policy_document: Option<String>,
    /// RAM角色描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// RAM角色最大会话时间。
    #[serde(rename = "MaxSessionDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_session_duration: Option<i64>,
    /// RAM角色名称。
    #[serde(rename = "RoleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    /// 创建时间。
    #[serde(rename = "CreateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    /// RAM角色ID。
    #[serde(rename = "RoleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
    /// RAM角色的资源描述符。
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

impl CreateRoleResponseRole {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.assume_role_policy_document {
            params.push(("AssumeRolePolicyDocument".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_session_duration {
            params.push(("MaxSessionDuration".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.role_name {
            params.push(("RoleName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_date {
            params.push(("CreateDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.role_id {
            params.push(("RoleId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.arn {
            params.push(("Arn".to_string(), v.to_string()));
        }
        params
    }
}

/// RAM角色信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateRoleResponseRole {
    /// RAM角色的信任策略。
    #[serde(rename = "AssumeRolePolicyDocument")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assume_role_policy_document: Option<String>,
    /// RAM角色的更新时间。
    #[serde(rename = "UpdateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_date: Option<String>,
    /// RAM角色描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// RAM角色最大会话时间。
    #[serde(rename = "MaxSessionDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_session_duration: Option<i64>,
    /// RAM角色名称。
    #[serde(rename = "RoleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    /// RAM角色的创建时间。
    #[serde(rename = "CreateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    /// RAM角色ID。
    #[serde(rename = "RoleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
    /// RAM角色的资源描述符。
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

impl UpdateRoleResponseRole {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.assume_role_policy_document {
            params.push(("AssumeRolePolicyDocument".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_date {
            params.push(("UpdateDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_session_duration {
            params.push(("MaxSessionDuration".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.role_name {
            params.push(("RoleName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_date {
            params.push(("CreateDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.role_id {
            params.push(("RoleId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.arn {
            params.push(("Arn".to_string(), v.to_string()));
        }
        params
    }
}

/// 角色信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetRoleResponseRole {
    /// 扮演角色的权限策略。
    #[serde(rename = "AssumeRolePolicyDocument")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assume_role_policy_document: Option<String>,
    /// 更新时间。
    #[serde(rename = "UpdateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_date: Option<String>,
    /// 角色描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 角色最大会话时间。
    #[serde(rename = "MaxSessionDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_session_duration: Option<i64>,
    /// 角色名称。
    #[serde(rename = "RoleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    /// 创建时间。
    #[serde(rename = "CreateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    /// 角色ID。
    #[serde(rename = "RoleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
    /// 角色的资源描述符。
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

impl GetRoleResponseRole {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.assume_role_policy_document {
            params.push(("AssumeRolePolicyDocument".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_date {
            params.push(("UpdateDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_session_duration {
            params.push(("MaxSessionDuration".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.role_name {
            params.push(("RoleName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_date {
            params.push(("CreateDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.role_id {
            params.push(("RoleId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.arn {
            params.push(("Arn".to_string(), v.to_string()));
        }
        params
    }
}

/// 标签。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListRolesRequestTagItem {
    /// 标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ListRolesRequestTagItem {
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
pub struct ListRolesResponseRolesRoleItemTagsTagItem {
    /// 标签键。
    #[serde(rename = "TagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    /// 标签值。
    #[serde(rename = "TagValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
}

impl ListRolesResponseRolesRoleItemTagsTagItem {
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
pub struct ListRolesResponseRolesRoleItemTags {
    /// 标签。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<ListRolesResponseRolesRoleItemTagsTagItem>>,
}

impl ListRolesResponseRolesRoleItemTags {
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

/// 角色信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListRolesResponseRolesRoleItem {
    /// 角色描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 更新时间。
    #[serde(rename = "UpdateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_date: Option<String>,
    /// 角色最大会话时间。
    #[serde(rename = "MaxSessionDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_session_duration: Option<i64>,
    /// 角色名称。
    #[serde(rename = "RoleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    /// 创建时间。
    #[serde(rename = "CreateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    /// 角色ID。
    #[serde(rename = "RoleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
    /// 角色的资源描述符。
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<ListRolesResponseRolesRoleItemTags>,
}

impl ListRolesResponseRolesRoleItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_date {
            params.push(("UpdateDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_session_duration {
            params.push(("MaxSessionDuration".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.role_name {
            params.push(("RoleName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_date {
            params.push(("CreateDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.role_id {
            params.push(("RoleId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.arn {
            params.push(("Arn".to_string(), v.to_string()));
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
pub struct ListRolesResponseRoles {
    /// 角色信息。
    #[serde(rename = "Role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<Vec<ListRolesResponseRolesRoleItem>>,
}

impl ListRolesResponseRoles {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.role {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Role.{}", i + 1);
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
pub struct CreatePolicyRequestTagItem {
    /// 标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreatePolicyRequestTagItem {
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

/// 权限策略信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreatePolicyResponsePolicy {
    /// 权限策略版本。默认值：v1。
    #[serde(rename = "DefaultVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version: Option<String>,
    /// 权限策略描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 权限策略名称。
    #[serde(rename = "PolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    /// 权限策略创建时间。
    #[serde(rename = "CreateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    /// 权限策略类型。取值：
    #[serde(rename = "PolicyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
}

impl CreatePolicyResponsePolicy {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.default_version {
            params.push(("DefaultVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_name {
            params.push(("PolicyName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_date {
            params.push(("CreateDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_type {
            params.push(("PolicyType".to_string(), v.to_string()));
        }
        params
    }
}

/// 权限策略的基本信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetPolicyResponsePolicy {
    /// 默认版本。
    #[serde(rename = "DefaultVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version: Option<String>,
    /// 修改时间。
    #[serde(rename = "UpdateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_date: Option<String>,
    /// 权限策略描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 已废弃。
    #[serde(rename = "PolicyDocument")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_document: Option<String>,
    /// 引用次数。
    #[serde(rename = "AttachmentCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_count: Option<i32>,
    /// 权限策略名称。
    #[serde(rename = "PolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    /// 创建时间。
    #[serde(rename = "CreateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    /// 权限策略类型。
    #[serde(rename = "PolicyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
}

impl GetPolicyResponsePolicy {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.default_version {
            params.push(("DefaultVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_date {
            params.push(("UpdateDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_document {
            params.push(("PolicyDocument".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.attachment_count {
            params.push(("AttachmentCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_name {
            params.push(("PolicyName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_date {
            params.push(("CreateDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_type {
            params.push(("PolicyType".to_string(), v.to_string()));
        }
        params
    }
}

/// 权限策略的默认版本。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetPolicyResponseDefaultPolicyVersion {
    /// `DefaultPolicyVersion`返回的数据结构固定为默认版本，因此`IsDefaultVersion`的取值固定为`true`。
    #[serde(rename = "IsDefaultVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default_version: Option<bool>,
    /// 权限策略内容。
    #[serde(rename = "PolicyDocument")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_document: Option<String>,
    /// 权限策略版本。
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    /// 创建时间。
    #[serde(rename = "CreateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
}

impl GetPolicyResponseDefaultPolicyVersion {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.is_default_version {
            params.push(("IsDefaultVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_document {
            params.push(("PolicyDocument".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version_id {
            params.push(("VersionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_date {
            params.push(("CreateDate".to_string(), v.to_string()));
        }
        params
    }
}

/// 权限策略信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdatePolicyDescriptionResponsePolicy {
    /// 权限策略名称。
    #[serde(rename = "PolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    /// 权限策略类型。取值：
    #[serde(rename = "PolicyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
    /// 权限策略创建时间。
    #[serde(rename = "CreateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    /// 权限策略更新时间。
    #[serde(rename = "UpdateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_date: Option<String>,
    /// 权限策略描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 权限策略版本。默认值：v1。
    #[serde(rename = "DefaultVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version: Option<String>,
}

impl UpdatePolicyDescriptionResponsePolicy {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.policy_name {
            params.push(("PolicyName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_type {
            params.push(("PolicyType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_date {
            params.push(("CreateDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_date {
            params.push(("UpdateDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.default_version {
            params.push(("DefaultVersion".to_string(), v.to_string()));
        }
        params
    }
}

/// 标签。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListPoliciesRequestTagItem {
    /// 标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ListPoliciesRequestTagItem {
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
pub struct ListPoliciesResponsePoliciesPolicyItemTagsTagItem {
    /// 标签键。
    #[serde(rename = "TagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    /// 标签值。
    #[serde(rename = "TagValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
}

impl ListPoliciesResponsePoliciesPolicyItemTagsTagItem {
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
pub struct ListPoliciesResponsePoliciesPolicyItemTags {
    /// 标签。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<ListPoliciesResponsePoliciesPolicyItemTagsTagItem>>,
}

impl ListPoliciesResponsePoliciesPolicyItemTags {
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

/// 权限策略信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListPoliciesResponsePoliciesPolicyItem {
    /// 默认版本。
    #[serde(rename = "DefaultVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version: Option<String>,
    /// 权限策略描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 修改时间。
    #[serde(rename = "UpdateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_date: Option<String>,
    /// 引用次数。
    #[serde(rename = "AttachmentCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_count: Option<i32>,
    /// 权限策略名称。
    #[serde(rename = "PolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    /// 创建时间。
    #[serde(rename = "CreateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    /// 权限策略类型。
    #[serde(rename = "PolicyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<ListPoliciesResponsePoliciesPolicyItemTags>,
}

impl ListPoliciesResponsePoliciesPolicyItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.default_version {
            params.push(("DefaultVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_date {
            params.push(("UpdateDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.attachment_count {
            params.push(("AttachmentCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_name {
            params.push(("PolicyName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_date {
            params.push(("CreateDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_type {
            params.push(("PolicyType".to_string(), v.to_string()));
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
pub struct ListPoliciesResponsePolicies {
    /// 权限策略信息。
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

/// 新建的权限策略版本的信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreatePolicyVersionResponsePolicyVersion {
    /// 是否默认版本。
    #[serde(rename = "IsDefaultVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default_version: Option<bool>,
    /// 权限策略内容。
    #[serde(rename = "PolicyDocument")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_document: Option<String>,
    /// 权限策略标识。
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    /// 创建时间。
    #[serde(rename = "CreateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
}

impl CreatePolicyVersionResponsePolicyVersion {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.is_default_version {
            params.push(("IsDefaultVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_document {
            params.push(("PolicyDocument".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version_id {
            params.push(("VersionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_date {
            params.push(("CreateDate".to_string(), v.to_string()));
        }
        params
    }
}

/// 权限策略版本信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetPolicyVersionResponsePolicyVersion {
    /// 是否默认版本。
    #[serde(rename = "IsDefaultVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default_version: Option<bool>,
    /// 权限策略内容。
    #[serde(rename = "PolicyDocument")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_document: Option<String>,
    /// 权限策略标识。
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    /// 创建时间。
    #[serde(rename = "CreateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
}

impl GetPolicyVersionResponsePolicyVersion {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.is_default_version {
            params.push(("IsDefaultVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_document {
            params.push(("PolicyDocument".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version_id {
            params.push(("VersionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_date {
            params.push(("CreateDate".to_string(), v.to_string()));
        }
        params
    }
}

/// 权限策略版本信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListPolicyVersionsResponsePolicyVersionsPolicyVersionItem {
    /// 是否默认版本。
    #[serde(rename = "IsDefaultVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default_version: Option<bool>,
    /// 权限策略内容。
    #[serde(rename = "PolicyDocument")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_document: Option<String>,
    /// 权限策略标识。
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    /// 创建时间。
    #[serde(rename = "CreateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
}

impl ListPolicyVersionsResponsePolicyVersionsPolicyVersionItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.is_default_version {
            params.push(("IsDefaultVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_document {
            params.push(("PolicyDocument".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version_id {
            params.push(("VersionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_date {
            params.push(("CreateDate".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListPolicyVersionsResponsePolicyVersions {
    /// 权限策略版本信息。
    #[serde(rename = "PolicyVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_version: Option<Vec<ListPolicyVersionsResponsePolicyVersionsPolicyVersionItem>>,
}

impl ListPolicyVersionsResponsePolicyVersions {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.policy_version {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("PolicyVersion.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 权限策略信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListPoliciesForUserResponsePoliciesPolicyItem {
    /// 当前版本。
    #[serde(rename = "DefaultVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version: Option<String>,
    /// 权限策略描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 权限策略名称。
    #[serde(rename = "PolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    /// 授权时间（UTC时间）。
    #[serde(rename = "AttachDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_date: Option<String>,
    /// 权限策略类型。取值：
    #[serde(rename = "PolicyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
}

impl ListPoliciesForUserResponsePoliciesPolicyItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.default_version {
            params.push(("DefaultVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_name {
            params.push(("PolicyName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.attach_date {
            params.push(("AttachDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_type {
            params.push(("PolicyType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListPoliciesForUserResponsePolicies {
    /// 权限策略信息。
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<Vec<ListPoliciesForUserResponsePoliciesPolicyItem>>,
}

impl ListPoliciesForUserResponsePolicies {
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

/// 权限策略信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListPoliciesForGroupResponsePoliciesPolicyItem {
    /// 默认版本。
    #[serde(rename = "DefaultVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version: Option<String>,
    /// 权限策略描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 权限策略名称。
    #[serde(rename = "PolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    /// 授权时间。
    #[serde(rename = "AttachDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_date: Option<String>,
    /// 权限策略类型。
    #[serde(rename = "PolicyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
}

impl ListPoliciesForGroupResponsePoliciesPolicyItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.default_version {
            params.push(("DefaultVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_name {
            params.push(("PolicyName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.attach_date {
            params.push(("AttachDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_type {
            params.push(("PolicyType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListPoliciesForGroupResponsePolicies {
    /// 权限策略信息。
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<Vec<ListPoliciesForGroupResponsePoliciesPolicyItem>>,
}

impl ListPoliciesForGroupResponsePolicies {
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

/// 权限策略信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListPoliciesForRoleResponsePoliciesPolicyItem {
    /// 默认版本。
    #[serde(rename = "DefaultVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version: Option<String>,
    /// 权限策略描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 权限策略名称。
    #[serde(rename = "PolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    /// 授权时间。
    #[serde(rename = "AttachDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_date: Option<String>,
    /// 权限策略类型。
    #[serde(rename = "PolicyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
}

impl ListPoliciesForRoleResponsePoliciesPolicyItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.default_version {
            params.push(("DefaultVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_name {
            params.push(("PolicyName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.attach_date {
            params.push(("AttachDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_type {
            params.push(("PolicyType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListPoliciesForRoleResponsePolicies {
    /// 权限策略信息。
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<Vec<ListPoliciesForRoleResponsePoliciesPolicyItem>>,
}

impl ListPoliciesForRoleResponsePolicies {
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

/// 用户组信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListEntitiesForPolicyResponseGroupsGroupItem {
    /// 组名称。
    #[serde(rename = "GroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// 备注信息。
    #[serde(rename = "Comments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// 授权时间。
    #[serde(rename = "AttachDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_date: Option<String>,
}

impl ListEntitiesForPolicyResponseGroupsGroupItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.group_name {
            params.push(("GroupName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.comments {
            params.push(("Comments".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.attach_date {
            params.push(("AttachDate".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListEntitiesForPolicyResponseGroups {
    /// 用户组信息。
    #[serde(rename = "Group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Vec<ListEntitiesForPolicyResponseGroupsGroupItem>>,
}

impl ListEntitiesForPolicyResponseGroups {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.group {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Group.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 角色信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListEntitiesForPolicyResponseRolesRoleItem {
    /// 角色的文字描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 角色名称。
    #[serde(rename = "RoleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    /// 授权时间。
    #[serde(rename = "AttachDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_date: Option<String>,
    /// 角色的资源描述符。
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// 角色ID。
    #[serde(rename = "RoleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
}

impl ListEntitiesForPolicyResponseRolesRoleItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.role_name {
            params.push(("RoleName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.attach_date {
            params.push(("AttachDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.arn {
            params.push(("Arn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.role_id {
            params.push(("RoleId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListEntitiesForPolicyResponseRoles {
    /// 角色信息。
    #[serde(rename = "Role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<Vec<ListEntitiesForPolicyResponseRolesRoleItem>>,
}

impl ListEntitiesForPolicyResponseRoles {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.role {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Role.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 用户信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListEntitiesForPolicyResponseUsersUserItem {
    /// 显示名称。
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// 用户唯一标识。
    #[serde(rename = "UserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户名称。
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// 授权时间。
    #[serde(rename = "AttachDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_date: Option<String>,
}

impl ListEntitiesForPolicyResponseUsersUserItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.display_name {
            params.push(("DisplayName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_id {
            params.push(("UserId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_name {
            params.push(("UserName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.attach_date {
            params.push(("AttachDate".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListEntitiesForPolicyResponseUsers {
    /// 用户信息。
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<Vec<ListEntitiesForPolicyResponseUsersUserItem>>,
}

impl ListEntitiesForPolicyResponseUsers {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.user {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("User.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 密码策略。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetPasswordPolicyResponsePasswordPolicy {
    /// 必须包含数字。
    #[serde(rename = "RequireNumbers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_numbers: Option<bool>,
    /// 必须包含小写字母。
    #[serde(rename = "RequireLowercaseCharacters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_lowercase_characters: Option<bool>,
    /// 密码是否过期。
    #[serde(rename = "HardExpiry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hard_expiry: Option<bool>,
    /// 禁止使用前N次密码。0表示不启用历史密码检查策略，默认不启用。
    #[serde(rename = "PasswordReusePrevention")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_reuse_prevention: Option<i32>,
    /// 必须包含字符。
    #[serde(rename = "RequireSymbols")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_symbols: Option<bool>,
    /// 密码有效期，单位是天（重置密码将重置密码过期时间）。0表示不启用密码过期策略，默认不启用。
    #[serde(rename = "MaxPasswordAge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_password_age: Option<i32>,
    /// 最小密码长度。
    #[serde(rename = "MinimumPasswordLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_password_length: Option<i32>,
    /// 必须包含大写字母。
    #[serde(rename = "RequireUppercaseCharacters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_uppercase_characters: Option<bool>,
    /// 一小时内使用错误密码尝试登录最大次数（重置密码可清除尝试登录计数）。
    #[serde(rename = "MaxLoginAttemps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_login_attemps: Option<i32>,
}

impl SetPasswordPolicyResponsePasswordPolicy {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.require_numbers {
            params.push(("RequireNumbers".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.require_lowercase_characters {
            params.push(("RequireLowercaseCharacters".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.hard_expiry {
            params.push(("HardExpiry".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.password_reuse_prevention {
            params.push(("PasswordReusePrevention".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.require_symbols {
            params.push(("RequireSymbols".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_password_age {
            params.push(("MaxPasswordAge".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.minimum_password_length {
            params.push(("MinimumPasswordLength".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.require_uppercase_characters {
            params.push(("RequireUppercaseCharacters".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_login_attemps {
            params.push(("MaxLoginAttemps".to_string(), v.to_string()));
        }
        params
    }
}

/// 密码策略。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetPasswordPolicyResponsePasswordPolicy {
    /// 必须包含数字。
    #[serde(rename = "RequireNumbers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_numbers: Option<bool>,
    /// 必须包含小写字母。
    #[serde(rename = "RequireLowercaseCharacters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_lowercase_characters: Option<bool>,
    /// 密码是否过期。
    #[serde(rename = "HardExpiry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hard_expiry: Option<bool>,
    /// 禁止使用前N次密码。0表示不启用历史密码检查策略，默认不启用。
    #[serde(rename = "PasswordReusePrevention")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_reuse_prevention: Option<i32>,
    /// 必须包含字符。
    #[serde(rename = "RequireSymbols")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_symbols: Option<bool>,
    /// 密码有效期，单位为天（重置密码将重置密码过期时间）。0表示不启用密码过期策略，默认不启用。
    #[serde(rename = "MaxPasswordAge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_password_age: Option<i32>,
    /// 最小密码长度。
    #[serde(rename = "MinimumPasswordLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_password_length: Option<i32>,
    /// 必须包含大写字母。
    #[serde(rename = "RequireUppercaseCharacters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_uppercase_characters: Option<bool>,
    /// 一小时内使用错误密码尝试登录最大次数（重置密码可清除尝试登录计数）。
    #[serde(rename = "MaxLoginAttemps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_login_attemps: Option<i32>,
}

impl GetPasswordPolicyResponsePasswordPolicy {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.require_numbers {
            params.push(("RequireNumbers".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.require_lowercase_characters {
            params.push(("RequireLowercaseCharacters".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.hard_expiry {
            params.push(("HardExpiry".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.password_reuse_prevention {
            params.push(("PasswordReusePrevention".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.require_symbols {
            params.push(("RequireSymbols".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_password_age {
            params.push(("MaxPasswordAge".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.minimum_password_length {
            params.push(("MinimumPasswordLength".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.require_uppercase_characters {
            params.push(("RequireUppercaseCharacters".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_login_attemps {
            params.push(("MaxLoginAttemps".to_string(), v.to_string()));
        }
        params
    }
}

/// 访问密钥首选项。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetSecurityPreferenceResponseSecurityPreferenceAccessKeyPreference {
    /// 是否允许RAM用户自主管理访问密钥。
    #[serde(rename = "AllowUserToManageAccessKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_user_to_manage_access_keys: Option<bool>,
}

impl SetSecurityPreferenceResponseSecurityPreferenceAccessKeyPreference {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.allow_user_to_manage_access_keys {
            params.push(("AllowUserToManageAccessKeys".to_string(), v.to_string()));
        }
        params
    }
}

/// 多因素认证首选项。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetSecurityPreferenceResponseSecurityPreferenceMFAPreference {
    /// 是否允许RAM用户自主管理多因素认证设备。
    #[serde(rename = "AllowUserToManageMFADevices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_user_to_manage_mfa_devices: Option<bool>,
}

impl SetSecurityPreferenceResponseSecurityPreferenceMFAPreference {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.allow_user_to_manage_mfa_devices {
            params.push(("AllowUserToManageMFADevices".to_string(), v.to_string()));
        }
        params
    }
}

/// 登录首选项。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetSecurityPreferenceResponseSecurityPreferenceLoginProfilePreference {
    /// 是否允许RAM用户在登录时保存多因素认证设备安全码。
    #[serde(rename = "EnableSaveMFATicket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_save_mfa_ticket: Option<bool>,
    /// RAM用户登录有效期。
    #[serde(rename = "LoginSessionDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_session_duration: Option<i32>,
    /// 登录掩码。
    #[serde(rename = "LoginNetworkMasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_network_masks: Option<String>,
    /// 是否允许RAM用户自主管理密码。
    #[serde(rename = "AllowUserToChangePassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_user_to_change_password: Option<bool>,
}

impl SetSecurityPreferenceResponseSecurityPreferenceLoginProfilePreference {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.enable_save_mfa_ticket {
            params.push(("EnableSaveMFATicket".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.login_session_duration {
            params.push(("LoginSessionDuration".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.login_network_masks {
            params.push(("LoginNetworkMasks".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.allow_user_to_change_password {
            params.push(("AllowUserToChangePassword".to_string(), v.to_string()));
        }
        params
    }
}

/// 公钥首选项。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetSecurityPreferenceResponseSecurityPreferencePublicKeyPreference {
    /// 是否允许RAM用户自主管理公钥。
    #[serde(rename = "AllowUserToManagePublicKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_user_to_manage_public_keys: Option<bool>,
}

impl SetSecurityPreferenceResponseSecurityPreferencePublicKeyPreference {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.allow_user_to_manage_public_keys {
            params.push(("AllowUserToManagePublicKeys".to_string(), v.to_string()));
        }
        params
    }
}

/// 安全首选项。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetSecurityPreferenceResponseSecurityPreference {
    /// 访问密钥首选项。
    #[serde(rename = "AccessKeyPreference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_preference: Option<SetSecurityPreferenceResponseSecurityPreferenceAccessKeyPreference>,
    /// 多因素认证首选项。
    #[serde(rename = "MFAPreference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_preference: Option<SetSecurityPreferenceResponseSecurityPreferenceMFAPreference>,
    /// 登录首选项。
    #[serde(rename = "LoginProfilePreference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_profile_preference: Option<SetSecurityPreferenceResponseSecurityPreferenceLoginProfilePreference>,
    /// 公钥首选项。
    #[serde(rename = "PublicKeyPreference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key_preference: Option<SetSecurityPreferenceResponseSecurityPreferencePublicKeyPreference>,
}

impl SetSecurityPreferenceResponseSecurityPreference {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.access_key_preference {
            for (k, v2) in v.to_query_params() {
                params.push((format!("AccessKeyPreference.{}", k), v2));
            }
        }
        if let Some(ref v) = self.mfa_preference {
            for (k, v2) in v.to_query_params() {
                params.push((format!("MFAPreference.{}", k), v2));
            }
        }
        if let Some(ref v) = self.login_profile_preference {
            for (k, v2) in v.to_query_params() {
                params.push((format!("LoginProfilePreference.{}", k), v2));
            }
        }
        if let Some(ref v) = self.public_key_preference {
            for (k, v2) in v.to_query_params() {
                params.push((format!("PublicKeyPreference.{}", k), v2));
            }
        }
        params
    }
}

/// 访问密钥首选项。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetSecurityPreferenceResponseSecurityPreferenceAccessKeyPreference {
    /// 是否允许用户自主管理访问密钥。取值：
    #[serde(rename = "AllowUserToManageAccessKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_user_to_manage_access_keys: Option<bool>,
}

impl GetSecurityPreferenceResponseSecurityPreferenceAccessKeyPreference {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.allow_user_to_manage_access_keys {
            params.push(("AllowUserToManageAccessKeys".to_string(), v.to_string()));
        }
        params
    }
}

/// 多因素认证首选项。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetSecurityPreferenceResponseSecurityPreferenceMFAPreference {
    /// 是否允许RAM用户自主管理多因素认证设备。取值：
    #[serde(rename = "AllowUserToManageMFADevices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_user_to_manage_mfa_devices: Option<bool>,
}

impl GetSecurityPreferenceResponseSecurityPreferenceMFAPreference {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.allow_user_to_manage_mfa_devices {
            params.push(("AllowUserToManageMFADevices".to_string(), v.to_string()));
        }
        params
    }
}

/// 登录首选项。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetSecurityPreferenceResponseSecurityPreferenceLoginProfilePreference {
    /// 是否允许RAM用户登录时保存多因素认证设备安全码，安全码有效期为7天。取值：
    #[serde(rename = "EnableSaveMFATicket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_save_mfa_ticket: Option<bool>,
    /// RAM用户登录有效期。单位：小时。
    #[serde(rename = "LoginSessionDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_session_duration: Option<i32>,
    /// 登录掩码。登录掩码决定哪些IP地址会受到登录控制台的影响，包括密码登录和SSO登录，但使用AccessKey发起的API访问并不受影响。
    #[serde(rename = "LoginNetworkMasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_network_masks: Option<String>,
    /// 是否允许RAM用户自主管理密码。取值：
    #[serde(rename = "AllowUserToChangePassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_user_to_change_password: Option<bool>,
}

impl GetSecurityPreferenceResponseSecurityPreferenceLoginProfilePreference {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.enable_save_mfa_ticket {
            params.push(("EnableSaveMFATicket".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.login_session_duration {
            params.push(("LoginSessionDuration".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.login_network_masks {
            params.push(("LoginNetworkMasks".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.allow_user_to_change_password {
            params.push(("AllowUserToChangePassword".to_string(), v.to_string()));
        }
        params
    }
}

/// 公钥首选项。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetSecurityPreferenceResponseSecurityPreferencePublicKeyPreference {
    /// 是否允许RAM用户自主管理公钥。取值：
    #[serde(rename = "AllowUserToManagePublicKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_user_to_manage_public_keys: Option<bool>,
}

impl GetSecurityPreferenceResponseSecurityPreferencePublicKeyPreference {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.allow_user_to_manage_public_keys {
            params.push(("AllowUserToManagePublicKeys".to_string(), v.to_string()));
        }
        params
    }
}

/// 安全首选项。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetSecurityPreferenceResponseSecurityPreference {
    /// 访问密钥首选项。
    #[serde(rename = "AccessKeyPreference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_preference: Option<GetSecurityPreferenceResponseSecurityPreferenceAccessKeyPreference>,
    /// 多因素认证首选项。
    #[serde(rename = "MFAPreference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_preference: Option<GetSecurityPreferenceResponseSecurityPreferenceMFAPreference>,
    /// 登录首选项。
    #[serde(rename = "LoginProfilePreference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_profile_preference: Option<GetSecurityPreferenceResponseSecurityPreferenceLoginProfilePreference>,
    /// 公钥首选项。
    #[serde(rename = "PublicKeyPreference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key_preference: Option<GetSecurityPreferenceResponseSecurityPreferencePublicKeyPreference>,
}

impl GetSecurityPreferenceResponseSecurityPreference {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.access_key_preference {
            for (k, v2) in v.to_query_params() {
                params.push((format!("AccessKeyPreference.{}", k), v2));
            }
        }
        if let Some(ref v) = self.mfa_preference {
            for (k, v2) in v.to_query_params() {
                params.push((format!("MFAPreference.{}", k), v2));
            }
        }
        if let Some(ref v) = self.login_profile_preference {
            for (k, v2) in v.to_query_params() {
                params.push((format!("LoginProfilePreference.{}", k), v2));
            }
        }
        if let Some(ref v) = self.public_key_preference {
            for (k, v2) in v.to_query_params() {
                params.push((format!("PublicKeyPreference.{}", k), v2));
            }
        }
        params
    }
}

/// 用户请求中用于鉴权的主体信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DecodeDiagnosticMessageResponseDecodedDiagnosticMessageAuthPrincipal {
    /// 用户请求中用于鉴权的身份类型。
    #[serde(rename = "AuthPrincipalType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_principal_type: Option<String>,
    /// 用户请求中用于鉴权的身份所属的阿里云账号UID信息。
    #[serde(rename = "AuthPrincipalOwnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_principal_owner_id: Option<String>,
    /// 用户请求中用于鉴权的身份标识。具体如下：
    #[serde(rename = "AuthPrincipalDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_principal_display_name: Option<String>,
}

impl DecodeDiagnosticMessageResponseDecodedDiagnosticMessageAuthPrincipal {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.auth_principal_type {
            params.push(("AuthPrincipalType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auth_principal_owner_id {
            params.push(("AuthPrincipalOwnerId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auth_principal_display_name {
            params.push(("AuthPrincipalDisplayName".to_string(), v.to_string()));
        }
        params
    }
}

/// 鉴权条件。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DecodeDiagnosticMessageResponseDecodedDiagnosticMessageAuthConditionsItem {
    /// 鉴权条件Key。
    #[serde(rename = "ConditionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_key: Option<String>,
    /// 鉴权条件Key对应的值列表。
    #[serde(rename = "ConditionValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_values: Option<Vec<String>>,
}

impl DecodeDiagnosticMessageResponseDecodedDiagnosticMessageAuthConditionsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.condition_key {
            params.push(("ConditionKey".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.condition_values {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("ConditionValues.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 鉴权命中的策略。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DecodeDiagnosticMessageResponseDecodedDiagnosticMessageMatchedPoliciesItem {
    /// 策略效果。
    #[serde(rename = "Effect")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    /// 策略名称信息。具体如下：
    #[serde(rename = "PolicyIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_identifier: Option<String>,
    /// 策略类型。
    #[serde(rename = "PolicyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
    /// 策略版本号。
    #[serde(rename = "PolicyVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_version: Option<String>,
    /// 策略授权的实体类型。
    #[serde(rename = "AttachedEntityType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_entity_type: Option<String>,
    /// 策略授权范围。
    #[serde(rename = "AttachedScope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_scope: Option<String>,
}

impl DecodeDiagnosticMessageResponseDecodedDiagnosticMessageMatchedPoliciesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.effect {
            params.push(("Effect".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_identifier {
            params.push(("PolicyIdentifier".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_type {
            params.push(("PolicyType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_version {
            params.push(("PolicyVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.attached_entity_type {
            params.push(("AttachedEntityType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.attached_scope {
            params.push(("AttachedScope".to_string(), v.to_string()));
        }
        params
    }
}

/// 解码的诊断信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DecodeDiagnosticMessageResponseDecodedDiagnosticMessage {
    /// 是否是显式拒绝。
    #[serde(rename = "ExplicitDeny")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_deny: Option<bool>,
    /// 产生无权限的策略类型。
    #[serde(rename = "NoPermissionPolicyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_permission_policy_type: Option<String>,
    /// 用户请求中用于鉴权的操作信息。
    #[serde(rename = "AuthAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_action: Option<String>,
    /// 用户请求中用于鉴权的资源信息。
    #[serde(rename = "AuthResource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_resource: Option<String>,
    /// 用户请求中用于鉴权的主体信息。
    #[serde(rename = "AuthPrincipal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_principal: Option<DecodeDiagnosticMessageResponseDecodedDiagnosticMessageAuthPrincipal>,
    /// 用户请求中用于鉴权的条件列表。
    #[serde(rename = "AuthConditions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_conditions: Option<Vec<DecodeDiagnosticMessageResponseDecodedDiagnosticMessageAuthConditionsItem>>,
    /// 鉴权命中的策略列表。
    #[serde(rename = "MatchedPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matched_policies: Option<Vec<DecodeDiagnosticMessageResponseDecodedDiagnosticMessageMatchedPoliciesItem>>,
}

impl DecodeDiagnosticMessageResponseDecodedDiagnosticMessage {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.explicit_deny {
            params.push(("ExplicitDeny".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.no_permission_policy_type {
            params.push(("NoPermissionPolicyType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auth_action {
            params.push(("AuthAction".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auth_resource {
            params.push(("AuthResource".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auth_principal {
            for (k, v2) in v.to_query_params() {
                params.push((format!("AuthPrincipal.{}", k), v2));
            }
        }
        if let Some(ref v) = self.auth_conditions {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("AuthConditions.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.matched_policies {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("MatchedPolicies.{}", i + 1);
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
pub struct TagResourcesRequestTagItem {
    /// 标签键。最多支持128个字符。
    #[serde(rename = "Key")]
    pub key: String,
    /// 标签值。最多支持256个字符。
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

/// 标签。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListTagResourcesRequestTagItem {
    /// 标签键。最多支持128个字符。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。最多支持256个字符。
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

/// 云资源绑定的标签。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListTagResourcesResponseTagResourcesItem {
    /// 资源名称。
    #[serde(rename = "ResourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    /// 资源类型。
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// 标签键。
    #[serde(rename = "TagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    /// 标签值。
    #[serde(rename = "TagValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
}

impl ListTagResourcesResponseTagResourcesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_name {
            params.push(("ResourceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_type {
            params.push(("ResourceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_key {
            params.push(("TagKey".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_value {
            params.push(("TagValue".to_string(), v.to_string()));
        }
        params
    }
}

/// CreateUser 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateUserRequest {
    /// RAM用户的名称。
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// RAM用户的显示名称。
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// RAM用户的手机号码。
    #[serde(rename = "MobilePhone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_phone: Option<String>,
    /// RAM用户的电子邮箱。
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 备注。
    #[serde(rename = "Comments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
}

impl CreateUserRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.user_name {
            params.push(("UserName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.display_name {
            params.push(("DisplayName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.mobile_phone {
            params.push(("MobilePhone".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.email {
            params.push(("Email".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.comments {
            params.push(("Comments".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果
#[derive(Debug, Clone, Deserialize)]
pub struct CreateUserResponse {
    /// RAM用户信息。
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<CreateUserResponseUser>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// GetUser 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetUserRequest {
    /// RAM用户的名称。
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

impl GetUserRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.user_name {
            params.push(("UserName".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果
#[derive(Debug, Clone, Deserialize)]
pub struct GetUserResponse {
    /// RAM用户信息。
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<GetUserResponseUser>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// UpdateUser 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateUserRequest {
    /// RAM用户名称。
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// RAM用户的新名称。
    #[serde(rename = "NewUserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_user_name: Option<String>,
    /// RAM用户的新显示名称。
    #[serde(rename = "NewDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_display_name: Option<String>,
    /// RAM用户的新手机号码。
    #[serde(rename = "NewMobilePhone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_mobile_phone: Option<String>,
    /// RAM用户的新电子邮箱。
    #[serde(rename = "NewEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_email: Option<String>,
    /// 新备注。
    #[serde(rename = "NewComments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_comments: Option<String>,
}

impl UpdateUserRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.user_name {
            params.push(("UserName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.new_user_name {
            params.push(("NewUserName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.new_display_name {
            params.push(("NewDisplayName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.new_mobile_phone {
            params.push(("NewMobilePhone".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.new_email {
            params.push(("NewEmail".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.new_comments {
            params.push(("NewComments".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateUserResponse {
    /// RAM用户信息。
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UpdateUserResponseUser>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteUser 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteUserRequest {
    /// 指定用户名。
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

impl DeleteUserRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.user_name {
            params.push(("UserName".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果。
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteUserResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListUsers 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListUsersRequest {
    /// 当请求的返回结果被截断时，可以使用`Marker`获取从当前截断位置之后的内容。
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// 返回结果的条数，当返回结果达到MaxItems限制被截断时，返回参数`IsTruncated`将等于`true`。
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

impl ListUsersRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.marker {
            params.push(("Marker".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_items {
            params.push(("MaxItems".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果。
#[derive(Debug, Clone, Deserialize)]
pub struct ListUsersResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 请求返回结果是否被截断。
    #[serde(rename = "IsTruncated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    /// 当`IsTruncated`为`true`时才有此字段，当返回`true`时，需要继续调用此接口，并且使用`Marker`获取截断后的内容 。
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "Users")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<ListUsersResponseUsers>,
}

/// CreateLoginProfile 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateLoginProfileRequest {
    /// 指定用户名。
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// 指定密码，密码必须符合密码强度要求。关于密码强度设置要求，请参见[GetPasswordPolicy](~~2337691~~)。
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// 指定用户在登录时是否需要修改密码。默认为`false`。
    #[serde(rename = "PasswordResetRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_reset_required: Option<bool>,
    /// 指定用户在下次登录时是否必须绑定多因素认证器。默认为`false`。
    #[serde(rename = "MFABindRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_bind_required: Option<bool>,
}

impl CreateLoginProfileRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.user_name {
            params.push(("UserName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.password {
            params.push(("Password".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.password_reset_required {
            params.push(("PasswordResetRequired".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.mfa_bind_required {
            params.push(("MFABindRequired".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果。
#[derive(Debug, Clone, Deserialize)]
pub struct CreateLoginProfileResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 登录配置信息。
    #[serde(rename = "LoginProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_profile: Option<CreateLoginProfileResponseLoginProfile>,
}

/// GetLoginProfile 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetLoginProfileRequest {
    /// 指定用户名。
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

impl GetLoginProfileRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.user_name {
            params.push(("UserName".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果。
#[derive(Debug, Clone, Deserialize)]
pub struct GetLoginProfileResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 登录配置信息。
    #[serde(rename = "LoginProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_profile: Option<GetLoginProfileResponseLoginProfile>,
}

/// UpdateLoginProfile 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateLoginProfileRequest {
    /// 指定用户名。
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// 指定密码，密码必须符合云账号的密码强度要求。
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// 指定用户在登录时是否需要修改密码。
    #[serde(rename = "PasswordResetRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_reset_required: Option<bool>,
    /// 指定用户在下次登录时是否必须绑定多因素认证设备。
    #[serde(rename = "MFABindRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_bind_required: Option<bool>,
}

impl UpdateLoginProfileRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.user_name {
            params.push(("UserName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.password {
            params.push(("Password".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.password_reset_required {
            params.push(("PasswordResetRequired".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.mfa_bind_required {
            params.push(("MFABindRequired".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果。
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateLoginProfileResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteLoginProfile 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteLoginProfileRequest {
    /// 指定用户名。
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

impl DeleteLoginProfileRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.user_name {
            params.push(("UserName".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果。
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteLoginProfileResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ChangePassword 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ChangePasswordRequest {
    /// RAM用户的控制台登录旧密码。
    #[serde(rename = "OldPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_password: Option<String>,
    /// RAM用户的控制台登录新密码。
    #[serde(rename = "NewPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_password: Option<String>,
}

impl ChangePasswordRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.old_password {
            params.push(("OldPassword".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.new_password {
            params.push(("NewPassword".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果。
#[derive(Debug, Clone, Deserialize)]
pub struct ChangePasswordResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreateAccessKey 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateAccessKeyRequest {
    /// 指定用户名，RAM用户调用此接口时，默认为自己创建访问密钥。
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

impl CreateAccessKeyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.user_name {
            params.push(("UserName".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果。
#[derive(Debug, Clone, Deserialize)]
pub struct CreateAccessKeyResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 访问密钥。
    #[serde(rename = "AccessKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key: Option<CreateAccessKeyResponseAccessKey>,
}

/// UpdateAccessKey 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateAccessKeyRequest {
    /// 指定用户名。
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// 指定要更新的`AccessKeyId`。
    #[serde(rename = "UserAccessKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_access_key_id: Option<String>,
    /// AccessKey的状态，取值为`Active`或`Inactive`。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl UpdateAccessKeyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.user_name {
            params.push(("UserName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_access_key_id {
            params.push(("UserAccessKeyId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果。
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateAccessKeyResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteAccessKey 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteAccessKeyRequest {
    /// 指定用户名。
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// 指定要删除的`AccessKeyId`。
    #[serde(rename = "UserAccessKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_access_key_id: Option<String>,
}

impl DeleteAccessKeyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.user_name {
            params.push(("UserName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_access_key_id {
            params.push(("UserAccessKeyId".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果。
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteAccessKeyResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListAccessKeys 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListAccessKeysRequest {
    /// 指定用户，RAM用户访问时不提供此参数则表示列出自己的访问密钥。
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

impl ListAccessKeysRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.user_name {
            params.push(("UserName".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果。
#[derive(Debug, Clone, Deserialize)]
pub struct ListAccessKeysResponse {
    #[serde(rename = "AccessKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_keys: Option<ListAccessKeysResponseAccessKeys>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// GetAccessKeyLastUsed 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAccessKeyLastUsedRequest {
    /// RAM用户名称。
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// 需要查询的访问密钥ID。
    #[serde(rename = "UserAccessKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_access_key_id: Option<String>,
}

impl GetAccessKeyLastUsedRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.user_name {
            params.push(("UserName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_access_key_id {
            params.push(("UserAccessKeyId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetAccessKeyLastUsedResponse {
    /// 访问密钥的最后使用信息。
    #[serde(rename = "AccessKeyLastUsed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_last_used: Option<GetAccessKeyLastUsedResponseAccessKeyLastUsed>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreateVirtualMFADevice 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateVirtualMFADeviceRequest {
    /// 多因素认证设备名称。
    #[serde(rename = "VirtualMFADeviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_mfa_device_name: Option<String>,
}

impl CreateVirtualMFADeviceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.virtual_mfa_device_name {
            params.push(("VirtualMFADeviceName".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回参数。
#[derive(Debug, Clone, Deserialize)]
pub struct CreateVirtualMFADeviceResponse {
    /// 多因素认证设备。
    #[serde(rename = "VirtualMFADevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_mfa_device: Option<CreateVirtualMFADeviceResponseVirtualMFADevice>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// GetUserMFAInfo 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetUserMFAInfoRequest {
    /// RAM用户名称。
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

impl GetUserMFAInfoRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.user_name {
            params.push(("UserName".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果。
#[derive(Debug, Clone, Deserialize)]
pub struct GetUserMFAInfoResponse {
    /// MFA设备信息。
    #[serde(rename = "MFADevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_device: Option<GetUserMFAInfoResponseMFADevice>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteVirtualMFADevice 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteVirtualMFADeviceRequest {
    /// 指定多因素认证设备的序列号。
    #[serde(rename = "SerialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
}

impl DeleteVirtualMFADeviceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.serial_number {
            params.push(("SerialNumber".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteVirtualMFADeviceResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListVirtualMFADevices 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListVirtualMFADevicesRequest {
}

impl ListVirtualMFADevicesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 返回结果。
#[derive(Debug, Clone, Deserialize)]
pub struct ListVirtualMFADevicesResponse {
    #[serde(rename = "VirtualMFADevices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_mfa_devices: Option<ListVirtualMFADevicesResponseVirtualMFADevices>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// BindMFADevice 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct BindMFADeviceRequest {
    /// 指定多因素认证设备的序列号。
    #[serde(rename = "SerialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// 指定用户名。
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// 验证第一组动态密码。
    #[serde(rename = "AuthenticationCode1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_code1: Option<String>,
    /// 验证第二组动态密码。
    #[serde(rename = "AuthenticationCode2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_code2: Option<String>,
}

impl BindMFADeviceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.serial_number {
            params.push(("SerialNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_name {
            params.push(("UserName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.authentication_code1 {
            params.push(("AuthenticationCode1".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.authentication_code2 {
            params.push(("AuthenticationCode2".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回参数。
#[derive(Debug, Clone, Deserialize)]
pub struct BindMFADeviceResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// UnbindMFADevice 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UnbindMFADeviceRequest {
    /// 指定用户名。
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

impl UnbindMFADeviceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.user_name {
            params.push(("UserName".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果。
#[derive(Debug, Clone, Deserialize)]
pub struct UnbindMFADeviceResponse {
    /// 多因素认证设备信息。
    #[serde(rename = "MFADevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_device: Option<UnbindMFADeviceResponseMFADevice>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreateGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateGroupRequest {
    /// 用户组名称。
    #[serde(rename = "GroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// 备注信息。
    #[serde(rename = "Comments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
}

impl CreateGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.group_name {
            params.push(("GroupName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.comments {
            params.push(("Comments".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果。
#[derive(Debug, Clone, Deserialize)]
pub struct CreateGroupResponse {
    /// 用户组信息。
    #[serde(rename = "Group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<CreateGroupResponseGroup>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// GetGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetGroupRequest {
    /// 用户组名称。
    #[serde(rename = "GroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}

impl GetGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.group_name {
            params.push(("GroupName".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果。
#[derive(Debug, Clone, Deserialize)]
pub struct GetGroupResponse {
    /// 用户组信息。
    #[serde(rename = "Group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<GetGroupResponseGroup>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// UpdateGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateGroupRequest {
    /// 用户组名称。
    #[serde(rename = "GroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// 新的用户组名称。
    #[serde(rename = "NewGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_group_name: Option<String>,
    /// 新的备注信息。
    #[serde(rename = "NewComments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_comments: Option<String>,
}

impl UpdateGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.group_name {
            params.push(("GroupName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.new_group_name {
            params.push(("NewGroupName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.new_comments {
            params.push(("NewComments".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果。
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateGroupResponse {
    /// 用户组信息。
    #[serde(rename = "Group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<UpdateGroupResponseGroup>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteGroupRequest {
    /// 用户组名称。
    #[serde(rename = "GroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}

impl DeleteGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.group_name {
            params.push(("GroupName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteGroupResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListGroups 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListGroupsRequest {
    /// 查询返回结果下一页的令牌。首次调用API不需要`Marker`。
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// 每页的最大数据条数。
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

impl ListGroupsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.marker {
            params.push(("Marker".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_items {
            params.push(("MaxItems".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果。
#[derive(Debug, Clone, Deserialize)]
pub struct ListGroupsResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Groups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<ListGroupsResponseGroups>,
    /// 请求返回结果是否被截断。取值：
    #[serde(rename = "IsTruncated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    /// 查询返回结果下一页的令牌。
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

/// ListGroupsForUser 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListGroupsForUserRequest {
    /// RAM用户名称。
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

impl ListGroupsForUserRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.user_name {
            params.push(("UserName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListGroupsForUserResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Groups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<ListGroupsForUserResponseGroups>,
}

/// ListUsersForGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListUsersForGroupRequest {
    /// 用户组名称。
    #[serde(rename = "GroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// 当请求的返回结果被截断时，可以使用`Marker`获取从当前截断位置之后的内容。
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// 指定返回结果的条数。当返回结果达到`MaxItems`限制被截断时，返回参数`IsTruncated`将等于`true`。
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

impl ListUsersForGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.group_name {
            params.push(("GroupName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.marker {
            params.push(("Marker".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_items {
            params.push(("MaxItems".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果。
#[derive(Debug, Clone, Deserialize)]
pub struct ListUsersForGroupResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 请求返回结果是否被截断。
    #[serde(rename = "IsTruncated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    /// 当`IsTruncated`为`true`时才有此字段。当返回`true`时，需要继续调用此接口，并且使用`Marker`获取截断后的内容。
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "Users")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<ListUsersForGroupResponseUsers>,
}

/// AddUserToGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct AddUserToGroupRequest {
    /// 用户名称。
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// 用户组名称。
    #[serde(rename = "GroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}

impl AddUserToGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.user_name {
            params.push(("UserName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.group_name {
            params.push(("GroupName".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果。
#[derive(Debug, Clone, Deserialize)]
pub struct AddUserToGroupResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// RemoveUserFromGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct RemoveUserFromGroupRequest {
    /// RAM用户名称。
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// 用户组名称。
    #[serde(rename = "GroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}

impl RemoveUserFromGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.user_name {
            params.push(("UserName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.group_name {
            params.push(("GroupName".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果。
#[derive(Debug, Clone, Deserialize)]
pub struct RemoveUserFromGroupResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreateRole 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateRoleRequest {
    /// RAM角色名称。
    #[serde(rename = "RoleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    /// RAM角色描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 信任策略。指定允许扮演该RAM角色的一个或多个主体，这个主体可以是阿里云账号、阿里云服务或身份提供商。
    #[serde(rename = "AssumeRolePolicyDocument")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assume_role_policy_document: Option<String>,
    /// RAM角色最大会话时间。
    #[serde(rename = "MaxSessionDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_session_duration: Option<i64>,
    /// 标签。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<CreateRoleRequestTagItem>>,
}

impl CreateRoleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.role_name {
            params.push(("RoleName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.assume_role_policy_document {
            params.push(("AssumeRolePolicyDocument".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_session_duration {
            params.push(("MaxSessionDuration".to_string(), v.to_string()));
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

/// 返回结果。
#[derive(Debug, Clone, Deserialize)]
pub struct CreateRoleResponse {
    /// RAM角色信息。
    #[serde(rename = "Role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<CreateRoleResponseRole>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteRole 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteRoleRequest {
    /// RAM角色名称。
    #[serde(rename = "RoleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
}

impl DeleteRoleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.role_name {
            params.push(("RoleName".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回参数。
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteRoleResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// UpdateRole 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateRoleRequest {
    /// RAM角色名称。
    #[serde(rename = "RoleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    /// RAM角色的信任策略。
    #[serde(rename = "NewAssumeRolePolicyDocument")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_assume_role_policy_document: Option<String>,
    /// RAM角色最大会话时间。
    #[serde(rename = "NewMaxSessionDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_max_session_duration: Option<i64>,
    /// RAM角色描述。
    #[serde(rename = "NewDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_description: Option<String>,
}

impl UpdateRoleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.role_name {
            params.push(("RoleName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.new_assume_role_policy_document {
            params.push(("NewAssumeRolePolicyDocument".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.new_max_session_duration {
            params.push(("NewMaxSessionDuration".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.new_description {
            params.push(("NewDescription".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果。
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateRoleResponse {
    /// RAM角色信息。
    #[serde(rename = "Role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<UpdateRoleResponseRole>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// GetRole 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetRoleRequest {
    /// 指定角色名。
    #[serde(rename = "RoleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
}

impl GetRoleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.role_name {
            params.push(("RoleName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetRoleResponse {
    /// 角色信息。
    #[serde(rename = "Role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<GetRoleResponseRole>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListRoles 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListRolesRequest {
    /// 当请求的返回结果被截断时，可以使用`Marker`获取从当前截断位置之后的内容。
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// 返回结果的条数。当返回结果达到`MaxItems`限制被截断时，返回参数`IsTruncated`将等于`true`。
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    /// 标签。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<ListRolesRequestTagItem>>,
}

impl ListRolesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.marker {
            params.push(("Marker".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_items {
            params.push(("MaxItems".to_string(), v.to_string()));
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
pub struct ListRolesResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 请求返回结果是否被截断。
    #[serde(rename = "IsTruncated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Roles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<ListRolesResponseRoles>,
    /// 当`IsTruncated`为`true`时才有此字段。当返回`true`时，需要继续调用此接口，并且使用`Marker`获取截断后的内容。
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

/// CreatePolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreatePolicyRequest {
    /// 权限策略名称。
    #[serde(rename = "PolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    /// 权限策略描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 权限策略内容。
    #[serde(rename = "PolicyDocument")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_document: Option<String>,
    /// 标签。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<CreatePolicyRequestTagItem>>,
}

impl CreatePolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.policy_name {
            params.push(("PolicyName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_document {
            params.push(("PolicyDocument".to_string(), v.to_string()));
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

/// 返回结果
#[derive(Debug, Clone, Deserialize)]
pub struct CreatePolicyResponse {
    /// 权限策略信息。
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<CreatePolicyResponsePolicy>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// GetPolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetPolicyRequest {
    /// 指定权限策略的类型，取值为`System`或`Custom`。
    #[serde(rename = "PolicyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
    /// 指定权限策略名称。
    #[serde(rename = "PolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}

impl GetPolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.policy_type {
            params.push(("PolicyType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_name {
            params.push(("PolicyName".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回参数。
#[derive(Debug, Clone, Deserialize)]
pub struct GetPolicyResponse {
    /// 权限策略的基本信息。
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<GetPolicyResponsePolicy>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 权限策略的默认版本。
    #[serde(rename = "DefaultPolicyVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_policy_version: Option<GetPolicyResponseDefaultPolicyVersion>,
}

/// UpdatePolicyDescription 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdatePolicyDescriptionRequest {
    /// 权限策略名称。
    #[serde(rename = "PolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    /// 权限策略描述。
    #[serde(rename = "NewDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_description: Option<String>,
}

impl UpdatePolicyDescriptionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.policy_name {
            params.push(("PolicyName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.new_description {
            params.push(("NewDescription".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果。
#[derive(Debug, Clone, Deserialize)]
pub struct UpdatePolicyDescriptionResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 权限策略信息。
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<UpdatePolicyDescriptionResponsePolicy>,
}

/// DeletePolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeletePolicyRequest {
    /// 指定权限策略名称。
    #[serde(rename = "PolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    /// 是否级联删除权限策略的所有版本。取值：
    #[serde(rename = "CascadingDelete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cascading_delete: Option<bool>,
}

impl DeletePolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.policy_name {
            params.push(("PolicyName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cascading_delete {
            params.push(("CascadingDelete".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果。
#[derive(Debug, Clone, Deserialize)]
pub struct DeletePolicyResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListPolicies 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListPoliciesRequest {
    /// 指定`Policy`的类型，取值为`System`或`Custom`，如果没有指定则列出所有权限策略。
    #[serde(rename = "PolicyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
    /// 当请求的返回结果被截断时，可以使用`Marker`获取从当前截断位置之后的内容。
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// 指定返回结果的条数，当返回结果达到`MaxItems`限制被截断时，返回参数`IsTruncated`将等于`true`。
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    /// 标签。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<ListPoliciesRequestTagItem>>,
}

impl ListPoliciesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.policy_type {
            params.push(("PolicyType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.marker {
            params.push(("Marker".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_items {
            params.push(("MaxItems".to_string(), v.to_string()));
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
pub struct ListPoliciesResponse {
    #[serde(rename = "Policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<ListPoliciesResponsePolicies>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 请求返回结果是否被截断。
    #[serde(rename = "IsTruncated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    /// 当`IsTruncated`为`true`时才有此字段，当返回`true`时，需要继续调用此接口，并且使用`Marker`获取截断后的内容。
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

/// CreatePolicyVersion 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreatePolicyVersionRequest {
    /// 权限策略名称。
    #[serde(rename = "PolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    /// 权限策略内容，最大长度6144字节。
    #[serde(rename = "PolicyDocument")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_document: Option<String>,
    /// 是否设置为默认权限策略，默认值为`false`。
    #[serde(rename = "SetAsDefault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_as_default: Option<bool>,
    /// 权限策略版本自动化轮转机制，可以删除历史权限策略版本。
    #[serde(rename = "RotateStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotate_strategy: Option<String>,
}

impl CreatePolicyVersionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.policy_name {
            params.push(("PolicyName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_document {
            params.push(("PolicyDocument".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.set_as_default {
            params.push(("SetAsDefault".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rotate_strategy {
            params.push(("RotateStrategy".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreatePolicyVersionResponse {
    /// 新建的权限策略版本的信息。
    #[serde(rename = "PolicyVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_version: Option<CreatePolicyVersionResponsePolicyVersion>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// GetPolicyVersion 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetPolicyVersionRequest {
    /// 指定权限策略的类型，取值`System`或`Custom`。
    #[serde(rename = "PolicyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
    /// 权限策略名称。
    #[serde(rename = "PolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    /// 指定目标版本的ID。
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

impl GetPolicyVersionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.policy_type {
            params.push(("PolicyType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_name {
            params.push(("PolicyName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version_id {
            params.push(("VersionId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetPolicyVersionResponse {
    /// 权限策略版本信息。
    #[serde(rename = "PolicyVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_version: Option<GetPolicyVersionResponsePolicyVersion>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeletePolicyVersion 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeletePolicyVersionRequest {
    /// 权限策略名称。
    #[serde(rename = "PolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    /// 指定目标版本的ID。
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

impl DeletePolicyVersionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.policy_name {
            params.push(("PolicyName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version_id {
            params.push(("VersionId".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果。
#[derive(Debug, Clone, Deserialize)]
pub struct DeletePolicyVersionResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListPolicyVersions 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListPolicyVersionsRequest {
    /// 指定权限策略的类型，取值为`System`或`Custom`。
    #[serde(rename = "PolicyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
    /// 权限策略名称。
    #[serde(rename = "PolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}

impl ListPolicyVersionsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.policy_type {
            params.push(("PolicyType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_name {
            params.push(("PolicyName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListPolicyVersionsResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "PolicyVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_versions: Option<ListPolicyVersionsResponsePolicyVersions>,
}

/// SetDefaultPolicyVersion 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetDefaultPolicyVersionRequest {
    /// 指定权限策略名称。
    #[serde(rename = "PolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    /// 新默认版本的ID。
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

impl SetDefaultPolicyVersionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.policy_name {
            params.push(("PolicyName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version_id {
            params.push(("VersionId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetDefaultPolicyVersionResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// AttachPolicyToUser 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct AttachPolicyToUserRequest {
    /// 指定权限策略的类型，取值为`System`或`Custom`。
    #[serde(rename = "PolicyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
    /// 指定权限策略名称。
    #[serde(rename = "PolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    /// 指定用户名。
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

impl AttachPolicyToUserRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.policy_type {
            params.push(("PolicyType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_name {
            params.push(("PolicyName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_name {
            params.push(("UserName".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果。
#[derive(Debug, Clone, Deserialize)]
pub struct AttachPolicyToUserResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DetachPolicyFromUser 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DetachPolicyFromUserRequest {
    /// 指定权限策略的类型，取值为`System`或`Custom`。
    #[serde(rename = "PolicyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
    /// 指定权限策略名称。
    #[serde(rename = "PolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    /// 指定用户名。
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

impl DetachPolicyFromUserRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.policy_type {
            params.push(("PolicyType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_name {
            params.push(("PolicyName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_name {
            params.push(("UserName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DetachPolicyFromUserResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// AttachPolicyToGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct AttachPolicyToGroupRequest {
    /// 指定权限策略的类型，取值为`System`或`Custom`。
    #[serde(rename = "PolicyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
    /// 指定权限策略名称。
    #[serde(rename = "PolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    /// 指定用户组名称。
    #[serde(rename = "GroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}

impl AttachPolicyToGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.policy_type {
            params.push(("PolicyType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_name {
            params.push(("PolicyName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.group_name {
            params.push(("GroupName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AttachPolicyToGroupResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DetachPolicyFromGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DetachPolicyFromGroupRequest {
    /// 指定权限策略的类型，取值为`System`或`Custom`。
    #[serde(rename = "PolicyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
    /// 指定权限策略名称。
    #[serde(rename = "PolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    /// 指定用户组名称。
    #[serde(rename = "GroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}

impl DetachPolicyFromGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.policy_type {
            params.push(("PolicyType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_name {
            params.push(("PolicyName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.group_name {
            params.push(("GroupName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DetachPolicyFromGroupResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// AttachPolicyToRole 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct AttachPolicyToRoleRequest {
    /// 指定权限策略的类型，取值为`System`或`Custom`。
    #[serde(rename = "PolicyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
    /// 指定权限策略名称。
    #[serde(rename = "PolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    /// 指定角色名。
    #[serde(rename = "RoleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
}

impl AttachPolicyToRoleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.policy_type {
            params.push(("PolicyType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_name {
            params.push(("PolicyName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.role_name {
            params.push(("RoleName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AttachPolicyToRoleResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DetachPolicyFromRole 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DetachPolicyFromRoleRequest {
    /// 指定权限策略的类型，取值为`System`或`Custom`。
    #[serde(rename = "PolicyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
    /// 权限策略名称。
    #[serde(rename = "PolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    /// 指定角色名。
    #[serde(rename = "RoleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
}

impl DetachPolicyFromRoleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.policy_type {
            params.push(("PolicyType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_name {
            params.push(("PolicyName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.role_name {
            params.push(("RoleName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DetachPolicyFromRoleResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListPoliciesForUser 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListPoliciesForUserRequest {
    /// RAM用户名。
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

impl ListPoliciesForUserRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.user_name {
            params.push(("UserName".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果。
#[derive(Debug, Clone, Deserialize)]
pub struct ListPoliciesForUserResponse {
    #[serde(rename = "Policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<ListPoliciesForUserResponsePolicies>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListPoliciesForGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListPoliciesForGroupRequest {
    /// 指定用户组名称。
    #[serde(rename = "GroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}

impl ListPoliciesForGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.group_name {
            params.push(("GroupName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListPoliciesForGroupResponse {
    #[serde(rename = "Policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<ListPoliciesForGroupResponsePolicies>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListPoliciesForRole 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListPoliciesForRoleRequest {
    /// 指定角色名。
    #[serde(rename = "RoleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
}

impl ListPoliciesForRoleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.role_name {
            params.push(("RoleName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListPoliciesForRoleResponse {
    #[serde(rename = "Policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<ListPoliciesForRoleResponsePolicies>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListEntitiesForPolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListEntitiesForPolicyRequest {
    /// 指定权限策略的类型，取值为`System`或`Custom`。
    #[serde(rename = "PolicyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
    /// 权限策略名称。
    #[serde(rename = "PolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}

impl ListEntitiesForPolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.policy_type {
            params.push(("PolicyType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_name {
            params.push(("PolicyName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListEntitiesForPolicyResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Groups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<ListEntitiesForPolicyResponseGroups>,
    #[serde(rename = "Roles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<ListEntitiesForPolicyResponseRoles>,
    #[serde(rename = "Users")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<ListEntitiesForPolicyResponseUsers>,
}

/// SetAccountAlias 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetAccountAliasRequest {
    /// 指定云账号的别名。
    #[serde(rename = "AccountAlias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_alias: Option<String>,
}

impl SetAccountAliasRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.account_alias {
            params.push(("AccountAlias".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回参数。
#[derive(Debug, Clone, Deserialize)]
pub struct SetAccountAliasResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// GetAccountAlias 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAccountAliasRequest {
}

impl GetAccountAliasRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 返回参数。
#[derive(Debug, Clone, Deserialize)]
pub struct GetAccountAliasResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 账号别名。
    #[serde(rename = "AccountAlias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_alias: Option<String>,
}

/// ClearAccountAlias 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ClearAccountAliasRequest {
}

impl ClearAccountAliasRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ClearAccountAliasResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// SetPasswordPolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetPasswordPolicyRequest {
    /// 最小密码长度。
    #[serde(rename = "MinimumPasswordLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_password_length: Option<i32>,
    /// 必须含有小写字母。
    #[serde(rename = "RequireLowercaseCharacters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_lowercase_characters: Option<bool>,
    /// 必须含有大写字母。
    #[serde(rename = "RequireUppercaseCharacters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_uppercase_characters: Option<bool>,
    /// 必须含有数字。
    #[serde(rename = "RequireNumbers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_numbers: Option<bool>,
    /// 必须含有特殊字符。
    #[serde(rename = "RequireSymbols")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_symbols: Option<bool>,
    /// 密码是否过期。
    #[serde(rename = "HardExpiry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hard_expiry: Option<bool>,
    /// 密码有效期，单位是天（重置密码将重置密码过期时间）。0表示不启用密码过期策略，默认不启用。
    #[serde(rename = "MaxPasswordAge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_password_age: Option<i32>,
    /// 禁止使用前N次密码。0表示不启用历史密码检查策略，默认不启用。
    #[serde(rename = "PasswordReusePrevention")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_reuse_prevention: Option<i32>,
    /// 一小时内使用错误密码尝试登录最大次数（重置密码可清除尝试登录计数）。
    #[serde(rename = "MaxLoginAttemps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_login_attemps: Option<i32>,
}

impl SetPasswordPolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.minimum_password_length {
            params.push(("MinimumPasswordLength".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.require_lowercase_characters {
            params.push(("RequireLowercaseCharacters".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.require_uppercase_characters {
            params.push(("RequireUppercaseCharacters".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.require_numbers {
            params.push(("RequireNumbers".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.require_symbols {
            params.push(("RequireSymbols".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.hard_expiry {
            params.push(("HardExpiry".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_password_age {
            params.push(("MaxPasswordAge".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.password_reuse_prevention {
            params.push(("PasswordReusePrevention".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_login_attemps {
            params.push(("MaxLoginAttemps".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回参数。
#[derive(Debug, Clone, Deserialize)]
pub struct SetPasswordPolicyResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 密码策略。
    #[serde(rename = "PasswordPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_policy: Option<SetPasswordPolicyResponsePasswordPolicy>,
}

/// GetPasswordPolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetPasswordPolicyRequest {
}

impl GetPasswordPolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetPasswordPolicyResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 密码策略。
    #[serde(rename = "PasswordPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_policy: Option<GetPasswordPolicyResponsePasswordPolicy>,
}

/// SetSecurityPreference 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetSecurityPreferenceRequest {
    /// 是否允许RAM用户在登录时保存多因素设备认证状态，有效期为7天。取值：
    #[serde(rename = "EnableSaveMFATicket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_save_mfa_ticket: Option<bool>,
    /// 是否允许RAM用户自主管理密码。取值：
    #[serde(rename = "AllowUserToChangePassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_user_to_change_password: Option<bool>,
    /// 是否允许RAM用户自主管理访问密钥。取值：
    #[serde(rename = "AllowUserToManageAccessKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_user_to_manage_access_keys: Option<bool>,
    /// 是否允许RAM用户自主管理公钥。取值：
    #[serde(rename = "AllowUserToManagePublicKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_user_to_manage_public_keys: Option<bool>,
    /// 是否允许RAM用户自主管理多因素认证设备。取值：
    #[serde(rename = "AllowUserToManageMFADevices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_user_to_manage_mfa_devices: Option<bool>,
    /// RAM用户登录有效期。
    #[serde(rename = "LoginSessionDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_session_duration: Option<i32>,
    /// 登录掩码。登录掩码决定哪些IP地址会受到登录控制台的影响，包括密码登录和单点登录（SSO），但使用访问密钥发起的API调用并不受影响。
    #[serde(rename = "LoginNetworkMasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_network_masks: Option<String>,
}

impl SetSecurityPreferenceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.enable_save_mfa_ticket {
            params.push(("EnableSaveMFATicket".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.allow_user_to_change_password {
            params.push(("AllowUserToChangePassword".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.allow_user_to_manage_access_keys {
            params.push(("AllowUserToManageAccessKeys".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.allow_user_to_manage_public_keys {
            params.push(("AllowUserToManagePublicKeys".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.allow_user_to_manage_mfa_devices {
            params.push(("AllowUserToManageMFADevices".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.login_session_duration {
            params.push(("LoginSessionDuration".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.login_network_masks {
            params.push(("LoginNetworkMasks".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果。
#[derive(Debug, Clone, Deserialize)]
pub struct SetSecurityPreferenceResponse {
    /// 安全首选项。
    #[serde(rename = "SecurityPreference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_preference: Option<SetSecurityPreferenceResponseSecurityPreference>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// GetSecurityPreference 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetSecurityPreferenceRequest {
}

impl GetSecurityPreferenceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetSecurityPreferenceResponse {
    /// 安全首选项。
    #[serde(rename = "SecurityPreference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_preference: Option<GetSecurityPreferenceResponseSecurityPreference>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DecodeDiagnosticMessage 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DecodeDiagnosticMessageRequest {
    /// API请求因无权限被拒绝访问时，请求响应体中返回的编码的诊断信息。
    #[serde(rename = "EncodedDiagnosticMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoded_diagnostic_message: Option<String>,
}

impl DecodeDiagnosticMessageRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.encoded_diagnostic_message {
            params.push(("EncodedDiagnosticMessage".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果。
#[derive(Debug, Clone, Deserialize)]
pub struct DecodeDiagnosticMessageResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 解码的诊断信息。
    #[serde(rename = "DecodedDiagnosticMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decoded_diagnostic_message: Option<DecodeDiagnosticMessageResponseDecodedDiagnosticMessage>,
}

/// TagResources 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct TagResourcesRequest {
    /// 资源类型。
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// 资源名称列表，一次最多支持输入50个资源名称。
    #[serde(rename = "ResourceNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_names: Option<Vec<String>>,
    /// 标签列表。一次最多支持输入20个标签。
    #[serde(rename = "Tag")]
    pub tag: Vec<TagResourcesRequestTagItem>,
}

impl TagResourcesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("ResourceType".to_string(), self.resource_type.to_string()));
        if let Some(ref v) = self.resource_names {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("ResourceNames.{}", i + 1), item.to_string()));
            }
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

/// 返回结果。
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
    /// 资源名称列表，一次最多支持输入50个资源名称。
    #[serde(rename = "ResourceNames")]
    pub resource_names: Vec<String>,
    /// 资源的标签键列表。一次最多支持输入20个标签键。
    #[serde(rename = "TagKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_keys: Option<Vec<String>>,
    /// 是否解绑资源上的全部标签。
    #[serde(rename = "All")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
}

impl UntagResourcesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("ResourceType".to_string(), self.resource_type.to_string()));
        for (i, item) in self.resource_names.iter().enumerate() {
            params.push((format!("ResourceNames.{}", i + 1), item.to_string()));
        }
        if let Some(ref v) = self.tag_keys {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("TagKeys.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.all {
            params.push(("All".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果。
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
    /// 资源类型。
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// 资源名称列表，一次最多支持输入50个资源名称。
    #[serde(rename = "ResourceNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_names: Option<Vec<String>>,
    /// 标签列表。一次最多支持输入20个标签。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<ListTagResourcesRequestTagItem>>,
    /// 分页查询时设置的每页行数。取值范围：1~100。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 下一页查询开始的Token。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl ListTagResourcesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("ResourceType".to_string(), self.resource_type.to_string()));
        if let Some(ref v) = self.resource_names {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("ResourceNames.{}", i + 1), item.to_string()));
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
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.next_token {
            params.push(("NextToken".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果。
#[derive(Debug, Clone, Deserialize)]
pub struct ListTagResourcesResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 云资源绑定的标签列表。
    #[serde(rename = "TagResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_resources: Option<Vec<ListTagResourcesResponseTagResourcesItem>>,
    /// 下一页查询开始的Token。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}
