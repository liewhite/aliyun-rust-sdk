//! API 调用 - 自动生成，请勿手动修改

use crate::client::{Client, ClientConfig, ApiRequest, HttpMethod};
use crate::error::SdkError;
use super::types::*;

/// ddoscoo API 版本
pub const API_VERSION: &str = "2020-01-01";

/// ddoscoo 客户端
#[derive(Debug, Clone)]
pub struct DdoscooClient {
    client: Client,
}

impl DdoscooClient {
    /// 创建新客户端
    pub fn new(config: ClientConfig) -> Result<Self, SdkError> {
        Ok(Self {
            client: Client::new(config)?,
        })
    }

    /// 释放已经到期的DDoS高防实例。
    pub async fn release_instance(
        &self,
        request: ReleaseInstanceRequest,
    ) -> Result<ReleaseInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ReleaseInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 编辑DDoS高防实例的备注。
    pub async fn modify_instance_remark(
        &self,
        request: ModifyInstanceRemarkRequest,
    ) -> Result<ModifyInstanceRemarkResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyInstanceRemark",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改DDoS高防（中国内地）实例的弹性防护带宽。
    pub async fn modify_elastic_band_width(
        &self,
        request: ModifyElasticBandWidthRequest,
    ) -> Result<ModifyElasticBandWidthResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyElasticBandWidth",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询DDoS高防实例的ID、版本、备注、IP类型信息。
    pub async fn describe_instance_ids(
        &self,
        request: DescribeInstanceIdsRequest,
    ) -> Result<DescribeInstanceIdsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeInstanceIds",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定的DDoS高防实例的状态。
    pub async fn describe_instance_status(
        &self,
        request: DescribeInstanceStatusRequest,
    ) -> Result<DescribeInstanceStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeInstanceStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询DDoS高防（中国内地）实例的可选弹性防护带宽规格。
    pub async fn describe_elastic_bandwidth_spec(
        &self,
        request: DescribeElasticBandwidthSpecRequest,
    ) -> Result<DescribeElasticBandwidthSpecResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeElasticBandwidthSpec",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询DDoS高防实例的统计信息，例如已防护的域名、端口数量等。
    pub async fn describe_instance_statistics(
        &self,
        request: DescribeInstanceStatisticsRequest,
    ) -> Result<DescribeInstanceStatisticsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeInstanceStatistics",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询DDoS高防实例的详情列表。
    pub async fn describe_instances(
        &self,
        request: DescribeInstancesRequest,
    ) -> Result<DescribeInstancesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeInstances",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询DDoS高防实例的规格配置。
    pub async fn describe_instance_specs(
        &self,
        request: DescribeInstanceSpecsRequest,
    ) -> Result<DescribeInstanceSpecsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeInstanceSpecs",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询DDoS高防实例的IP和线路信息。
    pub async fn describe_instance_details(
        &self,
        request: DescribeInstanceDetailsRequest,
    ) -> Result<DescribeInstanceDetailsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeInstanceDetails",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置DDoS高防实例的弹性QPS和模式。
    pub async fn modify_elastic_biz_qps(
        &self,
        request: ModifyElasticBizQpsRequest,
    ) -> Result<ModifyElasticBizQpsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyElasticBizQps",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 切换弹性业务带宽的95计费模式。
    pub async fn modify_qps_mode(
        &self,
        request: ModifyQpsModeRequest,
    ) -> Result<ModifyQpsModeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyQpsMode",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询DDoS高防实例的扩展信息。
    pub async fn describe_instance_ext(
        &self,
        request: DescribeInstanceExtRequest,
    ) -> Result<DescribeInstanceExtResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeInstanceExt",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建一条网站业务转发规则。
    pub async fn create_web_rule(
        &self,
        request: CreateWebRuleRequest,
    ) -> Result<CreateWebRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateWebRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除网站业务转发规则。
    pub async fn delete_web_rule(
        &self,
        request: DeleteWebRuleRequest,
    ) -> Result<DeleteWebRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteWebRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改已经创建的网站业务转发规则。
    pub async fn modify_web_rule(
        &self,
        request: ModifyWebRuleRequest,
    ) -> Result<ModifyWebRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyWebRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 编辑网站业务转发规则的TLS安全策略。
    pub async fn modify_tls_config(
        &self,
        request: ModifyTlsConfigRequest,
    ) -> Result<ModifyTlsConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyTlsConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置网站业务转发规则的HTTP2.0开关状态。
    pub async fn modify_http2_enable(
        &self,
        request: ModifyHttp2EnableRequest,
    ) -> Result<ModifyHttp2EnableResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyHttp2Enable",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置网站业务的接入模式。
    pub async fn modify_web_access_mode(
        &self,
        request: ModifyWebAccessModeRequest,
    ) -> Result<ModifyWebAccessModeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyWebAccessMode",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为网站业务开启或关闭CNAME复用。
    pub async fn modify_cname_reuse(
        &self,
        request: ModifyCnameReuseRequest,
    ) -> Result<ModifyCnameReuseResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyCnameReuse",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询网站业务转发规则的配置。
    pub async fn describe_web_rules(
        &self,
        request: DescribeWebRulesRequest,
    ) -> Result<DescribeWebRulesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeWebRules",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询网站业务关联的DDoS高防实例信息。
    pub async fn describe_web_instance_relations(
        &self,
        request: DescribeWebInstanceRelationsRequest,
    ) -> Result<DescribeWebInstanceRelationsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeWebInstanceRelations",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询用户适用于当前域名的所有证书信息，而非当前使用的证书。
    pub async fn describe_certs(
        &self,
        request: DescribeCertsRequest,
    ) -> Result<DescribeCertsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCerts",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询DDoS高防支持的网站业务自定义端口范围。
    pub async fn describe_web_custom_ports(
        &self,
        request: DescribeWebCustomPortsRequest,
    ) -> Result<DescribeWebCustomPortsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeWebCustomPorts",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询网站业务的接入模式。
    pub async fn describe_web_access_mode(
        &self,
        request: DescribeWebAccessModeRequest,
    ) -> Result<DescribeWebAccessModeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeWebAccessMode",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询网站业务的CNAME复用信息。
    pub async fn describe_cname_reuses(
        &self,
        request: DescribeCnameReusesRequest,
    ) -> Result<DescribeCnameReusesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCnameReuses",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询网站业务转发规则的回源策略。
    pub async fn describe_l7_rs_policy(
        &self,
        request: DescribeL7RsPolicyRequest,
    ) -> Result<DescribeL7RsPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeL7RsPolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为网站业务转发规则关联SSL证书。
    pub async fn associate_web_cert(
        &self,
        request: AssociateWebCertRequest,
    ) -> Result<AssociateWebCertResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AssociateWebCert",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为已经创建的网站业务转发规则设置回源策略。
    pub async fn config_l7_rs_policy(
        &self,
        request: ConfigL7RsPolicyRequest,
    ) -> Result<ConfigL7RsPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ConfigL7RsPolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 是否启用 OCSP （Online Certificate Status Protocol）功能。
    pub async fn modify_ocsp_status(
        &self,
        request: ModifyOcspStatusRequest,
    ) -> Result<ModifyOcspStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyOcspStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 配置域名回源长连接设置。
    pub async fn config_l7_us_keepalive(
        &self,
        request: ConfigL7UsKeepaliveRequest,
    ) -> Result<ConfigL7UsKeepaliveResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ConfigL7UsKeepalive",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询域名回源长连接设置。
    pub async fn describe_l7_us_keepalive(
        &self,
        request: DescribeL7UsKeepaliveRequest,
    ) -> Result<DescribeL7UsKeepaliveResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeL7UsKeepalive",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改DDoS高防实例中已创建的域名自定义Header。
    pub async fn modify_headers(
        &self,
        request: ModifyHeadersRequest,
    ) -> Result<ModifyHeadersResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyHeaders",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询域名的自定义Header。
    pub async fn describe_headers(
        &self,
        request: DescribeHeadersRequest,
    ) -> Result<DescribeHeadersResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeHeaders",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询网站的HTTP 2.0 指纹信息。
    pub async fn describe_domain_h2_fingerprint(
        &self,
        request: DescribeDomainH2FingerprintRequest,
    ) -> Result<DescribeDomainH2FingerprintResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainH2Fingerprint",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询网站Top N Referer。
    pub async fn describe_domain_top_referer(
        &self,
        request: DescribeDomainTopRefererRequest,
    ) -> Result<DescribeDomainTopRefererResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainTopReferer",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询网站Top UserAgent。
    pub async fn describe_domain_top_user_agent(
        &self,
        request: DescribeDomainTopUserAgentRequest,
    ) -> Result<DescribeDomainTopUserAgentResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainTopUserAgent",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询网站Top N 客户端指纹。
    pub async fn describe_domain_top_fingerprint(
        &self,
        request: DescribeDomainTopFingerprintRequest,
    ) -> Result<DescribeDomainTopFingerprintResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainTopFingerprint",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询网站带宽流量。
    pub async fn describe_domain_bps(
        &self,
        request: DescribeDomainBpsRequest,
    ) -> Result<DescribeDomainBpsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainBps",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询网站Top N的HTTP_Method。
    pub async fn describe_domain_top_http_method(
        &self,
        request: DescribeDomainTopHttpMethodRequest,
    ) -> Result<DescribeDomainTopHttpMethodResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainTopHttpMethod",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建异步导出任务，例如导出网站业务转发规则、端口转发规则、会话保持和健康检查配置、DDoS防护策略、IP黑白名单。
    pub async fn create_async_task(
        &self,
        request: CreateAsyncTaskRequest,
    ) -> Result<CreateAsyncTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateAsyncTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除异步导出任务。
    pub async fn delete_async_task(
        &self,
        request: DeleteAsyncTaskRequest,
    ) -> Result<DeleteAsyncTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteAsyncTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建端口转发规则。
    pub async fn create_network_rules(
        &self,
        request: CreateNetworkRulesRequest,
    ) -> Result<CreateNetworkRulesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateNetworkRules",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 目前不支持批量删除，每次只允许删除一个对象。
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

