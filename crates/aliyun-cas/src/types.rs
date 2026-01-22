//! 类型定义 - 自动生成，请勿手动修改

#![allow(unused_mut)]

use serde::{Deserialize, Serialize};

/// 标签列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateClientCertificateRequestTagsItem {
    /// 标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreateClientCertificateRequestTagsItem {
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
pub struct CreateClientCertificateWithCsrRequestTagsItem {
    /// 标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签的值（value）。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreateClientCertificateWithCsrRequestTagsItem {
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

/// 自定义证书的Subject属性。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateCustomCertificateRequestApiPassthroughSubjectCustomAttributesItem {
    /// 自定义属性键值，需符合行业标准。如：
    #[serde(rename = "ObjectIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_identifier: Option<String>,
    /// 自定义属性属性值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreateCustomCertificateRequestApiPassthroughSubjectCustomAttributesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.object_identifier {
            params.push(("ObjectIdentifier".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        params
    }
}

/// 证书主体名称。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateCustomCertificateRequestApiPassthroughSubject {
    /// 国家代码。使用ISO 3166-1的二位国家代码。参考[ISO](https://www.iso.org/obp/ui/#search/code/)。
    #[serde(rename = "Country")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// <props="china">CA证书关联的组织机构所在省份、直辖市或自治区的名称。</props>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// 组织机构所在城市的名称。支持使用中文、英文字符等。
    #[serde(rename = "Locality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    /// 组织机构名称。
    #[serde(rename = "Organization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// 组织机构下部门或分支的名称。
    #[serde(rename = "OrganizationUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_unit: Option<String>,
    /// 证书使用者的通用名称。
    #[serde(rename = "CommonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_name: Option<String>,
    /// 自定义证书的Subject属性。
    #[serde(rename = "CustomAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_attributes: Option<Vec<CreateCustomCertificateRequestApiPassthroughSubjectCustomAttributesItem>>,
}

