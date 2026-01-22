//! 类型定义 - 自动生成，请勿手动修改

use serde::{Deserialize, Serialize};

/// 角色扮演时的临时身份。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AssumeRoleResponseAssumedRoleUser {
    /// 临时身份的ID。
    #[serde(rename = "AssumedRoleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assumed_role_id: Option<String>,
    /// 临时身份的ARN。
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

impl AssumeRoleResponseAssumedRoleUser {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.assumed_role_id {
            params.push(("AssumedRoleId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.arn {
            params.push(("Arn".to_string(), v.to_string()));
        }
        params
    }
}

/// 访问凭证。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AssumeRoleResponseCredentials {
    /// 安全令牌。
    #[serde(rename = "SecurityToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_token: Option<String>,
    /// Token到期失效时间（UTC时间）。
    #[serde(rename = "Expiration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    /// 访问密钥。
    #[serde(rename = "AccessKeySecret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_secret: Option<String>,
    /// 访问密钥ID。
    #[serde(rename = "AccessKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
}

impl AssumeRoleResponseCredentials {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.security_token {
            params.push(("SecurityToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.expiration {
            params.push(("Expiration".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.access_key_secret {
            params.push(("AccessKeySecret".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.access_key_id {
            params.push(("AccessKeyId".to_string(), v.to_string()));
        }
        params
    }
}

/// SAML断言中的部分信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AssumeRoleWithSAMLResponseSAMLAssertionInfo {
    /// SAML断言中`NameID`的格式。当前缀为`urn:oasis:names:tc:SAML:2.0:nameid-format:`时，前缀会被移除。例如：`persistent/transi...
    #[serde(rename = "SubjectType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_type: Option<String>,
    /// SAML断言中`Subject - NameID`字段的值。
    #[serde(rename = "Subject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// SAML断言中`Issuer`字段的值。
    #[serde(rename = "Issuer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    /// SAML断言中`Subject - SubjectConfirmation - SubjectConfirmationData`字段中`Recipient`属性的值。
    #[serde(rename = "Recipient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
}

impl AssumeRoleWithSAMLResponseSAMLAssertionInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.subject_type {
            params.push(("SubjectType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.subject {
            params.push(("Subject".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.issuer {
            params.push(("Issuer".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.recipient {
            params.push(("Recipient".to_string(), v.to_string()));
        }
        params
    }
}

/// 角色扮演临时身份。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AssumeRoleWithSAMLResponseAssumedRoleUser {
    /// 临时身份的ID。
    #[serde(rename = "AssumedRoleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assumed_role_id: Option<String>,
    /// 临时身份的ARN。
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

impl AssumeRoleWithSAMLResponseAssumedRoleUser {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.assumed_role_id {
            params.push(("AssumedRoleId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.arn {
            params.push(("Arn".to_string(), v.to_string()));
        }
        params
    }
}

/// 访问凭证。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AssumeRoleWithSAMLResponseCredentials {
    /// 安全令牌。
    #[serde(rename = "SecurityToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_token: Option<String>,
    /// Token到期失效时间（UTC时间）。
    #[serde(rename = "Expiration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    /// 访问密钥。
    #[serde(rename = "AccessKeySecret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_secret: Option<String>,
    /// 访问密钥ID。
    #[serde(rename = "AccessKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
}

impl AssumeRoleWithSAMLResponseCredentials {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.security_token {
            params.push(("SecurityToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.expiration {
            params.push(("Expiration".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.access_key_secret {
            params.push(("AccessKeySecret".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.access_key_id {
            params.push(("AccessKeyId".to_string(), v.to_string()));
        }
        params
    }
}

/// 解析的OIDC Token信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AssumeRoleWithOIDCResponseOIDCTokenInfo {
    /// OIDC主体。
    #[serde(rename = "Subject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// OIDC颁发者URL。
    #[serde(rename = "Issuer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    /// OIDC受众。多个之间用半角逗号（,）分隔。
    #[serde(rename = "ClientIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_ids: Option<String>,
    /// OIDC Token的过期时间。
    #[serde(rename = "ExpirationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    /// OIDC Token的签发时间。
    #[serde(rename = "IssuanceTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuance_time: Option<String>,
    /// OIDC Token的检验信息。更多信息，请参见[管理OIDC身份提供商](~~327123~~)。
    #[serde(rename = "VerificationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_info: Option<String>,
}

impl AssumeRoleWithOIDCResponseOIDCTokenInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.subject {
            params.push(("Subject".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.issuer {
            params.push(("Issuer".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_ids {
            params.push(("ClientIds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.expiration_time {
            params.push(("ExpirationTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.issuance_time {
            params.push(("IssuanceTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.verification_info {
            params.push(("VerificationInfo".to_string(), v.to_string()));
        }
        params
    }
}

/// 角色扮演临时身份。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AssumeRoleWithOIDCResponseAssumedRoleUser {
    /// 临时身份的ID。
    #[serde(rename = "AssumedRoleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assumed_role_id: Option<String>,
    /// 临时身份的ARN。
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

impl AssumeRoleWithOIDCResponseAssumedRoleUser {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.assumed_role_id {
            params.push(("AssumedRoleId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.arn {
            params.push(("Arn".to_string(), v.to_string()));
        }
        params
    }
}

/// 临时访问凭证（STS Token）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AssumeRoleWithOIDCResponseCredentials {
    /// 安全令牌。
    #[serde(rename = "SecurityToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_token: Option<String>,
    /// Token到期失效时间（UTC时间）。
    #[serde(rename = "Expiration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    /// 访问密钥。
    #[serde(rename = "AccessKeySecret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_secret: Option<String>,
    /// 访问密钥ID。
    #[serde(rename = "AccessKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
}

impl AssumeRoleWithOIDCResponseCredentials {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.security_token {
            params.push(("SecurityToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.expiration {
            params.push(("Expiration".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.access_key_secret {
            params.push(("AccessKeySecret".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.access_key_id {
            params.push(("AccessKeyId".to_string(), v.to_string()));
        }
        params
    }
}

/// AssumeRole 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct AssumeRoleRequest {
    /// Token有效期。单位：秒。
    #[serde(rename = "DurationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i64>,
    /// 为STS Token额外添加的一个权限策略，进一步限制STS Token的权限。具体如下：
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    /// 要扮演的RAM角色ARN。
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// 角色会话名称。
    #[serde(rename = "RoleSessionName")]
    pub role_session_name: String,
    /// 角色外部ID。
    #[serde(rename = "ExternalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// 源身份信息。
    #[serde(rename = "SourceIdentity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_identity: Option<String>,
}

impl AssumeRoleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.duration_seconds {
            params.push(("DurationSeconds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy {
            params.push(("Policy".to_string(), v.to_string()));
        }
        params.push(("RoleArn".to_string(), self.role_arn.to_string()));
        params.push(("RoleSessionName".to_string(), self.role_session_name.to_string()));
        if let Some(ref v) = self.external_id {
            params.push(("ExternalId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source_identity {
            params.push(("SourceIdentity".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回参数。
#[derive(Debug, Clone, Deserialize)]
pub struct AssumeRoleResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 角色扮演时的临时身份。
    #[serde(rename = "AssumedRoleUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assumed_role_user: Option<AssumeRoleResponseAssumedRoleUser>,
    /// 访问凭证。
    #[serde(rename = "Credentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<AssumeRoleResponseCredentials>,
    /// 源身份信息。
    #[serde(rename = "SourceIdentity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_identity: Option<String>,
}

/// AssumeRoleWithSAML 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct AssumeRoleWithSAMLRequest {
    /// RAM中创建的SAML身份提供商的ARN。
    #[serde(rename = "SAMLProviderArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saml_provider_arn: Option<String>,
    /// 要扮演的RAM角色的ARN。
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// Base64编码后的SAML断言。
    #[serde(rename = "SAMLAssertion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saml_assertion: Option<String>,
    /// 为STS Token额外添加的一个权限策略，进一步限制STS Token的权限。具体如下：
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    /// Token有效期。单位：秒。
    #[serde(rename = "DurationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i64>,
}

impl AssumeRoleWithSAMLRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.saml_provider_arn {
            params.push(("SAMLProviderArn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.role_arn {
            params.push(("RoleArn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.saml_assertion {
            params.push(("SAMLAssertion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy {
            params.push(("Policy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.duration_seconds {
            params.push(("DurationSeconds".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回参数。
#[derive(Debug, Clone, Deserialize)]
pub struct AssumeRoleWithSAMLResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// SAML断言中的部分信息。
    #[serde(rename = "SAMLAssertionInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saml_assertion_info: Option<AssumeRoleWithSAMLResponseSAMLAssertionInfo>,
    /// 角色扮演临时身份。
    #[serde(rename = "AssumedRoleUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assumed_role_user: Option<AssumeRoleWithSAMLResponseAssumedRoleUser>,
    /// 访问凭证。
    #[serde(rename = "Credentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<AssumeRoleWithSAMLResponseCredentials>,
    /// 源身份信息。
    #[serde(rename = "SourceIdentity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_identity: Option<String>,
}

/// AssumeRoleWithOIDC 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct AssumeRoleWithOIDCRequest {
    /// OIDC身份提供商的ARN。
    #[serde(rename = "OIDCProviderArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oidc_provider_arn: Option<String>,
    /// 需要扮演的RAM角色ARN。
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// 由外部IdP签发的OIDC令牌（OIDC Token）。
    #[serde(rename = "OIDCToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oidc_token: Option<String>,
    /// 为STS Token额外添加的一个权限策略，进一步限制STS Token的权限。具体如下：
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    /// Token有效期。单位：秒。
    #[serde(rename = "DurationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i64>,
    /// 角色会话名称。
    #[serde(rename = "RoleSessionName")]
    pub role_session_name: String,
}

impl AssumeRoleWithOIDCRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.oidc_provider_arn {
            params.push(("OIDCProviderArn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.role_arn {
            params.push(("RoleArn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.oidc_token {
            params.push(("OIDCToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy {
            params.push(("Policy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.duration_seconds {
            params.push(("DurationSeconds".to_string(), v.to_string()));
        }
        params.push(("RoleSessionName".to_string(), self.role_session_name.to_string()));
        params
    }
}

/// 返回参数。
#[derive(Debug, Clone, Deserialize)]
pub struct AssumeRoleWithOIDCResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 解析的OIDC Token信息。
    #[serde(rename = "OIDCTokenInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oidc_token_info: Option<AssumeRoleWithOIDCResponseOIDCTokenInfo>,
    /// 角色扮演临时身份。
    #[serde(rename = "AssumedRoleUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assumed_role_user: Option<AssumeRoleWithOIDCResponseAssumedRoleUser>,
    /// 临时访问凭证（STS Token）。
    #[serde(rename = "Credentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<AssumeRoleWithOIDCResponseCredentials>,
    /// 源身份信息。
    #[serde(rename = "SourceIdentity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_identity: Option<String>,
}

/// GetCallerIdentity 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetCallerIdentityRequest {
}

impl GetCallerIdentityRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 返回参数。
#[derive(Debug, Clone, Deserialize)]
pub struct GetCallerIdentityResponse {
    /// 身份类型。取值：
    #[serde(rename = "IdentityType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_type: Option<String>,
    /// 当前调用者所属阿里云账号ID。
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 身份标识。
    #[serde(rename = "PrincipalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    /// 用户ID。具体如下：
    #[serde(rename = "UserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 当前调用者的ARN。
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// RAM角色ID。
    #[serde(rename = "RoleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
}
