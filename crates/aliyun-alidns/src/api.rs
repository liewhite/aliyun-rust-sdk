//! API 调用 - 自动生成，请勿手动修改

use aliyun_sdk_core::{Client, ClientConfig, ApiRequest, HttpMethod, SdkError};
use super::types::*;

/// Alidns API 版本
pub const API_VERSION: &str = "2015-01-09";

/// Alidns 客户端
#[derive(Debug, Clone)]
pub struct AlidnsClient {
    client: Client,
}

impl AlidnsClient {
    /// 创建新客户端
    pub fn new(config: ClientConfig) -> Result<Self, SdkError> {
        Ok(Self {
            client: Client::new(config)?,
        })
    }

    /// 用于更新特定域名的状态信息。
    pub async fn update_rsp_domain_server_prohibit_status_for_gateway_ote(
        &self,
        request: UpdateRspDomainServerProhibitStatusForGatewayOteRequest,
    ) -> Result<UpdateRspDomainServerProhibitStatusForGatewayOteResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateRspDomainServerProhibitStatusForGatewayOte",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 用于删除指定域名的serverHold状态信息。
    pub async fn remove_rsp_domain_server_hold_status_for_gateway_ote(
        &self,
        request: RemoveRspDomainServerHoldStatusForGatewayOteRequest,
    ) -> Result<RemoveRspDomainServerHoldStatusForGatewayOteResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RemoveRspDomainServerHoldStatusForGatewayOte",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 用于添加特定域名的serverHold状态信息。
    pub async fn add_rsp_domain_server_hold_status_for_gateway_ote(
        &self,
        request: AddRspDomainServerHoldStatusForGatewayOteRequest,
    ) -> Result<AddRspDomainServerHoldStatusForGatewayOteResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddRspDomainServerHoldStatusForGatewayOte",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 用于更新特定域名的状态信息。
    pub async fn update_rsp_domain_server_prohibit_status_for_gateway(
        &self,
        request: UpdateRspDomainServerProhibitStatusForGatewayRequest,
    ) -> Result<UpdateRspDomainServerProhibitStatusForGatewayResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateRspDomainServerProhibitStatusForGateway",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 用于删除特定域名的serverHold状态信息。
    pub async fn remove_rsp_domain_server_hold_status_for_gateway(
        &self,
        request: RemoveRspDomainServerHoldStatusForGatewayRequest,
    ) -> Result<RemoveRspDomainServerHoldStatusForGatewayResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RemoveRspDomainServerHoldStatusForGateway",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 用于添加特定域名的serverHold状态信息。
    pub async fn add_rsp_domain_server_hold_status_for_gateway(
        &self,
        request: AddRspDomainServerHoldStatusForGatewayRequest,
    ) -> Result<AddRspDomainServerHoldStatusForGatewayResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddRspDomainServerHoldStatusForGateway",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据实例ID获取云解析收费版本产品实例的详情信息。
    pub async fn describe_dns_product_instance(
        &self,
        request: DescribeDnsProductInstanceRequest,
    ) -> Result<DescribeDnsProductInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDnsProductInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数获取付费版DNS产品实例列表。
    pub async fn describe_dns_product_instances(
        &self,
        request: DescribeDnsProductInstancesRequest,
    ) -> Result<DescribeDnsProductInstancesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDnsProductInstances",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取实例绑定域名列表。
    pub async fn describe_instance_domains(
        &self,
        request: DescribeInstanceDomainsRequest,
    ) -> Result<DescribeInstanceDomainsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeInstanceDomains",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数添加域名。
    pub async fn add_domain(
        &self,
        request: AddDomainRequest,
    ) -> Result<AddDomainResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddDomain",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数删除域名。
    pub async fn delete_domain(
        &self,
        request: DeleteDomainRequest,
    ) -> Result<DeleteDomainResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteDomain",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数查询该用户的域名列表。
    pub async fn describe_domains(
        &self,
        request: DescribeDomainsRequest,
    ) -> Result<DescribeDomainsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomains",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数查询指定域名的信息。
    pub async fn describe_domain_info(
        &self,
        request: DescribeDomainInfoRequest,
    ) -> Result<DescribeDomainInfoResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainInfo",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入的域名名称和备份周期给该域名建立备份。
    pub async fn add_domain_backup(
        &self,
        request: AddDomainBackupRequest,
    ) -> Result<AddDomainBackupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddDomainBackup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更换云解析产品绑定的域名。
    pub async fn change_domain_of_dns_product(
        &self,
        request: ChangeDomainOfDnsProductRequest,
    ) -> Result<ChangeDomainOfDnsProductResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ChangeDomainOfDnsProduct",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 变更域名绑定的DNS服务器名称，从其他服务商的DNS服务器名称，修改为阿里云解析DNS提供的DNS服务器名称。
    pub async fn modify_hichina_domain_dns(
        &self,
        request: ModifyHichinaDomainDNSRequest,
    ) -> Result<ModifyHichinaDomainDNSResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyHichinaDomainDNS",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数修改域名的备注。
    pub async fn update_domain_remark(
        &self,
        request: UpdateDomainRemarkRequest,
    ) -> Result<UpdateDomainRemarkResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateDomainRemark",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 通过输入的参数，获取主域名名称。
    pub async fn get_main_domain_name(
        &self,
        request: GetMainDomainNameRequest,
    ) -> Result<GetMainDomainNameResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetMainDomainName",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入域名获取域名当前的NS服务器列表，以及NS是否属于阿里云解析管理。
    pub async fn describe_domain_ns(
        &self,
        request: DescribeDomainNsRequest,
    ) -> Result<DescribeDomainNsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainNs",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 开启/关闭域名的DNSSEC服务，仅支持付费版DNS用户。
    pub async fn set_domain_dnssec_status(
        &self,
        request: SetDomainDnssecStatusRequest,
    ) -> Result<SetDomainDnssecStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SetDomainDnssecStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数查询指定域名的DNSSEC信息。
    pub async fn describe_domain_dnssec_info(
        &self,
        request: DescribeDomainDnssecInfoRequest,
    ) -> Result<DescribeDomainDnssecInfoResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainDnssecInfo",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 绑定付费版DNS域名到实例ID。
    pub async fn bind_instance_domains(
        &self,
        request: BindInstanceDomainsRequest,
    ) -> Result<BindInstanceDomainsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "BindInstanceDomains",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据实例ID解绑付费版DNS的域名。
    pub async fn unbind_instance_domains(
        &self,
        request: UnbindInstanceDomainsRequest,
    ) -> Result<UnbindInstanceDomainsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UnbindInstanceDomains",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 进行批量操作以添加或删除域名和解析记录。
    pub async fn operate_batch_domain(
        &self,
        request: OperateBatchDomainRequest,
    ) -> Result<OperateBatchDomainResponse, SdkError> {
        let api_request = ApiRequest {
            action: "OperateBatchDomain",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数添加域名分组。
    pub async fn add_domain_group(
        &self,
        request: AddDomainGroupRequest,
    ) -> Result<AddDomainGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddDomainGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数获取域名分组的列表。
    pub async fn describe_domain_groups(
        &self,
        request: DescribeDomainGroupsRequest,
    ) -> Result<DescribeDomainGroupsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainGroups",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除域名分组，分组下的域名会被移动到默认分组。
    pub async fn delete_domain_group(
        &self,
        request: DeleteDomainGroupRequest,
    ) -> Result<DeleteDomainGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteDomainGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数修改域名分组名称。
    pub async fn update_domain_group(
        &self,
        request: UpdateDomainGroupRequest,
    ) -> Result<UpdateDomainGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateDomainGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数将域名从原分组更换到新分组。
    pub async fn change_domain_group(
        &self,
        request: ChangeDomainGroupRequest,
    ) -> Result<ChangeDomainGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ChangeDomainGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数批量跨账号转移DNS权限至目标账户。
    pub async fn transfer_domain(
        &self,
        request: TransferDomainRequest,
    ) -> Result<TransferDomainResponse, SdkError> {
        let api_request = ApiRequest {
            action: "TransferDomain",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 执行找回域名操作。
    pub async fn retrieve_domain(
        &self,
        request: RetrieveDomainRequest,
    ) -> Result<RetrieveDomainResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RetrieveDomain",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数获取跨账号转移DNS列表。
    pub async fn describe_transfer_domains(
        &self,
        request: DescribeTransferDomainsRequest,
    ) -> Result<DescribeTransferDomainsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeTransferDomains",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定主域名请求量的实时数据。
    pub async fn describe_domain_statistics(
        &self,
        request: DescribeDomainStatisticsRequest,
    ) -> Result<DescribeDomainStatisticsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainStatistics",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询用户账号下所有付费域名的请求量列表。
    pub async fn describe_domain_statistics_summary(
        &self,
        request: DescribeDomainStatisticsSummaryRequest,
    ) -> Result<DescribeDomainStatisticsSummaryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainStatisticsSummary",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定子域名请求量的实时数据。
    pub async fn describe_record_statistics(
        &self,
        request: DescribeRecordStatisticsRequest,
    ) -> Result<DescribeRecordStatisticsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeRecordStatistics",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定域名下的全部子域名的请求量统计。
    pub async fn describe_record_statistics_summary(
        &self,
        request: DescribeRecordStatisticsSummaryRequest,
    ) -> Result<DescribeRecordStatisticsSummaryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeRecordStatisticsSummary",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数获取权重配置子域名列表。
    pub async fn describe_dnsslb_sub_domains(
        &self,
        request: DescribeDNSSLBSubDomainsRequest,
    ) -> Result<DescribeDNSSLBSubDomainsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDNSSLBSubDomains",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数获取域名的操作日志。
    pub async fn describe_domain_logs(
        &self,
        request: DescribeDomainLogsRequest,
    ) -> Result<DescribeDomainLogsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainLogs",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数添加解析记录。
    pub async fn add_domain_record(
        &self,
        request: AddDomainRecordRequest,
    ) -> Result<AddDomainRecordResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddDomainRecord",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数删除解析记录。
    pub async fn delete_domain_record(
        &self,
        request: DeleteDomainRecordRequest,
    ) -> Result<DeleteDomainRecordResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteDomainRecord",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数删除主机记录对应的解析记录。
    pub async fn delete_sub_domain_records(
        &self,
        request: DeleteSubDomainRecordsRequest,
    ) -> Result<DeleteSubDomainRecordsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteSubDomainRecords",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数修改解析记录。
    pub async fn update_domain_record(
        &self,
        request: UpdateDomainRecordRequest,
    ) -> Result<UpdateDomainRecordResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateDomainRecord",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数修改解析记录的备注。
    pub async fn update_domain_record_remark(
        &self,
        request: UpdateDomainRecordRemarkRequest,
    ) -> Result<UpdateDomainRecordRemarkResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateDomainRecordRemark",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数设置解析记录状态。
    pub async fn set_domain_record_status(
        &self,
        request: SetDomainRecordStatusRequest,
    ) -> Result<SetDomainRecordStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SetDomainRecordStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 通过解析记录的ID获取解析记录的详细信息。
    pub async fn describe_domain_record_info(
        &self,
        request: DescribeDomainRecordInfoRequest,
    ) -> Result<DescribeDomainRecordInfoResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainRecordInfo",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数获取指定主域名的所有解析记录列表。
    pub async fn describe_domain_records(
        &self,
        request: DescribeDomainRecordsRequest,
    ) -> Result<DescribeDomainRecordsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainRecords",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数获取域名的解析操作日志。
    pub async fn describe_record_logs(
        &self,
        request: DescribeRecordLogsRequest,
    ) -> Result<DescribeRecordLogsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeRecordLogs",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数获取某个固定子域名的所有解析记录列表。
    pub async fn describe_sub_domain_records(
        &self,
        request: DescribeSubDomainRecordsRequest,
    ) -> Result<DescribeSubDomainRecordsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeSubDomainRecords",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 生成txt记录。用于域名、子域名找回、添加子域名验证、批量找回等功能。
    pub async fn get_txt_record_for_verify(
        &self,
        request: GetTxtRecordForVerifyRequest,
    ) -> Result<GetTxtRecordForVerifyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetTxtRecordForVerify",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 给域名添加自定义线路。
    pub async fn add_custom_line(
        &self,
        request: AddCustomLineRequest,
    ) -> Result<AddCustomLineResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddCustomLine",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 通过自定义线路唯一ID批量删除自定义线路。
    pub async fn delete_custom_lines(
        &self,
        request: DeleteCustomLinesRequest,
    ) -> Result<DeleteCustomLinesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteCustomLines",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 通过自定义线路唯一ID编辑自定义线路。
    pub async fn update_custom_line(
        &self,
        request: UpdateCustomLineRequest,
    ) -> Result<UpdateCustomLineResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateCustomLine",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 通过自定义线路唯一ID查询自定义线路。
    pub async fn describe_custom_line(
        &self,
        request: DescribeCustomLineRequest,
    ) -> Result<DescribeCustomLineResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCustomLine",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 通过域名名称查询自定义线路列表。
    pub async fn describe_custom_lines(
        &self,
        request: DescribeCustomLinesRequest,
    ) -> Result<DescribeCustomLinesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCustomLines",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询云解析支持的所有线路列表。
    pub async fn describe_support_lines(
        &self,
        request: DescribeSupportLinesRequest,
    ) -> Result<DescribeSupportLinesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeSupportLines",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数开启关闭权重配置。
    pub async fn set_dnsslb_status(
        &self,
        request: SetDNSSLBStatusRequest,
    ) -> Result<SetDNSSLBStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SetDNSSLBStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数修改解析记录的权重。
    pub async fn update_dnsslb_weight(
        &self,
        request: UpdateDNSSLBWeightRequest,
    ) -> Result<UpdateDNSSLBWeightResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateDNSSLBWeight",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据任务ID查询其批量操作任务的执行结果，如果任务ID为空，返回最后一次批量操作任务的执行结果。
    pub async fn describe_batch_result_count(
        &self,
        request: DescribeBatchResultCountRequest,
    ) -> Result<DescribeBatchResultCountResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeBatchResultCount",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询批量处理结果的详细信息。
    pub async fn describe_batch_result_detail(
        &self,
        request: DescribeBatchResultDetailRequest,
    ) -> Result<DescribeBatchResultDetailResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeBatchResultDetail",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询已有标签。
    pub async fn describe_tags(
        &self,
        request: DescribeTagsRequest,
    ) -> Result<DescribeTagsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeTags",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 按照标签查询资源。
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

    /// 通过进行添加、修改资源的标签。
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

    /// 进行删除资源的标签。
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

    /// 更换资源组。
    pub async fn move_domain_resource_group(
        &self,
        request: MoveDomainResourceGroupRequest,
    ) -> Result<MoveDomainResourceGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "MoveDomainResourceGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数添加 DNS 权威代理域名。
    pub async fn add_dns_cache_domain(
        &self,
        request: AddDnsCacheDomainRequest,
    ) -> Result<AddDnsCacheDomainResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddDnsCacheDomain",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数更新 DNS 权威代理域名。
    pub async fn update_dns_cache_domain(
        &self,
        request: UpdateDnsCacheDomainRequest,
    ) -> Result<UpdateDnsCacheDomainResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateDnsCacheDomain",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新目标域名的DNS 权威代理域名备注。
    pub async fn update_dns_cache_domain_remark(
        &self,
        request: UpdateDnsCacheDomainRemarkRequest,
    ) -> Result<UpdateDnsCacheDomainRemarkResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateDnsCacheDomainRemark",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除指定的DNS权威代理域名。
    pub async fn delete_dns_cache_domain(
        &self,
        request: DeleteDnsCacheDomainRequest,
    ) -> Result<DeleteDnsCacheDomainResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteDnsCacheDomain",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数查询该用户的权威代理域名列表。
    pub async fn describe_dns_cache_domains(
        &self,
        request: DescribeDnsCacheDomainsRequest,
    ) -> Result<DescribeDnsCacheDomainsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDnsCacheDomains",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建gtm实例配置。
    pub async fn create_cloud_gtm_instance_config(
        &self,
        request: CreateCloudGtmInstanceConfigRequest,
    ) -> Result<CreateCloudGtmInstanceConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateCloudGtmInstanceConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 
    pub async fn update_cloud_gtm_instance_name(
        &self,
        request: UpdateCloudGtmInstanceNameRequest,
    ) -> Result<UpdateCloudGtmInstanceNameResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateCloudGtmInstanceName",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 按需启用或停用全局流量管理（GTM）实例的流量分析功能。
    pub async fn set_cloud_gtm_instance_config_log_switch(
        &self,
        request: SetCloudGtmInstanceConfigLogSwitchRequest,
    ) -> Result<SetCloudGtmInstanceConfigLogSwitchResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SetCloudGtmInstanceConfigLogSwitch",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取全局流量管理实例配置的完整信息，包括接入域名、告警通知、地址池、地址详细信息等。
    pub async fn describe_cloud_gtm_instance_config_full_info(
        &self,
        request: DescribeCloudGtmInstanceConfigFullInfoRequest,
    ) -> Result<DescribeCloudGtmInstanceConfigFullInfoResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCloudGtmInstanceConfigFullInfo",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取实例配置列表，包括接入域名、地址池信息等。
    pub async fn list_cloud_gtm_instance_configs(
        &self,
        request: ListCloudGtmInstanceConfigsRequest,
    ) -> Result<ListCloudGtmInstanceConfigsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListCloudGtmInstanceConfigs",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数删除GTM 3.0实例配置的接入域名。
    pub async fn delete_cloud_gtm_instance_config(
        &self,
        request: DeleteCloudGtmInstanceConfigRequest,
    ) -> Result<DeleteCloudGtmInstanceConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteCloudGtmInstanceConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数更新GTM3.0 实例全局TTL配置。
    pub async fn update_cloud_gtm_instance_config_basic(
        &self,
        request: UpdateCloudGtmInstanceConfigBasicRequest,
    ) -> Result<UpdateCloudGtmInstanceConfigBasicResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateCloudGtmInstanceConfigBasic",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数更新实例负载均衡策略。
    pub async fn update_cloud_gtm_instance_config_lb_strategy(
        &self,
        request: UpdateCloudGtmInstanceConfigLbStrategyRequest,
    ) -> Result<UpdateCloudGtmInstanceConfigLbStrategyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateCloudGtmInstanceConfigLbStrategy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数更新实例启动状态。
    pub async fn update_cloud_gtm_instance_config_enable_status(
        &self,
        request: UpdateCloudGtmInstanceConfigEnableStatusRequest,
    ) -> Result<UpdateCloudGtmInstanceConfigEnableStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateCloudGtmInstanceConfigEnableStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入的参数更新实例备注信息。
    pub async fn update_cloud_gtm_instance_config_remark(
        &self,
        request: UpdateCloudGtmInstanceConfigRemarkRequest,
    ) -> Result<UpdateCloudGtmInstanceConfigRemarkResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateCloudGtmInstanceConfigRemark",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数获取GTM 3.0实例列表。
    pub async fn list_cloud_gtm_instances(
        &self,
        request: ListCloudGtmInstancesRequest,
    ) -> Result<ListCloudGtmInstancesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListCloudGtmInstances",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数搜索实例列表。
    pub async fn search_cloud_gtm_instances(
        &self,
        request: SearchCloudGtmInstancesRequest,
    ) -> Result<SearchCloudGtmInstancesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SearchCloudGtmInstances",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数搜索实例配置列表。
    pub async fn search_cloud_gtm_instance_configs(
        &self,
        request: SearchCloudGtmInstanceConfigsRequest,
    ) -> Result<SearchCloudGtmInstanceConfigsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SearchCloudGtmInstanceConfigs",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 
    pub async fn describe_cloud_gtm_summary(
        &self,
        request: DescribeCloudGtmSummaryRequest,
    ) -> Result<DescribeCloudGtmSummaryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCloudGtmSummary",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数创建地址。
    pub async fn create_cloud_gtm_address(
        &self,
        request: CreateCloudGtmAddressRequest,
    ) -> Result<CreateCloudGtmAddressResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateCloudGtmAddress",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用 UpdateCloudGtmAddress 修改指定地址基础配置。
    pub async fn update_cloud_gtm_address(
        &self,
        request: UpdateCloudGtmAddressRequest,
    ) -> Result<UpdateCloudGtmAddressResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateCloudGtmAddress",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数获取地址配置信息。
    pub async fn describe_cloud_gtm_address(
        &self,
        request: DescribeCloudGtmAddressRequest,
    ) -> Result<DescribeCloudGtmAddressResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCloudGtmAddress",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数删除地址。
    pub async fn delete_cloud_gtm_address(
        &self,
        request: DeleteCloudGtmAddressRequest,
    ) -> Result<DeleteCloudGtmAddressResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteCloudGtmAddress",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 通过地址的 ID修改地址备注信息。
    pub async fn update_cloud_gtm_address_remark(
        &self,
        request: UpdateCloudGtmAddressRemarkRequest,
    ) -> Result<UpdateCloudGtmAddressRemarkResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateCloudGtmAddressRemark",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数获取地址列表。
    pub async fn list_cloud_gtm_addresses(
        &self,
        request: ListCloudGtmAddressesRequest,
    ) -> Result<ListCloudGtmAddressesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListCloudGtmAddresses",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 按照地址名称、备注信息、地址引用的探测模板或地址DI等信息搜索地址列表。
    pub async fn search_cloud_gtm_addresses(
        &self,
        request: SearchCloudGtmAddressesRequest,
    ) -> Result<SearchCloudGtmAddressesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SearchCloudGtmAddresses",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数获取地址被引用的地址池、GTM3.0实例信息。
    pub async fn describe_cloud_gtm_address_reference(
        &self,
        request: DescribeCloudGtmAddressReferenceRequest,
    ) -> Result<DescribeCloudGtmAddressReferenceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCloudGtmAddressReference",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数修改地址启用状态。
    pub async fn update_cloud_gtm_address_enable_status(
        &self,
        request: UpdateCloudGtmAddressEnableStatusRequest,
    ) -> Result<UpdateCloudGtmAddressEnableStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateCloudGtmAddressEnableStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数修改地址探测异常切换方式。
    pub async fn update_cloud_gtm_address_manual_available_status(
        &self,
        request: UpdateCloudGtmAddressManualAvailableStatusRequest,
    ) -> Result<UpdateCloudGtmAddressManualAvailableStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateCloudGtmAddressManualAvailableStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数创建地址池。
    pub async fn create_cloud_gtm_address_pool(
        &self,
        request: CreateCloudGtmAddressPoolRequest,
    ) -> Result<CreateCloudGtmAddressPoolResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateCloudGtmAddressPool",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取地址池列表。
    pub async fn list_cloud_gtm_address_pools(
        &self,
        request: ListCloudGtmAddressPoolsRequest,
    ) -> Result<ListCloudGtmAddressPoolsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListCloudGtmAddressPools",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数获取指定地址池配置信息。
    pub async fn describe_cloud_gtm_address_pool(
        &self,
        request: DescribeCloudGtmAddressPoolRequest,
    ) -> Result<DescribeCloudGtmAddressPoolResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCloudGtmAddressPool",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数删除地址池。
    pub async fn delete_cloud_gtm_address_pool(
        &self,
        request: DeleteCloudGtmAddressPoolRequest,
    ) -> Result<DeleteCloudGtmAddressPoolResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteCloudGtmAddressPool",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数搜索地址池列表。
    pub async fn search_cloud_gtm_address_pools(
        &self,
        request: SearchCloudGtmAddressPoolsRequest,
    ) -> Result<SearchCloudGtmAddressPoolsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SearchCloudGtmAddressPools",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数替换实例关联的地址池。
    pub async fn replace_cloud_gtm_instance_config_address_pool(
        &self,
        request: ReplaceCloudGtmInstanceConfigAddressPoolRequest,
    ) -> Result<ReplaceCloudGtmInstanceConfigAddressPoolResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ReplaceCloudGtmInstanceConfigAddressPool",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数修改地址池启用状态。
    pub async fn update_cloud_gtm_address_pool_enable_status(
        &self,
        request: UpdateCloudGtmAddressPoolEnableStatusRequest,
    ) -> Result<UpdateCloudGtmAddressPoolEnableStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateCloudGtmAddressPoolEnableStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数修改地址池备注信息。
    pub async fn update_cloud_gtm_address_pool_remark(
        &self,
        request: UpdateCloudGtmAddressPoolRemarkRequest,
    ) -> Result<UpdateCloudGtmAddressPoolRemarkResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateCloudGtmAddressPoolRemark",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数修改地址池负载均衡策略。
    pub async fn update_cloud_gtm_address_pool_lb_strategy(
        &self,
        request: UpdateCloudGtmAddressPoolLbStrategyRequest,
    ) -> Result<UpdateCloudGtmAddressPoolLbStrategyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateCloudGtmAddressPoolLbStrategy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数修改地址池基础配置。
    pub async fn update_cloud_gtm_address_pool_basic_config(
        &self,
        request: UpdateCloudGtmAddressPoolBasicConfigRequest,
    ) -> Result<UpdateCloudGtmAddressPoolBasicConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateCloudGtmAddressPoolBasicConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数替换地址池引用的地址。
    pub async fn replace_cloud_gtm_address_pool_address(
        &self,
        request: ReplaceCloudGtmAddressPoolAddressRequest,
    ) -> Result<ReplaceCloudGtmAddressPoolAddressResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ReplaceCloudGtmAddressPoolAddress",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数获取地址池被实例引用的信息。
    pub async fn describe_cloud_gtm_address_pool_reference(
        &self,
        request: DescribeCloudGtmAddressPoolReferenceRequest,
    ) -> Result<DescribeCloudGtmAddressPoolReferenceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCloudGtmAddressPoolReference",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询健康检查模板列表信息。
    pub async fn search_cloud_gtm_monitor_templates(
        &self,
        request: SearchCloudGtmMonitorTemplatesRequest,
    ) -> Result<SearchCloudGtmMonitorTemplatesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SearchCloudGtmMonitorTemplates",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数创建健康探测模板。
    pub async fn create_cloud_gtm_monitor_template(
        &self,
        request: CreateCloudGtmMonitorTemplateRequest,
    ) -> Result<CreateCloudGtmMonitorTemplateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateCloudGtmMonitorTemplate",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数删除健康探测模板。
    pub async fn delete_cloud_gtm_monitor_template(
        &self,
        request: DeleteCloudGtmMonitorTemplateRequest,
    ) -> Result<DeleteCloudGtmMonitorTemplateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteCloudGtmMonitorTemplate",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数修改探测模板信息。
    pub async fn update_cloud_gtm_monitor_template(
        &self,
        request: UpdateCloudGtmMonitorTemplateRequest,
    ) -> Result<UpdateCloudGtmMonitorTemplateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateCloudGtmMonitorTemplate",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数获取健康探测模板列表。
    pub async fn list_cloud_gtm_monitor_templates(
        &self,
        request: ListCloudGtmMonitorTemplatesRequest,
    ) -> Result<ListCloudGtmMonitorTemplatesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListCloudGtmMonitorTemplates",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数获取指定健康探测模板配置。
    pub async fn describe_cloud_gtm_monitor_template(
        &self,
        request: DescribeCloudGtmMonitorTemplateRequest,
    ) -> Result<DescribeCloudGtmMonitorTemplateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCloudGtmMonitorTemplate",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 
    pub async fn update_cloud_gtm_monitor_template_remark(
        &self,
        request: UpdateCloudGtmMonitorTemplateRemarkRequest,
    ) -> Result<UpdateCloudGtmMonitorTemplateRemarkResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateCloudGtmMonitorTemplateRemark",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数获取健康探测检查探点列表。
    pub async fn list_cloud_gtm_monitor_nodes(
        &self,
        request: ListCloudGtmMonitorNodesRequest,
    ) -> Result<ListCloudGtmMonitorNodesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListCloudGtmMonitorNodes",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询GTM支持的系统线路
    pub async fn describe_cloud_gtm_system_lines(
        &self,
        request: DescribeCloudGtmSystemLinesRequest,
    ) -> Result<DescribeCloudGtmSystemLinesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCloudGtmSystemLines",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取告警日志列表
    pub async fn list_cloud_gtm_alert_logs(
        &self,
        request: ListCloudGtmAlertLogsRequest,
    ) -> Result<ListCloudGtmAlertLogsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListCloudGtmAlertLogs",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取实例配置的告警配置
    pub async fn describe_cloud_gtm_instance_config_alert(
        &self,
        request: DescribeCloudGtmInstanceConfigAlertRequest,
    ) -> Result<DescribeCloudGtmInstanceConfigAlertResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCloudGtmInstanceConfigAlert",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新全局流量管理告警设置
    pub async fn update_cloud_gtm_global_alert(
        &self,
        request: UpdateCloudGtmGlobalAlertRequest,
    ) -> Result<UpdateCloudGtmGlobalAlertResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateCloudGtmGlobalAlert",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询全局流量管理告警配置
    pub async fn describe_cloud_gtm_global_alert(
        &self,
        request: DescribeCloudGtmGlobalAlertRequest,
    ) -> Result<DescribeCloudGtmGlobalAlertResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCloudGtmGlobalAlert",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 
    pub async fn update_cloud_gtm_instance_config_alert(
        &self,
        request: UpdateCloudGtmInstanceConfigAlertRequest,
    ) -> Result<UpdateCloudGtmInstanceConfigAlertResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateCloudGtmInstanceConfigAlert",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 
    pub async fn list_cloud_gtm_available_alert_groups(
        &self,
        request: ListCloudGtmAvailableAlertGroupsRequest,
    ) -> Result<ListCloudGtmAvailableAlertGroupsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListCloudGtmAvailableAlertGroups",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数修改生效地址池切换策略。
    pub async fn set_gtm_access_mode(
        &self,
        request: SetGtmAccessModeRequest,
    ) -> Result<SetGtmAccessModeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SetGtmAccessMode",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 通过GTM实例ID修改实例配置。
    pub async fn update_dns_gtm_instance_global_config(
        &self,
        request: UpdateDnsGtmInstanceGlobalConfigRequest,
    ) -> Result<UpdateDnsGtmInstanceGlobalConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateDnsGtmInstanceGlobalConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数修改GTM实例配置。
    pub async fn update_gtm_instance_global_config(
        &self,
        request: UpdateGtmInstanceGlobalConfigRequest,
    ) -> Result<UpdateGtmInstanceGlobalConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateGtmInstanceGlobalConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改访问策略。
    pub async fn update_dns_gtm_access_strategy(
        &self,
        request: UpdateDnsGtmAccessStrategyRequest,
    ) -> Result<UpdateDnsGtmAccessStrategyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateDnsGtmAccessStrategy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询地址归属区域。
    pub async fn describe_dns_gtm_addr_attribute_info(
        &self,
        request: DescribeDnsGtmAddrAttributeInfoRequest,
    ) -> Result<DescribeDnsGtmAddrAttributeInfoResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDnsGtmAddrAttributeInfo",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取可设置的报警组。
    pub async fn describe_dns_gtm_available_alert_group(
        &self,
        request: DescribeDnsGtmAvailableAlertGroupRequest,
    ) -> Result<DescribeDnsGtmAvailableAlertGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDnsGtmAvailableAlertGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 通过实例ID获取实例详情。
    pub async fn describe_dns_gtm_instance(
        &self,
        request: DescribeDnsGtmInstanceRequest,
    ) -> Result<DescribeDnsGtmInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDnsGtmInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取GTM实例的详情信息。
    pub async fn describe_gtm_instance(
        &self,
        request: DescribeGtmInstanceRequest,
    ) -> Result<DescribeGtmInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeGtmInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取实例列表。
    pub async fn describe_dns_gtm_instances(
        &self,
        request: DescribeDnsGtmInstancesRequest,
    ) -> Result<DescribeDnsGtmInstancesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDnsGtmInstances",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取GTM实例列表。
    pub async fn describe_gtm_instances(
        &self,
        request: DescribeGtmInstancesRequest,
    ) -> Result<DescribeGtmInstancesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeGtmInstances",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 通过实例ID获取实例当前状态。
    pub async fn describe_dns_gtm_instance_status(
        &self,
        request: DescribeDnsGtmInstanceStatusRequest,
    ) -> Result<DescribeDnsGtmInstanceStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDnsGtmInstanceStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取实例的当前状态。
    pub async fn describe_gtm_instance_status(
        &self,
        request: DescribeGtmInstanceStatusRequest,
    ) -> Result<DescribeGtmInstanceStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeGtmInstanceStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 通过实例ID获取操作日志列表。
    pub async fn describe_dns_gtm_logs(
        &self,
        request: DescribeDnsGtmLogsRequest,
    ) -> Result<DescribeDnsGtmLogsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDnsGtmLogs",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取GTM实例可用的报警组列表。
    pub async fn describe_gtm_available_alert_group(
        &self,
        request: DescribeGtmAvailableAlertGroupRequest,
    ) -> Result<DescribeGtmAvailableAlertGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeGtmAvailableAlertGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取日志列表。
    pub async fn describe_gtm_logs(
        &self,
        request: DescribeGtmLogsRequest,
    ) -> Result<DescribeGtmLogsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeGtmLogs",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取系统分配的cname域名。
    pub async fn describe_gtm_instance_system_cname(
        &self,
        request: DescribeGtmInstanceSystemCnameRequest,
    ) -> Result<DescribeGtmInstanceSystemCnameResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeGtmInstanceSystemCname",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 通过实例ID获取系统分配的CNAME域名。
    pub async fn describe_dns_gtm_instance_system_cname(
        &self,
        request: DescribeDnsGtmInstanceSystemCnameRequest,
    ) -> Result<DescribeDnsGtmInstanceSystemCnameResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDnsGtmInstanceSystemCname",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 复制GTM配置。
    pub async fn copy_gtm_config(
        &self,
        request: CopyGtmConfigRequest,
    ) -> Result<CopyGtmConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CopyGtmConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数修改GTM实例的资源组ID。
    pub async fn move_gtm_resource_group(
        &self,
        request: MoveGtmResourceGroupRequest,
    ) -> Result<MoveGtmResourceGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "MoveGtmResourceGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 检查实例主机名是否可添加
    pub async fn validate_dns_gtm_cname_rr_can_use(
        &self,
        request: ValidateDnsGtmCnameRrCanUseRequest,
    ) -> Result<ValidateDnsGtmCnameRrCanUseResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ValidateDnsGtmCnameRrCanUse",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 新增地址池。
    pub async fn add_gtm_address_pool(
        &self,
        request: AddGtmAddressPoolRequest,
    ) -> Result<AddGtmAddressPoolResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddGtmAddressPool",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 新增地址池。
    pub async fn add_dns_gtm_address_pool(
        &self,
        request: AddDnsGtmAddressPoolRequest,
    ) -> Result<AddDnsGtmAddressPoolResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddDnsGtmAddressPool",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 通过地址池ID删除地址池。
    pub async fn delete_dns_gtm_address_pool(
        &self,
        request: DeleteDnsGtmAddressPoolRequest,
    ) -> Result<DeleteDnsGtmAddressPoolResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteDnsGtmAddressPool",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除地址池。
    pub async fn delete_gtm_address_pool(
        &self,
        request: DeleteGtmAddressPoolRequest,
    ) -> Result<DeleteGtmAddressPoolResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteGtmAddressPool",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据填写的参数修改地址池。
    pub async fn update_dns_gtm_address_pool(
        &self,
        request: UpdateDnsGtmAddressPoolRequest,
    ) -> Result<UpdateDnsGtmAddressPoolResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateDnsGtmAddressPool",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改地址池。
    pub async fn update_gtm_address_pool(
        &self,
        request: UpdateGtmAddressPoolRequest,
    ) -> Result<UpdateGtmAddressPoolResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateGtmAddressPool",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 通过地址池ID获取地址池详情。
    pub async fn describe_dns_gtm_instance_address_pool(
        &self,
        request: DescribeDnsGtmInstanceAddressPoolRequest,
    ) -> Result<DescribeDnsGtmInstanceAddressPoolResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDnsGtmInstanceAddressPool",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 通过GTM实例ID获取地址池列表。
    pub async fn describe_dns_gtm_instance_address_pools(
        &self,
        request: DescribeDnsGtmInstanceAddressPoolsRequest,
    ) -> Result<DescribeDnsGtmInstanceAddressPoolsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDnsGtmInstanceAddressPools",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 通过GTM实例ID获取地址池可设置配置。
    pub async fn describe_dns_gtm_address_pool_available_config(
        &self,
        request: DescribeDnsGtmAddressPoolAvailableConfigRequest,
    ) -> Result<DescribeDnsGtmAddressPoolAvailableConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDnsGtmAddressPoolAvailableConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取地址池详细信息。
    pub async fn describe_gtm_instance_address_pool(
        &self,
        request: DescribeGtmInstanceAddressPoolRequest,
    ) -> Result<DescribeGtmInstanceAddressPoolResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeGtmInstanceAddressPool",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取GTM实例地址池列表。
    pub async fn describe_gtm_instance_address_pools(
        &self,
        request: DescribeGtmInstanceAddressPoolsRequest,
    ) -> Result<DescribeGtmInstanceAddressPoolsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeGtmInstanceAddressPools",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建健康检查。
    pub async fn add_dns_gtm_monitor(
        &self,
        request: AddDnsGtmMonitorRequest,
    ) -> Result<AddDnsGtmMonitorResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddDnsGtmMonitor",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建健康检查。
    pub async fn add_gtm_monitor(
        &self,
        request: AddGtmMonitorRequest,
    ) -> Result<AddGtmMonitorResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddGtmMonitor",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改健康检查。
    pub async fn update_dns_gtm_monitor(
        &self,
        request: UpdateDnsGtmMonitorRequest,
    ) -> Result<UpdateDnsGtmMonitorResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateDnsGtmMonitor",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改健康检查。
    pub async fn update_gtm_monitor(
        &self,
        request: UpdateGtmMonitorRequest,
    ) -> Result<UpdateGtmMonitorResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateGtmMonitor",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置地址池健康检查状态。
    pub async fn set_dns_gtm_monitor_status(
        &self,
        request: SetDnsGtmMonitorStatusRequest,
    ) -> Result<SetDnsGtmMonitorStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SetDnsGtmMonitorStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置地址池健康检查状态。
    pub async fn set_gtm_monitor_status(
        &self,
        request: SetGtmMonitorStatusRequest,
    ) -> Result<SetGtmMonitorStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SetGtmMonitorStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取健康检查可设置的配置。
    pub async fn describe_gtm_monitor_available_config(
        &self,
        request: DescribeGtmMonitorAvailableConfigRequest,
    ) -> Result<DescribeGtmMonitorAvailableConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeGtmMonitorAvailableConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取地址池健康检查配置。
    pub async fn describe_gtm_monitor_config(
        &self,
        request: DescribeGtmMonitorConfigRequest,
    ) -> Result<DescribeGtmMonitorConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeGtmMonitorConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取 DNS 健康检查可设置的配置。
    pub async fn describe_dns_gtm_monitor_available_config(
        &self,
        request: DescribeDnsGtmMonitorAvailableConfigRequest,
    ) -> Result<DescribeDnsGtmMonitorAvailableConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDnsGtmMonitorAvailableConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取地址池健康检查配置。
    pub async fn describe_dns_gtm_monitor_config(
        &self,
        request: DescribeDnsGtmMonitorConfigRequest,
    ) -> Result<DescribeDnsGtmMonitorConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDnsGtmMonitorConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 新增访问策略。
    pub async fn add_dns_gtm_access_strategy(
        &self,
        request: AddDnsGtmAccessStrategyRequest,
    ) -> Result<AddDnsGtmAccessStrategyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddDnsGtmAccessStrategy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数新增访问策略。
    pub async fn add_gtm_access_strategy(
        &self,
        request: AddGtmAccessStrategyRequest,
    ) -> Result<AddGtmAccessStrategyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddGtmAccessStrategy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 通过策略ID删除访问策略。
    pub async fn delete_dns_gtm_access_strategy(
        &self,
        request: DeleteDnsGtmAccessStrategyRequest,
    ) -> Result<DeleteDnsGtmAccessStrategyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteDnsGtmAccessStrategy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数删除访问策略。
    pub async fn delete_gtm_access_strategy(
        &self,
        request: DeleteGtmAccessStrategyRequest,
    ) -> Result<DeleteGtmAccessStrategyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteGtmAccessStrategy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据传入参数修改访问策略。
    pub async fn update_gtm_access_strategy(
        &self,
        request: UpdateGtmAccessStrategyRequest,
    ) -> Result<UpdateGtmAccessStrategyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateGtmAccessStrategy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调整生效地址池访问策略。
    pub async fn set_dns_gtm_access_mode(
        &self,
        request: SetDnsGtmAccessModeRequest,
    ) -> Result<SetDnsGtmAccessModeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SetDnsGtmAccessMode",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 通过实例ID获取实例访问策略列表。
    pub async fn describe_dns_gtm_access_strategies(
        &self,
        request: DescribeDnsGtmAccessStrategiesRequest,
    ) -> Result<DescribeDnsGtmAccessStrategiesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDnsGtmAccessStrategies",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 通过访问策略ID获取访问策略详细信息。
    pub async fn describe_dns_gtm_access_strategy(
        &self,
        request: DescribeDnsGtmAccessStrategyRequest,
    ) -> Result<DescribeDnsGtmAccessStrategyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDnsGtmAccessStrategy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 通过实例ID获取访问策略可设置的配置。
    pub async fn describe_dns_gtm_access_strategy_available_config(
        &self,
        request: DescribeDnsGtmAccessStrategyAvailableConfigRequest,
    ) -> Result<DescribeDnsGtmAccessStrategyAvailableConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDnsGtmAccessStrategyAvailableConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取当前实例的访问策略列表。
    pub async fn describe_gtm_access_strategies(
        &self,
        request: DescribeGtmAccessStrategiesRequest,
    ) -> Result<DescribeGtmAccessStrategiesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeGtmAccessStrategies",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据策略ID获取策略详细信息。
    pub async fn describe_gtm_access_strategy(
        &self,
        request: DescribeGtmAccessStrategyRequest,
    ) -> Result<DescribeGtmAccessStrategyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeGtmAccessStrategy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取当前访问策略可设置的配置。
    pub async fn describe_gtm_access_strategy_available_config(
        &self,
        request: DescribeGtmAccessStrategyAvailableConfigRequest,
    ) -> Result<DescribeGtmAccessStrategyAvailableConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeGtmAccessStrategyAvailableConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 通过实例ID切换访问策略类型。
    pub async fn switch_dns_gtm_instance_strategy_mode(
        &self,
        request: SwitchDnsGtmInstanceStrategyModeRequest,
    ) -> Result<SwitchDnsGtmInstanceStrategyModeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SwitchDnsGtmInstanceStrategyMode",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 新增容灾预案。
    pub async fn add_gtm_recovery_plan(
        &self,
        request: AddGtmRecoveryPlanRequest,
    ) -> Result<AddGtmRecoveryPlanResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddGtmRecoveryPlan",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除容灾预案。
    pub async fn delete_gtm_recovery_plan(
        &self,
        request: DeleteGtmRecoveryPlanRequest,
    ) -> Result<DeleteGtmRecoveryPlanResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteGtmRecoveryPlan",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改容灾预案。
    pub async fn update_gtm_recovery_plan(
        &self,
        request: UpdateGtmRecoveryPlanRequest,
    ) -> Result<UpdateGtmRecoveryPlanResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateGtmRecoveryPlan",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取容灾预案详情。
    pub async fn describe_gtm_recovery_plan(
        &self,
        request: DescribeGtmRecoveryPlanRequest,
    ) -> Result<DescribeGtmRecoveryPlanResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeGtmRecoveryPlan",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取容灾预案可设置的配置。
    pub async fn describe_gtm_recovery_plan_available_config(
        &self,
        request: DescribeGtmRecoveryPlanAvailableConfigRequest,
    ) -> Result<DescribeGtmRecoveryPlanAvailableConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeGtmRecoveryPlanAvailableConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取容灾预案列表。
    pub async fn describe_gtm_recovery_plans(
        &self,
        request: DescribeGtmRecoveryPlansRequest,
    ) -> Result<DescribeGtmRecoveryPlansResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeGtmRecoveryPlans",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 执行容灾预案。
    pub async fn execute_gtm_recovery_plan(
        &self,
        request: ExecuteGtmRecoveryPlanRequest,
    ) -> Result<ExecuteGtmRecoveryPlanResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ExecuteGtmRecoveryPlan",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 预览容灾预案。
    pub async fn preview_gtm_recovery_plan(
        &self,
        request: PreviewGtmRecoveryPlanRequest,
    ) -> Result<PreviewGtmRecoveryPlanResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PreviewGtmRecoveryPlan",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 回滚容灾预案。
    pub async fn rollback_gtm_recovery_plan(
        &self,
        request: RollbackGtmRecoveryPlanRequest,
    ) -> Result<RollbackGtmRecoveryPlanResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RollbackGtmRecoveryPlan",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 新增HTTPDNS内置权威域名zone
    pub async fn add_recursion_zone(
        &self,
        request: AddRecursionZoneRequest,
    ) -> Result<AddRecursionZoneResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddRecursionZone",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询HTTPDNS内置权威域名zone详情
    pub async fn describe_recursion_zone(
        &self,
        request: DescribeRecursionZoneRequest,
    ) -> Result<DescribeRecursionZoneResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeRecursionZone",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询HTTPDNS内置权威域名zone内置权威域名zone
    pub async fn list_recursion_zones(
        &self,
        request: ListRecursionZonesRequest,
    ) -> Result<ListRecursionZonesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListRecursionZones",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除HTTPDNS内置权威域名zone
    pub async fn delete_recursion_zone(
        &self,
        request: DeleteRecursionZoneRequest,
    ) -> Result<DeleteRecursionZoneResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteRecursionZone",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 搜索HTTPDNS内置权威域名zone
    pub async fn search_recursion_zones(
        &self,
        request: SearchRecursionZonesRequest,
    ) -> Result<SearchRecursionZonesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SearchRecursionZones",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改HTTPDNS内置权威域名zone生效范围
    pub async fn update_recursion_zone_effective_scope(
        &self,
        request: UpdateRecursionZoneEffectiveScopeRequest,
    ) -> Result<UpdateRecursionZoneEffectiveScopeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateRecursionZoneEffectiveScope",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改HTTPDNS内置权威域名zone备注
    pub async fn update_recursion_zone_remark(
        &self,
        request: UpdateRecursionZoneRemarkRequest,
    ) -> Result<UpdateRecursionZoneRemarkResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateRecursionZoneRemark",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改HTTPDNS内置权威域名zone递归代理模式
    pub async fn update_recursion_zone_proxy_pattern(
        &self,
        request: UpdateRecursionZoneProxyPatternRequest,
    ) -> Result<UpdateRecursionZoneProxyPatternResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateRecursionZoneProxyPattern",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 新增递归解析内置权威解析记录。
    pub async fn add_recursion_record(
        &self,
        request: AddRecursionRecordRequest,
    ) -> Result<AddRecursionRecordResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddRecursionRecord",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询HTTPDNS内置权威解析记录详情
    pub async fn describe_recursion_record(
        &self,
        request: DescribeRecursionRecordRequest,
    ) -> Result<DescribeRecursionRecordResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeRecursionRecord",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改HTTPDNS内置权威解析记录
    pub async fn update_recursion_record(
        &self,
        request: UpdateRecursionRecordRequest,
    ) -> Result<UpdateRecursionRecordResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateRecursionRecord",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询HTTPDNS内置权威解析记录
    pub async fn list_recursion_records(
        &self,
        request: ListRecursionRecordsRequest,
    ) -> Result<ListRecursionRecordsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListRecursionRecords",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除HTTPDNS内置权威解析记录
    pub async fn delete_recursion_record(
        &self,
        request: DeleteRecursionRecordRequest,
    ) -> Result<DeleteRecursionRecordResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteRecursionRecord",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 搜索HTTPDNS内置权威解析记录
    pub async fn search_recursion_records(
        &self,
        request: SearchRecursionRecordsRequest,
    ) -> Result<SearchRecursionRecordsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SearchRecursionRecords",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改HTTPDNS内置权威解析记录启用状态
    pub async fn update_recursion_record_enable_status(
        &self,
        request: UpdateRecursionRecordEnableStatusRequest,
    ) -> Result<UpdateRecursionRecordEnableStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateRecursionRecordEnableStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改HTTPDNS内置权威解析记录权重
    pub async fn update_recursion_record_weight(
        &self,
        request: UpdateRecursionRecordWeightRequest,
    ) -> Result<UpdateRecursionRecordWeightResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateRecursionRecordWeight",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改递归解析内置权威解析记录权重算法启用状态
    pub async fn update_recursion_record_weight_enable_status(
        &self,
        request: UpdateRecursionRecordWeightEnableStatusRequest,
    ) -> Result<UpdateRecursionRecordWeightEnableStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateRecursionRecordWeightEnableStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改HTTPDNS内置权威解析记录备注
    pub async fn update_recursion_record_remark(
        &self,
        request: UpdateRecursionRecordRemarkRequest,
    ) -> Result<UpdateRecursionRecordRemarkResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateRecursionRecordRemark",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取DoH账户请求量统计概览。
    pub async fn describe_doh_account_statistics(
        &self,
        request: DescribeDohAccountStatisticsRequest,
    ) -> Result<DescribeDohAccountStatisticsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDohAccountStatistics",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取DoH域名请求量统计概览。
    pub async fn describe_doh_domain_statistics(
        &self,
        request: DescribeDohDomainStatisticsRequest,
    ) -> Result<DescribeDohDomainStatisticsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDohDomainStatistics",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取DoH域名的请求量统计列表。
    pub async fn describe_doh_domain_statistics_summary(
        &self,
        request: DescribeDohDomainStatisticsSummaryRequest,
    ) -> Result<DescribeDohDomainStatisticsSummaryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDohDomainStatisticsSummary",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取DoH子域名请求量统计。
    pub async fn describe_doh_sub_domain_statistics(
        &self,
        request: DescribeDohSubDomainStatisticsRequest,
    ) -> Result<DescribeDohSubDomainStatisticsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDohSubDomainStatistics",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取DoH子域名的请求量统计列表。
    pub async fn describe_doh_sub_domain_statistics_summary(
        &self,
        request: DescribeDohSubDomainStatisticsSummaryRequest,
    ) -> Result<DescribeDohSubDomainStatisticsSummaryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDohSubDomainStatisticsSummary",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取DoH用户基本信息。
    pub async fn describe_doh_user_info(
        &self,
        request: DescribeDohUserInfoRequest,
    ) -> Result<DescribeDohUserInfoResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDohUserInfo",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建公共DNS密钥 AccessKey。
    pub async fn create_pdns_app_key(
        &self,
        request: CreatePdnsAppKeyRequest,
    ) -> Result<CreatePdnsAppKeyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreatePdnsAppKey",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除公共DNS AppKey
    pub async fn remove_pdns_app_key(
        &self,
        request: RemovePdnsAppKeyRequest,
    ) -> Result<RemovePdnsAppKeyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RemovePdnsAppKey",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取公共DNS AppKey 详情
    pub async fn describe_pdns_app_key(
        &self,
        request: DescribePdnsAppKeyRequest,
    ) -> Result<DescribePdnsAppKeyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribePdnsAppKey",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取公共DNS AppKey 列表
    pub async fn describe_pdns_app_keys(
        &self,
        request: DescribePdnsAppKeysRequest,
    ) -> Result<DescribePdnsAppKeysResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribePdnsAppKeys",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改 AppKey 状态
    pub async fn update_app_key_state(
        &self,
        request: UpdateAppKeyStateRequest,
    ) -> Result<UpdateAppKeyStateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateAppKeyState",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建公共DNS Udp Ip地址段
    pub async fn create_pdns_udp_ip_segment(
        &self,
        request: CreatePdnsUdpIpSegmentRequest,
    ) -> Result<CreatePdnsUdpIpSegmentResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreatePdnsUdpIpSegment",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 验证公共DNS Udp Ip地址段
    pub async fn validate_pdns_udp_ip_segment(
        &self,
        request: ValidatePdnsUdpIpSegmentRequest,
    ) -> Result<ValidatePdnsUdpIpSegmentResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ValidatePdnsUdpIpSegment",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取公共DNS Udp IP段列表
    pub async fn describe_pdns_udp_ip_segments(
        &self,
        request: DescribePdnsUdpIpSegmentsRequest,
    ) -> Result<DescribePdnsUdpIpSegmentsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribePdnsUdpIpSegments",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除公共DNS Udp Ip地址段
    pub async fn remove_pdns_udp_ip_segment(
        &self,
        request: RemovePdnsUdpIpSegmentRequest,
    ) -> Result<RemovePdnsUdpIpSegmentResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RemovePdnsUdpIpSegment",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取公共DNS 威胁统计列表
    pub async fn describe_pdns_threat_statistics(
        &self,
        request: DescribePdnsThreatStatisticsRequest,
    ) -> Result<DescribePdnsThreatStatisticsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribePdnsThreatStatistics",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取公共DNS用户信息。
    pub async fn describe_pdns_user_info(
        &self,
        request: DescribePdnsUserInfoRequest,
    ) -> Result<DescribePdnsUserInfoResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribePdnsUserInfo",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询公共DNS的请求量列表。
    pub async fn describe_pdns_request_statistic(
        &self,
        request: DescribePdnsRequestStatisticRequest,
    ) -> Result<DescribePdnsRequestStatisticResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribePdnsRequestStatistic",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取公共DNS 威胁日志列表
    pub async fn describe_pdns_threat_logs(
        &self,
        request: DescribePdnsThreatLogsRequest,
    ) -> Result<DescribePdnsThreatLogsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribePdnsThreatLogs",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定子域名请求量的统计数据。
    pub async fn describe_pdns_request_statistics(
        &self,
        request: DescribePdnsRequestStatisticsRequest,
    ) -> Result<DescribePdnsRequestStatisticsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribePdnsRequestStatistics",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取公共DNS 操作日志列表
    pub async fn describe_pdns_operate_logs(
        &self,
        request: DescribePdnsOperateLogsRequest,
    ) -> Result<DescribePdnsOperateLogsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribePdnsOperateLogs",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 恢复公共DNS服务
    pub async fn resume_pdns_service(
        &self,
        request: ResumePdnsServiceRequest,
    ) -> Result<ResumePdnsServiceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ResumePdnsService",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取公共DNS用户数据概览
    pub async fn describe_pdns_account_summary(
        &self,
        request: DescribePdnsAccountSummaryRequest,
    ) -> Result<DescribePdnsAccountSummaryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribePdnsAccountSummary",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 暂停公共DNS服务
    pub async fn pause_pdns_service(
        &self,
        request: PausePdnsServiceRequest,
    ) -> Result<PausePdnsServiceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PausePdnsService",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取公共DNS 威胁统计
    pub async fn describe_pdns_threat_statistic(
        &self,
        request: DescribePdnsThreatStatisticRequest,
    ) -> Result<DescribePdnsThreatStatisticResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribePdnsThreatStatistic",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改缓存刷新套餐包配置
    pub async fn update_isp_flush_cache_instance_config(
        &self,
        request: UpdateIspFlushCacheInstanceConfigRequest,
    ) -> Result<UpdateIspFlushCacheInstanceConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateIspFlushCacheInstanceConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 提交缓存刷新任务
    pub async fn submit_isp_flush_cache_task(
        &self,
        request: SubmitIspFlushCacheTaskRequest,
    ) -> Result<SubmitIspFlushCacheTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SubmitIspFlushCacheTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取剩余可缓存刷新次数
    pub async fn describe_isp_flush_cache_remain_quota(
        &self,
        request: DescribeIspFlushCacheRemainQuotaRequest,
    ) -> Result<DescribeIspFlushCacheRemainQuotaResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeIspFlushCacheRemainQuota",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取缓存刷新套餐包列表
    pub async fn describe_isp_flush_cache_instances(
        &self,
        request: DescribeIspFlushCacheInstancesRequest,
    ) -> Result<DescribeIspFlushCacheInstancesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeIspFlushCacheInstances",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取缓存刷新任务详情
    pub async fn describe_isp_flush_cache_task(
        &self,
        request: DescribeIspFlushCacheTaskRequest,
    ) -> Result<DescribeIspFlushCacheTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeIspFlushCacheTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取缓存刷新任务列表
    pub async fn describe_isp_flush_cache_tasks(
        &self,
        request: DescribeIspFlushCacheTasksRequest,
    ) -> Result<DescribeIspFlushCacheTasksResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeIspFlushCacheTasks",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定域名下的全部子域名的请求量统计。
    pub async fn describe_record_resolve_statistics_summary(
        &self,
        request: DescribeRecordResolveStatisticsSummaryRequest,
    ) -> Result<DescribeRecordResolveStatisticsSummaryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeRecordResolveStatisticsSummary",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询用户账号下所有付费域名的请求量列表。
    pub async fn describe_domain_resolve_statistics_summary(
        &self,
        request: DescribeDomainResolveStatisticsSummaryRequest,
    ) -> Result<DescribeDomainResolveStatisticsSummaryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainResolveStatisticsSummary",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询解析日志
    pub async fn describe_internet_dns_logs(
        &self,
        request: DescribeInternetDnsLogsRequest,
    ) -> Result<DescribeInternetDnsLogsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeInternetDnsLogs",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

}