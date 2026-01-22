//! API 调用 - 自动生成，请勿手动修改

use aliyun_sdk_core::{Client, ClientConfig, ApiRequest, HttpMethod, SdkError};
use super::types::*;

/// Kms API 版本
pub const API_VERSION: &str = "2016-01-20";

/// Kms 客户端
#[derive(Debug, Clone)]
pub struct KmsClient {
    client: Client,
}

impl KmsClient {
    /// 创建新客户端
    pub fn new(config: ClientConfig) -> Result<Self, SdkError> {
        Ok(Self {
            client: Client::new(config)?,
        })
    }

    /// 查询当前账号的可用地域列表。
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

    /// 查询当前阿里云账号的密钥管理服务状态。
    pub async fn describe_account_kms_status(
        &self,
        request: DescribeAccountKmsStatusRequest,
    ) -> Result<DescribeAccountKmsStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeAccountKmsStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为当前阿里云账号开通密钥管理服务。
    pub async fn open_kms_service(
        &self,
        request: OpenKmsServiceRequest,
    ) -> Result<OpenKmsServiceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "OpenKmsService",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询 KMS 实例列表。
    pub async fn list_kms_instances(
        &self,
        request: ListKmsInstancesRequest,
    ) -> Result<ListKmsInstancesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListKmsInstances",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 启用一个KMS实例。
    pub async fn connect_kms_instance(
        &self,
        request: ConnectKmsInstanceRequest,
    ) -> Result<ConnectKmsInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ConnectKmsInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询一个KMS实例的详情。
    pub async fn get_kms_instance(
        &self,
        request: GetKmsInstanceRequest,
    ) -> Result<GetKmsInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetKmsInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新KMS实例配置的VPC。
    pub async fn update_kms_instance_bind_vpc(
        &self,
        request: UpdateKmsInstanceBindVpcRequest,
    ) -> Result<UpdateKmsInstanceBindVpcResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateKmsInstanceBindVpc",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 释放KMS按量付费实例。
    pub async fn release_kms_instance(
        &self,
        request: ReleaseKmsInstanceRequest,
    ) -> Result<ReleaseKmsInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ReleaseKmsInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取默认KMS实例。
    pub async fn get_default_kms_instance(
        &self,
        request: GetDefaultKmsInstanceRequest,
    ) -> Result<GetDefaultKmsInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetDefaultKmsInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建一个主密钥。
    pub async fn create_key(
        &self,
        request: CreateKeyRequest,
    ) -> Result<CreateKeyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateKey",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询调用者在调用地域的所有主密钥ID。
    pub async fn list_keys(
        &self,
        request: ListKeysRequest,
    ) -> Result<ListKeysResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListKeys",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询用户主密钥（CMK）详情。
    pub async fn describe_key(
        &self,
        request: DescribeKeyRequest,
    ) -> Result<DescribeKeyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeKey",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新主密钥的描述信息。
    pub async fn update_key_description(
        &self,
        request: UpdateKeyDescriptionRequest,
    ) -> Result<UpdateKeyDescriptionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateKeyDescription",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 启用指定的主密钥进行加解密。
    pub async fn enable_key(
        &self,
        request: EnableKeyRequest,
    ) -> Result<EnableKeyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "EnableKey",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 禁用指定的主密钥（CMK）进行加解密。
    pub async fn disable_key(
        &self,
        request: DisableKeyRequest,
    ) -> Result<DisableKeyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DisableKey",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取非对称密钥的公钥。可以在本地使用公钥进行加密、验签。
    pub async fn get_public_key(
        &self,
        request: GetPublicKeyRequest,
    ) -> Result<GetPublicKeyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetPublicKey",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为主密钥（CMK）创建一个别名。
    pub async fn create_alias(
        &self,
        request: CreateAliasRequest,
    ) -> Result<CreateAliasResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateAlias",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询当前用户在当前地域的所有别名。
    pub async fn list_aliases(
        &self,
        request: ListAliasesRequest,
    ) -> Result<ListAliasesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListAliases",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询与指定主密钥（CMK）对应的所有别名。
    pub async fn list_aliases_by_key_id(
        &self,
        request: ListAliasesByKeyIdRequest,
    ) -> Result<ListAliasesByKeyIdResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListAliasesByKeyId",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除别名。
    pub async fn delete_alias(
        &self,
        request: DeleteAliasRequest,
    ) -> Result<DeleteAliasResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteAlias",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新已存在的别名所代表的主密钥（CMK）ID。
    pub async fn update_alias(
        &self,
        request: UpdateAliasRequest,
    ) -> Result<UpdateAliasResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateAlias",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取导入主密钥材料的参数。
    pub async fn get_parameters_for_import(
        &self,
        request: GetParametersForImportRequest,
    ) -> Result<GetParametersForImportResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetParametersForImport",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 导入密钥材料。
    pub async fn import_key_material(
        &self,
        request: ImportKeyMaterialRequest,
    ) -> Result<ImportKeyMaterialResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ImportKeyMaterial",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除已导入的密钥材料。
    pub async fn delete_key_material(
        &self,
        request: DeleteKeyMaterialRequest,
    ) -> Result<DeleteKeyMaterialResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteKeyMaterial",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 申请删除一个指定的主密钥（CMK)。
    pub async fn schedule_key_deletion(
        &self,
        request: ScheduleKeyDeletionRequest,
    ) -> Result<ScheduleKeyDeletionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ScheduleKeyDeletion",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 撤销密钥删除。
    pub async fn cancel_key_deletion(
        &self,
        request: CancelKeyDeletionRequest,
    ) -> Result<CancelKeyDeletionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CancelKeyDeletion",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为用户主密钥（CMK）开启或关闭删除保护。
    pub async fn set_deletion_protection(
        &self,
        request: SetDeletionProtectionRequest,
    ) -> Result<SetDeletionProtectionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SetDeletionProtection",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新密钥的周期性自动轮转策略。
    pub async fn update_rotation_policy(
        &self,
        request: UpdateRotationPolicyRequest,
    ) -> Result<UpdateRotationPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateRotationPolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定密钥版本信息。
    pub async fn describe_key_version(
        &self,
        request: DescribeKeyVersionRequest,
    ) -> Result<DescribeKeyVersionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeKeyVersion",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为用户主密钥（CMK）创建密钥版本。
    pub async fn create_key_version(
        &self,
        request: CreateKeyVersionRequest,
    ) -> Result<CreateKeyVersionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateKeyVersion",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 列出主密钥的所有密钥版本。
    pub async fn list_key_versions(
        &self,
        request: ListKeyVersionsRequest,
    ) -> Result<ListKeyVersionsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListKeyVersions",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为KMS实例中的密钥设置密钥策略。
    pub async fn set_key_policy(
        &self,
        request: SetKeyPolicyRequest,
    ) -> Result<SetKeyPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SetKeyPolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定密钥的密钥策略。
    pub async fn get_key_policy(
        &self,
        request: GetKeyPolicyRequest,
    ) -> Result<GetKeyPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetKeyPolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 生成一个随机的数据密钥，用于本地数据加密。
    pub async fn generate_data_key(
        &self,
        request: GenerateDataKeyRequest,
    ) -> Result<GenerateDataKeyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GenerateDataKey",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 随机生成一个数据密钥，通过您指定的主密钥（CMK）和公钥加密后，返回CMK加密数据密钥的密文和公钥加密数据密钥的密文。
    pub async fn generate_and_export_data_key(
        &self,
        request: GenerateAndExportDataKeyRequest,
    ) -> Result<GenerateAndExportDataKeyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GenerateAndExportDataKey",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 使用对称密钥将明文加密为密文。
    pub async fn encrypt(
        &self,
        request: EncryptRequest,
    ) -> Result<EncryptResponse, SdkError> {
        let api_request = ApiRequest {
            action: "Encrypt",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 解密密文。
    pub async fn decrypt(
        &self,
        request: DecryptRequest,
    ) -> Result<DecryptResponse, SdkError> {
        let api_request = ApiRequest {
            action: "Decrypt",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 对密文进行转加密。即先将密文解密，然后将解密得到的数据或者数据密钥使用新的主密钥再次进行加密，返回加密结果。
    pub async fn re_encrypt(
        &self,
        request: ReEncryptRequest,
    ) -> Result<ReEncryptResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ReEncrypt",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 使用传入的公钥加密导出数据密钥。
    pub async fn export_data_key(
        &self,
        request: ExportDataKeyRequest,
    ) -> Result<ExportDataKeyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ExportDataKey",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 生成一个随机的数据密钥。可以用数据密钥进行本地数据的加密。
    pub async fn generate_data_key_without_plaintext(
        &self,
        request: GenerateDataKeyWithoutPlaintextRequest,
    ) -> Result<GenerateDataKeyWithoutPlaintextResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GenerateDataKeyWithoutPlaintext",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 使用非对称密钥进行签名。
    pub async fn asymmetric_sign(
        &self,
        request: AsymmetricSignRequest,
    ) -> Result<AsymmetricSignResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AsymmetricSign",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 使用非对称密钥进行验签。
    pub async fn asymmetric_verify(
        &self,
        request: AsymmetricVerifyRequest,
    ) -> Result<AsymmetricVerifyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AsymmetricVerify",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 使用非对称密钥进行加密。
    pub async fn asymmetric_encrypt(
        &self,
        request: AsymmetricEncryptRequest,
    ) -> Result<AsymmetricEncryptResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AsymmetricEncrypt",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 使用非对称密钥进行解密。
    pub async fn asymmetric_decrypt(
        &self,
        request: AsymmetricDecryptRequest,
    ) -> Result<AsymmetricDecryptResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AsymmetricDecrypt",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建凭据并存入凭据的初始版本。
    pub async fn create_secret(
        &self,
        request: CreateSecretRequest,
    ) -> Result<CreateSecretResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateSecret",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除凭据对象。
    pub async fn delete_secret(
        &self,
        request: DeleteSecretRequest,
    ) -> Result<DeleteSecretResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteSecret",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新凭据的元数据。
    pub async fn update_secret(
        &self,
        request: UpdateSecretRequest,
    ) -> Result<UpdateSecretResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateSecret",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新凭据的版本状态。
    pub async fn update_secret_version_stage(
        &self,
        request: UpdateSecretVersionStageRequest,
    ) -> Result<UpdateSecretVersionStageResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateSecretVersionStage",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新凭据轮转策略。
    pub async fn update_secret_rotation_policy(
        &self,
        request: UpdateSecretRotationPolicyRequest,
    ) -> Result<UpdateSecretRotationPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateSecretRotationPolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询当前用户在当前地域创建的所有凭据。
    pub async fn list_secrets(
        &self,
        request: ListSecretsRequest,
    ) -> Result<ListSecretsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListSecrets",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询凭据的元数据信息。
    pub async fn describe_secret(
        &self,
        request: DescribeSecretRequest,
    ) -> Result<DescribeSecretResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeSecret",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取凭据值。
    pub async fn get_secret_value(
        &self,
        request: GetSecretValueRequest,
    ) -> Result<GetSecretValueResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetSecretValue",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询凭据的所有版本信息。
    pub async fn list_secret_version_ids(
        &self,
        request: ListSecretVersionIdsRequest,
    ) -> Result<ListSecretVersionIdsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListSecretVersionIds",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获得一个随机口令字符串。
    pub async fn get_random_password(
        &self,
        request: GetRandomPasswordRequest,
    ) -> Result<GetRandomPasswordResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetRandomPassword",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为通用凭据存入一个新版本的凭据值。
    pub async fn put_secret_value(
        &self,
        request: PutSecretValueRequest,
    ) -> Result<PutSecretValueResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutSecretValue",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 恢复被删除的凭据。
    pub async fn restore_secret(
        &self,
        request: RestoreSecretRequest,
    ) -> Result<RestoreSecretResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RestoreSecret",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 立即轮转凭据。
    pub async fn rotate_secret(
        &self,
        request: RotateSecretRequest,
    ) -> Result<RotateSecretResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RotateSecret",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为KMS实例中的凭据钥设置凭据策略。
    pub async fn set_secret_policy(
        &self,
        request: SetSecretPolicyRequest,
    ) -> Result<SetSecretPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SetSecretPolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定凭据的凭据策略。
    pub async fn get_secret_policy(
        &self,
        request: GetSecretPolicyRequest,
    ) -> Result<GetSecretPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetSecretPolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为密钥或凭据解绑标签。
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

    /// 查询密钥或凭据的标签。
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

    /// 为密钥或凭据绑定标签。
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

    /// 获取用户主密钥的标签。
    pub async fn list_resource_tags(
        &self,
        request: ListResourceTagsRequest,
    ) -> Result<ListResourceTagsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListResourceTags",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为主密钥、凭据或证书绑定标签。
    pub async fn tag_resource(
        &self,
        request: TagResourceRequest,
    ) -> Result<TagResourceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "TagResource",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为主密钥、凭据或证书解绑标签。
    pub async fn untag_resource(
        &self,
        request: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UntagResource",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建一条网络控制规则，设置允许访问KMS的私网IP或私网的网段。
    pub async fn create_network_rule(
        &self,
        request: CreateNetworkRuleRequest,
    ) -> Result<CreateNetworkRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateNetworkRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询网络控制规则列表。
    pub async fn list_network_rules(
        &self,
        request: ListNetworkRulesRequest,
    ) -> Result<ListNetworkRulesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListNetworkRules",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询一个网络控制规则的详情。
    pub async fn describe_network_rule(
        &self,
        request: DescribeNetworkRuleRequest,
    ) -> Result<DescribeNetworkRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeNetworkRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新一个网络控制规则。
    pub async fn update_network_rule(
        &self,
        request: UpdateNetworkRuleRequest,
    ) -> Result<UpdateNetworkRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateNetworkRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除一条网络控制规则。
    pub async fn delete_network_rule(
        &self,
        request: DeleteNetworkRuleRequest,
    ) -> Result<DeleteNetworkRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteNetworkRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建一个权限策略，设置允许应用访问的密钥和凭据。
    pub async fn create_policy(
        &self,
        request: CreatePolicyRequest,
    ) -> Result<CreatePolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreatePolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询权限策略列表。
    pub async fn list_policies(
        &self,
        request: ListPoliciesRequest,
    ) -> Result<ListPoliciesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListPolicies",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询一个权限策略的详情。
    pub async fn describe_policy(
        &self,
        request: DescribePolicyRequest,
    ) -> Result<DescribePolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribePolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新一个权限策略。
    pub async fn update_policy(
        &self,
        request: UpdatePolicyRequest,
    ) -> Result<UpdatePolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdatePolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除一条权限策略。
    pub async fn delete_policy(
        &self,
        request: DeletePolicyRequest,
    ) -> Result<DeletePolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeletePolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建一个应用接入点。
    pub async fn create_application_access_point(
        &self,
        request: CreateApplicationAccessPointRequest,
    ) -> Result<CreateApplicationAccessPointResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateApplicationAccessPoint",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询应用接入点列表。
    pub async fn list_application_access_points(
        &self,
        request: ListApplicationAccessPointsRequest,
    ) -> Result<ListApplicationAccessPointsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListApplicationAccessPoints",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询一个应用接入点的详情。
    pub async fn describe_application_access_point(
        &self,
        request: DescribeApplicationAccessPointRequest,
    ) -> Result<DescribeApplicationAccessPointResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeApplicationAccessPoint",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新一个应用接入点信息。
    pub async fn update_application_access_point(
        &self,
        request: UpdateApplicationAccessPointRequest,
    ) -> Result<UpdateApplicationAccessPointResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateApplicationAccessPoint",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除一个应用接入点。
    pub async fn delete_application_access_point(
        &self,
        request: DeleteApplicationAccessPointRequest,
    ) -> Result<DeleteApplicationAccessPointResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteApplicationAccessPoint",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建一个应用身份凭证（ClientKey）。
    pub async fn create_client_key(
        &self,
        request: CreateClientKeyRequest,
    ) -> Result<CreateClientKeyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateClientKey",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询应用身份凭证（ClientKey）列表。
    pub async fn list_client_keys(
        &self,
        request: ListClientKeysRequest,
    ) -> Result<ListClientKeysResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListClientKeys",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取一个应用身份凭证（ClientKey）信息。
    pub async fn get_client_key(
        &self,
        request: GetClientKeyRequest,
    ) -> Result<GetClientKeyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetClientKey",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除一个应用身份凭证（ClientKey）。
    pub async fn delete_client_key(
        &self,
        request: DeleteClientKeyRequest,
    ) -> Result<DeleteClientKeyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteClientKey",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取实例配额信息
    pub async fn get_kms_instance_quota_infos(
        &self,
        request: GetKmsInstanceQuotaInfosRequest,
    ) -> Result<GetKmsInstanceQuotaInfosResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetKmsInstanceQuotaInfos",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

}