    /// 编辑端口转发规则的健康检查配置（四层或七层）。
    pub async fn modify_health_check_config(
        &self,
        request: ModifyHealthCheckConfigRequest,
    ) -> Result<ModifyHealthCheckConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyHealthCheckConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询端口转发规则。
    pub async fn describe_network_rules(
        &self,
        request: DescribeNetworkRulesRequest,
    ) -> Result<DescribeNetworkRulesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeNetworkRules",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询源站健康检查状态信息。
    pub async fn describe_health_check_status(
        &self,
        request: DescribeHealthCheckStatusRequest,
    ) -> Result<DescribeHealthCheckStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeHealthCheckStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改源站IP地址。
    pub async fn config_network_rules(
        &self,
        request: ConfigNetworkRulesRequest,
    ) -> Result<ConfigNetworkRulesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ConfigNetworkRules",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建流量调度器调度规则。
    pub async fn create_scheduler_rule(
        &self,
        request: CreateSchedulerRuleRequest,
    ) -> Result<CreateSchedulerRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateSchedulerRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除流量调度器调度规则。
    pub async fn delete_scheduler_rule(
        &self,
        request: DeleteSchedulerRuleRequest,
    ) -> Result<DeleteSchedulerRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteSchedulerRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 编辑流量调度器调度规则。
    pub async fn modify_scheduler_rule(
        &self,
        request: ModifySchedulerRuleRequest,
    ) -> Result<ModifySchedulerRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifySchedulerRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询流量调度器的调度规则。
    pub async fn describe_scheduler_rules(
        &self,
        request: DescribeSchedulerRulesRequest,
    ) -> Result<DescribeSchedulerRulesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeSchedulerRules",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 将业务流量切换到DDoS高防实例进行清洗、回切到联动资源。
    pub async fn switch_scheduler_rule(
        &self,
        request: SwitchSchedulerRuleRequest,
    ) -> Result<SwitchSchedulerRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SwitchSchedulerRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 此API用于查询指定域名下的CDN联动规则配置。
    pub async fn describe_cdn_linkage_rules(
        &self,
        request: DescribeCdnLinkageRulesRequest,
    ) -> Result<DescribeCdnLinkageRulesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCdnLinkageRules",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 添加针对DDoS高防实例的黑名单IP。
    pub async fn add_auto_cc_blacklist(
        &self,
        request: AddAutoCcBlacklistRequest,
    ) -> Result<AddAutoCcBlacklistResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddAutoCcBlacklist",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 添加针对DDoS高防实例的白名单IP。
    pub async fn add_auto_cc_whitelist(
        &self,
        request: AddAutoCcWhitelistRequest,
    ) -> Result<AddAutoCcWhitelistResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddAutoCcWhitelist",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除针对DDoS高防实例的黑名单IP。
    pub async fn delete_auto_cc_blacklist(
        &self,
        request: DeleteAutoCcBlacklistRequest,
    ) -> Result<DeleteAutoCcBlacklistResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteAutoCcBlacklist",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除针对DDoS高防实例的白名单IP。
    pub async fn delete_auto_cc_whitelist(
        &self,
        request: DeleteAutoCcWhitelistRequest,
    ) -> Result<DeleteAutoCcWhitelistResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteAutoCcWhitelist",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 执行黑洞解封。
    pub async fn modify_blackhole_status(
        &self,
        request: ModifyBlackholeStatusRequest,
    ) -> Result<ModifyBlackholeStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyBlackholeStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置DDoS高防（中国内地）实例的近源流量压制。
    pub async fn modify_block_status(
        &self,
        request: ModifyBlockStatusRequest,
    ) -> Result<ModifyBlockStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyBlockStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询针对DDoS高防实例的黑名单和白名单IP的数量。
    pub async fn describe_auto_cc_list_count(
        &self,
        request: DescribeAutoCcListCountRequest,
    ) -> Result<DescribeAutoCcListCountResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeAutoCcListCount",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询针对DDoS高防实例的黑名单IP。
    pub async fn describe_auto_cc_blacklist(
        &self,
        request: DescribeAutoCcBlacklistRequest,
    ) -> Result<DescribeAutoCcBlacklistResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeAutoCcBlacklist",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询针对DDoS高防实例的白名单IP。
    pub async fn describe_auto_cc_whitelist(
        &self,
        request: DescribeAutoCcWhitelistRequest,
    ) -> Result<DescribeAutoCcWhitelistResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeAutoCcWhitelist",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询黑洞解封次数。
    pub async fn describe_un_blackhole_count(
        &self,
        request: DescribeUnBlackholeCountRequest,
    ) -> Result<DescribeUnBlackholeCountResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeUnBlackholeCount",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询DDoS高防实例的黑洞状态。
    pub async fn describe_blackhole_status(
        &self,
        request: DescribeBlackholeStatusRequest,
    ) -> Result<DescribeBlackholeStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeBlackholeStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询针对DDoS高防实例的区域封禁配置。
    pub async fn describe_network_region_block(
        &self,
        request: DescribeNetworkRegionBlockRequest,
    ) -> Result<DescribeNetworkRegionBlockResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeNetworkRegionBlock",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询DDoS高防（中国内地）实例的近源流量压制配置。
    pub async fn describe_block_status(
        &self,
        request: DescribeBlockStatusRequest,
    ) -> Result<DescribeBlockStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeBlockStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询可用的近源流量压制次数。
    pub async fn describe_un_block_count(
        &self,
        request: DescribeUnBlockCountRequest,
    ) -> Result<DescribeUnBlockCountResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeUnBlockCount",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 清空针对DDoS高防实例的黑名单IP。
    pub async fn empty_auto_cc_blacklist(
        &self,
        request: EmptyAutoCcBlacklistRequest,
    ) -> Result<EmptyAutoCcBlacklistResponse, SdkError> {
        let api_request = ApiRequest {
            action: "EmptyAutoCcBlacklist",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 清空针对DDoS高防实例的白名单IP。
    pub async fn empty_auto_cc_whitelist(
        &self,
        request: EmptyAutoCcWhitelistRequest,
    ) -> Result<EmptyAutoCcWhitelistResponse, SdkError> {
        let api_request = ApiRequest {
            action: "EmptyAutoCcWhitelist",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置针对DDoS高防实例的区域封禁。
    pub async fn config_network_region_block(
        &self,
        request: ConfigNetworkRegionBlockRequest,
    ) -> Result<ConfigNetworkRegionBlockResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ConfigNetworkRegionBlock",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定的DDoS高防实例上被UDP反射攻击防护策略过滤的反射源端口。
    pub async fn describe_udp_reflect(
        &self,
        request: DescribeUdpReflectRequest,
    ) -> Result<DescribeUdpReflectResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeUdpReflect",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 添加UDP反射攻击防护策略，过滤指定的反射源端口。
    pub async fn config_udp_reflect(
        &self,
        request: ConfigUdpReflectRequest,
    ) -> Result<ConfigUdpReflectResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ConfigUdpReflect",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建网站业务频率控制防护（CC防护）的自定义规则。
    pub async fn create_web_cc_rule(
        &self,
        request: CreateWebCCRuleRequest,
    ) -> Result<CreateWebCCRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateWebCCRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除网站业务频率控制防护（CC防护）的自定义规则。
    pub async fn delete_web_cc_rule(
        &self,
        request: DeleteWebCCRuleRequest,
    ) -> Result<DeleteWebCCRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteWebCCRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除网站业务精确访问控制规则。
    pub async fn delete_web_precise_access_rule(
        &self,
        request: DeleteWebPreciseAccessRuleRequest,
    ) -> Result<DeleteWebPreciseAccessRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteWebPreciseAccessRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置网站业务AI智能防护的开关状态。
    pub async fn modify_web_ai_protect_switch(
        &self,
        request: ModifyWebAIProtectSwitchRequest,
    ) -> Result<ModifyWebAIProtectSwitchResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyWebAIProtectSwitch",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置网站业务AI智能防护的模式。
    pub async fn modify_web_ai_protect_mode(
        &self,
        request: ModifyWebAIProtectModeRequest,
    ) -> Result<ModifyWebAIProtectModeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyWebAIProtectMode",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置网站业务黑白名单（针对域名）的开关状态。
    pub async fn modify_web_ip_set_switch(
        &self,
        request: ModifyWebIpSetSwitchRequest,
    ) -> Result<ModifyWebIpSetSwitchResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyWebIpSetSwitch",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 开启网站业务频率控制防护（CC防护）的开关。
    pub async fn enable_web_cc(
        &self,
        request: EnableWebCCRequest,
    ) -> Result<EnableWebCCResponse, SdkError> {
        let api_request = ApiRequest {
            action: "EnableWebCC",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 关闭网站业务频率控制防护（CC防护）的开关。
    pub async fn disable_web_cc(
        &self,
        request: DisableWebCCRequest,
    ) -> Result<DisableWebCCResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DisableWebCC",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 开启网站业务频率控制防护（CC防护）的自定义规则开关。
    pub async fn enable_web_cc_rule(
        &self,
        request: EnableWebCCRuleRequest,
    ) -> Result<EnableWebCCRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "EnableWebCCRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 关闭网站业务频率控制防护（CC防护）的自定义规则开关。
    pub async fn disable_web_cc_rule(
        &self,
        request: DisableWebCCRuleRequest,
    ) -> Result<DisableWebCCRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DisableWebCCRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 编辑网站业务频率控制防护（CC防护）的自定义规则。
    pub async fn modify_web_cc_rule(
        &self,
        request: ModifyWebCCRuleRequest,
    ) -> Result<ModifyWebCCRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyWebCCRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置网站业务精确访问控制的开关状态。
    pub async fn modify_web_precise_access_switch(
        &self,
        request: ModifyWebPreciseAccessSwitchRequest,
    ) -> Result<ModifyWebPreciseAccessSwitchResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyWebPreciseAccessSwitch",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建或编辑网站业务精确访问控制规则。
    pub async fn modify_web_precise_access_rule(
        &self,
        request: ModifyWebPreciseAccessRuleRequest,
    ) -> Result<ModifyWebPreciseAccessRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyWebPreciseAccessRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置网站业务区域封禁（针对域名）的开关状态。
    pub async fn modify_web_area_block_switch(
        &self,
        request: ModifyWebAreaBlockSwitchRequest,
    ) -> Result<ModifyWebAreaBlockSwitchResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyWebAreaBlockSwitch",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置网站业务区域封禁（针对域名）的封禁地区。
    pub async fn modify_web_area_block(
        &self,
        request: ModifyWebAreaBlockRequest,
    ) -> Result<ModifyWebAreaBlockResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyWebAreaBlock",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询网站业务各防护功能的开关状态。
    pub async fn describe_web_cc_protect_switch(
        &self,
        request: DescribeWebCcProtectSwitchRequest,
    ) -> Result<DescribeWebCcProtectSwitchResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeWebCcProtectSwitch",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询网站业务频率控制防护（CC防护）的自定义规则。
    pub async fn describe_web_cc_rules(
        &self,
        request: DescribeWebCCRulesRequest,
    ) -> Result<DescribeWebCCRulesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeWebCCRules",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询网站业务精确访问控制规则。
    pub async fn describe_web_precise_access_rule(
        &self,
        request: DescribeWebPreciseAccessRuleRequest,
    ) -> Result<DescribeWebPreciseAccessRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeWebPreciseAccessRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询网站业务区域封禁（针对域名）的配置信息。
    pub async fn describe_web_area_block_configs(
        &self,
        request: DescribeWebAreaBlockConfigsRequest,
    ) -> Result<DescribeWebAreaBlockConfigsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeWebAreaBlockConfigs",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置针对网站业务的黑名单和白名单IP。
    pub async fn config_web_ip_set(
        &self,
        request: ConfigWebIpSetRequest,
    ) -> Result<ConfigWebIpSetResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ConfigWebIpSet",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置网站业务频率控制防护（CC防护）的防护模式。
    pub async fn config_web_cc_template(
        &self,
        request: ConfigWebCCTemplateRequest,
    ) -> Result<ConfigWebCCTemplateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ConfigWebCCTemplate",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除网站业务频率控制防护（CC防护）的自定义规则。
    pub async fn delete_web_cc_rule_v2(
        &self,
        request: DeleteWebCCRuleV2Request,
    ) -> Result<DeleteWebCCRuleV2Response, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteWebCCRuleV2",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询网站业务频率控制防护（CC防护）的自定义规则。
    pub async fn describe_web_cc_rules_v2(
        &self,
        request: DescribeWebCCRulesV2Request,
    ) -> Result<DescribeWebCCRulesV2Response, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeWebCCRulesV2",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建或编辑网站业务频率控制防护（CC防护）的自定义规则。
    pub async fn config_web_cc_rule_v2(
        &self,
        request: ConfigWebCCRuleV2Request,
    ) -> Result<ConfigWebCCRuleV2Response, SdkError> {
        let api_request = ApiRequest {
            action: "ConfigWebCCRuleV2",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于设置网站业务CC安全防护模块的开关状态。
    pub async fn modify_web_cc_global_switch(
        &self,
        request: ModifyWebCCGlobalSwitchRequest,
    ) -> Result<ModifyWebCCGlobalSwitchResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyWebCCGlobalSwitch",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 配置DDoS全局防护策略。
    pub async fn config_l7_global_rule(
        &self,
        request: ConfigL7GlobalRuleRequest,
    ) -> Result<ConfigL7GlobalRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ConfigL7GlobalRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询DDoS全局防护策略。
    pub async fn describe_l7_global_rule(
        &self,
        request: DescribeL7GlobalRuleRequest,
    ) -> Result<DescribeL7GlobalRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeL7GlobalRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定域名的cc防护功能开关状态
    pub async fn describe_domain_cc_protect_switch(
        &self,
        request: DescribeDomainCcProtectSwitchRequest,
    ) -> Result<DescribeDomainCcProtectSwitchResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainCcProtectSwitch",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 配置DDoS全局防护策略的开关和等级。
    pub async fn config_domain_security_profile(
        &self,
        request: ConfigDomainSecurityProfileRequest,
    ) -> Result<ConfigDomainSecurityProfileResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ConfigDomainSecurityProfile",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置非网站业务AI智能防护。
    pub async fn modify_port_auto_cc_status(
        &self,
        request: ModifyPortAutoCcStatusRequest,
    ) -> Result<ModifyPortAutoCcStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyPortAutoCcStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 编辑端口转发规则的会话保持和DDoS防护策略设置。
    pub async fn modify_network_rule_attribute(
        &self,
        request: ModifyNetworkRuleAttributeRequest,
    ) -> Result<ModifyNetworkRuleAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyNetworkRuleAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询非网站业务AI智能防护的配置信息。
    pub async fn describe_port_auto_cc_status(
        &self,
        request: DescribePortAutoCcStatusRequest,
    ) -> Result<DescribePortAutoCcStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribePortAutoCcStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询已配置网站业务转发规则的域名。
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

    /// 查询端口转发规则的健康检查配置（四层或七层）。
    pub async fn describe_health_check_list(
        &self,
        request: DescribeHealthCheckListRequest,
    ) -> Result<DescribeHealthCheckListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeHealthCheckList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询非网站业务端口转发规则的防护配置，包括会话保持和DDoS防护策略。
    pub async fn describe_network_rule_attributes(
        &self,
        request: DescribeNetworkRuleAttributesRequest,
    ) -> Result<DescribeNetworkRuleAttributesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeNetworkRuleAttributes",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建定制场景策略。
    pub async fn create_scene_defense_policy(
        &self,
        request: CreateSceneDefensePolicyRequest,
    ) -> Result<CreateSceneDefensePolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateSceneDefensePolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除定制场景策略。
    pub async fn delete_scene_defense_policy(
        &self,
        request: DeleteSceneDefensePolicyRequest,
    ) -> Result<DeleteSceneDefensePolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteSceneDefensePolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 编辑定制场景策略。
    pub async fn modify_scene_defense_policy(
        &self,
        request: ModifySceneDefensePolicyRequest,
    ) -> Result<ModifySceneDefensePolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifySceneDefensePolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为定制场景策略添加防护对象。
    pub async fn attach_scene_defense_object(
        &self,
        request: AttachSceneDefenseObjectRequest,
    ) -> Result<AttachSceneDefenseObjectResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AttachSceneDefenseObject",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 移除定制场景策略的防护对象。
    pub async fn detach_scene_defense_object(
        &self,
        request: DetachSceneDefenseObjectRequest,
    ) -> Result<DetachSceneDefenseObjectResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DetachSceneDefenseObject",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 启用定制场景策略。
    pub async fn enable_scene_defense_policy(
        &self,
        request: EnableSceneDefensePolicyRequest,
    ) -> Result<EnableSceneDefensePolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "EnableSceneDefensePolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 禁用定制场景策略。
    pub async fn disable_scene_defense_policy(
        &self,
        request: DisableSceneDefensePolicyRequest,
    ) -> Result<DisableSceneDefensePolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DisableSceneDefensePolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询定制场景策略的详细配置。
    pub async fn describe_scene_defense_policies(
        &self,
        request: DescribeSceneDefensePoliciesRequest,
    ) -> Result<DescribeSceneDefensePoliciesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeSceneDefensePolicies",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询定制场景策略的防护对象。
    pub async fn describe_scene_defense_objects(
        &self,
        request: DescribeSceneDefenseObjectsRequest,
    ) -> Result<DescribeSceneDefenseObjectsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeSceneDefenseObjects",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除网站业务静态页面缓存的自定义规则。
    pub async fn delete_web_cache_custom_rule(
        &self,
        request: DeleteWebCacheCustomRuleRequest,
    ) -> Result<DeleteWebCacheCustomRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteWebCacheCustomRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置网站业务静态页面缓存的开关状态。
    pub async fn modify_web_cache_switch(
        &self,
        request: ModifyWebCacheSwitchRequest,
    ) -> Result<ModifyWebCacheSwitchResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyWebCacheSwitch",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置网站业务静态页面缓存的缓存模式。
    pub async fn modify_web_cache_mode(
        &self,
        request: ModifyWebCacheModeRequest,
    ) -> Result<ModifyWebCacheModeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyWebCacheMode",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置网站业务静态页面缓存的自定义规则。
    pub async fn modify_web_cache_custom_rule(
        &self,
        request: ModifyWebCacheCustomRuleRequest,
    ) -> Result<ModifyWebCacheCustomRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyWebCacheCustomRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询网站业务静态页面缓存的配置。
    pub async fn describe_web_cache_configs(
        &self,
        request: DescribeWebCacheConfigsRequest,
    ) -> Result<DescribeWebCacheConfigsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeWebCacheConfigs",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询流量型攻击峰值（bps）、连接型攻击峰值（cps）、Web资源耗尽型攻击峰值（qps）。
    pub async fn describe_d_dos_event_max(
        &self,
        request: DescribeDDosEventMaxRequest,
    ) -> Result<DescribeDDosEventMaxResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDDosEventMax",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询某次流量型攻击的来源地域详情。
    pub async fn describe_d_dos_event_area(
        &self,
        request: DescribeDDosEventAreaRequest,
    ) -> Result<DescribeDDosEventAreaResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDDosEventArea",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询某次流量型攻击的攻击类型详情。
    pub async fn describe_d_dos_event_attack_type(
        &self,
        request: DescribeDDosEventAttackTypeRequest,
    ) -> Result<DescribeDDosEventAttackTypeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDDosEventAttackType",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询某次流量型攻击的攻击来源网络运营商（ISP）信息。
    pub async fn describe_d_dos_event_isp(
        &self,
        request: DescribeDDosEventIspRequest,
    ) -> Result<DescribeDDosEventIspResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDDosEventIsp",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询某次流量型攻击的攻击来源IP详情。
    pub async fn describe_d_dos_event_src_ip(
        &self,
        request: DescribeDDosEventSrcIpRequest,
    ) -> Result<DescribeDDosEventSrcIpResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDDosEventSrcIp",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询DDoS高防的回源IP网段。
    pub async fn describe_back_source_cidr(
        &self,
        request: DescribeBackSourceCidrRequest,
    ) -> Result<DescribeBackSourceCidrResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeBackSourceCidr",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询DDoS攻击事件列表。
    pub async fn describe_d_dos_all_event_list(
        &self,
        request: DescribeDDosAllEventListRequest,
    ) -> Result<DescribeDDosAllEventListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDDosAllEventList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询针对DDoS高防实例的攻击事件。
    pub async fn describe_d_do_s_events(
        &self,
        request: DescribeDDoSEventsRequest,
    ) -> Result<DescribeDDoSEventsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDDoSEvents",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询目的限速事件列表。
    pub async fn describe_sla_event_list(
        &self,
        request: DescribeSlaEventListRequest,
    ) -> Result<DescribeSlaEventListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeSlaEventList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询针对网站业务的攻击事件。
    pub async fn describe_domain_attack_events(
        &self,
        request: DescribeDomainAttackEventsRequest,
    ) -> Result<DescribeDomainAttackEventsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainAttackEvents",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询DDoS高防（非中国内地）服务的高级防护次数统计数据。
    pub async fn describe_defense_count_statistics(
        &self,
        request: DescribeDefenseCountStatisticsRequest,
    ) -> Result<DescribeDefenseCountStatisticsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDefenseCountStatistics",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定时间段内DDoS攻击的峰值（qps）。
    pub async fn describe_attack_analysis_max_qps(
        &self,
        request: DescribeAttackAnalysisMaxQpsRequest,
    ) -> Result<DescribeAttackAnalysisMaxQpsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeAttackAnalysisMaxQps",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询DDoS高防被攻击IP的目的端口包数量信息。
    pub async fn describe_destination_port_event(
        &self,
        request: DescribeDestinationPortEventRequest,
    ) -> Result<DescribeDestinationPortEventResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDestinationPortEvent",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询DDoS高防实例的流量数据列表。
    pub async fn describe_port_flow_list(
        &self,
        request: DescribePortFlowListRequest,
    ) -> Result<DescribePortFlowListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribePortFlowList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询DDoS高防实例的端口连接数列表。
    pub async fn describe_port_conns_list(
        &self,
        request: DescribePortConnsListRequest,
    ) -> Result<DescribePortConnsListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribePortConnsList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询DDoS高防实例的端口连接数统计信息。
    pub async fn describe_port_conns_count(
        &self,
        request: DescribePortConnsCountRequest,
    ) -> Result<DescribePortConnsCountResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribePortConnsCount",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询DDoS高防实例的端口连接峰值信息。
    pub async fn describe_port_max_conns(
        &self,
        request: DescribePortMaxConnsRequest,
    ) -> Result<DescribePortMaxConnsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribePortMaxConns",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定时间段内DDoS高防受到的攻击带宽和包速峰值。
    pub async fn describe_port_attack_max_flow(
        &self,
        request: DescribePortAttackMaxFlowRequest,
    ) -> Result<DescribePortAttackMaxFlowResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribePortAttackMaxFlow",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定时间段内DDoS高防实例的请求来源国家分布。
    pub async fn describe_port_view_source_countries(
        &self,
        request: DescribePortViewSourceCountriesRequest,
    ) -> Result<DescribePortViewSourceCountriesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribePortViewSourceCountries",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定时间段内DDoS高防实例的请求来源运营商分布。
    pub async fn describe_port_view_source_isps(
        &self,
        request: DescribePortViewSourceIspsRequest,
    ) -> Result<DescribePortViewSourceIspsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribePortViewSourceIsps",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定时间段内DDoS高防实例的请求来源（中国）省份分布。
    pub async fn describe_port_view_source_provinces(
        &self,
        request: DescribePortViewSourceProvincesRequest,
    ) -> Result<DescribePortViewSourceProvincesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribePortViewSourceProvinces",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询网站业务的QPS统计信息。
    pub async fn describe_domain_qps_list(
        &self,
        request: DescribeDomainQPSListRequest,
    ) -> Result<DescribeDomainQPSListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainQPSList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询网站业务的响应状态码统计信息。
    pub async fn describe_domain_status_code_list(
        &self,
        request: DescribeDomainStatusCodeListRequest,
    ) -> Result<DescribeDomainStatusCodeListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainStatusCodeList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询网站业务攻击总览，包括HTTP攻击峰值、HTTPS攻击峰值。
    pub async fn describe_domain_overview(
        &self,
        request: DescribeDomainOverviewRequest,
    ) -> Result<DescribeDomainOverviewResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainOverview",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定时间段内网站业务的各类响应状态码的统计信息。
    pub async fn describe_domain_status_code_count(
        &self,
        request: DescribeDomainStatusCodeCountRequest,
    ) -> Result<DescribeDomainStatusCodeCountResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainStatusCodeCount",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定时间段内网站业务的QPS峰值数据，包括攻击QPS、总QPS。
    pub async fn describe_domain_top_attack_list(
        &self,
        request: DescribeDomainTopAttackListRequest,
    ) -> Result<DescribeDomainTopAttackListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainTopAttackList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定时间段内网站业务的请求来源国家分布。
    pub async fn describe_domain_view_source_countries(
        &self,
        request: DescribeDomainViewSourceCountriesRequest,
    ) -> Result<DescribeDomainViewSourceCountriesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainViewSourceCountries",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定时间段内网站业务的请求来源（中国）省份分布。
    pub async fn describe_domain_view_source_provinces(
        &self,
        request: DescribeDomainViewSourceProvincesRequest,
    ) -> Result<DescribeDomainViewSourceProvincesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainViewSourceProvinces",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定时间段内网站业务的请求耗时最大的前N个URL。
    pub async fn describe_domain_view_top_cost_time(
        &self,
        request: DescribeDomainViewTopCostTimeRequest,
    ) -> Result<DescribeDomainViewTopCostTimeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainViewTopCostTime",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定时间段内网站业务访问量最大的前N个URL。
    pub async fn describe_domain_view_top_url(
        &self,
        request: DescribeDomainViewTopUrlRequest,
    ) -> Result<DescribeDomainViewTopUrlResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainViewTopUrl",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为网站业务开启全量日志分析。
    pub async fn enable_web_access_log_config(
        &self,
        request: EnableWebAccessLogConfigRequest,
    ) -> Result<EnableWebAccessLogConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "EnableWebAccessLogConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 编辑DDoS高防全量日志的存储时长。
    pub async fn modify_full_log_ttl(
        &self,
        request: ModifyFullLogTtlRequest,
    ) -> Result<ModifyFullLogTtlResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyFullLogTtl",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为网站业务关闭全量日志分析。
    pub async fn disable_web_access_log_config(
        &self,
        request: DisableWebAccessLogConfigRequest,
    ) -> Result<DisableWebAccessLogConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DisableWebAccessLogConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询所有域名的全量日志开关状态。
    pub async fn describe_web_access_log_dispatch_status(
        &self,
        request: DescribeWebAccessLogDispatchStatusRequest,
    ) -> Result<DescribeWebAccessLogDispatchStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeWebAccessLogDispatchStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 例如开关状态、对接的日志项目、日志库。
    pub async fn describe_web_access_log_status(
        &self,
        request: DescribeWebAccessLogStatusRequest,
    ) -> Result<DescribeWebAccessLogStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeWebAccessLogStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询是否已创建DDoS高防的日志库。
    pub async fn describe_log_store_exist_status(
        &self,
        request: DescribeLogStoreExistStatusRequest,
    ) -> Result<DescribeLogStoreExistStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeLogStoreExistStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询DDoS高防（中国内地）的操作日志。
    pub async fn describe_op_entities(
        &self,
        request: DescribeOpEntitiesRequest,
    ) -> Result<DescribeOpEntitiesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeOpEntities",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 即是否授权DDoS高防访问日志服务。
    pub async fn describe_sls_auth_status(
        &self,
        request: DescribeSlsAuthStatusRequest,
    ) -> Result<DescribeSlsAuthStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeSlsAuthStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 例如日志存储容量、日志存储时长。
    pub async fn describe_sls_logstore_info(
        &self,
        request: DescribeSlsLogstoreInfoRequest,
    ) -> Result<DescribeSlsLogstoreInfoResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeSlsLogstoreInfo",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询阿里云日志服务SLS的开通状态。
    pub async fn describe_sls_open_status(
        &self,
        request: DescribeSlsOpenStatusRequest,
    ) -> Result<DescribeSlsOpenStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeSlsOpenStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询可用的清空日志库的次数。
    pub async fn describe_web_access_log_empty_count(
        &self,
        request: DescribeWebAccessLogEmptyCountRequest,
    ) -> Result<DescribeWebAccessLogEmptyCountResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeWebAccessLogEmptyCount",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询DDoS高防（国际）的高级防护日志。
    pub async fn describe_defense_records(
        &self,
        request: DescribeDefenseRecordsRequest,
    ) -> Result<DescribeDefenseRecordsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDefenseRecords",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 清空DDoS高防的日志库。
    pub async fn empty_sls_logstore(
        &self,
        request: EmptySlsLogstoreRequest,
    ) -> Result<EmptySlsLogstoreResponse, SdkError> {
        let api_request = ApiRequest {
            action: "EmptySlsLogstore",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询是否授权DDoS高防服务访问其他云产品。
    pub async fn describe_sts_grant_status(
        &self,
        request: DescribeStsGrantStatusRequest,
    ) -> Result<DescribeStsGrantStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeStsGrantStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 例如任务ID、任务开始和结束时间、任务状态、任务参数、任务结果等。
    pub async fn describe_async_tasks(
        &self,
        request: DescribeAsyncTasksRequest,
    ) -> Result<DescribeAsyncTasksResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeAsyncTasks",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询弹性业务带宽的账单详情。
    pub async fn describe_system_log(
        &self,
        request: DescribeSystemLogRequest,
    ) -> Result<DescribeSystemLogResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeSystemLog",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询DDoS高防实例弹性QPS账单曲线图。
    pub async fn describe_elastic_qps(
        &self,
        request: DescribeElasticQpsRequest,
    ) -> Result<DescribeElasticQpsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeElasticQps",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询实例弹性QPS详情。
    pub async fn describe_elastic_qps_record(
        &self,
        request: DescribeElasticQpsRecordRequest,
    ) -> Result<DescribeElasticQpsRecordResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeElasticQpsRecord",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为DDoS高防实例绑定标签。
    pub async fn create_tag_resources(
        &self,
        request: CreateTagResourcesRequest,
    ) -> Result<CreateTagResourcesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateTagResources",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为DDoS高防（中国内地）实例移除标签。
    pub async fn delete_tag_resources(
        &self,
        request: DeleteTagResourcesRequest,
    ) -> Result<DeleteTagResourcesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteTagResources",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询所有标签键及标签键关联的DDoS高防（中国内地）实例的数量。
    pub async fn describe_tag_keys(
        &self,
        request: DescribeTagKeysRequest,
    ) -> Result<DescribeTagKeysResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeTagKeys",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询DDoS高防（中国内地）实例绑定的标签信息。
    pub async fn describe_tag_resources(
        &self,
        request: DescribeTagResourcesRequest,
    ) -> Result<DescribeTagResourcesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeTagResources",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建一条端口转发规则。
    pub async fn create_port(
        &self,
        request: CreatePortRequest,
    ) -> Result<CreatePortResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreatePort",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除指定的端口转发规则。
    pub async fn delete_port(
        &self,
        request: DeletePortRequest,
    ) -> Result<DeletePortResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeletePort",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改已创建的端口转发规则。
    pub async fn modify_port(
        &self,
        request: ModifyPortRequest,
    ) -> Result<ModifyPortResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyPort",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询DDoS高防实例下已创建的端口转发规则。
    pub async fn describe_port(
        &self,
        request: DescribePortRequest,
    ) -> Result<DescribePortResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribePort",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建一条网站业务转发规则。
    pub async fn create_domain_resource(
        &self,
        request: CreateDomainResourceRequest,
    ) -> Result<CreateDomainResourceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateDomainResource",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除指定的网站业务转发规则。
    pub async fn delete_domain_resource(
        &self,
        request: DeleteDomainResourceRequest,
    ) -> Result<DeleteDomainResourceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteDomainResource",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询网站业务转发规则的配置。
    pub async fn describe_domain_resource(
        &self,
        request: DescribeDomainResourceRequest,
    ) -> Result<DescribeDomainResourceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainResource",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改已创建的网站业务转发规则。
    pub async fn modify_domain_resource(
        &self,
        request: ModifyDomainResourceRequest,
    ) -> Result<ModifyDomainResourceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDomainResource",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为端口转发规则添加备注。
    pub async fn config_layer4_remark(
        &self,
        request: ConfigLayer4RemarkRequest,
    ) -> Result<ConfigLayer4RemarkResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ConfigLayer4Remark",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改端口转发规则的回源模式（启用或关闭主备回源）。
    pub async fn config_layer4_rule_bak_mode(
        &self,
        request: ConfigLayer4RuleBakModeRequest,
    ) -> Result<ConfigLayer4RuleBakModeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ConfigLayer4RuleBakMode",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为端口转发规则设置主、备源站IP地址。
    pub async fn config_layer4_rule_policy(
        &self,
        request: ConfigLayer4RulePolicyRequest,
    ) -> Result<ConfigLayer4RulePolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ConfigLayer4RulePolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询端口转发规则的回源设置。
    pub async fn describe_layer4_rule_policy(
        &self,
        request: DescribeLayer4RulePolicyRequest,
    ) -> Result<DescribeLayer4RulePolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeLayer4RulePolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// DDoS高防实例规格变配。
    pub async fn modify_instance(
        &self,
        request: ModifyInstanceRequest,
    ) -> Result<ModifyInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

}