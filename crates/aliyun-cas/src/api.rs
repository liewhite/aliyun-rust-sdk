//! API 调用 - 自动生成，请勿手动修改

use aliyun_sdk_core::{Client, ClientConfig, ApiRequest, HttpMethod, SdkError};
use super::types::*;

/// cas API 版本
pub const API_VERSION: &str = "2020-06-30";

/// cas 客户端
#[derive(Debug, Clone)]
pub struct CasClient {
    client: Client,
}

impl CasClient {
    /// 创建新客户端
    pub fn new(config: ClientConfig) -> Result<Self, SdkError> {
        Ok(Self {
            client: Client::new(config)?,
        })
    }

    /// 分配证书使用数量
    pub async fn assign_certificate_count(
        &self,
        request: AssignCertificateCountRequest,
    ) -> Result<AssignCertificateCountResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AssignCertificateCount",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 基于系统自动生成的CSR签发单个客户端证书
    pub async fn create_client_certificate(
        &self,
        request: CreateClientCertificateRequest,
    ) -> Result<CreateClientCertificateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateClientCertificate",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 基于自定义的CSR签发单个客户端证书
    pub async fn create_client_certificate_with_csr(
        &self,
        request: CreateClientCertificateWithCsrRequest,
    ) -> Result<CreateClientCertificateWithCsrResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateClientCertificateWithCsr",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 使用指定的证书主体名称、证书主题别名、密钥用法、扩展密钥用法颁发数字证书
    pub async fn create_custom_certificate(
        &self,
        request: CreateCustomCertificateRequest,
    ) -> Result<CreateCustomCertificateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateCustomCertificate",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 基于CSR和API参数创建并颁发外部子CA证书。
    pub async fn create_external_ca_certificate(
        &self,
        request: CreateExternalCACertificateRequest,
    ) -> Result<CreateExternalCACertificateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateExternalCACertificate",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 吊销某个客户端证书或服务端证书。
    pub async fn create_revoke_client_certificate(
        &self,
        request: CreateRevokeClientCertificateRequest,
    ) -> Result<CreateRevokeClientCertificateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateRevokeClientCertificate",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建一个根CA证书。
    pub async fn create_root_ca_certificate(
        &self,
        request: CreateRootCACertificateRequest,
    ) -> Result<CreateRootCACertificateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateRootCACertificate",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 基于系统自动生成的CSR签发单个服务端证书
    pub async fn create_server_certificate(
        &self,
        request: CreateServerCertificateRequest,
    ) -> Result<CreateServerCertificateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateServerCertificate",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 基于自定义的CSR签发单个服务端证书
    pub async fn create_server_certificate_with_csr(
        &self,
        request: CreateServerCertificateWithCsrRequest,
    ) -> Result<CreateServerCertificateWithCsrResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateServerCertificateWithCsr",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建一个子CA证书。
    pub async fn create_sub_ca_certificate(
        &self,
        request: CreateSubCACertificateRequest,
    ) -> Result<CreateSubCACertificateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateSubCACertificate",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除已吊销的客户端证书或服务端证书。
    pub async fn delete_client_certificate(
        &self,
        request: DeleteClientCertificateRequest,
    ) -> Result<DeleteClientCertificateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteClientCertificate",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询CA详情。
    pub async fn describe_ca_certificate(
        &self,
        request: DescribeCACertificateRequest,
    ) -> Result<DescribeCACertificateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCACertificate",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询已创建的CA证书的数量。
    pub async fn describe_ca_certificate_count(
        &self,
        request: DescribeCACertificateCountRequest,
    ) -> Result<DescribeCACertificateCountResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCACertificateCount",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询所有根CA证书和子CA证书的信息。
    pub async fn describe_ca_certificate_list(
        &self,
        request: DescribeCACertificateListRequest,
    ) -> Result<DescribeCACertificateListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCACertificateList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取单个客户端证书或服务端证书的私钥（加密格式）。
    pub async fn describe_certificate_private_key(
        &self,
        request: DescribeCertificatePrivateKeyRequest,
    ) -> Result<DescribeCertificatePrivateKeyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCertificatePrivateKey",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 通过证书的唯一识别码查询单个客户端证书或服务端证书的详细信息。
    pub async fn describe_client_certificate(
        &self,
        request: DescribeClientCertificateRequest,
    ) -> Result<DescribeClientCertificateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeClientCertificate",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 通过证书的序列号批量查询客户端证书或服务端证书的详细信息。
    pub async fn describe_client_certificate_for_serial_number(
        &self,
        request: DescribeClientCertificateForSerialNumberRequest,
    ) -> Result<DescribeClientCertificateForSerialNumberResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeClientCertificateForSerialNumber",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 通过证书的唯一识别码查询客户端证书和服务端证书的状态信息。
    pub async fn describe_client_certificate_status(
        &self,
        request: DescribeClientCertificateStatusRequest,
    ) -> Result<DescribeClientCertificateStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeClientCertificateStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 通过证书的序列号查询客户端证书和服务端证书的状态信息。
    pub async fn describe_client_certificate_status_for_serial_number(
        &self,
        request: DescribeClientCertificateStatusForSerialNumberRequest,
    ) -> Result<DescribeClientCertificateStatusForSerialNumberResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeClientCertificateStatusForSerialNumber",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 返回用户所有CaCertificate，包括PCA内部产生的与导入的外部证书。
    pub async fn describe_pca_and_external_ca_certificate_list(
        &self,
        request: DescribePcaAndExternalCACertificateListRequest,
    ) -> Result<DescribePcaAndExternalCACertificateListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribePcaAndExternalCACertificateList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询您通过SSL证书服务控制台购买的私有根CA实例或私有子CA实例的状态信息。
    pub async fn get_ca_instance_status(
        &self,
        request: GetCAInstanceStatusRequest,
    ) -> Result<GetCAInstanceStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetCAInstanceStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询终端实例
    pub async fn list_all_end_entity_instance(
        &self,
        request: ListAllEndEntityInstanceRequest,
    ) -> Result<ListAllEndEntityInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListAllEndEntityInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询CA证书的操作日志。
    pub async fn list_ca_certificate_log(
        &self,
        request: ListCACertificateLogRequest,
    ) -> Result<ListCACertificateLogResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListCACertificateLog",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取证书列表。
    pub async fn list_cert(
        &self,
        request: ListCertRequest,
    ) -> Result<ListCertResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListCert",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询所有客户端证书和服务端证书的详细信息。
    pub async fn list_client_certificate(
        &self,
        request: ListClientCertificateRequest,
    ) -> Result<ListClientCertificateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListClientCertificate",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询私有CA机构证书。
    pub async fn list_pca_ca_certificate(
        &self,
        request: ListPcaCaCertificateRequest,
    ) -> Result<ListPcaCaCertificateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListPcaCaCertificate",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询所有已被吊销的客户端证书和服务端证书的详细信息。
    pub async fn list_revoke_certificate(
        &self,
        request: ListRevokeCertificateRequest,
    ) -> Result<ListRevokeCertificateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListRevokeCertificate",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询资源（SSL证书实例）和标签的对应关系。
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

    /// 修改资源所属的资源组。
    pub async fn move_resource_group(
        &self,
        request: MoveResourceGroupRequest,
    ) -> Result<MoveResourceGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "MoveResourceGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为指定资源（SSL证书实例）绑定标签。
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

    /// 移除标签。
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

    /// 将根CA证书或子CA证书的状态由正常签发修改为吊销。
    pub async fn update_ca_certificate_status(
        &self,
        request: UpdateCACertificateStatusRequest,
    ) -> Result<UpdateCACertificateStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateCACertificateStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新证书属性
    pub async fn update_pca_certificate(
        &self,
        request: UpdatePcaCertificateRequest,
    ) -> Result<UpdatePcaCertificateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdatePcaCertificate",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 上传PCA证书到SSL上传证书。
    pub async fn upload_pca_cert_to_cas(
        &self,
        request: UploadPcaCertToCasRequest,
    ) -> Result<UploadPcaCertToCasResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UploadPcaCertToCas",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

}