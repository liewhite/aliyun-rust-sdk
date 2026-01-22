//! API 调用 - 自动生成，请勿手动修改

use crate::client::{Client, ClientConfig, ApiRequest, HttpMethod};
use crate::error::SdkError;
use super::types::*;

/// Slb API 版本
pub const API_VERSION: &str = "2014-05-15";

/// Slb 客户端
#[derive(Debug, Clone)]
pub struct SlbClient {
    client: Client,
}

impl SlbClient {
    /// 创建新客户端
    pub fn new(config: ClientConfig) -> Result<Self, SdkError> {
        Ok(Self {
            client: Client::new(config)?,
        })
    }

    /// 查询某个Region的可用区支持的资源售卖情况，可用的资源。
    pub async fn describe_available_resource(
        &self,
        request: DescribeAvailableResourceRequest,
    ) -> Result<DescribeAvailableResourceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeAvailableResource",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询可用地域。
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

    /// 查询指定地域的可用区信息。
    pub async fn describe_zones(
        &self,
        request: DescribeZonesRequest,
    ) -> Result<DescribeZonesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeZones",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建负载均衡实例。
    pub async fn create_load_balancer(
        &self,
        request: CreateLoadBalancerRequest,
    ) -> Result<CreateLoadBalancerResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateLoadBalancer",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除后付费的负载均衡实例。
    pub async fn delete_load_balancer(
        &self,
        request: DeleteLoadBalancerRequest,
    ) -> Result<DeleteLoadBalancerResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteLoadBalancer",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改负载均衡的实例规格。
    pub async fn modify_load_balancer_instance_spec(
        &self,
        request: ModifyLoadBalancerInstanceSpecRequest,
    ) -> Result<ModifyLoadBalancerInstanceSpecResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyLoadBalancerInstanceSpec",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改公网负载均衡实例的计费方式。
    pub async fn modify_load_balancer_internet_spec(
        &self,
        request: ModifyLoadBalancerInternetSpecRequest,
    ) -> Result<ModifyLoadBalancerInternetSpecResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyLoadBalancerInternetSpec",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 将按量计费实例转换为包年包月实例。
    pub async fn modify_load_balancer_pay_type(
        &self,
        request: ModifyLoadBalancerPayTypeRequest,
    ) -> Result<ModifyLoadBalancerPayTypeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyLoadBalancerPayType",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置实例删除保护状态。
    pub async fn set_load_balancer_delete_protection(
        &self,
        request: SetLoadBalancerDeleteProtectionRequest,
    ) -> Result<SetLoadBalancerDeleteProtectionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SetLoadBalancerDeleteProtection",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置负载均衡实例修改保护状态。
    pub async fn set_load_balancer_modification_protection(
        &self,
        request: SetLoadBalancerModificationProtectionRequest,
    ) -> Result<SetLoadBalancerModificationProtectionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SetLoadBalancerModificationProtection",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改负载均衡实例的名称。
    pub async fn set_load_balancer_name(
        &self,
        request: SetLoadBalancerNameRequest,
    ) -> Result<SetLoadBalancerNameResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SetLoadBalancerName",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置负载均衡实例的状态。
    pub async fn set_load_balancer_status(
        &self,
        request: SetLoadBalancerStatusRequest,
    ) -> Result<SetLoadBalancerStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SetLoadBalancerStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 变更按量付费CLB实例的实例计费方式。
    pub async fn modify_load_balancer_instance_charge_type(
        &self,
        request: ModifyLoadBalancerInstanceChargeTypeRequest,
    ) -> Result<ModifyLoadBalancerInstanceChargeTypeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyLoadBalancerInstanceChargeType",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定负载均衡实例的详细信息。
    pub async fn describe_load_balancer_attribute(
        &self,
        request: DescribeLoadBalancerAttributeRequest,
    ) -> Result<DescribeLoadBalancerAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeLoadBalancerAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询已创建的负载均衡实例。
    pub async fn describe_load_balancers(
        &self,
        request: DescribeLoadBalancersRequest,
    ) -> Result<DescribeLoadBalancersResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeLoadBalancers",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除监听。
    pub async fn delete_load_balancer_listener(
        &self,
        request: DeleteLoadBalancerListenerRequest,
    ) -> Result<DeleteLoadBalancerListenerResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteLoadBalancerListener",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 启动监听。
    pub async fn start_load_balancer_listener(
        &self,
        request: StartLoadBalancerListenerRequest,
    ) -> Result<StartLoadBalancerListenerResponse, SdkError> {
        let api_request = ApiRequest {
            action: "StartLoadBalancerListener",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 停止监听。
    pub async fn stop_load_balancer_listener(
        &self,
        request: StopLoadBalancerListenerRequest,
    ) -> Result<StopLoadBalancerListenerResponse, SdkError> {
        let api_request = ApiRequest {
            action: "StopLoadBalancerListener",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询负载均衡监听列表详情。
    pub async fn describe_load_balancer_listeners(
        &self,
        request: DescribeLoadBalancerListenersRequest,
    ) -> Result<DescribeLoadBalancerListenersResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeLoadBalancerListeners",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建TCP监听。
    pub async fn create_load_balancer_tcp_listener(
        &self,
        request: CreateLoadBalancerTCPListenerRequest,
    ) -> Result<CreateLoadBalancerTCPListenerResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateLoadBalancerTCPListener",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改TCP监听的配置。
    pub async fn set_load_balancer_tcp_listener_attribute(
        &self,
        request: SetLoadBalancerTCPListenerAttributeRequest,
    ) -> Result<SetLoadBalancerTCPListenerAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SetLoadBalancerTCPListenerAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询TCP监听配置。
    pub async fn describe_load_balancer_tcp_listener_attribute(
        &self,
        request: DescribeLoadBalancerTCPListenerAttributeRequest,
    ) -> Result<DescribeLoadBalancerTCPListenerAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeLoadBalancerTCPListenerAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建UDP监听。
    pub async fn create_load_balancer_udp_listener(
        &self,
        request: CreateLoadBalancerUDPListenerRequest,
    ) -> Result<CreateLoadBalancerUDPListenerResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateLoadBalancerUDPListener",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改UDP协议监听的配置。
    pub async fn set_load_balancer_udp_listener_attribute(
        &self,
        request: SetLoadBalancerUDPListenerAttributeRequest,
    ) -> Result<SetLoadBalancerUDPListenerAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SetLoadBalancerUDPListenerAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询UDP监听的配置。
    pub async fn describe_load_balancer_udp_listener_attribute(
        &self,
        request: DescribeLoadBalancerUDPListenerAttributeRequest,
    ) -> Result<DescribeLoadBalancerUDPListenerAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeLoadBalancerUDPListenerAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建HTTP监听。
    pub async fn create_load_balancer_http_listener(
        &self,
        request: CreateLoadBalancerHTTPListenerRequest,
    ) -> Result<CreateLoadBalancerHTTPListenerResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateLoadBalancerHTTPListener",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改HTTP监听的配置。
    pub async fn set_load_balancer_http_listener_attribute(
        &self,
        request: SetLoadBalancerHTTPListenerAttributeRequest,
    ) -> Result<SetLoadBalancerHTTPListenerAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SetLoadBalancerHTTPListenerAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询HTTP监听配置。
    pub async fn describe_load_balancer_http_listener_attribute(
        &self,
        request: DescribeLoadBalancerHTTPListenerAttributeRequest,
    ) -> Result<DescribeLoadBalancerHTTPListenerAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeLoadBalancerHTTPListenerAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建HTTPS监听。
    pub async fn create_load_balancer_https_listener(
        &self,
        request: CreateLoadBalancerHTTPSListenerRequest,
    ) -> Result<CreateLoadBalancerHTTPSListenerResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateLoadBalancerHTTPSListener",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改HTTPS监听的配置。
    pub async fn set_load_balancer_https_listener_attribute(
        &self,
        request: SetLoadBalancerHTTPSListenerAttributeRequest,
    ) -> Result<SetLoadBalancerHTTPSListenerAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SetLoadBalancerHTTPSListenerAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询HTTPS监听配置。
    pub async fn describe_load_balancer_https_listener_attribute(
        &self,
        request: DescribeLoadBalancerHTTPSListenerAttributeRequest,
    ) -> Result<DescribeLoadBalancerHTTPSListenerAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeLoadBalancerHTTPSListenerAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为指定的HTTP或HTTPS监听添加转发规则。
    pub async fn create_rules(
        &self,
        request: CreateRulesRequest,
    ) -> Result<CreateRulesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateRules",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改目标虚拟服务器组的转发规则。
    pub async fn set_rule(
        &self,
        request: SetRuleRequest,
    ) -> Result<SetRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SetRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除转发规则。
    pub async fn delete_rules(
        &self,
        request: DeleteRulesRequest,
    ) -> Result<DeleteRulesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteRules",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定转发规则的配置详情。
    pub async fn describe_rule_attribute(
        &self,
        request: DescribeRuleAttributeRequest,
    ) -> Result<DescribeRuleAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeRuleAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定监听已配置的转发规则。
    pub async fn describe_rules(
        &self,
        request: DescribeRulesRequest,
    ) -> Result<DescribeRulesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeRules",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 添加后端服务器。
    pub async fn add_backend_servers(
        &self,
        request: AddBackendServersRequest,
    ) -> Result<AddBackendServersResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddBackendServers",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询后端服务器的健康状态。
    pub async fn describe_health_status(
        &self,
        request: DescribeHealthStatusRequest,
    ) -> Result<DescribeHealthStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeHealthStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 移除后端服务器。
    pub async fn remove_backend_servers(
        &self,
        request: RemoveBackendServersRequest,
    ) -> Result<RemoveBackendServersResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RemoveBackendServers",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置后端服务器权重。
    pub async fn set_backend_servers(
        &self,
        request: SetBackendServersRequest,
    ) -> Result<SetBackendServersResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SetBackendServers",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建虚拟服务器组并向指定的服务器组中添加后端服务器。
    pub async fn create_v_server_group(
        &self,
        request: CreateVServerGroupRequest,
    ) -> Result<CreateVServerGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateVServerGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除虚拟服务器组。
    pub async fn delete_v_server_group(
        &self,
        request: DeleteVServerGroupRequest,
    ) -> Result<DeleteVServerGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteVServerGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改虚拟服务器组的配置。
    pub async fn set_v_server_group_attribute(
        &self,
        request: SetVServerGroupAttributeRequest,
    ) -> Result<SetVServerGroupAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SetVServerGroupAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询虚拟服务器组列表。
    pub async fn describe_v_server_groups(
        &self,
        request: DescribeVServerGroupsRequest,
    ) -> Result<DescribeVServerGroupsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeVServerGroups",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询虚拟服务器组的详细信息。
    pub async fn describe_v_server_group_attribute(
        &self,
        request: DescribeVServerGroupAttributeRequest,
    ) -> Result<DescribeVServerGroupAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeVServerGroupAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 向指定的虚拟服务器组中添加后端服务器。
    pub async fn add_v_server_group_backend_servers(
        &self,
        request: AddVServerGroupBackendServersRequest,
    ) -> Result<AddVServerGroupBackendServersResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddVServerGroupBackendServers",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 替换虚拟服务器组中的后端服务器。
    pub async fn modify_v_server_group_backend_servers(
        &self,
        request: ModifyVServerGroupBackendServersRequest,
    ) -> Result<ModifyVServerGroupBackendServersResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyVServerGroupBackendServers",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 从指定的虚拟服务器组中移除后端服务器。
    pub async fn remove_v_server_group_backend_servers(
        &self,
        request: RemoveVServerGroupBackendServersRequest,
    ) -> Result<RemoveVServerGroupBackendServersResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RemoveVServerGroupBackendServers",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建主备服务器组。一个主备服务器组只能包含两个后端服务器，一个为主服务器，另一个为备服务器。
    pub async fn create_master_slave_server_group(
        &self,
        request: CreateMasterSlaveServerGroupRequest,
    ) -> Result<CreateMasterSlaveServerGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateMasterSlaveServerGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除指定的主备服务器组。
    pub async fn delete_master_slave_server_group(
        &self,
        request: DeleteMasterSlaveServerGroupRequest,
    ) -> Result<DeleteMasterSlaveServerGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteMasterSlaveServerGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定主备服务器组的详细信息。
    pub async fn describe_master_slave_server_group_attribute(
        &self,
        request: DescribeMasterSlaveServerGroupAttributeRequest,
    ) -> Result<DescribeMasterSlaveServerGroupAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeMasterSlaveServerGroupAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询主备服务器组列表。
    pub async fn describe_master_slave_server_groups(
        &self,
        request: DescribeMasterSlaveServerGroupsRequest,
    ) -> Result<DescribeMasterSlaveServerGroupsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeMasterSlaveServerGroups",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除CA证书。
    pub async fn delete_ca_certificate(
        &self,
        request: DeleteCACertificateRequest,
    ) -> Result<DeleteCACertificateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteCACertificate",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除服务器证书。
    pub async fn delete_server_certificate(
        &self,
        request: DeleteServerCertificateRequest,
    ) -> Result<DeleteServerCertificateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteServerCertificate",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置CA证书名称。
    pub async fn set_ca_certificate_name(
        &self,
        request: SetCACertificateNameRequest,
    ) -> Result<SetCACertificateNameResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SetCACertificateName",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置服务器证书名称。
    pub async fn set_server_certificate_name(
        &self,
        request: SetServerCertificateNameRequest,
    ) -> Result<SetServerCertificateNameResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SetServerCertificateName",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询CA证书列表。
    pub async fn describe_ca_certificates(
        &self,
        request: DescribeCACertificatesRequest,
    ) -> Result<DescribeCACertificatesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCACertificates",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定地域的服务器证书列表。
    pub async fn describe_server_certificates(
        &self,
        request: DescribeServerCertificatesRequest,
    ) -> Result<DescribeServerCertificatesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeServerCertificates",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 上传CA证书。
    pub async fn upload_ca_certificate(
        &self,
        request: UploadCACertificateRequest,
    ) -> Result<UploadCACertificateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UploadCACertificate",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 上传服务器证书。
    pub async fn upload_server_certificate(
        &self,
        request: UploadServerCertificateRequest,
    ) -> Result<UploadServerCertificateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UploadServerCertificate",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建扩展域名。
    pub async fn create_domain_extension(
        &self,
        request: CreateDomainExtensionRequest,
    ) -> Result<CreateDomainExtensionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateDomainExtension",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改扩展域名的证书。
    pub async fn set_domain_extension_attribute(
        &self,
        request: SetDomainExtensionAttributeRequest,
    ) -> Result<SetDomainExtensionAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SetDomainExtensionAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除扩展域名。
    pub async fn delete_domain_extension(
        &self,
        request: DeleteDomainExtensionRequest,
    ) -> Result<DeleteDomainExtensionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteDomainExtension",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询已添加的扩展域名属性。
    pub async fn describe_domain_extension_attribute(
        &self,
        request: DescribeDomainExtensionAttributeRequest,
    ) -> Result<DescribeDomainExtensionAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainExtensionAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询已添加的扩展域名。
    pub async fn describe_domain_extensions(
        &self,
        request: DescribeDomainExtensionsRequest,
    ) -> Result<DescribeDomainExtensionsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainExtensions",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建TLS策略。
    pub async fn create_tls_cipher_policy(
        &self,
        request: CreateTLSCipherPolicyRequest,
    ) -> Result<CreateTLSCipherPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateTLSCipherPolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除TLS策略。
    pub async fn delete_tls_cipher_policy(
        &self,
        request: DeleteTLSCipherPolicyRequest,
    ) -> Result<DeleteTLSCipherPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteTLSCipherPolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置TLS策略。
    pub async fn set_tls_cipher_policy_attribute(
        &self,
        request: SetTLSCipherPolicyAttributeRequest,
    ) -> Result<SetTLSCipherPolicyAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SetTLSCipherPolicyAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询TLS策略。
    pub async fn list_tls_cipher_policies(
        &self,
        request: ListTLSCipherPoliciesRequest,
    ) -> Result<ListTLSCipherPoliciesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListTLSCipherPolicies",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建访问控制策略组。
    pub async fn create_access_control_list(
        &self,
        request: CreateAccessControlListRequest,
    ) -> Result<CreateAccessControlListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateAccessControlList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 在访问控制策略组中添加IP条目。
    pub async fn add_access_control_list_entry(
        &self,
        request: AddAccessControlListEntryRequest,
    ) -> Result<AddAccessControlListEntryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddAccessControlListEntry",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除访问控制策略组。
    pub async fn delete_access_control_list(
        &self,
        request: DeleteAccessControlListRequest,
    ) -> Result<DeleteAccessControlListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteAccessControlList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改访问控制策略组的名称。
    pub async fn set_access_control_list_attribute(
        &self,
        request: SetAccessControlListAttributeRequest,
    ) -> Result<SetAccessControlListAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SetAccessControlListAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询访问控制策略组的配置。
    pub async fn describe_access_control_list_attribute(
        &self,
        request: DescribeAccessControlListAttributeRequest,
    ) -> Result<DescribeAccessControlListAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeAccessControlListAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询已创建的访问控制策略组。
    pub async fn describe_access_control_lists(
        &self,
        request: DescribeAccessControlListsRequest,
    ) -> Result<DescribeAccessControlListsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeAccessControlLists",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除访问控制策略组中的IP条目。
    pub async fn remove_access_control_list_entry(
        &self,
        request: RemoveAccessControlListEntryRequest,
    ) -> Result<RemoveAccessControlListEntryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RemoveAccessControlListEntry",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 是否开启指定监听的白名单访问控制。
    pub async fn set_listener_access_control_status(
        &self,
        request: SetListenerAccessControlStatusRequest,
    ) -> Result<SetListenerAccessControlStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SetListenerAccessControlStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除监听白名单中的IP。
    pub async fn remove_listener_white_list_item(
        &self,
        request: RemoveListenerWhiteListItemRequest,
    ) -> Result<RemoveListenerWhiteListItemResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RemoveListenerWhiteListItem",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 添加监听访问控制白名单。
    pub async fn add_listener_white_list_item(
        &self,
        request: AddListenerWhiteListItemRequest,
    ) -> Result<AddListenerWhiteListItemResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddListenerWhiteListItem",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询监听的白名单配置。
    pub async fn describe_listener_access_control_attribute(
        &self,
        request: DescribeListenerAccessControlAttributeRequest,
    ) -> Result<DescribeListenerAccessControlAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeListenerAccessControlAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为指定的负载均衡实例添加标签。
    pub async fn add_tags(
        &self,
        request: AddTagsRequest,
    ) -> Result<AddTagsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddTags",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询标签列表。
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

    /// 查询实例已经绑定的标签列表。
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

    /// 为指定的资源列表统一创建并绑定标签。
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

    /// 为指定的资源列表统一解绑标签。
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

    /// 解绑指定负载均衡实例下的标签。
    pub async fn remove_tags(
        &self,
        request: RemoveTagsRequest,
    ) -> Result<RemoveTagsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RemoveTags",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除指定传统型负载均衡实例的访问日志。
    pub async fn delete_access_logs_download_attribute(
        &self,
        request: DeleteAccessLogsDownloadAttributeRequest,
    ) -> Result<DeleteAccessLogsDownloadAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteAccessLogsDownloadAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为传统型负载均衡实例添加访问日志转发规则。
    pub async fn set_access_logs_download_attribute(
        &self,
        request: SetAccessLogsDownloadAttributeRequest,
    ) -> Result<SetAccessLogsDownloadAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SetAccessLogsDownloadAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定传统型负载均衡实例的访问日志。
    pub async fn describe_access_logs_download_attribute(
        &self,
        request: DescribeAccessLogsDownloadAttributeRequest,
    ) -> Result<DescribeAccessLogsDownloadAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeAccessLogsDownloadAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 启用当前地域的秒级监控功能。
    pub async fn enable_high_defination_monitor(
        &self,
        request: EnableHighDefinationMonitorRequest,
    ) -> Result<EnableHighDefinationMonitorResponse, SdkError> {
        let api_request = ApiRequest {
            action: "EnableHighDefinationMonitor",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改高精度秒级监控的配置信息。
    pub async fn modify_high_defination_monitor(
        &self,
        request: ModifyHighDefinationMonitorRequest,
    ) -> Result<ModifyHighDefinationMonitorResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyHighDefinationMonitor",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定地域的秒级监控配置信息。
    pub async fn describe_high_defination_monitor(
        &self,
        request: DescribeHighDefinationMonitorRequest,
    ) -> Result<DescribeHighDefinationMonitorResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeHighDefinationMonitor",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改云资源所属的资源组。
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

}