impl CreateCustomCertificateRequestApiPassthroughSubject {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.country {
            params.push(("Country".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.state {
            params.push(("State".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.locality {
            params.push(("Locality".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.organization {
            params.push(("Organization".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.organization_unit {
            params.push(("OrganizationUnit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.common_name {
            params.push(("CommonName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.custom_attributes {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("CustomAttributes.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 密钥用法。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateCustomCertificateRequestApiPassthroughExtensionsKeyUsage {
    /// 数字签名。允许使用证书私钥进行数字签名，允许使用证书公钥验证数字签名。
    #[serde(rename = "DigitalSignature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digital_signature: Option<bool>,
    /// 内容承诺。原名称NonRepudiation。允许证书密钥用于内容承诺。
    #[serde(rename = "ContentCommitment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_commitment: Option<bool>,
    /// 抗抵赖。X.509标准中已更名为ContentCommitment。
    #[serde(rename = "NonRepudiation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_repudiation: Option<bool>,
    /// 密钥加密。允许证书密钥加密保护其他密钥。
    #[serde(rename = "KeyEncipherment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_encipherment: Option<bool>,
    /// 数据加密。
    #[serde(rename = "DataEncipherment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_encipherment: Option<bool>,
    /// 密钥协商。
    #[serde(rename = "KeyAgreement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_agreement: Option<bool>,
    /// 在KeyAgreement为true时，用于标记该证书密钥只能用于加密。
    #[serde(rename = "EncipherOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encipher_only: Option<bool>,
    /// 在KeyAgreement为true时，用于标记该证书密钥只能用于解密。
    #[serde(rename = "DecipherOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decipher_only: Option<bool>,
}

impl CreateCustomCertificateRequestApiPassthroughExtensionsKeyUsage {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.digital_signature {
            params.push(("DigitalSignature".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.content_commitment {
            params.push(("ContentCommitment".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.non_repudiation {
            params.push(("NonRepudiation".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key_encipherment {
            params.push(("KeyEncipherment".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.data_encipherment {
            params.push(("DataEncipherment".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key_agreement {
            params.push(("KeyAgreement".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.encipher_only {
            params.push(("EncipherOnly".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.decipher_only {
            params.push(("DecipherOnly".to_string(), v.to_string()));
        }
        params
    }
}

/// 证书主体别名。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateCustomCertificateRequestApiPassthroughExtensionsSubjectAlternativeNamesItem {
    /// 允许使用以下值：
    #[serde(rename = "Type")]
    pub r#type: String,
    /// 符合Type定义的值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreateCustomCertificateRequestApiPassthroughExtensionsSubjectAlternativeNamesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Type".to_string(), self.r#type.to_string()));
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        params
    }
}

/// 证书扩展项。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateCustomCertificateRequestApiPassthroughExtensions {
    /// 密钥用法。
    #[serde(rename = "KeyUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_usage: Option<CreateCustomCertificateRequestApiPassthroughExtensionsKeyUsage>,
    /// 扩展密钥用法。
    #[serde(rename = "ExtendedKeyUsages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_key_usages: Option<Vec<String>>,
    /// 证书主体别名。
    #[serde(rename = "SubjectAlternativeNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_alternative_names: Option<Vec<CreateCustomCertificateRequestApiPassthroughExtensionsSubjectAlternativeNamesItem>>,
    /// 如果是必要参数，则criticals列表中包含参数名。
    #[serde(rename = "Criticals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criticals: Option<Vec<String>>,
}

impl CreateCustomCertificateRequestApiPassthroughExtensions {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key_usage {
            for (k, v2) in v.to_query_params() {
                params.push((format!("KeyUsage.{}", k), v2));
            }
        }
        if let Some(ref v) = self.extended_key_usages {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("ExtendedKeyUsages.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.subject_alternative_names {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("SubjectAlternativeNames.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.criticals {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Criticals.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 透传参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateCustomCertificateRequestApiPassthrough {
    /// 证书主体名称。
    #[serde(rename = "Subject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<CreateCustomCertificateRequestApiPassthroughSubject>,
    /// 证书扩展项。
    #[serde(rename = "Extensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<CreateCustomCertificateRequestApiPassthroughExtensions>,
    /// 自定义证书的序列号（必须是长整型）。
    #[serde(rename = "SerialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
}

impl CreateCustomCertificateRequestApiPassthrough {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.subject {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Subject.{}", k), v2));
            }
        }
        if let Some(ref v) = self.extensions {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Extensions.{}", k), v2));
            }
        }
        if let Some(ref v) = self.serial_number {
            params.push(("SerialNumber".to_string(), v.to_string()));
        }
        params
    }
}

/// 标签列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateCustomCertificateRequestTagsItem {
    /// 标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreateCustomCertificateRequestTagsItem {
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

/// CA证书主体信息。该值存在时会覆盖CSR中的SubjectDN。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateExternalCACertificateRequestApiPassthroughSubject {
    /// 所属国家。使用ISO 3166-1的二位国家代码。
    #[serde(rename = "Country")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// 所属省/自治区/直辖市
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// 所属城市/区域
    #[serde(rename = "Locality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    /// 所属组织/公司
    #[serde(rename = "Organization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// 所属组织内部的子单位（部门、团队、项目组或分支机构）
    #[serde(rename = "OrganizationUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_unit: Option<String>,
    /// 当前CA证书名称
    #[serde(rename = "CommonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_name: Option<String>,
}

impl CreateExternalCACertificateRequestApiPassthroughSubject {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.country {
            params.push(("Country".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.state {
            params.push(("State".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.locality {
            params.push(("Locality".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.organization {
            params.push(("Organization".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.organization_unit {
            params.push(("OrganizationUnit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.common_name {
            params.push(("CommonName".to_string(), v.to_string()));
        }
        params
    }
}

/// CA证书扩展。该值存在时会覆盖CSR中的扩展项值，或添加到CA证书扩展项中。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateExternalCACertificateRequestApiPassthroughExtensions {
    /// 证书路径长度限制。EndEntity CA 该值必须传0，即当前CA证书用于颁发End Entity证书。
    #[serde(rename = "PathLenConstraint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_len_constraint: Option<i32>,
    /// 扩展密钥用法。
    #[serde(rename = "ExtendedKeyUsages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_key_usages: Option<Vec<String>>,
}

impl CreateExternalCACertificateRequestApiPassthroughExtensions {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.path_len_constraint {
            params.push(("PathLenConstraint".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.extended_key_usages {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("ExtendedKeyUsages.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 通过API参数覆盖CSR内容或添加到CA证书中。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateExternalCACertificateRequestApiPassthrough {
    /// CA证书主体信息。该值存在时会覆盖CSR中的SubjectDN。
    #[serde(rename = "Subject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<CreateExternalCACertificateRequestApiPassthroughSubject>,
    /// CA证书扩展。该值存在时会覆盖CSR中的扩展项值，或添加到CA证书扩展项中。
    #[serde(rename = "Extensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<CreateExternalCACertificateRequestApiPassthroughExtensions>,
}

impl CreateExternalCACertificateRequestApiPassthrough {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.subject {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Subject.{}", k), v2));
            }
        }
        if let Some(ref v) = self.extensions {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Extensions.{}", k), v2));
            }
        }
        params
    }
}

/// 标签列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateExternalCACertificateRequestTagsItem {
    /// 标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreateExternalCACertificateRequestTagsItem {
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
pub struct CreateRootCACertificateRequestTagsItem {
    /// 标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreateRootCACertificateRequestTagsItem {
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
pub struct CreateServerCertificateRequestTagsItem {
    /// 标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreateServerCertificateRequestTagsItem {
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
pub struct CreateServerCertificateWithCsrRequestTagsItem {
    /// 标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签的值（value）。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreateServerCertificateWithCsrRequestTagsItem {
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
pub struct CreateSubCACertificateRequestTagsItem {
    /// 标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreateSubCACertificateRequestTagsItem {
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

/// 标签值。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCACertificateResponseCertificateTagsItem {
    /// 标签 Key
    #[serde(rename = "TagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    /// 标签值。
    #[serde(rename = "TagValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
}

impl DescribeCACertificateResponseCertificateTagsItem {
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

/// CA证书的详细信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCACertificateResponseCertificate {
    /// CA证书的类型。取值：
    #[serde(rename = "CertificateType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_type: Option<String>,
    /// CA证书的内容。
    #[serde(rename = "X509Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x509_certificate: Option<String>,
    /// CA证书的唯一识别码。
    #[serde(rename = "Identifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// CA证书的序列号。
    #[serde(rename = "SerialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// CA证书的使用者属性，包含以下信息：
    #[serde(rename = "SubjectDN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_dn: Option<String>,
    /// CA证书关联的组织机构的通用名称或简称。
    #[serde(rename = "CommonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_name: Option<String>,
    /// CA证书关联的组织机构下部门的名称。
    #[serde(rename = "OrganizationUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_unit: Option<String>,
    /// CA证书关联的组织机构的名称。
    #[serde(rename = "Organization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// CA证书关联的组织机构所在城市的名称。
    #[serde(rename = "Locality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    /// <props="china">CA证书关联的组织机构所在省份、直辖市或自治区的名称。</props>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// CA证书关联的组织机构所在国家的代码。
    #[serde(rename = "CountryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// 该参数已废弃。
    #[serde(rename = "Sans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sans: Option<String>,
    /// CA证书的状态。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// CA证书的加密算法类型。取值：
    #[serde(rename = "Algorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    /// CA证书的密钥长度。
    #[serde(rename = "KeySize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_size: Option<i32>,
    /// CA证书的签名算法。
    #[serde(rename = "SignAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign_algorithm: Option<String>,
    /// CA证书的签发日期。使用时间戳表示，单位：毫秒。
    #[serde(rename = "BeforeDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_date: Option<i64>,
    /// CA证书的到期日期。使用时间戳表示，单位：毫秒。
    #[serde(rename = "AfterDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_date: Option<i64>,
    /// 签发该CA证书的根CA证书的唯一识别码。
    #[serde(rename = "ParentIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_identifier: Option<String>,
    /// CA证书的SHA256数字指纹。
    #[serde(rename = "Sha2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha2: Option<String>,
    /// CA证书的MD5数字指纹。
    #[serde(rename = "Md5")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md5: Option<String>,
    /// CRL状态（启用状态）。
    #[serde(rename = "CrlStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crl_status: Option<String>,
    /// CRL地址。
    #[serde(rename = "CrlUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crl_url: Option<String>,
    /// 已购买证书配额总数。
    #[serde(rename = "CertTotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_total_count: Option<i64>,
    /// 剩余可分配证书配额数量。
    #[serde(rename = "CertRemainingCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_remaining_count: Option<i64>,
    /// 私有CA实例已经签发证书的数量。
    #[serde(rename = "CertIssuedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_issued_count: Option<i64>,
    /// 完整证书链。
    #[serde(rename = "CaCertChain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_cert_chain: Option<String>,
    /// CRL有效期 1-365天。
    #[serde(rename = "CrlDay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crl_day: Option<i32>,
    /// 签发CA的机构。取值：
    #[serde(rename = "IssuerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer_type: Option<String>,
    /// CA证书的有效期，单位：年。
    #[serde(rename = "Years")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub years: Option<i32>,
    /// 携带长度的算法。
    #[serde(rename = "FullAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_algorithm: Option<String>,
    /// 证书所属资源组的ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 加密机集群标识。（通过加密机启用CA）
    #[serde(rename = "ClusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// 加密机密钥索引位置。（通过加密启用CA）
    #[serde(rename = "KeyIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_index: Option<i32>,
    /// 标签列表。
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<DescribeCACertificateResponseCertificateTagsItem>>,
}

impl DescribeCACertificateResponseCertificate {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.certificate_type {
            params.push(("CertificateType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.x509_certificate {
            params.push(("X509Certificate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.identifier {
            params.push(("Identifier".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.serial_number {
            params.push(("SerialNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.subject_dn {
            params.push(("SubjectDN".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.common_name {
            params.push(("CommonName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.organization_unit {
            params.push(("OrganizationUnit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.organization {
            params.push(("Organization".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.locality {
            params.push(("Locality".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.state {
            params.push(("State".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.country_code {
            params.push(("CountryCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sans {
            params.push(("Sans".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.algorithm {
            params.push(("Algorithm".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key_size {
            params.push(("KeySize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sign_algorithm {
            params.push(("SignAlgorithm".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.before_date {
            params.push(("BeforeDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.after_date {
            params.push(("AfterDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.parent_identifier {
            params.push(("ParentIdentifier".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sha2 {
            params.push(("Sha2".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.md5 {
            params.push(("Md5".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.crl_status {
            params.push(("CrlStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.crl_url {
            params.push(("CrlUrl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_total_count {
            params.push(("CertTotalCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_remaining_count {
            params.push(("CertRemainingCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_issued_count {
            params.push(("CertIssuedCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ca_cert_chain {
            params.push(("CaCertChain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.crl_day {
            params.push(("CrlDay".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.issuer_type {
            params.push(("IssuerType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.years {
            params.push(("Years".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.full_algorithm {
            params.push(("FullAlgorithm".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_id {
            params.push(("ClusterId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key_index {
            params.push(("KeyIndex".to_string(), v.to_string()));
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

/// 证书信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCACertificateListResponseCertificateListItem {
    /// CA证书的有效期，单位：年。
    #[serde(rename = "Years")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub years: Option<i32>,
    /// CA证书的状态。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// CA证书的序列号。
    #[serde(rename = "SerialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// CA证书的类型。取值：
    #[serde(rename = "CertificateType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_type: Option<String>,
    /// CA证书的加密算法类型。取值：
    #[serde(rename = "Algorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    /// <props="china">CA证书关联的组织机构所在省份、直辖市或自治区的名称。</props>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// CA证书关联的组织机构的名称。
    #[serde(rename = "Organization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// 签发该CA证书的根CA证书的唯一识别码。
    #[serde(rename = "ParentIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_identifier: Option<String>,
    /// CA证书关联的组织机构所在城市的名称。
    #[serde(rename = "Locality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    /// CA证书的唯一识别码。
    #[serde(rename = "Identifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// 该参数已废弃。
    #[serde(rename = "Sans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sans: Option<String>,
    /// CA证书的密钥长度。
    #[serde(rename = "KeySize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_size: Option<i32>,
    /// CA证书的内容。
    #[serde(rename = "X509Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x509_certificate: Option<String>,
    /// CA证书的DN（Distinguished Name）属性，表示证书的使用者，具体包含以下信息：
    #[serde(rename = "SubjectDN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_dn: Option<String>,
    /// CA证书的签名算法。
    #[serde(rename = "SignAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign_algorithm: Option<String>,
    /// CA证书关联的组织机构下部门的名称。
    #[serde(rename = "OrganizationUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_unit: Option<String>,
    /// CA证书的到期日期。使用时间戳表示，单位：毫秒。
    #[serde(rename = "AfterDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_date: Option<i64>,
    /// CA证书的SHA256数字指纹。
    #[serde(rename = "Sha2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha2: Option<String>,
    /// CA证书关联的组织机构的通用名称或简称。
    #[serde(rename = "CommonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_name: Option<String>,
    /// CA证书的MD5数字指纹。
    #[serde(rename = "Md5")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md5: Option<String>,
    /// CA证书关联的组织机构所在国家的代码。
    #[serde(rename = "CountryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// CA证书的签发日期。使用时间戳表示，单位：毫秒。
    #[serde(rename = "BeforeDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_date: Option<i64>,
    /// 实例别名。
    #[serde(rename = "Alias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    /// 是否为试用实例。取值：
    #[serde(rename = "Trial")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial: Option<i32>,
    /// 是否为赠送实例。取值：
    #[serde(rename = "Gift")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gift: Option<i32>,
    /// 证书所属资源组的ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl DescribeCACertificateListResponseCertificateListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.years {
            params.push(("Years".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.serial_number {
            params.push(("SerialNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.certificate_type {
            params.push(("CertificateType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.algorithm {
            params.push(("Algorithm".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.state {
            params.push(("State".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.organization {
            params.push(("Organization".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.parent_identifier {
            params.push(("ParentIdentifier".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.locality {
            params.push(("Locality".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.identifier {
            params.push(("Identifier".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sans {
            params.push(("Sans".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key_size {
            params.push(("KeySize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.x509_certificate {
            params.push(("X509Certificate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.subject_dn {
            params.push(("SubjectDN".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sign_algorithm {
            params.push(("SignAlgorithm".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.organization_unit {
            params.push(("OrganizationUnit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.after_date {
            params.push(("AfterDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sha2 {
            params.push(("Sha2".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.common_name {
            params.push(("CommonName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.md5 {
            params.push(("Md5".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.country_code {
            params.push(("CountryCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.before_date {
            params.push(("BeforeDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.alias {
            params.push(("Alias".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.trial {
            params.push(("Trial".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.gift {
            params.push(("Gift".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

/// 标签值。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClientCertificateResponseCertificateTagsItem {
    /// 标签 Key
    #[serde(rename = "TagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    /// 标签值。
    #[serde(rename = "TagValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
}

impl DescribeClientCertificateResponseCertificateTagsItem {
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

/// 客户端证书或服务端证书的详细信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClientCertificateResponseCertificate {
    /// 证书的内容。
    #[serde(rename = "X509Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x509_certificate: Option<String>,
    /// 证书的唯一识别码。
    #[serde(rename = "Identifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// 证书的序列号。
    #[serde(rename = "SerialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// 证书的DN（Distinguished Name）属性，表示证书的使用者，具体包含以下信息：
    #[serde(rename = "SubjectDN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_dn: Option<String>,
    /// 证书的公用名。
    #[serde(rename = "CommonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_name: Option<String>,
    /// 签发该证书的子CA证书关联的组织机构下部门的名称。
    #[serde(rename = "OrganizationUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_unit: Option<String>,
    /// 签发该证书的子CA证书关联的组织机构的名称。
    #[serde(rename = "Organization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// 签发该证书的子CA证书关联的组织机构所在城市的名称。
    #[serde(rename = "Locality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    /// <props="china">签发该证书的子CA证书关联的组织机构所在省份、直辖市或自治区的名称。</props>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// 签发该证书的子CA证书关联的组织机构所在国家的代码。
    #[serde(rename = "CountryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// 证书的SAN（Subject Alternative Name）扩展属性，表示证书关联的其他域名、IP地址等。
    #[serde(rename = "Sans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sans: Option<String>,
    /// 证书的状态。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 证书的签发日期。使用时间戳表示，单位：毫秒。
    #[serde(rename = "BeforeDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_date: Option<i64>,
    /// 证书的到期日期。使用时间戳表示，单位：毫秒。
    #[serde(rename = "AfterDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_date: Option<i64>,
    /// 证书的加密算法类型。取值：
    #[serde(rename = "Algorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    /// 证书的密钥长度。
    #[serde(rename = "KeySize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_size: Option<i32>,
    /// 证书的签名算法。
    #[serde(rename = "SignAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign_algorithm: Option<String>,
    /// 证书的类型。取值：
    #[serde(rename = "CertificateType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_type: Option<String>,
    /// 签发该证书的子CA证书的唯一识别码。
    #[serde(rename = "ParentIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_identifier: Option<String>,
    /// 证书的SHA256数字指纹。
    #[serde(rename = "Sha2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha2: Option<String>,
    /// 证书的MD5数字指纹。
    #[serde(rename = "Md5")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md5: Option<String>,
    /// 证书的有效期。单位：天。
    #[serde(rename = "Days")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i32>,
    /// 证书所属资源组的ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 完整的证书链。
    #[serde(rename = "CertChain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_chain: Option<String>,
    /// 用户自定义标识，唯一键。
    #[serde(rename = "CustomIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_identifier: Option<String>,
    /// 表示该证书是否已同步到数字证书服务。
    #[serde(rename = "UploadFlag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_flag: Option<i32>,
    /// 携带长度的算法。
    #[serde(rename = "FullAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_algorithm: Option<String>,
    /// 设置签发证书的名称。
    #[serde(rename = "AliasName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_name: Option<String>,
    /// 证书订单所属数据源ID。
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// 标签列表。
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<DescribeClientCertificateResponseCertificateTagsItem>>,
}

impl DescribeClientCertificateResponseCertificate {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.x509_certificate {
            params.push(("X509Certificate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.identifier {
            params.push(("Identifier".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.serial_number {
            params.push(("SerialNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.subject_dn {
            params.push(("SubjectDN".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.common_name {
            params.push(("CommonName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.organization_unit {
            params.push(("OrganizationUnit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.organization {
            params.push(("Organization".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.locality {
            params.push(("Locality".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.state {
            params.push(("State".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.country_code {
            params.push(("CountryCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sans {
            params.push(("Sans".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.before_date {
            params.push(("BeforeDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.after_date {
            params.push(("AfterDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.algorithm {
            params.push(("Algorithm".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key_size {
            params.push(("KeySize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sign_algorithm {
            params.push(("SignAlgorithm".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.certificate_type {
            params.push(("CertificateType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.parent_identifier {
            params.push(("ParentIdentifier".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sha2 {
            params.push(("Sha2".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.md5 {
            params.push(("Md5".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.days {
            params.push(("Days".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_chain {
            params.push(("CertChain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.custom_identifier {
            params.push(("CustomIdentifier".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.upload_flag {
            params.push(("UploadFlag".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.full_algorithm {
            params.push(("FullAlgorithm".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.alias_name {
            params.push(("AliasName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.id {
            params.push(("Id".to_string(), v.to_string()));
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

/// 证书信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClientCertificateForSerialNumberResponseCertificateListItem {
    /// 证书的内容。
    #[serde(rename = "X509Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x509_certificate: Option<String>,
    /// 证书的唯一识别码。
    #[serde(rename = "Identifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// 证书的序列号。
    #[serde(rename = "SerialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// 证书的DN（Distinguished Name）属性，表示证书的使用者，具体包含以下信息：
    #[serde(rename = "SubjectDN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_dn: Option<String>,
    /// 证书的公用名。
    #[serde(rename = "CommonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_name: Option<String>,
    /// 签发该证书的子CA证书关联的组织机构下部门的名称。
    #[serde(rename = "OrganizationUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_unit: Option<String>,
    /// 签发该证书的子CA证书关联的组织机构的名称。
    #[serde(rename = "Organization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// 签发该证书的子CA证书关联的组织机构所在城市的名称。
    #[serde(rename = "Locality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    /// <props="china">签发该证书的子CA证书关联的组织机构所在省份、直辖市或自治区的名称。</props>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// 签发该证书的子CA证书关联的组织机构所在国家的代码。
    #[serde(rename = "CountryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// 证书的SAN（Subject Alternative Name）扩展属性，表示证书关联的其他域名、IP地址等。
    #[serde(rename = "Sans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sans: Option<String>,
    /// 证书的状态。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 证书的签发时间。
    #[serde(rename = "BeforeDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_date: Option<String>,
    /// 证书的到期时间。
    #[serde(rename = "AfterDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_date: Option<String>,
    /// 证书的加密算法类型。取值：
    #[serde(rename = "Algorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    /// 证书的密钥长度。
    #[serde(rename = "KeySize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_size: Option<i32>,
    /// 证书的签名算法。
    #[serde(rename = "SignAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign_algorithm: Option<String>,
    /// 证书类型。
    #[serde(rename = "CertificateType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_type: Option<String>,
    /// 如果该参数不为空，代表该客户端证书的颁发机构为阿里云。
    #[serde(rename = "ParentIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_identifier: Option<String>,
    /// 证书的SHA256数字指纹。
    #[serde(rename = "Sha2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha2: Option<String>,
    /// 证书的MD5数字指纹。
    #[serde(rename = "Md5")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md5: Option<String>,
    /// 该参数已废弃。
    #[serde(rename = "Years")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub years: Option<i32>,
}

impl DescribeClientCertificateForSerialNumberResponseCertificateListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.x509_certificate {
            params.push(("X509Certificate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.identifier {
            params.push(("Identifier".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.serial_number {
            params.push(("SerialNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.subject_dn {
            params.push(("SubjectDN".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.common_name {
            params.push(("CommonName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.organization_unit {
            params.push(("OrganizationUnit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.organization {
            params.push(("Organization".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.locality {
            params.push(("Locality".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.state {
            params.push(("State".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.country_code {
            params.push(("CountryCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sans {
            params.push(("Sans".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.before_date {
            params.push(("BeforeDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.after_date {
            params.push(("AfterDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.algorithm {
            params.push(("Algorithm".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key_size {
            params.push(("KeySize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sign_algorithm {
            params.push(("SignAlgorithm".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.certificate_type {
            params.push(("CertificateType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.parent_identifier {
            params.push(("ParentIdentifier".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sha2 {
            params.push(("Sha2".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.md5 {
            params.push(("Md5".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.years {
            params.push(("Years".to_string(), v.to_string()));
        }
        params
    }
}

/// 对象。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClientCertificateStatusResponseCertificateStatusItem {
    /// 证书的序列号。
    #[serde(rename = "SerialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// 证书的当前状态。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 证书被吊销的日期。
    #[serde(rename = "RevokeTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoke_time: Option<i64>,
}

impl DescribeClientCertificateStatusResponseCertificateStatusItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.serial_number {
            params.push(("SerialNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.revoke_time {
            params.push(("RevokeTime".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClientCertificateStatusForSerialNumberResponseCertificateStatusItem {
    /// 证书的序列号。
    #[serde(rename = "SerialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// 证书的当前状态。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 证书被吊销的日期。
    #[serde(rename = "RevokeTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoke_time: Option<i64>,
}

impl DescribeClientCertificateStatusForSerialNumberResponseCertificateStatusItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.serial_number {
            params.push(("SerialNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.revoke_time {
            params.push(("RevokeTime".to_string(), v.to_string()));
        }
        params
    }
}

/// 证书显示列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePcaAndExternalCACertificateListResponseCertificateListItem {
    /// 证书购买年限。
    #[serde(rename = "Years")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub years: Option<i32>,
    /// 证书状态。取值：-**payed**：已付款-**checking**：审核中-**issued**：已签发-**revoked**：已吊销-**checked_fail**：审核失败
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 证书序列号。
    #[serde(rename = "SerialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// 证书类型。
    #[serde(rename = "CertificateType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_type: Option<String>,
    /// 证书id。
    #[serde(rename = "Algorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    /// 证书状态。-**success**：已生效-**checking**：检测域名是否在阿里云全站加速-**cname_error**：域名没有切到阿里云全球加速实例-**domain_invali...
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// 证书组织。
    #[serde(rename = "Organization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// 父证书id。
    #[serde(rename = "ParentIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_identifier: Option<String>,
    /// 证书绑定的主域名。
    #[serde(rename = "Locality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    /// 证书id。
    #[serde(rename = "Identifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// 证书绑定的所有域名。
    #[serde(rename = "Sans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sans: Option<String>,
    /// 证书key的大小，单位gb。
    #[serde(rename = "KeySize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_size: Option<i32>,
    /// x09证书。
    #[serde(rename = "X509Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x509_certificate: Option<String>,
    /// 证书主体（拥有者），采用dn标识。
    #[serde(rename = "SubjectDN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_dn: Option<String>,
    /// 证书签名算法，取值：-**prefix**：前缀验证。-**match**：完全匹配。-**any**：全部匹配。
    #[serde(rename = "SignAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign_algorithm: Option<String>,
    /// 证书颁发机构。
    #[serde(rename = "OrganizationUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_unit: Option<String>,
    /// 证书到期时间。
    #[serde(rename = "AfterDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_date: Option<i64>,
    /// 证书绑定的主域名。
    #[serde(rename = "Sha2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha2: Option<String>,
    /// 证书绑定的主域名。
    #[serde(rename = "CommonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_name: Option<String>,
    /// 证书绑定的md5值。
    #[serde(rename = "Md5")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md5: Option<String>,
    /// 证书国家标准编码。
    #[serde(rename = "CountryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// 证书签发时间。
    #[serde(rename = "BeforeDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_date: Option<i64>,
}

impl DescribePcaAndExternalCACertificateListResponseCertificateListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.years {
            params.push(("Years".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.serial_number {
            params.push(("SerialNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.certificate_type {
            params.push(("CertificateType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.algorithm {
            params.push(("Algorithm".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.state {
            params.push(("State".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.organization {
            params.push(("Organization".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.parent_identifier {
            params.push(("ParentIdentifier".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.locality {
            params.push(("Locality".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.identifier {
            params.push(("Identifier".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sans {
            params.push(("Sans".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key_size {
            params.push(("KeySize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.x509_certificate {
            params.push(("X509Certificate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.subject_dn {
            params.push(("SubjectDN".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sign_algorithm {
            params.push(("SignAlgorithm".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.organization_unit {
            params.push(("OrganizationUnit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.after_date {
            params.push(("AfterDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sha2 {
            params.push(("Sha2".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.common_name {
            params.push(("CommonName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.md5 {
            params.push(("Md5".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.country_code {
            params.push(("CountryCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.before_date {
            params.push(("BeforeDate".to_string(), v.to_string()));
        }
        params
    }
}

/// 对象。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetCAInstanceStatusResponseInstanceStatusListItem {
    /// 私有CA实例能够签发证书的数量。
    #[serde(rename = "CertTotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_total_count: Option<i32>,
    /// 私有CA实例的状态。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 私有CA实例的类型。取值：
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 私有CA实例已经签发证书的数量。
    #[serde(rename = "CertIssuedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_issued_count: Option<i32>,
    /// 私有CA证书的签发日期。使用时间戳表示，单位：毫秒。
    #[serde(rename = "BeforeTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_time: Option<i64>,
    /// 私有CA证书的唯一标识码。
    #[serde(rename = "Identifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// 私有CA证书的到期日期。使用时间戳格式表示，单位：毫秒。
    #[serde(rename = "AfterTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_time: Option<i64>,
    /// 私有CA实例的ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 私有CA实例的到期日期。使用时间戳表示，单位：毫秒。
    #[serde(rename = "UseExpireTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_expire_time: Option<i64>,
}

impl GetCAInstanceStatusResponseInstanceStatusListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cert_total_count {
            params.push(("CertTotalCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_issued_count {
            params.push(("CertIssuedCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.before_time {
            params.push(("BeforeTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.identifier {
            params.push(("Identifier".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.after_time {
            params.push(("AfterTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.use_expire_time {
            params.push(("UseExpireTime".to_string(), v.to_string()));
        }
        params
    }
}

/// 对象。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListCACertificateLogResponseLogListItem {
    /// CA证书的唯一识别码。
    #[serde(rename = "Identifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// 操作内容描述。
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 操作时间。使用时间戳表示，单位：毫秒。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 操作类型。取值：
    #[serde(rename = "OpType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_type: Option<String>,
}

impl ListCACertificateLogResponseLogListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.identifier {
            params.push(("Identifier".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.content {
            params.push(("Content".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.op_type {
            params.push(("OpType".to_string(), v.to_string()));
        }
        params
    }
}

/// 证书信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListCertResponseListItem {
    /// 证书状态。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 证书到期时间。
    #[serde(rename = "AfterDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_date: Option<String>,
    /// 证书组织。
    #[serde(rename = "Organization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// 是否可以使用该证书。取值：
    #[serde(rename = "KeyExportable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_exportable: Option<bool>,
    /// 订阅关系id。
    #[serde(rename = "SubjectDn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_dn: Option<String>,
    /// 算法类型。
    #[serde(rename = "Algorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    /// 证书类型。取值：
    #[serde(rename = "CertificateType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_type: Option<String>,
    /// 证书标识。
    #[serde(rename = "Identifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// 证书序列号。
    #[serde(rename = "SerialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// 扩展字段。
    #[serde(rename = "Extra")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
    /// 购买证书的用户所属的公司或组织名称。
    #[serde(rename = "OrganizationUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_unit: Option<String>,
    /// 客户端证书的签发时间，使用时间戳格式，默认为您调用该接口的时间。单位：毫秒。
    #[serde(rename = "BeforeTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_time: Option<i64>,
    /// 设置签发证书的名称。
    #[serde(rename = "AliasName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_name: Option<String>,
    /// 客户端证书的服务到期时间，使用时间戳格式。单位：毫秒。
    #[serde(rename = "AfterTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_time: Option<i64>,
    /// 证书订单所属数据源ID。
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// 证书标签。
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// 证书绑定的主域名。
    #[serde(rename = "CommonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_name: Option<String>,
    /// 证书签发时间。
    #[serde(rename = "BeforeDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_date: Option<String>,
    /// 用户自定义标识，唯一键。
    #[serde(rename = "CustomIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_identifier: Option<String>,
}

impl ListCertResponseListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.after_date {
            params.push(("AfterDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.organization {
            params.push(("Organization".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key_exportable {
            params.push(("KeyExportable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.subject_dn {
            params.push(("SubjectDn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.algorithm {
            params.push(("Algorithm".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.certificate_type {
            params.push(("CertificateType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.identifier {
            params.push(("Identifier".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.serial_number {
            params.push(("SerialNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.extra {
            params.push(("Extra".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.organization_unit {
            params.push(("OrganizationUnit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.before_time {
            params.push(("BeforeTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.alias_name {
            params.push(("AliasName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.after_time {
            params.push(("AfterTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.id {
            params.push(("Id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Tags.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.common_name {
            params.push(("CommonName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.before_date {
            params.push(("BeforeDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.custom_identifier {
            params.push(("CustomIdentifier".to_string(), v.to_string()));
        }
        params
    }
}

/// 对象。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListClientCertificateResponseCertificateListItem {
    /// 证书的内容。
    #[serde(rename = "X509Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x509_certificate: Option<String>,
    /// 证书的唯一识别码。
    #[serde(rename = "Identifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// 证书的序列号。
    #[serde(rename = "SerialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// 证书的DN（Distinguished Name）属性，表示证书的使用者，具体包含以下信息：
    #[serde(rename = "SubjectDN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_dn: Option<String>,
    /// 证书的公用名。
    #[serde(rename = "CommonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_name: Option<String>,
    /// 签发该证书的子CA证书关联的组织机构下部门的名称。
    #[serde(rename = "OrganizationUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_unit: Option<String>,
    /// 签发该证书的子CA证书关联的组织机构的名称。
    #[serde(rename = "Organization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// 签发该证书的子CA证书关联的组织机构所在城市的名称。
    #[serde(rename = "Locality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    /// <props="china">签发该证书的子CA证书关联的组织机构所在省份、直辖市或自治区的名称。</props>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// 签发该证书的子CA证书关联的组织机构所在国家的代码。
    #[serde(rename = "CountryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// 证书的SAN（Subject Alternative Name）扩展属性，表示证书关联的其他域名、IP地址等。
    #[serde(rename = "Sans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sans: Option<String>,
    /// 证书的状态。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 证书的签发日期。使用时间戳表示，单位：毫秒。
    #[serde(rename = "BeforeDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_date: Option<i64>,
    /// 证书的到期日期。使用时间戳表示，单位：毫秒。
    #[serde(rename = "AfterDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_date: Option<i64>,
    /// 证书的加密算法类型。取值：
    #[serde(rename = "Algorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    /// 证书的密钥长度。
    #[serde(rename = "KeySize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_size: Option<i32>,
    /// 证书的签名算法。
    #[serde(rename = "SignAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign_algorithm: Option<String>,
    /// 证书的类型。取值：
    #[serde(rename = "CertificateType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_type: Option<String>,
    /// 签发该证书的子CA证书的唯一识别码。
    #[serde(rename = "ParentIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_identifier: Option<String>,
    /// 证书的SHA256数字指纹。
    #[serde(rename = "Sha2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha2: Option<String>,
    /// 证书的MD5数字指纹。
    #[serde(rename = "Md5")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md5: Option<String>,
    /// 证书的有效期。单位：天。
    #[serde(rename = "Days")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i32>,
    /// 证书所属资源组的ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 用户自定义标识，唯一键。
    #[serde(rename = "CustomIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_identifier: Option<String>,
    /// 证书主键ID。
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// 设置签发证书的名称。
    #[serde(rename = "AliasName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_name: Option<String>,
}

impl ListClientCertificateResponseCertificateListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.x509_certificate {
            params.push(("X509Certificate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.identifier {
            params.push(("Identifier".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.serial_number {
            params.push(("SerialNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.subject_dn {
            params.push(("SubjectDN".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.common_name {
            params.push(("CommonName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.organization_unit {
            params.push(("OrganizationUnit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.organization {
            params.push(("Organization".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.locality {
            params.push(("Locality".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.state {
            params.push(("State".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.country_code {
            params.push(("CountryCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sans {
            params.push(("Sans".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.before_date {
            params.push(("BeforeDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.after_date {
            params.push(("AfterDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.algorithm {
            params.push(("Algorithm".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key_size {
            params.push(("KeySize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sign_algorithm {
            params.push(("SignAlgorithm".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.certificate_type {
            params.push(("CertificateType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.parent_identifier {
            params.push(("ParentIdentifier".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sha2 {
            params.push(("Sha2".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.md5 {
            params.push(("Md5".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.days {
            params.push(("Days".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.custom_identifier {
            params.push(("CustomIdentifier".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.id {
            params.push(("Id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.alias_name {
            params.push(("AliasName".to_string(), v.to_string()));
        }
        params
    }
}

/// 蜜罐探针数据。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListPcaCaCertificateResponseListItem {
    /// 证书标识。可用于查询证书详情。
    #[serde(rename = "CertIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_identifier: Option<String>,
    /// 证书状态。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 证书通用名称。与证书主体信息（Subject）的CommonName一致。
    #[serde(rename = "CommonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_name: Option<String>,
    /// 颁发者证书标识。可用于查询颁发者证书。
    #[serde(rename = "IssuerIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer_identifier: Option<String>,
    /// 私有CA实例ID。
    #[serde(rename = "PrivateCaInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ca_instance_id: Option<String>,
    /// 私有CA实例地域ID。
    #[serde(rename = "PrivateCaRegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ca_region_id: Option<String>,
    /// 账号ID。
    #[serde(rename = "UserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl ListPcaCaCertificateResponseListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cert_identifier {
            params.push(("CertIdentifier".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.common_name {
            params.push(("CommonName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.issuer_identifier {
            params.push(("IssuerIdentifier".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.private_ca_instance_id {
            params.push(("PrivateCaInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.private_ca_region_id {
            params.push(("PrivateCaRegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_id {
            params.push(("UserId".to_string(), v.to_string()));
        }
        params
    }
}

/// 对象。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListRevokeCertificateResponseCertificateListItem {
    /// 证书被吊销的日期。格式为`yyyy-MM-ddT00:00Z`，例如，`2021-09-01T00:00Z`表示2021年09月01日。
    #[serde(rename = "RevokeDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoke_date: Option<String>,
    /// 证书的唯一识别码。
    #[serde(rename = "Identifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// 证书的序列号。
    #[serde(rename = "SerialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// 证书的DN（Distinguished Name）属性，表示证书的使用者，具体包含以下信息：
    #[serde(rename = "SubjectDN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_dn: Option<String>,
    /// 证书的公用名。
    #[serde(rename = "CommonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_name: Option<String>,
    /// 签发该证书的子CA证书关联的组织机构下部门的名称。
    #[serde(rename = "OrganizationUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_unit: Option<String>,
    /// 签发该证书的子CA证书关联的组织机构的名称。
    #[serde(rename = "Organization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// 签发该证书的子CA证书关联的组织机构所在城市的名称。
    #[serde(rename = "Locality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    /// <props="china">签发该证书的子CA证书关联的组织机构所在省份、直辖市或自治区的名称。</props>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// 签发该证书的子CA证书关联的组织机构所在国家的代码。
    #[serde(rename = "CountryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// 证书的SAN（Subject Alternative Name）扩展属性。
    #[serde(rename = "Sans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sans: Option<String>,
    /// 状态。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 证书的签发日期。格式为`yyyy-MM-ddT00:00Z`，例如，`2021-01-01T00:00Z`表示2021年01月01日。
    #[serde(rename = "BeforeDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_date: Option<String>,
    /// 证书的到期日期。格式为`yyyy-MM-ddT00:00Z`，例如，`2021-12-31T00:00Z`表示2021年12月31日。
    #[serde(rename = "AfterDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_date: Option<String>,
    /// 证书的加密算法类型。取值：
    #[serde(rename = "Algorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    /// 证书的密钥长度。
    #[serde(rename = "KeySize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_size: Option<i32>,
    /// 证书的签名算法。
    #[serde(rename = "SignAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign_algorithm: Option<String>,
    /// 证书类型。
    #[serde(rename = "CertificateType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_type: Option<String>,
    /// 父证书标识。
    #[serde(rename = "ParentIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_identifier: Option<String>,
    /// 证书的SHA256数字指纹。
    #[serde(rename = "Sha2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha2: Option<String>,
    /// 证书的MD5数字指纹。
    #[serde(rename = "Md5")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md5: Option<String>,
    /// 用户自定义标识，唯一键。
    #[serde(rename = "CustomIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_identifier: Option<String>,
}

impl ListRevokeCertificateResponseCertificateListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.revoke_date {
            params.push(("RevokeDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.identifier {
            params.push(("Identifier".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.serial_number {
            params.push(("SerialNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.subject_dn {
            params.push(("SubjectDN".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.common_name {
            params.push(("CommonName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.organization_unit {
            params.push(("OrganizationUnit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.organization {
            params.push(("Organization".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.locality {
            params.push(("Locality".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.state {
            params.push(("State".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.country_code {
            params.push(("CountryCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sans {
            params.push(("Sans".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.before_date {
            params.push(("BeforeDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.after_date {
            params.push(("AfterDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.algorithm {
            params.push(("Algorithm".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key_size {
            params.push(("KeySize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sign_algorithm {
            params.push(("SignAlgorithm".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.certificate_type {
            params.push(("CertificateType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.parent_identifier {
            params.push(("ParentIdentifier".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sha2 {
            params.push(("Sha2".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.md5 {
            params.push(("Md5".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.custom_identifier {
            params.push(("CustomIdentifier".to_string(), v.to_string()));
        }
        params
    }
}

/// 查询到的实例和标签的信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListTagResourcesRequestTagItem {
    /// 要查询的标签的key，可有多个。n为正整数。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。n的取值范围：1~20。
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

/// 返回数据。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListTagResourcesResponseDataTagResourcesTagResourcesItem {
    /// 要操作的证书实例ID。
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// 资源的类型。取值：**instance**。
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// 要绑定的标签键。
    #[serde(rename = "TagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    /// 标签值。
    #[serde(rename = "TagValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
}

impl ListTagResourcesResponseDataTagResourcesTagResourcesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_id {
            params.push(("ResourceId".to_string(), v.to_string()));
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListTagResourcesResponseDataTagResources {
    /// 返回数据。
    #[serde(rename = "TagResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_resources: Option<Vec<ListTagResourcesResponseDataTagResourcesTagResourcesItem>>,
}

impl ListTagResourcesResponseDataTagResources {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.tag_resources {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("TagResources.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 请求接口返回的数据。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListTagResourcesResponseData {
    /// 下一个查询开始Token，NextToken为空说明没有下一个。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TagResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_resources: Option<ListTagResourcesResponseDataTagResources>,
    /// 本次读取的最大数据量。
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
}

impl ListTagResourcesResponseData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.next_token {
            params.push(("NextToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_resources {
            for (k, v2) in v.to_query_params() {
                params.push((format!("TagResources.{}", k), v2));
            }
        }
        if let Some(ref v) = self.max_results {
            params.push(("MaxResults".to_string(), v.to_string()));
        }
        params
    }
}

/// 资源的标签信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TagResourcesRequestTagItem {
    /// 标签键。n的取值范围为\[1,20]，用于指定多个标签键，最多20个。例如：tag.1.key，tag.2.key，...，tag.20.key。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。n的取值范围：1~20。
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

/// 标签列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdatePcaCertificateRequestTagsItem {
    /// 标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl UpdatePcaCertificateRequestTagsItem {
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

/// 鉴权报错信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdatePcaCertificateResponseAccessDeniedDetail {
    /// 尝试执行的未授权操作。
    #[serde(rename = "AuthAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_action: Option<String>,
    /// 请求中用于鉴权的身份标识。具体取值为：
    #[serde(rename = "AuthPrincipalDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_principal_display_name: Option<String>,
    /// AuthPrincipalOwnerId
    #[serde(rename = "AuthPrincipalOwnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_principal_owner_id: Option<String>,
    /// 身份类型
    #[serde(rename = "AuthPrincipalType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_principal_type: Option<String>,
    /// 加密后的完整诊断信息。
    #[serde(rename = "EncodedDiagnosticMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoded_diagnostic_message: Option<String>,
    /// 导致鉴权失败的原因。具体取值为：
    #[serde(rename = "NoPermissionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_permission_type: Option<String>,
    /// 策略类型
    #[serde(rename = "PolicyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
}

impl UpdatePcaCertificateResponseAccessDeniedDetail {
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

/// AssignCertificateCount 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct AssignCertificateCountRequest {
    /// 证书所属数据源id。
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// 证书记录总数。
    #[serde(rename = "CertTotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_total_count: Option<i32>,
}

impl AssignCertificateCountRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.id {
            params.push(("Id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_total_count {
            params.push(("CertTotalCount".to_string(), v.to_string()));
        }
        params
    }
}

/// OpenApiResponse<AssignCertificateCountResult>
#[derive(Debug, Clone, Deserialize)]
pub struct AssignCertificateCountResponse {
    /// 请求id。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 已分配证书数量。
    #[serde(rename = "CertCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_count: Option<i32>,
    /// 当前免费证书数量。
    #[serde(rename = "CurrentYearFreeCertCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_year_free_cert_count: Option<i32>,
}

/// CreateClientCertificate 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateClientCertificateRequest {
    /// 客户端证书支持的扩展信息SAN（Subject Alternative Name）类型。取值：
    #[serde(rename = "SanType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub san_type: Option<i32>,
    /// 客户端证书的具体扩展信息。支持输入多个扩展信息，如果您需要输入多个扩展信息，请用半角逗号（,）将其隔开。
    #[serde(rename = "SanValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub san_value: Option<String>,
    /// 机构名称，默认：Alibaba Inc。
    #[serde(rename = "Organization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// 部门名称，默认：Aliyun CDN。
    #[serde(rename = "OrganizationUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_unit: Option<String>,
    /// 所属国家，默认：CN。
    #[serde(rename = "Country")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// 表示证书使用者的名称。客户端认证（ClientAuth）证书使用者一般是自然人、公司、组织或某个应用，建议填写使用者的通用名称。例如，张三、阿里巴巴、阿里云密码平台、天猫精灵等。
    #[serde(rename = "CommonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_name: Option<String>,
    /// <props="china">设置证书组织机构所在省份、直辖市或自治区的名称。支持使用中文、英文字符等。默认为签发该证书的子CA证书组织机构所在省份、直辖市或自治区的名称。</props>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// 设置证书组织机构所在城市的名称。支持使用中文、英文字符等。
    #[serde(rename = "Locality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    /// 客户端证书的密钥算法。密钥算法使用`<加密算法>_<密钥长度>`格式表示。取值：
    #[serde(rename = "Algorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    /// 签发该证书的子CA证书的唯一识别码。
    #[serde(rename = "ParentIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_identifier: Option<String>,
    /// 购买证书的时长。单位：年。
    #[serde(rename = "Years")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub years: Option<i32>,
    /// 购买证书的时长。单位：月。
    #[serde(rename = "Months")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub months: Option<i32>,
    /// 客户端证书的有效期。单位：天。
    #[serde(rename = "Days")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i32>,
    /// 客户端证书的签发时间，使用时间戳格式，默认为您调用该接口的时间。单位：秒。
    #[serde(rename = "BeforeTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_time: Option<i64>,
    /// 客户端证书的服务到期时间，使用时间戳格式。单位：秒。
    #[serde(rename = "AfterTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_time: Option<i64>,
    /// 立即返回数字证书。
    #[serde(rename = "Immediately")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub immediately: Option<i32>,
    /// 是否包含CRL地址
    #[serde(rename = "EnableCrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_crl: Option<i64>,
    /// 标签列表。
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<CreateClientCertificateRequestTagsItem>>,
    /// 资源分组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 用户自定义标识，唯一键。
    #[serde(rename = "CustomIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_identifier: Option<String>,
    /// 设置签发证书的名称。
    #[serde(rename = "AliasName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_name: Option<String>,
}

impl CreateClientCertificateRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.san_type {
            params.push(("SanType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.san_value {
            params.push(("SanValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.organization {
            params.push(("Organization".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.organization_unit {
            params.push(("OrganizationUnit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.country {
            params.push(("Country".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.common_name {
            params.push(("CommonName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.state {
            params.push(("State".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.locality {
            params.push(("Locality".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.algorithm {
            params.push(("Algorithm".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.parent_identifier {
            params.push(("ParentIdentifier".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.years {
            params.push(("Years".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.months {
            params.push(("Months".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.days {
            params.push(("Days".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.before_time {
            params.push(("BeforeTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.after_time {
            params.push(("AfterTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.immediately {
            params.push(("Immediately".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_crl {
            params.push(("EnableCrl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tags.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.custom_identifier {
            params.push(("CustomIdentifier".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.alias_name {
            params.push(("AliasName".to_string(), v.to_string()));
        }
        params
    }
}

/// CreateCertificateResponse<CertificateIdentifierWithParentDTO>
#[derive(Debug, Clone, Deserialize)]
pub struct CreateClientCertificateResponse {
    /// 客户端证书的内容。
    #[serde(rename = "X509Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x509_certificate: Option<String>,
    /// CA证书链。
    #[serde(rename = "CertificateChain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_chain: Option<String>,
    /// 客户端证书的唯一识别码。
    #[serde(rename = "Identifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// 证书序列号。
    #[serde(rename = "SerialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreateClientCertificateWithCsr 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateClientCertificateWithCsrRequest {
    /// CSR内容。您可以通过OpenSSL工具或者Keytool工具生成CSR。更多信息，请参见[如何制作CSR文件](~~42218~~)。
    #[serde(rename = "Csr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csr: Option<String>,
    /// 客户端证书的扩展信息SAN（Subject Alternative Name）的类型。取值：
    #[serde(rename = "SanType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub san_type: Option<i32>,
    /// 客户端证书的具体扩展信息。支持输入多个扩展信息，如果您需要输入多个扩展信息，请用半角逗号（,）将其隔开。
    #[serde(rename = "SanValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub san_value: Option<String>,
    /// 机构名称，默认：Alibaba Inc。
    #[serde(rename = "Organization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// 部门名称，默认：Aliyun CDN。
    #[serde(rename = "OrganizationUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_unit: Option<String>,
    /// 国家代码，例如**CN**、**US**。
    #[serde(rename = "Country")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// 设置证书的公用名。支持使用中文、英文字符等。
    #[serde(rename = "CommonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_name: Option<String>,
    /// <props="china">设置证书组织机构所在省份、直辖市或自治区的名称。支持使用中文、英文字符等。默认为签发该证书的子CA证书组织机构所在省份、直辖市或自治区的名称。</props>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// 设置证书组织机构所在城市的名称。支持使用中文、英文字符等。
    #[serde(rename = "Locality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    /// 客户端证书的密钥算法。密钥算法使用`<加密算法>_<密钥长度>`格式表示。取值：
    #[serde(rename = "Algorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    /// 签发该证书的子CA证书的唯一识别码。
    #[serde(rename = "ParentIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_identifier: Option<String>,
    /// 证书有效期。单位：年。
    #[serde(rename = "Years")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub years: Option<i32>,
    /// 证书有效期。单位：月。
    #[serde(rename = "Months")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub months: Option<i32>,
    /// 客户端证书的有效期。单位：天。
    #[serde(rename = "Days")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i32>,
    /// 客户端证书的签发时间，使用时间戳格式，默认为您调用该接口的时间。单位：秒。
    #[serde(rename = "BeforeTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_time: Option<i64>,
    /// 客户端证书的服务到期时间，使用时间戳格式。单位：秒。
    #[serde(rename = "AfterTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_time: Option<i64>,
    /// 立即返回数字证书。
    #[serde(rename = "Immediately")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub immediately: Option<i32>,
    /// 是否包含CRL地址
    #[serde(rename = "EnableCrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_crl: Option<i64>,
    /// 标签列表。
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<CreateClientCertificateWithCsrRequestTagsItem>>,
    /// 证书所属资源组的ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 用户自定义标识，唯一键。
    #[serde(rename = "CustomIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_identifier: Option<String>,
}

impl CreateClientCertificateWithCsrRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.csr {
            params.push(("Csr".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.san_type {
            params.push(("SanType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.san_value {
            params.push(("SanValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.organization {
            params.push(("Organization".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.organization_unit {
            params.push(("OrganizationUnit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.country {
            params.push(("Country".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.common_name {
            params.push(("CommonName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.state {
            params.push(("State".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.locality {
            params.push(("Locality".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.algorithm {
            params.push(("Algorithm".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.parent_identifier {
            params.push(("ParentIdentifier".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.years {
            params.push(("Years".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.months {
            params.push(("Months".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.days {
            params.push(("Days".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.before_time {
            params.push(("BeforeTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.after_time {
            params.push(("AfterTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.immediately {
            params.push(("Immediately".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_crl {
            params.push(("EnableCrl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tags.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.custom_identifier {
            params.push(("CustomIdentifier".to_string(), v.to_string()));
        }
        params
    }
}

/// CreateCertificateResponse<CertificateIdentifierWithParentDTO>
#[derive(Debug, Clone, Deserialize)]
pub struct CreateClientCertificateWithCsrResponse {
    /// 客户端证书的内容。
    #[serde(rename = "X509Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x509_certificate: Option<String>,
    /// CA证书链。
    #[serde(rename = "CertificateChain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_chain: Option<String>,
    /// 客户端证书的唯一识别码。
    #[serde(rename = "Identifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// 证书序列号。
    #[serde(rename = "SerialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 加密证书内容。
    #[serde(rename = "CertSignBufKmc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_sign_buf_kmc: Option<String>,
    /// 加密证书密文。
    #[serde(rename = "CertKmcRep1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_kmc_rep1: Option<String>,
}

/// CreateCustomCertificate 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateCustomCertificateRequest {
    /// CA证书识别码。
    #[serde(rename = "ParentIdentifier")]
    pub parent_identifier: String,
    /// CSR内容。您可以通过OpenSSL工具或者Keytool工具生成CSR。更多信息，请参见[如何制作CSR文件](~~42218~~)。
    #[serde(rename = "Csr")]
    pub csr: String,
    /// 证书有效期。不可超过实例有效期。支持使用相对时间和绝对时间。
    #[serde(rename = "Validity")]
    pub validity: String,
    /// 透传参数。
    #[serde(rename = "ApiPassthrough")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_passthrough: Option<CreateCustomCertificateRequestApiPassthrough>,
    /// 立即获取证书。
    #[serde(rename = "Immediately")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub immediately: Option<i32>,
    /// 是否包含CRL地址
    #[serde(rename = "EnableCrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_crl: Option<i64>,
    /// 标签列表。
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<CreateCustomCertificateRequestTagsItem>>,
    /// 资源组ID。此ID可通过调用[ListResources](~~2716559~~)接口获取。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 用户自定义标识。
    #[serde(rename = "customIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_identifier: Option<String>,
}

impl CreateCustomCertificateRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("ParentIdentifier".to_string(), self.parent_identifier.to_string()));
        params.push(("Csr".to_string(), self.csr.to_string()));
        params.push(("Validity".to_string(), self.validity.to_string()));
        if let Some(ref v) = self.api_passthrough {
            for (k, v2) in v.to_query_params() {
                params.push((format!("ApiPassthrough.{}", k), v2));
            }
        }
        if let Some(ref v) = self.immediately {
            params.push(("Immediately".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_crl {
            params.push(("EnableCrl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tags.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.custom_identifier {
            params.push(("customIdentifier".to_string(), v.to_string()));
        }
        params
    }
}

/// OpenApiResponseV1<EnrollCertificateResultV2>
#[derive(Debug, Clone, Deserialize)]
pub struct CreateCustomCertificateResponse {
    /// 证书唯一标识。
    #[serde(rename = "Identifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// 证书内容。
    #[serde(rename = "Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    /// CA证书链。
    #[serde(rename = "CertificateChain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_chain: Option<String>,
    /// 证书序列号。
    #[serde(rename = "SerialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreateExternalCACertificate 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateExternalCACertificateRequest {
    /// 需要启用的外部子CA实例ID
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 证书签名请求。可包含CA证书的SubjectDN、CA证书自定义扩展项等。SubjectKeyIdentifier、AuthorityKeyIdentifier、CRLDistributionP...
    #[serde(rename = "Csr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csr: Option<String>,
    /// 证书有效期。支持使用相对时间和绝对时间。
    #[serde(rename = "Validity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity: Option<String>,
    /// 通过API参数覆盖CSR内容或添加到CA证书中。
    #[serde(rename = "ApiPassthrough")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_passthrough: Option<CreateExternalCACertificateRequestApiPassthrough>,
    /// 标签列表。
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<CreateExternalCACertificateRequestTagsItem>>,
    /// 资源分组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl CreateExternalCACertificateRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.csr {
            params.push(("Csr".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.validity {
            params.push(("Validity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.api_passthrough {
            for (k, v2) in v.to_query_params() {
                params.push((format!("ApiPassthrough.{}", k), v2));
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
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

/// OpenApiResponse<CreateCaCertificateResult>
#[derive(Debug, Clone, Deserialize)]
pub struct CreateExternalCACertificateResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 证书唯一标识。
    #[serde(rename = "Identifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// 证书内容。
    #[serde(rename = "Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    /// CA证书链。
    #[serde(rename = "CertificateChain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_chain: Option<String>,
}

/// CreateRevokeClientCertificate 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateRevokeClientCertificateRequest {
    /// 要吊销的客户端证书或服务端证书的唯一识别码。
    #[serde(rename = "Identifier")]
    pub identifier: String,
}

impl CreateRevokeClientCertificateRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Identifier".to_string(), self.identifier.to_string()));
        params
    }
}

/// SuccessResponse
#[derive(Debug, Clone, Deserialize)]
pub struct CreateRevokeClientCertificateResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreateRootCACertificate 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateRootCACertificateRequest {
    /// 组织机构的通用名称或简称。支持使用中文、英文字符等。
    #[serde(rename = "CommonName")]
    pub common_name: String,
    /// 组织机构下部门或分支的名称。支持使用中文、英文字符等。
    #[serde(rename = "OrganizationUnit")]
    pub organization_unit: String,
    /// 根CA证书关联的组织机构（对应您的企业或单位）的名称。支持使用中文、英文字符等。
    #[serde(rename = "Organization")]
    pub organization: String,
    /// 组织机构所在城市的名称。支持使用中文、英文字符等。
    #[serde(rename = "Locality")]
    pub locality: String,
    /// <props="china">组织机构所在省份、直辖市或自治区的名称。支持使用中文、英文字符等。</props>
    #[serde(rename = "State")]
    pub state: String,
    /// 组织机构所在国家或地区的代码，使用两位大写英文字母缩写表示。例如，**CN**表示中国，**US**表示美国。
    #[serde(rename = "CountryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// 根CA证书的密钥算法类型。密钥算法使用`<加密算法>_<密钥长度>`格式表示。取值：
    #[serde(rename = "Algorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    /// 根CA证书的有效期，单位：年。
    #[serde(rename = "Years")]
    pub years: i32,
    /// 客户端Token，用于保证请求的幂等性。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// 标签列表。
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<CreateRootCACertificateRequestTagsItem>>,
    /// 资源分组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl CreateRootCACertificateRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("CommonName".to_string(), self.common_name.to_string()));
        params.push(("OrganizationUnit".to_string(), self.organization_unit.to_string()));
        params.push(("Organization".to_string(), self.organization.to_string()));
        params.push(("Locality".to_string(), self.locality.to_string()));
        params.push(("State".to_string(), self.state.to_string()));
        if let Some(ref v) = self.country_code {
            params.push(("CountryCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.algorithm {
            params.push(("Algorithm".to_string(), v.to_string()));
        }
        params.push(("Years".to_string(), self.years.to_string()));
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tags.{}", i + 1);
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

/// 对象。
#[derive(Debug, Clone, Deserialize)]
pub struct CreateRootCACertificateResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 本次请求创建的根CA证书的唯一识别码。
    #[serde(rename = "Identifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// 本次调用创建的PEM格式根证书。
    #[serde(rename = "Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    /// 本次调用创建的根证书的CA证书链。
    #[serde(rename = "CertificateChain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_chain: Option<String>,
}

/// CreateServerCertificate 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateServerCertificateRequest {
    /// 服务端证书的扩展域名、扩展IP地址。为证书添加扩展信息后，您可以将证书应用到多个域名、IP地址。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 机构名称，默认：Alibaba Inc。
    #[serde(rename = "Organization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// 部门名称，默认：Aliyun CDN。
    #[serde(rename = "OrganizationUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_unit: Option<String>,
    /// 国家代码，如CN，US。
    #[serde(rename = "Country")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// 表示证书使用者的名称。服务端认证（ServerAuth）证书使用者是服务器，建议填写服务器绑定的域名或IP。
    #[serde(rename = "CommonName")]
    pub common_name: String,
    /// <props="china">设置证书组织机构所在省份、直辖市或自治区的名称。支持使用中文、英文字符等。默认为签发该证书的子CA证书组织机构所在省份、直辖市或自治区的名称。</props>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// 设置证书组织机构所在城市的名称。支持使用中文、英文字符等。
    #[serde(rename = "Locality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    /// 服务端证书的密钥算法。密钥算法使用`<加密算法>_<密钥长度>`格式表示。取值：
    #[serde(rename = "Algorithm")]
    pub algorithm: String,
    /// 签发该证书的子CA证书的唯一识别码。
    #[serde(rename = "ParentIdentifier")]
    pub parent_identifier: String,
    /// 证书有效期。单位：年。
    #[serde(rename = "Years")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub years: Option<i32>,
    /// 证书有效期。单位：月。
    #[serde(rename = "Months")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub months: Option<i32>,
    /// 服务端证书的有效期。单位：天。
    #[serde(rename = "Days")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i32>,
    /// 服务端证书的签发时间，使用时间戳格式，默认为您调用该接口的时间。单位：秒。
    #[serde(rename = "BeforeTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_time: Option<i64>,
    /// 服务端证书的服务到期时间，使用时间戳格式。单位：秒。
    #[serde(rename = "AfterTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_time: Option<i64>,
    /// 立即返回数字证书。
    #[serde(rename = "Immediately")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub immediately: Option<i32>,
    /// 是否包含CRL地址
    #[serde(rename = "EnableCrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_crl: Option<i64>,
    /// 标签列表。
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<CreateServerCertificateRequestTagsItem>>,
    /// 资源组ID。此ID可通过调用[ListResources](~~2716559~~)接口获取。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 用户自定义标识，唯一键。
    #[serde(rename = "CustomIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_identifier: Option<String>,
}

impl CreateServerCertificateRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.organization {
            params.push(("Organization".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.organization_unit {
            params.push(("OrganizationUnit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.country {
            params.push(("Country".to_string(), v.to_string()));
        }
        params.push(("CommonName".to_string(), self.common_name.to_string()));
        if let Some(ref v) = self.state {
            params.push(("State".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.locality {
            params.push(("Locality".to_string(), v.to_string()));
        }
        params.push(("Algorithm".to_string(), self.algorithm.to_string()));
        params.push(("ParentIdentifier".to_string(), self.parent_identifier.to_string()));
        if let Some(ref v) = self.years {
            params.push(("Years".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.months {
            params.push(("Months".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.days {
            params.push(("Days".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.before_time {
            params.push(("BeforeTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.after_time {
            params.push(("AfterTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.immediately {
            params.push(("Immediately".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_crl {
            params.push(("EnableCrl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tags.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.custom_identifier {
            params.push(("CustomIdentifier".to_string(), v.to_string()));
        }
        params
    }
}

/// CreateCertificateResponse<CertificateIdentifierWithParentDTO>
#[derive(Debug, Clone, Deserialize)]
pub struct CreateServerCertificateResponse {
    /// 服务端证书的内容。
    #[serde(rename = "X509Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x509_certificate: Option<String>,
    /// CA证书链。
    #[serde(rename = "CertificateChain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_chain: Option<String>,
    /// 服务端证书的唯一识别码。
    #[serde(rename = "Identifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// 证书序列号。
    #[serde(rename = "SerialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreateServerCertificateWithCsr 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateServerCertificateWithCsrRequest {
    /// CSR内容。
    #[serde(rename = "Csr")]
    pub csr: String,
    /// 服务端证书的扩展域名或者扩展IP地址。为证书添加扩展信息后，您可以将证书应用到多个域名或者IP地址。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 机构名称，默认：Alibaba Inc。
    #[serde(rename = "Organization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// 部门名称，默认：Aliyun CDN。
    #[serde(rename = "OrganizationUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_unit: Option<String>,
    /// 国家代码，例如**CN**。
    #[serde(rename = "Country")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// 设置证书的公用名。支持使用中文、英文字符等。
    #[serde(rename = "CommonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_name: Option<String>,
    /// <props="china">设置证书组织机构所在省份、直辖市或自治区的名称。支持使用中文、英文字符等。默认为签发该证书的子CA证书组织机构所在省份、直辖市或自治区的名称。</props>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// 设置证书组织机构所在城市的名称。支持使用中文、英文字符等。
    #[serde(rename = "Locality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    /// 服务端证书的密钥算法。密钥算法使用`<加密算法>_<密钥长度>`格式表示。取值：
    #[serde(rename = "Algorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    /// 签发该证书的子CA证书的唯一识别码。
    #[serde(rename = "ParentIdentifier")]
    pub parent_identifier: String,
    /// 证书有效期。单位：年。
    #[serde(rename = "Years")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub years: Option<i32>,
    /// 证书有效期。单位：月。
    #[serde(rename = "Months")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub months: Option<i32>,
    /// 服务端证书的有效期。单位：天。
    #[serde(rename = "Days")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i32>,
    /// 服务端证书的签发时间，使用时间戳格式，默认为您调用该接口的时间。单位：秒。
    #[serde(rename = "BeforeTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_time: Option<i64>,
    /// 服务端证书的服务到期时间，使用时间戳格式。单位：秒。
    #[serde(rename = "AfterTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_time: Option<i64>,
    /// 立即返回数字证书。
    #[serde(rename = "Immediately")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub immediately: Option<i32>,
    /// 是否包含CRL地址
    #[serde(rename = "EnableCrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_crl: Option<i64>,
    /// 标签列表。
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<CreateServerCertificateWithCsrRequestTagsItem>>,
    /// 资源分组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 用户自定义标识，唯一键。
    #[serde(rename = "CustomIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_identifier: Option<String>,
}

impl CreateServerCertificateWithCsrRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Csr".to_string(), self.csr.to_string()));
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.organization {
            params.push(("Organization".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.organization_unit {
            params.push(("OrganizationUnit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.country {
            params.push(("Country".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.common_name {
            params.push(("CommonName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.state {
            params.push(("State".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.locality {
            params.push(("Locality".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.algorithm {
            params.push(("Algorithm".to_string(), v.to_string()));
        }
        params.push(("ParentIdentifier".to_string(), self.parent_identifier.to_string()));
        if let Some(ref v) = self.years {
            params.push(("Years".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.months {
            params.push(("Months".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.days {
            params.push(("Days".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.before_time {
            params.push(("BeforeTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.after_time {
            params.push(("AfterTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.immediately {
            params.push(("Immediately".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_crl {
            params.push(("EnableCrl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tags.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.custom_identifier {
            params.push(("CustomIdentifier".to_string(), v.to_string()));
        }
        params
    }
}

/// CreateCertificateResponse<CertificateIdentifierWithParentDTO>
#[derive(Debug, Clone, Deserialize)]
pub struct CreateServerCertificateWithCsrResponse {
    /// 服务端证书的内容。
    #[serde(rename = "X509Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x509_certificate: Option<String>,
    /// CA证书链。
    #[serde(rename = "CertificateChain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_chain: Option<String>,
    /// 服务端证书的唯一识别码。
    #[serde(rename = "Identifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// 证书序列号。
    #[serde(rename = "SerialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreateSubCACertificate 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateSubCACertificateRequest {
    /// 根CA证书的唯一识别码。
    #[serde(rename = "ParentIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_identifier: Option<String>,
    /// 组织机构的通用名称或简称。支持使用中文、英文字符等。
    #[serde(rename = "CommonName")]
    pub common_name: String,
    /// 组织机构下部门或分支的名称。支持使用中文、英文字符等。
    #[serde(rename = "OrganizationUnit")]
    pub organization_unit: String,
    /// 子CA证书关联的组织机构（对应您的企业或单位）的名称。支持使用中文、英文字符等。
    #[serde(rename = "Organization")]
    pub organization: String,
    /// 组织机构所在城市的名称。支持使用中文、英文字符等。
    #[serde(rename = "Locality")]
    pub locality: String,
    /// <props="china">组织机构所在省份、直辖市或自治区的名称。支持使用中文、英文字符等。</props>
    #[serde(rename = "State")]
    pub state: String,
    /// 组织机构所在国家或地区的代码，使用两位或三位大写英文字母缩写表示。例如，**CN**表示中国，**US**表示美国。
    #[serde(rename = "CountryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// 子CA证书的密钥算法类型。密钥算法使用`<加密算法>_<密钥长度>`格式表示。取值：
    #[serde(rename = "Algorithm")]
    pub algorithm: String,
    /// 子CA证书的有效期，单位：年。
    #[serde(rename = "Years")]
    pub years: i32,
    /// 证书路径长度限制，默认0。
    #[serde(rename = "PathLenConstraint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_len_constraint: Option<i32>,
    /// 扩展密钥用法
    #[serde(rename = "ExtendedKeyUsages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_key_usages: Option<Vec<String>>,
    /// 是否启用CRL服务
    #[serde(rename = "EnableCrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_crl: Option<bool>,
    /// CRL有效期 1-365天
    #[serde(rename = "CrlDay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crl_day: Option<i32>,
    /// 标签列表。
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<CreateSubCACertificateRequestTagsItem>>,
    /// 资源分组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl CreateSubCACertificateRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.parent_identifier {
            params.push(("ParentIdentifier".to_string(), v.to_string()));
        }
        params.push(("CommonName".to_string(), self.common_name.to_string()));
        params.push(("OrganizationUnit".to_string(), self.organization_unit.to_string()));
        params.push(("Organization".to_string(), self.organization.to_string()));
        params.push(("Locality".to_string(), self.locality.to_string()));
        params.push(("State".to_string(), self.state.to_string()));
        if let Some(ref v) = self.country_code {
            params.push(("CountryCode".to_string(), v.to_string()));
        }
        params.push(("Algorithm".to_string(), self.algorithm.to_string()));
        params.push(("Years".to_string(), self.years.to_string()));
        if let Some(ref v) = self.path_len_constraint {
            params.push(("PathLenConstraint".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.extended_key_usages {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("ExtendedKeyUsages.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.enable_crl {
            params.push(("EnableCrl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.crl_day {
            params.push(("CrlDay".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tags.{}", i + 1);
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

/// 对象。
#[derive(Debug, Clone, Deserialize)]
pub struct CreateSubCACertificateResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 本次请求创建的子CA证书的唯一识别码。
    #[serde(rename = "Identifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// 本次调用创建的PEM格式证书。
    #[serde(rename = "Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    /// 本次调用创建的证书的CA证书链。
    #[serde(rename = "CertificateChain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_chain: Option<String>,
}

/// DeleteClientCertificate 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteClientCertificateRequest {
    /// 要删除的客户端证书或服务端证书的唯一识别码。证书的当前状态必须是**REVOKE**。
    #[serde(rename = "Identifier")]
    pub identifier: String,
}

impl DeleteClientCertificateRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Identifier".to_string(), self.identifier.to_string()));
        params
    }
}

/// SuccessResponse
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteClientCertificateResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeCACertificate 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCACertificateRequest {
    /// 要查询的CA证书的唯一识别码。
    #[serde(rename = "Identifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}

impl DescribeCACertificateRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.identifier {
            params.push(("Identifier".to_string(), v.to_string()));
        }
        params
    }
}

/// DescribeCertificateResponse<CaCertificateDTO>。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeCACertificateResponse {
    /// CA证书的详细信息。
    #[serde(rename = "Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<DescribeCACertificateResponseCertificate>,
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// CA证书的有效期，单位：年。
    #[serde(rename = "Years")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub years: Option<i32>,
}

/// DescribeCACertificateCount 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCACertificateCountRequest {
}

impl DescribeCACertificateCountRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 对象。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeCACertificateCountResponse {
    /// 已创建的CA证书（包括根CA证书、子CA证书）的数量。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeCACertificateList 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCACertificateListRequest {
    /// 分页查询时，设置当前页面的页码。默认值为**1**。
    #[serde(rename = "CurrentPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_page: Option<i32>,
    /// 分页查询时，设置每页包含CA证书的数量。默认值为**20**。
    #[serde(rename = "ShowSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_size: Option<i32>,
    /// CA证书的唯一识别码。
    #[serde(rename = "Identifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// CA的类型 。取值：
    #[serde(rename = "CertType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_type: Option<String>,
    /// 当前的CA状态，取值：
    #[serde(rename = "CaStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_status: Option<String>,
    /// CA时间状态。取值：
    #[serde(rename = "ValidStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_status: Option<String>,
    /// 签发CA的机构。取值：
    #[serde(rename = "IssuerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer_type: Option<String>,
    /// 资源组ID。此ID可通过调用[ListResources](~~2716559~~)接口获取。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl DescribeCACertificateListRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.current_page {
            params.push(("CurrentPage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.show_size {
            params.push(("ShowSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.identifier {
            params.push(("Identifier".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_type {
            params.push(("CertType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ca_status {
            params.push(("CaStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.valid_status {
            params.push(("ValidStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.issuer_type {
            params.push(("IssuerType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

/// 对象。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeCACertificateListResponse {
    /// 当前页面的页码。
    #[serde(rename = "CurrentPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_page: Option<i32>,
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 查询到的根CA证书和子CA证书的总数量。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 返回结果的页数。
    #[serde(rename = "PageCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_count: Option<i32>,
    /// 每页包含CA证书的数量。
    #[serde(rename = "ShowSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_size: Option<i32>,
    /// CA证书的详情列表。
    #[serde(rename = "CertificateList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_list: Option<Vec<DescribeCACertificateListResponseCertificateListItem>>,
}

/// DescribeCertificatePrivateKey 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCertificatePrivateKeyRequest {
    /// 要获取私钥的客户端证书或服务端证书的唯一识别码。
    #[serde(rename = "Identifier")]
    pub identifier: String,
    /// 设置用于加密私钥的密码。支持使用英文大小写字母、数字、特殊字符（例如，`,.+-_#`）等。最大长度为32字节。
    #[serde(rename = "EncryptedCode")]
    pub encrypted_code: String,
    /// 证书所属资源组的ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl DescribeCertificatePrivateKeyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Identifier".to_string(), self.identifier.to_string()));
        params.push(("EncryptedCode".to_string(), self.encrypted_code.to_string()));
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

/// GetCertificatePrivateKeyResponse。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeCertificatePrivateKeyResponse {
    /// 加密后的私钥内容。
    #[serde(rename = "EncryptedData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted_data: Option<String>,
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeClientCertificate 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeClientCertificateRequest {
    /// 要查询的客户端证书或服务端证书的唯一识别码。
    #[serde(rename = "Identifier")]
    pub identifier: String,
}

impl DescribeClientCertificateRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Identifier".to_string(), self.identifier.to_string()));
        params
    }
}

/// DescribeCertificateResponse<CertificateWithContentDTO>
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeClientCertificateResponse {
    /// 客户端证书或服务端证书的详细信息。
    #[serde(rename = "Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<DescribeClientCertificateResponseCertificate>,
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeClientCertificateForSerialNumber 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeClientCertificateForSerialNumberRequest {
    /// 要查询的客户端证书或服务端证书的序列号。多个序列号之间使用半角逗号（,）分隔。
    #[serde(rename = "SerialNumber")]
    pub serial_number: String,
}

impl DescribeClientCertificateForSerialNumberRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("SerialNumber".to_string(), self.serial_number.to_string()));
        params
    }
}

/// ListCertificateResponse<CertificateWithContentDTO>
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeClientCertificateForSerialNumberResponse {
    /// 客户端证书或服务端证书的详细信息。
    #[serde(rename = "CertificateList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_list: Option<Vec<DescribeClientCertificateForSerialNumberResponseCertificateListItem>>,
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeClientCertificateStatus 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeClientCertificateStatusRequest {
    /// 要查询的客户端证书或服务端证书的唯一识别码。多个证书识别码之间使用半角逗号（,）分隔。
    #[serde(rename = "Identifier")]
    pub identifier: String,
}

impl DescribeClientCertificateStatusRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Identifier".to_string(), self.identifier.to_string()));
        params
    }
}

/// 对象。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeClientCertificateStatusResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 证书状态的详细信息。
    #[serde(rename = "CertificateStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_status: Option<Vec<DescribeClientCertificateStatusResponseCertificateStatusItem>>,
}

/// DescribeClientCertificateStatusForSerialNumber 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeClientCertificateStatusForSerialNumberRequest {
    /// 要查询的客户端证书或服务端证书的序列号。多个证书序列号之间使用半角逗号（,）分隔。
    #[serde(rename = "SerialNumber")]
    pub serial_number: String,
}

impl DescribeClientCertificateStatusForSerialNumberRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("SerialNumber".to_string(), self.serial_number.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeClientCertificateStatusForSerialNumberResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 证书状态的详细信息。
    #[serde(rename = "CertificateStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_status: Option<Vec<DescribeClientCertificateStatusForSerialNumberResponseCertificateStatusItem>>,
}

/// DescribePcaAndExternalCACertificateList 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribePcaAndExternalCACertificateListRequest {
    /// 当前页码。
    #[serde(rename = "CurrentPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_page: Option<i32>,
    /// 指定每页显示多少条记录。默认值为50。
    #[serde(rename = "ShowSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_size: Option<i32>,
}

impl DescribePcaAndExternalCACertificateListRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.current_page {
            params.push(("CurrentPage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.show_size {
            params.push(("ShowSize".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribePcaAndExternalCACertificateListResponse {
    /// 当前页码。
    #[serde(rename = "CurrentPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_page: Option<i32>,
    /// 结果的请求id。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 总记录数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 列表条目数。
    #[serde(rename = "PageCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_count: Option<i32>,
    /// 指定每页显示多少条记录。默认值为50。
    #[serde(rename = "ShowSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_size: Option<i32>,
    /// 证书显示列表。
    #[serde(rename = "CertificateList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_list: Option<Vec<DescribePcaAndExternalCACertificateListResponseCertificateListItem>>,
}

/// GetCAInstanceStatus 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetCAInstanceStatusRequest {
    /// 要查询的私有CA实例的ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 要查询的客户端证书或服务端证书的唯一识别码。
    #[serde(rename = "Identifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}

impl GetCAInstanceStatusRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.identifier {
            params.push(("Identifier".to_string(), v.to_string()));
        }
        params
    }
}

/// 对象。
#[derive(Debug, Clone, Deserialize)]
pub struct GetCAInstanceStatusResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 私有CA实例的状态详情。
    #[serde(rename = "InstanceStatusList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_status_list: Option<Vec<GetCAInstanceStatusResponseInstanceStatusListItem>>,
}

/// ListAllEndEntityInstance 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListAllEndEntityInstanceRequest {
    /// 父实例id。
    #[serde(rename = "ParentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<i64>,
    /// 指定显示返回结果中的收费类型信息，取值：-**0**：不返回。-**1**：返回。
    #[serde(rename = "RecursiveChildren")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive_children: Option<i32>,
    /// 实例列表的页码。
    #[serde(rename = "CurrentPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_page: Option<i32>,
    /// 分页查询时，显示的每页数据的最大条数。
    #[serde(rename = "ShowSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_size: Option<i32>,
    /// 本次读取的最大数据量。
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// 下一个查询开始Token，NextToken为空说明没有下一个。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl ListAllEndEntityInstanceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.parent_id {
            params.push(("ParentId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.recursive_children {
            params.push(("RecursiveChildren".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.current_page {
            params.push(("CurrentPage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.show_size {
            params.push(("ShowSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_results {
            params.push(("MaxResults".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.next_token {
            params.push(("NextToken".to_string(), v.to_string()));
        }
        params
    }
}

/// OpenApiResponse<PagedResultData<Map<String, Object>>>
#[derive(Debug, Clone, Deserialize)]
pub struct ListAllEndEntityInstanceResponse {
    /// 请求id。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 总记录数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 总页数。
    #[serde(rename = "PageCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_count: Option<i32>,
    /// 实例列表的页码。
    #[serde(rename = "CurrentPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_page: Option<i32>,
    /// 分页查询时，显示的每页数据的最大条数。
    #[serde(rename = "ShowSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_size: Option<i32>,
    /// 实例列表。
    #[serde(rename = "List")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<serde_json::Value>>,
    /// 下一个查询开始Token，NextToken为空说明没有下一个。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 本次读取的最大数据量。
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
}

/// ListCACertificateLog 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListCACertificateLogRequest {
    /// 要查询的CA证书的唯一识别码。
    #[serde(rename = "Identifier")]
    pub identifier: String,
}

impl ListCACertificateLogRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Identifier".to_string(), self.identifier.to_string()));
        params
    }
}

/// 对象。
#[derive(Debug, Clone, Deserialize)]
pub struct ListCACertificateLogResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// CA证书的操作日志列表。
    #[serde(rename = "LogList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_list: Option<Vec<ListCACertificateLogResponseLogListItem>>,
}

/// ListCert 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListCertRequest {
    /// 实例的uuid。
    #[serde(rename = "InstanceUuid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_uuid: Option<String>,
    /// 证书的修改时间。
    #[serde(rename = "BeforeDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_date: Option<String>,
    /// 证书绑定的主机记录。
    #[serde(rename = "AfterDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_date: Option<String>,
    /// 证书状态。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 证书类型。取值：
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 当前页的页码。
    #[serde(rename = "CurrentPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_page: Option<i32>,
    /// 证书的总大小。单位：字节。
    #[serde(rename = "ShowSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_size: Option<i32>,
    /// 本次读取的最大数据量。
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// 下一个查询开始Token，NextToken为空说明没有下一个。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// CA标识。标识签发证书的中间CA。
    #[serde(rename = "ParentIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_identifier: Option<String>,
}

impl ListCertRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.instance_uuid {
            params.push(("InstanceUuid".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.before_date {
            params.push(("BeforeDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.after_date {
            params.push(("AfterDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.current_page {
            params.push(("CurrentPage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.show_size {
            params.push(("ShowSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_results {
            params.push(("MaxResults".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.next_token {
            params.push(("NextToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.parent_identifier {
            params.push(("ParentIdentifier".to_string(), v.to_string()));
        }
        params
    }
}

/// OpenApiResponse<PagedResultData<CertificateMeta>>
#[derive(Debug, Clone, Deserialize)]
pub struct ListCertResponse {
    /// 证书总数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// 页码数。
    #[serde(rename = "PageCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_count: Option<i32>,
    /// 当前页码。
    #[serde(rename = "CurrentPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_page: Option<i32>,
    /// 证书的总大小。单位：字节。
    #[serde(rename = "ShowSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_size: Option<i32>,
    /// 证书所属数据源id。
    #[serde(rename = "List")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<ListCertResponseListItem>>,
    /// 请求id。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 下一个查询开始Token，NextToken为空说明没有下一个。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 本次读取的最大数据量。
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
}

/// ListClientCertificate 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListClientCertificateRequest {
    /// 分页查询时，设置当前页面的页码。默认值为**1**。
    #[serde(rename = "CurrentPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_page: Option<i32>,
    /// 分页查询时，设置每页显示证书的数量。默认值为**20**。
    #[serde(rename = "ShowSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_size: Option<i32>,
    /// 要查询的客户端证书或服务端证书的唯一识别码。
    #[serde(rename = "Identifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// 资源组ID。此ID可通过调用[ListResources](~~2716559~~)接口获取。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl ListClientCertificateRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.current_page {
            params.push(("CurrentPage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.show_size {
            params.push(("ShowSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.identifier {
            params.push(("Identifier".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

/// ListCertificateResponse2<CertificateWithContentDTO>
#[derive(Debug, Clone, Deserialize)]
pub struct ListClientCertificateResponse {
    /// 客户端证书或服务端证书的详情列表。
    #[serde(rename = "CertificateList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_list: Option<Vec<ListClientCertificateResponseCertificateListItem>>,
    /// 当前页面的页码。
    #[serde(rename = "CurrentPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_page: Option<i32>,
    /// 总页数。
    #[serde(rename = "PageCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_count: Option<i32>,
    /// 每页显示证书的数量。
    #[serde(rename = "ShowSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_size: Option<i32>,
    /// 查询到的客户端证书和服务端证书的总数量。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 分页参数：结果集的最大数量。
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
}

/// ListPcaCaCertificate 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListPcaCaCertificateRequest {
    /// 分页参数：页面令牌。请求置空表示从头开始。返回为空时表示最后一页。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 分页参数：结果集的最大数量，默认值为20。
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
}

impl ListPcaCaCertificateRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.next_token {
            params.push(("NextToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_results {
            params.push(("MaxResults".to_string(), v.to_string()));
        }
        params
    }
}

/// NextTokenWithTotalCountPaginationResponse<ListCaCertificateResult>
#[derive(Debug, Clone, Deserialize)]
pub struct ListPcaCaCertificateResponse {
    /// 蜜罐探针数据。
    #[serde(rename = "List")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<ListPcaCaCertificateResponseListItem>>,
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 结果集总数量。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// 分页参数：页面令牌。请求置空表示从头开始。返回为空时表示最后一页。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 分页参数：结果集的最大数量。
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
}

/// ListRevokeCertificate 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListRevokeCertificateRequest {
    /// 分页查询时，设置当前页面的页码。默认值为**1**。
    #[serde(rename = "CurrentPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_page: Option<i32>,
    /// 分页查询时，设置每页包含已被吊销证书的数量。默认值为**20**。
    #[serde(rename = "ShowSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_size: Option<i32>,
}

impl ListRevokeCertificateRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.current_page {
            params.push(("CurrentPage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.show_size {
            params.push(("ShowSize".to_string(), v.to_string()));
        }
        params
    }
}

/// ListCertificateResponse2<RevokedCertificateDTO>。
#[derive(Debug, Clone, Deserialize)]
pub struct ListRevokeCertificateResponse {
    /// 已被吊销的客户端证书或服务端证书的详细信息。
    #[serde(rename = "CertificateList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_list: Option<Vec<ListRevokeCertificateResponseCertificateListItem>>,
    /// 当前页面的页码。
    #[serde(rename = "CurrentPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_page: Option<i32>,
    /// 总页数。
    #[serde(rename = "PageCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_count: Option<i32>,
    /// 每页包含已被吊销证书的数量。
    #[serde(rename = "ShowSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_size: Option<i32>,
    /// 已被吊销的客户端证书和服务端证书的总数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListTagResources 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListTagResourcesRequest {
    /// 地域id。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 资源类型。默认值：**instance**
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// 下一个查询开始token，nexttoken为空说明没有下一个。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 资源id。
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<Vec<String>>,
    /// 查询到的实例和标签的信息。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<ListTagResourcesRequestTagItem>>,
    /// 本次读取的最大数据量。
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
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
        if let Some(ref v) = self.max_results {
            params.push(("MaxResults".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListTagResourcesResponse {
    /// 请求id。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 请求接口返回的数据。
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ListTagResourcesResponseData>,
}

/// MoveResourceGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct MoveResourceGroupRequest {
    /// 要迁移的资源组所属的资源id。
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// 资源组id。
    #[serde(rename = "ResourceGroupId")]
    pub resource_group_id: String,
    /// 资源类型，取值：**instance**。
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// 地域id。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
}

impl MoveResourceGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("ResourceId".to_string(), self.resource_id.to_string()));
        params.push(("ResourceGroupId".to_string(), self.resource_group_id.to_string()));
        params.push(("ResourceType".to_string(), self.resource_type.to_string()));
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct MoveResourceGroupResponse {
    /// 请求id。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// TagResources 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct TagResourcesRequest {
    /// 证书所有者所属组织的区域。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 资源类型。默认值：**instance**
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// 资源id。
    #[serde(rename = "ResourceId")]
    pub resource_id: Vec<String>,
    /// 资源的标签信息。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<TagResourcesRequestTagItem>>,
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
pub struct TagResourcesResponse {
    /// 请求id。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// UntagResources 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UntagResourcesRequest {
    /// 地域id。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 资源类型定义。取值固定为**instance**。
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// 是否全部删除，仅当tagkey.n为空时有效。取值范围：-true-false默认值：false
    #[serde(rename = "All")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
    /// 资源id。
    #[serde(rename = "ResourceId")]
    pub resource_id: Vec<String>,
    /// 标签键。
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
    /// 请求id。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// UpdateCACertificateStatus 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateCACertificateStatusRequest {
    /// 要修改状态的CA证书的唯一识别码。
    #[serde(rename = "Identifier")]
    pub identifier: String,
    /// 对该CA证书执行的操作。取值固定为**REVOKE**，表示吊销CA证书，即将CA证书的状态修改为**REVOKE**。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 客户端Token，用于保证请求的幂等性。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

impl UpdateCACertificateStatusRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Identifier".to_string(), self.identifier.to_string()));
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        params
    }
}

/// 对象。
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateCACertificateStatusResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// UpdatePcaCertificate 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdatePcaCertificateRequest {
    /// 设置签发证书的名称。
    #[serde(rename = "AliasName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_name: Option<String>,
    /// 要查询的CA证书的唯一识别码。
    #[serde(rename = "Identifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// 标签列表。
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<UpdatePcaCertificateRequestTagsItem>>,
    /// 资源分组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl UpdatePcaCertificateRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.alias_name {
            params.push(("AliasName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.identifier {
            params.push(("Identifier".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tags.{}", i + 1);
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

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct UpdatePcaCertificateResponse {
    /// Id of the request
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 鉴权报错信息
    #[serde(rename = "AccessDeniedDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_denied_detail: Option<UpdatePcaCertificateResponseAccessDeniedDetail>,
}

/// UploadPcaCertToCas 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UploadPcaCertToCasRequest {
    /// 待同步到证书服务的主键标识列表。多个ID使用半角逗号（,）间隔。
    #[serde(rename = "Ids")]
    pub ids: String,
}

impl UploadPcaCertToCasRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Ids".to_string(), self.ids.to_string()));
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct UploadPcaCertToCasResponse {
    /// Id of the request
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
