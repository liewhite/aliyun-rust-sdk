//! API 调用 - 自动生成，请勿手动修改

use aliyun_sdk_core::{Client, ClientConfig, ApiRequest, HttpMethod, SdkError};
use super::types::*;

/// Sts API 版本
pub const API_VERSION: &str = "2015-04-01";

/// Sts 客户端
#[derive(Debug, Clone)]
pub struct StsClient {
    client: Client,
}

impl StsClient {
    /// 创建新客户端
    pub fn new(config: ClientConfig) -> Result<Self, SdkError> {
        Ok(Self {
            client: Client::new(config)?,
        })
    }

    /// 通过调用AssumeRole接口，获取一个扮演RAM角色的临时身份凭证（STS Token）。
    pub async fn assume_role(
        &self,
        request: AssumeRoleRequest,
    ) -> Result<AssumeRoleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AssumeRole",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 进行SAML角色SSO时，通过调用AssumeRoleWithSAML接口，获取扮演RAM角色的临时身份凭证（STS Token）。
    pub async fn assume_role_with_saml(
        &self,
        request: AssumeRoleWithSAMLRequest,
    ) -> Result<AssumeRoleWithSAMLResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AssumeRoleWithSAML",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: false,
        };
        self.client.request(api_request).await
    }

    /// 进行OIDC角色SSO时，通过调用AssumeRoleWithOIDC接口，获取扮演RAM角色的临时身份凭证（STS Token）。
    pub async fn assume_role_with_oidc(
        &self,
        request: AssumeRoleWithOIDCRequest,
    ) -> Result<AssumeRoleWithOIDCResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AssumeRoleWithOIDC",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: false,
        };
        self.client.request(api_request).await
    }

    /// 调用GetCallerIdentity获取当前调用者的身份信息。
    pub async fn get_caller_identity(
        &self,
        request: GetCallerIdentityRequest,
    ) -> Result<GetCallerIdentityResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetCallerIdentity",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

}