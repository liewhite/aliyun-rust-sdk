//! API 调用 - 自动生成，请勿手动修改

use crate::client::{Client, ClientConfig, ApiRequest, HttpMethod};
use crate::error::SdkError;
use super::types::*;

/// Vpc API 版本
pub const API_VERSION: &str = "2016-04-28";

/// Vpc 客户端
#[derive(Debug, Clone)]
pub struct VpcClient {
    client: Client,
}

impl VpcClient {
    /// 创建新客户端
    pub fn new(config: ClientConfig) -> Result<Self, SdkError> {
        Ok(Self {
            client: Client::new(config)?,
        })
    }

    /// 创建一个专有网络VPC。
    pub async fn create_vpc(
        &self,
        request: CreateVpcRequest,
    ) -> Result<CreateVpcResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateVpc",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建一个默认的专有网络VPC。
    pub async fn create_default_vpc(
        &self,
        request: CreateDefaultVpcRequest,
    ) -> Result<CreateDefaultVpcResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateDefaultVpc",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为云企业网实例授权。
    pub async fn grant_instance_to_cen(
        &self,
        request: GrantInstanceToCenRequest,
    ) -> Result<GrantInstanceToCenResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GrantInstanceToCen",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为VPC添加附加网段。
    pub async fn associate_vpc_cidr_block(
        &self,
        request: AssociateVpcCidrBlockRequest,
    ) -> Result<AssociateVpcCidrBlockResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AssociateVpcCidrBlock",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 预留指定的IPv6地址段。
    pub async fn allocate_vpc_ipv6_cidr(
        &self,
        request: AllocateVpcIpv6CidrRequest,
    ) -> Result<AllocateVpcIpv6CidrResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AllocateVpcIpv6Cidr",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteVpc接口删除一个专有网络VPC（Virtual Private Cloud）。
    pub async fn delete_vpc(
        &self,
        request: DeleteVpcRequest,
    ) -> Result<DeleteVpcResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteVpc",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用RevokeInstanceFromCen撤销网络实例对指定云企业网实例的授权。
    pub async fn revoke_instance_from_cen(
        &self,
        request: RevokeInstanceFromCenRequest,
    ) -> Result<RevokeInstanceFromCenResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RevokeInstanceFromCen",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用UnassociateVpcCidrBlock删除VPC的附加网段。
    pub async fn unassociate_vpc_cidr_block(
        &self,
        request: UnassociateVpcCidrBlockRequest,
    ) -> Result<UnassociateVpcCidrBlockResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UnassociateVpcCidrBlock",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改指定VPC的配置信息。
    pub async fn modify_vpc_attribute(
        &self,
        request: ModifyVpcAttributeRequest,
    ) -> Result<ModifyVpcAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyVpcAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 开启ClassicLink。
    pub async fn enable_vpc_classic_link(
        &self,
        request: EnableVpcClassicLinkRequest,
    ) -> Result<EnableVpcClassicLinkResponse, SdkError> {
        let api_request = ApiRequest {
            action: "EnableVpcClassicLink",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 关闭ClassicLink。
    pub async fn disable_vpc_classic_link(
        &self,
        request: DisableVpcClassicLinkRequest,
    ) -> Result<DisableVpcClassicLinkResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DisableVpcClassicLink",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置实例删除保护功能。
    pub async fn deletion_protection(
        &self,
        request: DeletionProtectionRequest,
    ) -> Result<DeletionProtectionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeletionProtection",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询已创建的VPC。
    pub async fn describe_vpcs(
        &self,
        request: DescribeVpcsRequest,
    ) -> Result<DescribeVpcsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeVpcs",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定VPC的配置信息。
    pub async fn describe_vpc_attribute(
        &self,
        request: DescribeVpcAttributeRequest,
    ) -> Result<DescribeVpcAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeVpcAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定网络实例（VPC、VBR和CCN）的云企业网跨账号授权信息。
    pub async fn describe_grant_rules_to_cen(
        &self,
        request: DescribeGrantRulesToCenRequest,
    ) -> Result<DescribeGrantRulesToCenResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeGrantRulesToCen",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyVRouterAttribute接口修改路由器的名称和描述信息。
    pub async fn modify_v_router_attribute(
        &self,
        request: ModifyVRouterAttributeRequest,
    ) -> Result<ModifyVRouterAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyVRouterAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeVRouters接口查询指定地域的路由器列表。
    pub async fn describe_v_routers(
        &self,
        request: DescribeVRoutersRequest,
    ) -> Result<DescribeVRoutersResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeVRouters",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询交换机预留网段。
    pub async fn list_v_switch_cidr_reservations(
        &self,
        request: ListVSwitchCidrReservationsRequest,
    ) -> Result<ListVSwitchCidrReservationsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListVSwitchCidrReservations",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateVSwitchCidrReservation接口创建交换机预留网段。
    pub async fn create_v_switch_cidr_reservation(
        &self,
        request: CreateVSwitchCidrReservationRequest,
    ) -> Result<CreateVSwitchCidrReservationResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateVSwitchCidrReservation",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询交换机预留网段的用量。
    pub async fn get_v_switch_cidr_reservation_usage(
        &self,
        request: GetVSwitchCidrReservationUsageRequest,
    ) -> Result<GetVSwitchCidrReservationUsageResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetVSwitchCidrReservationUsage",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改交换机预留网段的名称和描述信息。
    pub async fn modify_v_switch_cidr_reservation_attribute(
        &self,
        request: ModifyVSwitchCidrReservationAttributeRequest,
    ) -> Result<ModifyVSwitchCidrReservationAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyVSwitchCidrReservationAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建一个交换机。
    pub async fn create_v_switch(
        &self,
        request: CreateVSwitchRequest,
    ) -> Result<CreateVSwitchResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateVSwitch",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateDefaultVSwitch接口创建一个默认的交换机。
    pub async fn create_default_v_switch(
        &self,
        request: CreateDefaultVSwitchRequest,
    ) -> Result<CreateDefaultVSwitchResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateDefaultVSwitch",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CheckCanAllocateVpcPrivateIpAddress接口查询交换机下的私网IP地址是否可用。
    pub async fn check_can_allocate_vpc_private_ip_address(
        &self,
        request: CheckCanAllocateVpcPrivateIpAddressRequest,
    ) -> Result<CheckCanAllocateVpcPrivateIpAddressResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CheckCanAllocateVpcPrivateIpAddress",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteVSwitch接口删除交换机。
    pub async fn delete_v_switch(
        &self,
        request: DeleteVSwitchRequest,
    ) -> Result<DeleteVSwitchResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteVSwitch",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyVSwitchAttribute接口修改指定交换机的配置信息。
    pub async fn modify_v_switch_attribute(
        &self,
        request: ModifyVSwitchAttributeRequest,
    ) -> Result<ModifyVSwitchAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyVSwitchAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询可组网的信息，内网按vswitch进行组网。
    pub async fn describe_v_switches(
        &self,
        request: DescribeVSwitchesRequest,
    ) -> Result<DescribeVSwitchesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeVSwitches",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeVSwitchAttributes接口查询指定交换机的配置信息。
    pub async fn describe_v_switch_attributes(
        &self,
        request: DescribeVSwitchAttributesRequest,
    ) -> Result<DescribeVSwitchAttributesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeVSwitchAttributes",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除交换机预留网段。
    pub async fn delete_v_switch_cidr_reservation(
        &self,
        request: DeleteVSwitchCidrReservationRequest,
    ) -> Result<DeleteVSwitchCidrReservationResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteVSwitchCidrReservation",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateRouteEntry在VPC路由器或边界路由器（VBR）上创建自定义路由条目。
    pub async fn create_route_entry(
        &self,
        request: CreateRouteEntryRequest,
    ) -> Result<CreateRouteEntryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateRouteEntry",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 在VPC路由器的路由表中批量添加自定义路由条目。
    pub async fn create_route_entries(
        &self,
        request: CreateRouteEntriesRequest,
    ) -> Result<CreateRouteEntriesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateRouteEntries",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建自定义路由表。
    pub async fn create_route_table(
        &self,
        request: CreateRouteTableRequest,
    ) -> Result<CreateRouteTableResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateRouteTable",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 将创建的自定义路由表和同一VPC内的交换机绑定。
    pub async fn associate_route_table(
        &self,
        request: AssociateRouteTableRequest,
    ) -> Result<AssociateRouteTableResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AssociateRouteTable",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteRouteEntry删除VPC路由器或边界路由器的路由表中的路由条目。
    pub async fn delete_route_entry(
        &self,
        request: DeleteRouteEntryRequest,
    ) -> Result<DeleteRouteEntryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteRouteEntry",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteRouteEntries批量删除自定义路由条目。
    pub async fn delete_route_entries(
        &self,
        request: DeleteRouteEntriesRequest,
    ) -> Result<DeleteRouteEntriesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteRouteEntries",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteRouteTable接口删除自定义路由表。
    pub async fn delete_route_table(
        &self,
        request: DeleteRouteTableRequest,
    ) -> Result<DeleteRouteTableResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteRouteTable",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用UnassociateRouteTable接口将路由表和交换机解绑。
    pub async fn unassociate_route_table(
        &self,
        request: UnassociateRouteTableRequest,
    ) -> Result<UnassociateRouteTableResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UnassociateRouteTable",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改路由表的名称、描述，或者修改是否接收动态路由。
    pub async fn modify_route_table_attributes(
        &self,
        request: ModifyRouteTableAttributesRequest,
    ) -> Result<ModifyRouteTableAttributesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyRouteTableAttributes",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyRouteEntry修改自定义路由条目的名称、描述和路由下一跳。
    pub async fn modify_route_entry(
        &self,
        request: ModifyRouteEntryRequest,
    ) -> Result<ModifyRouteEntryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyRouteEntry",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用GetVpcRouteEntrySummary接口查询路由类型的明细。
    pub async fn get_vpc_route_entry_summary(
        &self,
        request: GetVpcRouteEntrySummaryRequest,
    ) -> Result<GetVpcRouteEntrySummaryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetVpcRouteEntrySummary",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeRouteTables接口查询路由表。
    pub async fn describe_route_tables(
        &self,
        request: DescribeRouteTablesRequest,
    ) -> Result<DescribeRouteTablesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeRouteTables",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeRouteEntryList查询路由条目列表。
    pub async fn describe_route_entry_list(
        &self,
        request: DescribeRouteEntryListRequest,
    ) -> Result<DescribeRouteEntryListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeRouteEntryList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeRouteTableList接口查询路由表列表。
    pub async fn describe_route_table_list(
        &self,
        request: DescribeRouteTableListRequest,
    ) -> Result<DescribeRouteTableListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeRouteTableList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询路由发布情况。
    pub async fn list_vpc_published_route_entries(
        &self,
        request: ListVpcPublishedRouteEntriesRequest,
    ) -> Result<ListVpcPublishedRouteEntriesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListVpcPublishedRouteEntries",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 发布VPC路由到外部组件。
    pub async fn publish_vpc_route_entries(
        &self,
        request: PublishVpcRouteEntriesRequest,
    ) -> Result<PublishVpcRouteEntriesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PublishVpcRouteEntries",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 撤回VPC已发布路由。
    pub async fn withdraw_vpc_published_route_entries(
        &self,
        request: WithdrawVpcPublishedRouteEntriesRequest,
    ) -> Result<WithdrawVpcPublishedRouteEntriesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "WithdrawVpcPublishedRouteEntries",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ListGatewayRouteTableEntries接口查询网关路由表列表信息。
    pub async fn list_gateway_route_table_entries(
        &self,
        request: ListGatewayRouteTableEntriesRequest,
    ) -> Result<ListGatewayRouteTableEntriesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListGatewayRouteTableEntries",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用AssociateRouteTableWithGateway接口将网关路由表和同一VPC内的IPv4网关、IPv6网关实例绑定。
    pub async fn associate_route_table_with_gateway(
        &self,
        request: AssociateRouteTableWithGatewayRequest,
    ) -> Result<AssociateRouteTableWithGatewayResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AssociateRouteTableWithGateway",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DissociateRouteTableFromGateway解绑网关路由表和IPv4网关、IPv6网关实例。
    pub async fn dissociate_route_table_from_gateway(
        &self,
        request: DissociateRouteTableFromGatewayRequest,
    ) -> Result<DissociateRouteTableFromGatewayResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DissociateRouteTableFromGateway",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改网关路由表的下一跳类型和下一跳。
    pub async fn update_gateway_route_table_entry_attribute(
        &self,
        request: UpdateGatewayRouteTableEntryAttributeRequest,
    ) -> Result<UpdateGatewayRouteTableEntryAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateGatewayRouteTableEntryAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateVpcPrefixList接口创建前缀列表。
    pub async fn create_vpc_prefix_list(
        &self,
        request: CreateVpcPrefixListRequest,
    ) -> Result<CreateVpcPrefixListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateVpcPrefixList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用RetryVpcPrefixListAssociation接口重新下发最新的前缀列表。
    pub async fn retry_vpc_prefix_list_association(
        &self,
        request: RetryVpcPrefixListAssociationRequest,
    ) -> Result<RetryVpcPrefixListAssociationResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RetryVpcPrefixListAssociation",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteVpcPrefixList接口删除前缀列表。
    pub async fn delete_vpc_prefix_list(
        &self,
        request: DeleteVpcPrefixListRequest,
    ) -> Result<DeleteVpcPrefixListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteVpcPrefixList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyVpcPrefixList接口修改前缀列表的配置信息。
    pub async fn modify_vpc_prefix_list(
        &self,
        request: ModifyVpcPrefixListRequest,
    ) -> Result<ModifyVpcPrefixListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyVpcPrefixList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ListPrefixLists接口查询前缀列表的列表信息。
    pub async fn list_prefix_lists(
        &self,
        request: ListPrefixListsRequest,
    ) -> Result<ListPrefixListsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListPrefixLists",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用GetVpcPrefixListAssociations接口查询前缀列表的关联关系。
    pub async fn get_vpc_prefix_list_associations(
        &self,
        request: GetVpcPrefixListAssociationsRequest,
    ) -> Result<GetVpcPrefixListAssociationsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetVpcPrefixListAssociations",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用GetVpcPrefixListEntries接口查询指定前缀列表的信息。
    pub async fn get_vpc_prefix_list_entries(
        &self,
        request: GetVpcPrefixListEntriesRequest,
    ) -> Result<GetVpcPrefixListEntriesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetVpcPrefixListEntries",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建DHCP选项集。
    pub async fn create_dhcp_options_set(
        &self,
        request: CreateDhcpOptionsSetRequest,
    ) -> Result<CreateDhcpOptionsSetResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateDhcpOptionsSet",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteDhcpOptionsSet接口删除DHCP选项集。
    pub async fn delete_dhcp_options_set(
        &self,
        request: DeleteDhcpOptionsSetRequest,
    ) -> Result<DeleteDhcpOptionsSetResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteDhcpOptionsSet",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用AttachDhcpOptionsSetToVpc接口将DHCP选项集关联到VPC。
    pub async fn attach_dhcp_options_set_to_vpc(
        &self,
        request: AttachDhcpOptionsSetToVpcRequest,
    ) -> Result<AttachDhcpOptionsSetToVpcResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AttachDhcpOptionsSetToVpc",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 取消DHCP选项集与VPC的关联。
    pub async fn detach_dhcp_options_set_from_vpc(
        &self,
        request: DetachDhcpOptionsSetFromVpcRequest,
    ) -> Result<DetachDhcpOptionsSetFromVpcResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DetachDhcpOptionsSetFromVpc",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ReplaceVpcDhcpOptionsSet接口更改DHCP选项集与VPC的关联。
    pub async fn replace_vpc_dhcp_options_set(
        &self,
        request: ReplaceVpcDhcpOptionsSetRequest,
    ) -> Result<ReplaceVpcDhcpOptionsSetResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ReplaceVpcDhcpOptionsSet",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用UpdateDhcpOptionsSetAttribute接口修改DHCP选项集配置信息。
    pub async fn update_dhcp_options_set_attribute(
        &self,
        request: UpdateDhcpOptionsSetAttributeRequest,
    ) -> Result<UpdateDhcpOptionsSetAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateDhcpOptionsSetAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用GetDhcpOptionsSet接口查询已创建的DHCP选项集。
    pub async fn get_dhcp_options_set(
        &self,
        request: GetDhcpOptionsSetRequest,
    ) -> Result<GetDhcpOptionsSetResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetDhcpOptionsSet",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ListDhcpOptionsSets接口查询已创建的DHCP选项集列表。
    pub async fn list_dhcp_options_sets(
        &self,
        request: ListDhcpOptionsSetsRequest,
    ) -> Result<ListDhcpOptionsSetsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListDhcpOptionsSets",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用OpenFlowLogService接口开通流日志功能。
    pub async fn open_flow_log_service(
        &self,
        request: OpenFlowLogServiceRequest,
    ) -> Result<OpenFlowLogServiceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "OpenFlowLogService",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建流日志。
    pub async fn create_flow_log(
        &self,
        request: CreateFlowLogRequest,
    ) -> Result<CreateFlowLogResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateFlowLog",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteFlowLog接口删除流日志。
    pub async fn delete_flow_log(
        &self,
        request: DeleteFlowLogRequest,
    ) -> Result<DeleteFlowLogResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteFlowLog",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyFlowLogAttribute接口编辑流日志的名称和描述。
    pub async fn modify_flow_log_attribute(
        &self,
        request: ModifyFlowLogAttributeRequest,
    ) -> Result<ModifyFlowLogAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyFlowLogAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ActiveFlowLog接口启动流日志，启动后开始捕获指定资源的流量。
    pub async fn active_flow_log(
        &self,
        request: ActiveFlowLogRequest,
    ) -> Result<ActiveFlowLogResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ActiveFlowLog",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeactiveFlowLog接口停止流日志，停止后不再捕获指定资源的流量。
    pub async fn deactive_flow_log(
        &self,
        request: DeactiveFlowLogRequest,
    ) -> Result<DeactiveFlowLogResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeactiveFlowLog",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用GetFlowLogServiceStatus接口查询流日志功能的开通状态。
    pub async fn get_flow_log_service_status(
        &self,
        request: GetFlowLogServiceStatusRequest,
    ) -> Result<GetFlowLogServiceStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetFlowLogServiceStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeFlowLogs接口查询流日志。
    pub async fn describe_flow_logs(
        &self,
        request: DescribeFlowLogsRequest,
    ) -> Result<DescribeFlowLogsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeFlowLogs",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用AssociateNetworkAcl接口，绑定网络ACL到交换机。
    pub async fn associate_network_acl(
        &self,
        request: AssociateNetworkAclRequest,
    ) -> Result<AssociateNetworkAclResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AssociateNetworkAcl",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateNetworkAcl接口创建网络ACL。
    pub async fn create_network_acl(
        &self,
        request: CreateNetworkAclRequest,
    ) -> Result<CreateNetworkAclResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateNetworkAcl",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CopyNetworkAclEntries接口复制网络ACL规则。
    pub async fn copy_network_acl_entries(
        &self,
        request: CopyNetworkAclEntriesRequest,
    ) -> Result<CopyNetworkAclEntriesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CopyNetworkAclEntries",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteNetworkAcl接口删除网络ACL。
    pub async fn delete_network_acl(
        &self,
        request: DeleteNetworkAclRequest,
    ) -> Result<DeleteNetworkAclResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteNetworkAcl",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用UnassociateNetworkAcl接口解除网络ACL与交换机的绑定。
    pub async fn unassociate_network_acl(
        &self,
        request: UnassociateNetworkAclRequest,
    ) -> Result<UnassociateNetworkAclResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UnassociateNetworkAcl",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyNetworkAclAttributes接口修改网络ACL的属性。
    pub async fn modify_network_acl_attributes(
        &self,
        request: ModifyNetworkAclAttributesRequest,
    ) -> Result<ModifyNetworkAclAttributesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyNetworkAclAttributes",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用UpdateNetworkAclEntries接口更新网络ACL规则。
    pub async fn update_network_acl_entries(
        &self,
        request: UpdateNetworkAclEntriesRequest,
    ) -> Result<UpdateNetworkAclEntriesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateNetworkAclEntries",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeNetworkAclAttributes接口查询网络ACL的详细信息。
    pub async fn describe_network_acl_attributes(
        &self,
        request: DescribeNetworkAclAttributesRequest,
    ) -> Result<DescribeNetworkAclAttributesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeNetworkAclAttributes",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeNetworkAcls接口查看网络ACL列表。
    pub async fn describe_network_acls(
        &self,
        request: DescribeNetworkAclsRequest,
    ) -> Result<DescribeNetworkAclsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeNetworkAcls",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateHaVip接口创建高可用虚拟IP（HaVip）。
    pub async fn create_ha_vip(
        &self,
        request: CreateHaVipRequest,
    ) -> Result<CreateHaVipResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateHaVip",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用AssociateHaVip接口将HaVip绑定到专有网络ECS实例或弹性网卡上。
    pub async fn associate_ha_vip(
        &self,
        request: AssociateHaVipRequest,
    ) -> Result<AssociateHaVipResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AssociateHaVip",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteHaVip接口删除高可用虚拟IP（HaVip）。
    pub async fn delete_ha_vip(
        &self,
        request: DeleteHaVipRequest,
    ) -> Result<DeleteHaVipResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteHaVip",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用UnassociateHaVip 接口将HaVip与专有网络ECS实例或弹性网卡解绑。
    pub async fn unassociate_ha_vip(
        &self,
        request: UnassociateHaVipRequest,
    ) -> Result<UnassociateHaVipResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UnassociateHaVip",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyHaVipAttribute接口修改HaVip的名称和描述。
    pub async fn modify_ha_vip_attribute(
        &self,
        request: ModifyHaVipAttributeRequest,
    ) -> Result<ModifyHaVipAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyHaVipAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeHaVips接口查询指定地域内的高可用虚拟IP（HaVip）。
    pub async fn describe_ha_vips(
        &self,
        request: DescribeHaVipsRequest,
    ) -> Result<DescribeHaVipsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeHaVips",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateTrafficMirrorFilterRules接口创建流量镜像入方向或出方向规则。
    pub async fn create_traffic_mirror_filter_rules(
        &self,
        request: CreateTrafficMirrorFilterRulesRequest,
    ) -> Result<CreateTrafficMirrorFilterRulesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateTrafficMirrorFilterRules",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteTrafficMirrorFilterRules删除流量镜像筛选条件的入方向或者出方向规则。
    pub async fn delete_traffic_mirror_filter_rules(
        &self,
        request: DeleteTrafficMirrorFilterRulesRequest,
    ) -> Result<DeleteTrafficMirrorFilterRulesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteTrafficMirrorFilterRules",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用UpdateTrafficMirrorFilterRuleAttribute接口修改流量镜像入方向或出方向规则的配置信息。
    pub async fn update_traffic_mirror_filter_rule_attribute(
        &self,
        request: UpdateTrafficMirrorFilterRuleAttributeRequest,
    ) -> Result<UpdateTrafficMirrorFilterRuleAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateTrafficMirrorFilterRuleAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateTrafficMirrorFilter接口创建流量镜像筛选条件。
    pub async fn create_traffic_mirror_filter(
        &self,
        request: CreateTrafficMirrorFilterRequest,
    ) -> Result<CreateTrafficMirrorFilterResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateTrafficMirrorFilter",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteTrafficMirrorFilter接口删除流量镜像筛选条件。
    pub async fn delete_traffic_mirror_filter(
        &self,
        request: DeleteTrafficMirrorFilterRequest,
    ) -> Result<DeleteTrafficMirrorFilterResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteTrafficMirrorFilter",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用UpdateTrafficMirrorFilterAttribute接口修改流量镜像筛选条件的配置信息。
    pub async fn update_traffic_mirror_filter_attribute(
        &self,
        request: UpdateTrafficMirrorFilterAttributeRequest,
    ) -> Result<UpdateTrafficMirrorFilterAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateTrafficMirrorFilterAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询流量镜像的筛选条件。
    pub async fn list_traffic_mirror_filters(
        &self,
        request: ListTrafficMirrorFiltersRequest,
    ) -> Result<ListTrafficMirrorFiltersResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListTrafficMirrorFilters",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用AddSourcesToTrafficMirrorSession接口为镜像会话增加镜像源。
    pub async fn add_sources_to_traffic_mirror_session(
        &self,
        request: AddSourcesToTrafficMirrorSessionRequest,
    ) -> Result<AddSourcesToTrafficMirrorSessionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddSourcesToTrafficMirrorSession",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateTrafficMirrorSession创建镜像会话。
    pub async fn create_traffic_mirror_session(
        &self,
        request: CreateTrafficMirrorSessionRequest,
    ) -> Result<CreateTrafficMirrorSessionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateTrafficMirrorSession",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除镜像会话。
    pub async fn delete_traffic_mirror_session(
        &self,
        request: DeleteTrafficMirrorSessionRequest,
    ) -> Result<DeleteTrafficMirrorSessionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteTrafficMirrorSession",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改镜像会话的配置信息。
    pub async fn update_traffic_mirror_session_attribute(
        &self,
        request: UpdateTrafficMirrorSessionAttributeRequest,
    ) -> Result<UpdateTrafficMirrorSessionAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateTrafficMirrorSessionAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询镜像会话的详细信息。
    pub async fn list_traffic_mirror_sessions(
        &self,
        request: ListTrafficMirrorSessionsRequest,
    ) -> Result<ListTrafficMirrorSessionsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListTrafficMirrorSessions",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用RemoveSourcesFromTrafficMirrorSession接口删除镜像会话中的镜像源。
    pub async fn remove_sources_from_traffic_mirror_session(
        &self,
        request: RemoveSourcesFromTrafficMirrorSessionRequest,
    ) -> Result<RemoveSourcesFromTrafficMirrorSessionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RemoveSourcesFromTrafficMirrorSession",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 开通流量镜像功能。
    pub async fn open_traffic_mirror_service(
        &self,
        request: OpenTrafficMirrorServiceRequest,
    ) -> Result<OpenTrafficMirrorServiceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "OpenTrafficMirrorService",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用GetTrafficMirrorServiceStatus接口查询流量镜像功能的状态。
    pub async fn get_traffic_mirror_service_status(
        &self,
        request: GetTrafficMirrorServiceStatusRequest,
    ) -> Result<GetTrafficMirrorServiceStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetTrafficMirrorServiceStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 路由目标组主备切换。
    pub async fn switch_active_route_target(
        &self,
        request: SwitchActiveRouteTargetRequest,
    ) -> Result<SwitchActiveRouteTargetResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SwitchActiveRouteTarget",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新路由目标组实例信息，包括名称、描述、未使能的成员。
    pub async fn update_route_target_group(
        &self,
        request: UpdateRouteTargetGroupRequest,
    ) -> Result<UpdateRouteTargetGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateRouteTargetGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询路由目标组列表。
    pub async fn list_route_target_groups(
        &self,
        request: ListRouteTargetGroupsRequest,
    ) -> Result<ListRouteTargetGroupsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListRouteTargetGroups",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取路由目标组实例信息。
    pub async fn get_route_target_group(
        &self,
        request: GetRouteTargetGroupRequest,
    ) -> Result<GetRouteTargetGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetRouteTargetGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除路由目标组实例。
    pub async fn delete_route_target_group(
        &self,
        request: DeleteRouteTargetGroupRequest,
    ) -> Result<DeleteRouteTargetGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteRouteTargetGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建路由目标组实例。
    pub async fn create_route_target_group(
        &self,
        request: CreateRouteTargetGroupRequest,
    ) -> Result<CreateRouteTargetGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateRouteTargetGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 申请弹性公网IP（Elastic IP Address，简称EIP）。
    pub async fn allocate_eip_address(
        &self,
        request: AllocateEipAddressRequest,
    ) -> Result<AllocateEipAddressResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AllocateEipAddress",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用AllocateEipAddressPro申请指定的EIP，可用于特定的网络配置需求。
    pub async fn allocate_eip_address_pro(
        &self,
        request: AllocateEipAddressProRequest,
    ) -> Result<AllocateEipAddressProResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AllocateEipAddressPro",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ReleaseEipAddress接口释放指定的弹性公网IP（EIP）。
    pub async fn release_eip_address(
        &self,
        request: ReleaseEipAddressRequest,
    ) -> Result<ReleaseEipAddressResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ReleaseEipAddress",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyEipAddressAttribute接口修改指定EIP的名称、描述信息和带宽峰值。
    pub async fn modify_eip_address_attribute(
        &self,
        request: ModifyEipAddressAttributeRequest,
    ) -> Result<ModifyEipAddressAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyEipAddressAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定地域已创建的EIP。
    pub async fn describe_eip_addresses(
        &self,
        request: DescribeEipAddressesRequest,
    ) -> Result<DescribeEipAddressesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeEipAddresses",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeEipGatewayInfo接口查询EIP的网关和掩码信息。
    pub async fn describe_eip_gateway_info(
        &self,
        request: DescribeEipGatewayInfoRequest,
    ) -> Result<DescribeEipGatewayInfoResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeEipGatewayInfo",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 连续EIP是一组按序排列的弹性公网IP地址，您可以在进行网络设计时，为需要连续公网IP地址的云资源批量申请和使用这些IP地址。您可以通过调用AllocateEipSegmentAddress接口...
    pub async fn allocate_eip_segment_address(
        &self,
        request: AllocateEipSegmentAddressRequest,
    ) -> Result<AllocateEipSegmentAddressResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AllocateEipSegmentAddress",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeEipSegment查询连续EIP组的信息。
    pub async fn describe_eip_segment(
        &self,
        request: DescribeEipSegmentRequest,
    ) -> Result<DescribeEipSegmentResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeEipSegment",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ReleaseEipSegmentAddress释放连续EIP。
    pub async fn release_eip_segment_address(
        &self,
        request: ReleaseEipSegmentAddressRequest,
    ) -> Result<ReleaseEipSegmentAddressResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ReleaseEipSegmentAddress",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改EIP转发模式。
    pub async fn modify_eip_forward_mode(
        &self,
        request: ModifyEipForwardModeRequest,
    ) -> Result<ModifyEipForwardModeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyEipForwardMode",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用AssociateEipAddress将弹性公网IP（EIP）绑定到同地域的云产品实例上。
    pub async fn associate_eip_address(
        &self,
        request: AssociateEipAddressRequest,
    ) -> Result<AssociateEipAddressResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AssociateEipAddress",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用AssociateEipAddressBatch接口批量将弹性公网IP（Elastic IP Address，简称EIP）绑定到同地域的云产品实例上。
    pub async fn associate_eip_address_batch(
        &self,
        request: AssociateEipAddressBatchRequest,
    ) -> Result<AssociateEipAddressBatchResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AssociateEipAddressBatch",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用UnassociateEipAddress将弹性公网IP（EIP）从绑定的云产品上解绑。
    pub async fn unassociate_eip_address(
        &self,
        request: UnassociateEipAddressRequest,
    ) -> Result<UnassociateEipAddressResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UnassociateEipAddress",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreatePublicIpAddressPool接口创建IP地址池。
    pub async fn create_public_ip_address_pool(
        &self,
        request: CreatePublicIpAddressPoolRequest,
    ) -> Result<CreatePublicIpAddressPoolResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreatePublicIpAddressPool",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用OpenPublicIpAddressPoolService接口开通IP地址池功能。
    pub async fn open_public_ip_address_pool_service(
        &self,
        request: OpenPublicIpAddressPoolServiceRequest,
    ) -> Result<OpenPublicIpAddressPoolServiceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "OpenPublicIpAddressPoolService",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用AddPublicIpAddressPoolCidrBlock接口将IP地址网段添加至IP地址池。
    pub async fn add_public_ip_address_pool_cidr_block(
        &self,
        request: AddPublicIpAddressPoolCidrBlockRequest,
    ) -> Result<AddPublicIpAddressPoolCidrBlockResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddPublicIpAddressPoolCidrBlock",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 将连续EIP组迁移至IP地址池。
    pub async fn transform_eip_segment_to_public_ip_address_pool(
        &self,
        request: TransformEipSegmentToPublicIpAddressPoolRequest,
    ) -> Result<TransformEipSegmentToPublicIpAddressPoolResponse, SdkError> {
        let api_request = ApiRequest {
            action: "TransformEipSegmentToPublicIpAddressPool",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeletePublicIpAddressPool接口删除IP地址池。
    pub async fn delete_public_ip_address_pool(
        &self,
        request: DeletePublicIpAddressPoolRequest,
    ) -> Result<DeletePublicIpAddressPoolResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeletePublicIpAddressPool",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeletePublicIpAddressPoolCidrBlock接口将IP地址网段从IP地址池中删除。
    pub async fn delete_public_ip_address_pool_cidr_block(
        &self,
        request: DeletePublicIpAddressPoolCidrBlockRequest,
    ) -> Result<DeletePublicIpAddressPoolCidrBlockResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeletePublicIpAddressPoolCidrBlock",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用UpdatePublicIpAddressPoolAttribute接口修改地址池属性。
    pub async fn update_public_ip_address_pool_attribute(
        &self,
        request: UpdatePublicIpAddressPoolAttributeRequest,
    ) -> Result<UpdatePublicIpAddressPoolAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdatePublicIpAddressPoolAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ListPublicIpAddressPools接口查询可使用的IP地址池信息。
    pub async fn list_public_ip_address_pools(
        &self,
        request: ListPublicIpAddressPoolsRequest,
    ) -> Result<ListPublicIpAddressPoolsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListPublicIpAddressPools",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ListPublicIpAddressPoolCidrBlocks接口查询IP地址池中的IP地址网段信息。
    pub async fn list_public_ip_address_pool_cidr_blocks(
        &self,
        request: ListPublicIpAddressPoolCidrBlocksRequest,
    ) -> Result<ListPublicIpAddressPoolCidrBlocksResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListPublicIpAddressPoolCidrBlocks",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用GetPublicIpAddressPoolServiceStatus接口查询IP地址池功能的开通状态。
    pub async fn get_public_ip_address_pool_service_status(
        &self,
        request: GetPublicIpAddressPoolServiceStatusRequest,
    ) -> Result<GetPublicIpAddressPoolServiceStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetPublicIpAddressPoolServiceStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeEipMonitorData接口查看弹性公网IP（Elastic IP Address，简称EIP）的监控信息，最多查询31天内的数据，单次最多查询400个流量点的数据。
    pub async fn describe_eip_monitor_data(
        &self,
        request: DescribeEipMonitorDataRequest,
    ) -> Result<DescribeEipMonitorDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeEipMonitorData",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用SetHighDefinitionMonitorLogStatus接口为EIP配置高精度秒级监控。
    pub async fn set_high_definition_monitor_log_status(
        &self,
        request: SetHighDefinitionMonitorLogStatusRequest,
    ) -> Result<SetHighDefinitionMonitorLogStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SetHighDefinitionMonitorLogStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeHighDefinitionMonitorLogAttribute接口查询EIP高精度秒级监控的配置信息。
    pub async fn describe_high_definition_monitor_log_attribute(
        &self,
        request: DescribeHighDefinitionMonitorLogAttributeRequest,
    ) -> Result<DescribeHighDefinitionMonitorLogAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeHighDefinitionMonitorLogAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribePublicIpAddress接口查询指定地域中位于专有网络的公网IP地址的范围。
    pub async fn describe_public_ip_address(
        &self,
        request: DescribePublicIpAddressRequest,
    ) -> Result<DescribePublicIpAddressResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribePublicIpAddress",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建共享带宽实例。
    pub async fn create_common_bandwidth_package(
        &self,
        request: CreateCommonBandwidthPackageRequest,
    ) -> Result<CreateCommonBandwidthPackageResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateCommonBandwidthPackage",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用AddCommonBandwidthPackageIp接口添加EIP到共享带宽中。
    pub async fn add_common_bandwidth_package_ip(
        &self,
        request: AddCommonBandwidthPackageIpRequest,
    ) -> Result<AddCommonBandwidthPackageIpResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddCommonBandwidthPackageIp",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用AddCommonBandwidthPackageIps接口批量添加EIP到共享带宽中。
    pub async fn add_common_bandwidth_package_ips(
        &self,
        request: AddCommonBandwidthPackageIpsRequest,
    ) -> Result<AddCommonBandwidthPackageIpsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddCommonBandwidthPackageIps",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用RemoveCommonBandwidthPackageIp接口移除共享带宽实例中的EIP。
    pub async fn remove_common_bandwidth_package_ip(
        &self,
        request: RemoveCommonBandwidthPackageIpRequest,
    ) -> Result<RemoveCommonBandwidthPackageIpResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RemoveCommonBandwidthPackageIp",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteCommonBandwidthPackage接口删除已创建的共享带宽实例。
    pub async fn delete_common_bandwidth_package(
        &self,
        request: DeleteCommonBandwidthPackageRequest,
    ) -> Result<DeleteCommonBandwidthPackageResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteCommonBandwidthPackage",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改共享带宽实例的名称和描述信息。
    pub async fn modify_common_bandwidth_package_attribute(
        &self,
        request: ModifyCommonBandwidthPackageAttributeRequest,
    ) -> Result<ModifyCommonBandwidthPackageAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyCommonBandwidthPackageAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyCommonBandwidthPackageSpec接口修改共享带宽的带宽峰值。
    pub async fn modify_common_bandwidth_package_spec(
        &self,
        request: ModifyCommonBandwidthPackageSpecRequest,
    ) -> Result<ModifyCommonBandwidthPackageSpecResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyCommonBandwidthPackageSpec",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeCommonBandwidthPackages接口查询指定地域中共享带宽实例列表及其信息。
    pub async fn describe_common_bandwidth_packages(
        &self,
        request: DescribeCommonBandwidthPackagesRequest,
    ) -> Result<DescribeCommonBandwidthPackagesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCommonBandwidthPackages",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyCommonBandwidthPackageIpBandwidth接口为已经加入到共享带宽的EIP设置最大可用带宽值。
    pub async fn modify_common_bandwidth_package_ip_bandwidth(
        &self,
        request: ModifyCommonBandwidthPackageIpBandwidthRequest,
    ) -> Result<ModifyCommonBandwidthPackageIpBandwidthResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyCommonBandwidthPackageIpBandwidth",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CancelCommonBandwidthPackageIpBandwidth接口取消已经加入共享带宽中的EIP的最大可用带宽值的设置。
    pub async fn cancel_common_bandwidth_package_ip_bandwidth(
        &self,
        request: CancelCommonBandwidthPackageIpBandwidthRequest,
    ) -> Result<CancelCommonBandwidthPackageIpBandwidthResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CancelCommonBandwidthPackageIpBandwidth",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用Describe95Traffic接口查询按增强型95计费共享带宽的95流量数据。
    pub async fn describe95_traffic(
        &self,
        request: Describe95TrafficRequest,
    ) -> Result<Describe95TrafficResponse, SdkError> {
        let api_request = ApiRequest {
            action: "Describe95Traffic",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateVirtualPhysicalConnection接口创建共享专线。
    pub async fn create_virtual_physical_connection(
        &self,
        request: CreateVirtualPhysicalConnectionRequest,
    ) -> Result<CreateVirtualPhysicalConnectionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateVirtualPhysicalConnection",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用UpdateVirtualPhysicalConnection接口修改共享专线信息。
    pub async fn update_virtual_physical_connection(
        &self,
        request: UpdateVirtualPhysicalConnectionRequest,
    ) -> Result<UpdateVirtualPhysicalConnectionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateVirtualPhysicalConnection",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ListVirtualPhysicalConnections接口查询共享专线的信息。
    pub async fn list_virtual_physical_connections(
        &self,
        request: ListVirtualPhysicalConnectionsRequest,
    ) -> Result<ListVirtualPhysicalConnectionsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListVirtualPhysicalConnections",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用OpenPhysicalConnectionService接口开通出方向流量服务。
    pub async fn open_physical_connection_service(
        &self,
        request: OpenPhysicalConnectionServiceRequest,
    ) -> Result<OpenPhysicalConnectionServiceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "OpenPhysicalConnectionService",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreatePhysicalConnection接口申请物理专线接入。
    pub async fn create_physical_connection(
        &self,
        request: CreatePhysicalConnectionRequest,
    ) -> Result<CreatePhysicalConnectionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreatePhysicalConnection",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ApplyPhysicalConnectionLOA接口申请LOA。
    pub async fn apply_physical_connection_loa(
        &self,
        request: ApplyPhysicalConnectionLOARequest,
    ) -> Result<ApplyPhysicalConnectionLOAResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ApplyPhysicalConnectionLOA",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 如果之前调用ApplyPhysicalConnectionLOA接口申请LOA被拒绝，可以调用本接口重新申请LOA。
    pub async fn second_apply_physical_connection_loa(
        &self,
        request: SecondApplyPhysicalConnectionLOARequest,
    ) -> Result<SecondApplyPhysicalConnectionLOAResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SecondApplyPhysicalConnectionLOA",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreatePhysicalConnectionSetupOrder创建端口初装费订单。
    pub async fn create_physical_connection_setup_order(
        &self,
        request: CreatePhysicalConnectionSetupOrderRequest,
    ) -> Result<CreatePhysicalConnectionSetupOrderResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreatePhysicalConnectionSetupOrder",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreatePhysicalConnectionOccupancyOrder接口创建资源占用费用的订单。
    pub async fn create_physical_connection_occupancy_order(
        &self,
        request: CreatePhysicalConnectionOccupancyOrderRequest,
    ) -> Result<CreatePhysicalConnectionOccupancyOrderResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreatePhysicalConnectionOccupancyOrder",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建高可靠模式物理专线，以满足您多线接入阿里云上业务的稳定性以及多线路容灾后可避免因为单线而导致的业务受损。
    pub async fn create_high_reliable_physical_connection(
        &self,
        request: CreateHighReliablePhysicalConnectionRequest,
    ) -> Result<CreateHighReliablePhysicalConnectionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateHighReliablePhysicalConnection",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 将共享专线从推送跨账号边界路由器VBR（Virtual Border Router）的方式转换为推送跨账号共享端口的方式。
    pub async fn create_vpconn_from_vbr(
        &self,
        request: CreateVpconnFromVbrRequest,
    ) -> Result<CreateVpconnFromVbrResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateVpconnFromVbr",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeletePhysicalConnection接口删除物理专线连接。
    pub async fn delete_physical_connection(
        &self,
        request: DeletePhysicalConnectionRequest,
    ) -> Result<DeletePhysicalConnectionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeletePhysicalConnection",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 取消物理专线接入，取消后物理专线进入Canceled状态。
    pub async fn cancel_physical_connection(
        &self,
        request: CancelPhysicalConnectionRequest,
    ) -> Result<CancelPhysicalConnectionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CancelPhysicalConnection",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyPhysicalConnectionAttribute接口修改物理专线的配置。
    pub async fn modify_physical_connection_attribute(
        &self,
        request: ModifyPhysicalConnectionAttributeRequest,
    ) -> Result<ModifyPhysicalConnectionAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyPhysicalConnectionAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 开通处于Confirmed状态的物理专线，开通完成后物理专线进入Enabled状态。
    pub async fn enable_physical_connection(
        &self,
        request: EnablePhysicalConnectionRequest,
    ) -> Result<EnablePhysicalConnectionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "EnablePhysicalConnection",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 在物理专线开通后，终止物理专线接入。
    pub async fn terminate_physical_connection(
        &self,
        request: TerminatePhysicalConnectionRequest,
    ) -> Result<TerminatePhysicalConnectionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "TerminatePhysicalConnection",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用RecoverPhysicalConnection接口恢复物理专线接入。
    pub async fn recover_physical_connection(
        &self,
        request: RecoverPhysicalConnectionRequest,
    ) -> Result<RecoverPhysicalConnectionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RecoverPhysicalConnection",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 关联边界路由器VBR（Virtual Border Router）实例至共享专线。
    pub async fn attach_vbr_to_vpconn(
        &self,
        request: AttachVbrToVpconnRequest,
    ) -> Result<AttachVbrToVpconnResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AttachVbrToVpconn",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查看当前账号是否已开通出云流量费。
    pub async fn get_physical_connection_service_status(
        &self,
        request: GetPhysicalConnectionServiceStatusRequest,
    ) -> Result<GetPhysicalConnectionServiceStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetPhysicalConnectionServiceStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeAccessPoints接口查询指定地域中的物理专线接入点。
    pub async fn describe_access_points(
        &self,
        request: DescribeAccessPointsRequest,
    ) -> Result<DescribeAccessPointsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeAccessPoints",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 确认物理专线为可用状态，物理专线的状态会变为Confirmed。
    pub async fn confirm_physical_connection(
        &self,
        request: ConfirmPhysicalConnectionRequest,
    ) -> Result<ConfirmPhysicalConnectionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ConfirmPhysicalConnection",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribePhysicalConnections接口查询指定地域内的物理专线信息。
    pub async fn describe_physical_connections(
        &self,
        request: DescribePhysicalConnectionsRequest,
    ) -> Result<DescribePhysicalConnectionsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribePhysicalConnections",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询物理专线支持的设备级能力
    pub async fn list_physical_connection_features(
        &self,
        request: ListPhysicalConnectionFeaturesRequest,
    ) -> Result<ListPhysicalConnectionFeaturesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListPhysicalConnectionFeatures",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CompletePhysicalConnectionLOA完成施工完竣。
    pub async fn complete_physical_connection_loa(
        &self,
        request: CompletePhysicalConnectionLOARequest,
    ) -> Result<CompletePhysicalConnectionLOAResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CompletePhysicalConnectionLOA",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribePhysicalConnectionLOA查询物理专线LOA信息。
    pub async fn describe_physical_connection_loa(
        &self,
        request: DescribePhysicalConnectionLOARequest,
    ) -> Result<DescribePhysicalConnectionLOAResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribePhysicalConnectionLOA",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ListGeographicSubRegions接口查询地域信息。
    pub async fn list_geographic_sub_regions(
        &self,
        request: ListGeographicSubRegionsRequest,
    ) -> Result<ListGeographicSubRegionsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListGeographicSubRegions",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询专线可购地域列表
    pub async fn list_business_regions(
        &self,
        request: ListBusinessRegionsRequest,
    ) -> Result<ListBusinessRegionsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListBusinessRegions",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ListBusinessAccessPoints接口查询物理专线的接入点信息。
    pub async fn list_business_access_points(
        &self,
        request: ListBusinessAccessPointsRequest,
    ) -> Result<ListBusinessAccessPointsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListBusinessAccessPoints",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateFailoverTestJob接口创建高速通道故障演练任务。
    pub async fn create_failover_test_job(
        &self,
        request: CreateFailoverTestJobRequest,
    ) -> Result<CreateFailoverTestJobResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateFailoverTestJob",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用StartFailoverTestJob接口开始高速通道故障演练任务。
    pub async fn start_failover_test_job(
        &self,
        request: StartFailoverTestJobRequest,
    ) -> Result<StartFailoverTestJobResponse, SdkError> {
        let api_request = ApiRequest {
            action: "StartFailoverTestJob",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用StopFailoverTestJob接口结束高速通道故障演练任务。
    pub async fn stop_failover_test_job(
        &self,
        request: StopFailoverTestJobRequest,
    ) -> Result<StopFailoverTestJobResponse, SdkError> {
        let api_request = ApiRequest {
            action: "StopFailoverTestJob",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用UpdateFailoverTestJob接口更新高速通道故障演练任务。
    pub async fn update_failover_test_job(
        &self,
        request: UpdateFailoverTestJobRequest,
    ) -> Result<UpdateFailoverTestJobResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateFailoverTestJob",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteFailoverTestJob接口删除高速通道故障演练任务。
    pub async fn delete_failover_test_job(
        &self,
        request: DeleteFailoverTestJobRequest,
    ) -> Result<DeleteFailoverTestJobResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteFailoverTestJob",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeFailoverTestJob接口查询高速通道故障演练任务详情。
    pub async fn describe_failover_test_job(
        &self,
        request: DescribeFailoverTestJobRequest,
    ) -> Result<DescribeFailoverTestJobResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeFailoverTestJob",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeFailoverTestJobs接口批量查询高速通道故障演练任务。
    pub async fn describe_failover_test_jobs(
        &self,
        request: DescribeFailoverTestJobsRequest,
    ) -> Result<DescribeFailoverTestJobsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeFailoverTestJobs",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeExpressConnectTrafficQosRule查询高速通道QoS规则，不支持分页参数。
    pub async fn describe_express_connect_traffic_qos_rule(
        &self,
        request: DescribeExpressConnectTrafficQosRuleRequest,
    ) -> Result<DescribeExpressConnectTrafficQosRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeExpressConnectTrafficQosRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeExpressConnectTrafficQosQueue查询高速通道QoS队列。
    pub async fn describe_express_connect_traffic_qos_queue(
        &self,
        request: DescribeExpressConnectTrafficQosQueueRequest,
    ) -> Result<DescribeExpressConnectTrafficQosQueueResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeExpressConnectTrafficQosQueue",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeExpressConnectTrafficQos接口查询高速通道QoS策略，支持分页参数。
    pub async fn describe_express_connect_traffic_qos(
        &self,
        request: DescribeExpressConnectTrafficQosRequest,
    ) -> Result<DescribeExpressConnectTrafficQosResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeExpressConnectTrafficQos",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteExpressConnectTrafficQosQueue接口删除高速通道QoS队列。
    pub async fn delete_express_connect_traffic_qos_queue(
        &self,
        request: DeleteExpressConnectTrafficQosQueueRequest,
    ) -> Result<DeleteExpressConnectTrafficQosQueueResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteExpressConnectTrafficQosQueue",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteExpressConnectTrafficQos接口删除高速通道QoS策略。
    pub async fn delete_express_connect_traffic_qos(
        &self,
        request: DeleteExpressConnectTrafficQosRequest,
    ) -> Result<DeleteExpressConnectTrafficQosResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteExpressConnectTrafficQos",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteExpressConnectTrafficQosRule接口删除高速通道QoS规则。
    pub async fn delete_express_connect_traffic_qos_rule(
        &self,
        request: DeleteExpressConnectTrafficQosRuleRequest,
    ) -> Result<DeleteExpressConnectTrafficQosRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteExpressConnectTrafficQosRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyExpressConnectTrafficQos接口修改高速通道QoS策略，也可通过本接口关联独享物理专线。
    pub async fn modify_express_connect_traffic_qos(
        &self,
        request: ModifyExpressConnectTrafficQosRequest,
    ) -> Result<ModifyExpressConnectTrafficQosResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyExpressConnectTrafficQos",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyExpressConnectTrafficQosQueue接口修改高速通道QoS队列。
    pub async fn modify_express_connect_traffic_qos_queue(
        &self,
        request: ModifyExpressConnectTrafficQosQueueRequest,
    ) -> Result<ModifyExpressConnectTrafficQosQueueResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyExpressConnectTrafficQosQueue",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyExpressConnectTrafficQosRule接口修改高速通道QoS规则。
    pub async fn modify_express_connect_traffic_qos_rule(
        &self,
        request: ModifyExpressConnectTrafficQosRuleRequest,
    ) -> Result<ModifyExpressConnectTrafficQosRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyExpressConnectTrafficQosRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateExpressConnectTrafficQos接口创建高速通道QoS策略。
    pub async fn create_express_connect_traffic_qos(
        &self,
        request: CreateExpressConnectTrafficQosRequest,
    ) -> Result<CreateExpressConnectTrafficQosResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateExpressConnectTrafficQos",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateExpressConnectTrafficQosQueue接口创建高速通道QoS队列。
    pub async fn create_express_connect_traffic_qos_queue(
        &self,
        request: CreateExpressConnectTrafficQosQueueRequest,
    ) -> Result<CreateExpressConnectTrafficQosQueueResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateExpressConnectTrafficQosQueue",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateExpressConnectTrafficQosRule接口创建高速通道QoS规则。
    pub async fn create_express_connect_traffic_qos_rule(
        &self,
        request: CreateExpressConnectTrafficQosRuleRequest,
    ) -> Result<CreateExpressConnectTrafficQosRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateExpressConnectTrafficQosRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateVbrHa接口创建VBR倒换组。
    pub async fn create_vbr_ha(
        &self,
        request: CreateVbrHaRequest,
    ) -> Result<CreateVbrHaResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateVbrHa",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteVbrHa删除VBR倒换组。
    pub async fn delete_vbr_ha(
        &self,
        request: DeleteVbrHaRequest,
    ) -> Result<DeleteVbrHaResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteVbrHa",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeVbrHa接口查询已创建的VBR倒换组。
    pub async fn describe_vbr_ha(
        &self,
        request: DescribeVbrHaRequest,
    ) -> Result<DescribeVbrHaResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeVbrHa",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用AssociatePhysicalConnectionToVirtualBorderRouter接口将边界路由器VBR（Virtual Border Router）关联到物理专线。
    pub async fn associate_physical_connection_to_virtual_border_router(
        &self,
        request: AssociatePhysicalConnectionToVirtualBorderRouterRequest,
    ) -> Result<AssociatePhysicalConnectionToVirtualBorderRouterResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AssociatePhysicalConnectionToVirtualBorderRouter",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateVirtualBorderRouter接口创建边界路由器VBR（Virtual Border Router）实例。
    pub async fn create_virtual_border_router(
        &self,
        request: CreateVirtualBorderRouterRequest,
    ) -> Result<CreateVirtualBorderRouterResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateVirtualBorderRouter",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteVirtualBorderRouter接口删除VBR实例。
    pub async fn delete_virtual_border_router(
        &self,
        request: DeleteVirtualBorderRouterRequest,
    ) -> Result<DeleteVirtualBorderRouterResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteVirtualBorderRouter",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用UnassociatePhysicalConnectionFromVirtualBorderRouter解绑VBR和物理专线。
    pub async fn unassociate_physical_connection_from_virtual_border_router(
        &self,
        request: UnassociatePhysicalConnectionFromVirtualBorderRouterRequest,
    ) -> Result<UnassociatePhysicalConnectionFromVirtualBorderRouterResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UnassociatePhysicalConnectionFromVirtualBorderRouter",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用UpdateVirtualBorderBandwidth接口更新边界路由器上下云双方向的带宽限速。
    pub async fn update_virtual_border_bandwidth(
        &self,
        request: UpdateVirtualBorderBandwidthRequest,
    ) -> Result<UpdateVirtualBorderBandwidthResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateVirtualBorderBandwidth",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyVirtualBorderRouterAttribute接口修改边界路由器VBR（Virtual Border Router）的配置。
    pub async fn modify_virtual_border_router_attribute(
        &self,
        request: ModifyVirtualBorderRouterAttributeRequest,
    ) -> Result<ModifyVirtualBorderRouterAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyVirtualBorderRouterAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用TerminateVirtualBorderRouter接口终止边界路由器（VBR）。
    pub async fn terminate_virtual_border_router(
        &self,
        request: TerminateVirtualBorderRouterRequest,
    ) -> Result<TerminateVirtualBorderRouterResponse, SdkError> {
        let api_request = ApiRequest {
            action: "TerminateVirtualBorderRouter",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用RecoverVirtualBorderRouter接口恢复被终止的VBR实例。
    pub async fn recover_virtual_border_router(
        &self,
        request: RecoverVirtualBorderRouterRequest,
    ) -> Result<RecoverVirtualBorderRouterResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RecoverVirtualBorderRouter",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeVirtualBorderRouters接口查询已创建的VBR实例。
    pub async fn describe_virtual_border_routers(
        &self,
        request: DescribeVirtualBorderRoutersRequest,
    ) -> Result<DescribeVirtualBorderRoutersResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeVirtualBorderRouters",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询VBR的专线网关跨账号授权列表。
    pub async fn describe_grant_rules_to_ecr(
        &self,
        request: DescribeGrantRulesToEcrRequest,
    ) -> Result<DescribeGrantRulesToEcrResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeGrantRulesToEcr",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定物理专线下的边界路由器（VBR），包括物理专线所有者的VBR和其他账号的VBR。
    pub async fn describe_virtual_border_routers_for_physical_connection(
        &self,
        request: DescribeVirtualBorderRoutersForPhysicalConnectionRequest,
    ) -> Result<DescribeVirtualBorderRoutersForPhysicalConnectionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeVirtualBorderRoutersForPhysicalConnection",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用AddBgpNetwork宣告BGP网络。
    pub async fn add_bgp_network(
        &self,
        request: AddBgpNetworkRequest,
    ) -> Result<AddBgpNetworkResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddBgpNetwork",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为指定的边界路由器（VBR）创建一个BGP组。
    pub async fn create_bgp_group(
        &self,
        request: CreateBgpGroupRequest,
    ) -> Result<CreateBgpGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateBgpGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateBgpPeer接口向指定的BGP组中添加BGP邻居。
    pub async fn create_bgp_peer(
        &self,
        request: CreateBgpPeerRequest,
    ) -> Result<CreateBgpPeerResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateBgpPeer",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 使用DeleteBgpGroup删除指定的BGP组。
    pub async fn delete_bgp_group(
        &self,
        request: DeleteBgpGroupRequest,
    ) -> Result<DeleteBgpGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteBgpGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 使用DeleteBgpNetwork删除已宣告的BGP网络。
    pub async fn delete_bgp_network(
        &self,
        request: DeleteBgpNetworkRequest,
    ) -> Result<DeleteBgpNetworkResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteBgpNetwork",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteBgpPeer删除指定的BGP邻居。
    pub async fn delete_bgp_peer(
        &self,
        request: DeleteBgpPeerRequest,
    ) -> Result<DeleteBgpPeerResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteBgpPeer",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyBgpGroupAttribute接口修改BGP组的配置。
    pub async fn modify_bgp_group_attribute(
        &self,
        request: ModifyBgpGroupAttributeRequest,
    ) -> Result<ModifyBgpGroupAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyBgpGroupAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyBgpPeerAttribute接口修改BGP邻居的属性。
    pub async fn modify_bgp_peer_attribute(
        &self,
        request: ModifyBgpPeerAttributeRequest,
    ) -> Result<ModifyBgpPeerAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyBgpPeerAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定地域下的BGP组。
    pub async fn describe_bgp_groups(
        &self,
        request: DescribeBgpGroupsRequest,
    ) -> Result<DescribeBgpGroupsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeBgpGroups",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeBgpNetworks查询已宣告的BGP网络。
    pub async fn describe_bgp_networks(
        &self,
        request: DescribeBgpNetworksRequest,
    ) -> Result<DescribeBgpNetworksResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeBgpNetworks",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeBgpPeers接口查询指定地域下的BGP邻居。
    pub async fn describe_bgp_peers(
        &self,
        request: DescribeBgpPeersRequest,
    ) -> Result<DescribeBgpPeersResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeBgpPeers",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateNatGateway接口创建增强型公网NAT网关或VPC NAT网关。
    pub async fn create_nat_gateway(
        &self,
        request: CreateNatGatewayRequest,
    ) -> Result<CreateNatGatewayResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateNatGateway",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteNatGateway接口删除指定的公网NAT网关。
    pub async fn delete_nat_gateway(
        &self,
        request: DeleteNatGatewayRequest,
    ) -> Result<DeleteNatGatewayResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteNatGateway",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyNatGatewayAttribute接口修改NAT网关的属性。
    pub async fn modify_nat_gateway_attribute(
        &self,
        request: ModifyNatGatewayAttributeRequest,
    ) -> Result<ModifyNatGatewayAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyNatGatewayAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 使用ModifyNatGatewaySpec接口修改预付费公网NAT网关的规格。
    pub async fn modify_nat_gateway_spec(
        &self,
        request: ModifyNatGatewaySpecRequest,
    ) -> Result<ModifyNatGatewaySpecResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyNatGatewaySpec",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeNatGateways以列表形式查询指定地域指定条件NAT网关的详细信息。
    pub async fn describe_nat_gateways(
        &self,
        request: DescribeNatGatewaysRequest,
    ) -> Result<DescribeNatGatewaysResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeNatGateways",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用GetNatGatewayAttribute接口查询单个NAT网关实例的信息。
    pub async fn get_nat_gateway_attribute(
        &self,
        request: GetNatGatewayAttributeRequest,
    ) -> Result<GetNatGatewayAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetNatGatewayAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ListEnhanhcedNatGatewayAvailableZones接口查询NAT网关的资源可用区。
    pub async fn list_enhanhced_nat_gateway_available_zones(
        &self,
        request: ListEnhanhcedNatGatewayAvailableZonesRequest,
    ) -> Result<ListEnhanhcedNatGatewayAvailableZonesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListEnhanhcedNatGatewayAvailableZones",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 将普通型NAT网关切换为增强型NAT网关。
    pub async fn update_nat_gateway_nat_type(
        &self,
        request: UpdateNatGatewayNatTypeRequest,
    ) -> Result<UpdateNatGatewayNatTypeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateNatGatewayNatType",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// VPC NAT网关作为服务资源对象被私网连接服务引用场景下，查询VPC NAT关联的弹性网卡列表，目前该特性暂不开放使用。
    pub async fn describe_nat_gateway_associate_network_interfaces(
        &self,
        request: DescribeNatGatewayAssociateNetworkInterfacesRequest,
    ) -> Result<DescribeNatGatewayAssociateNetworkInterfacesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeNatGatewayAssociateNetworkInterfaces",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用VpcDescribeVpcNatGatewayNetworkInterfaceQuota接口查询反向访问VPC NAT当前可以创建的EP个数。
    pub async fn vpc_describe_vpc_nat_gateway_network_interface_quota(
        &self,
        request: VpcDescribeVpcNatGatewayNetworkInterfaceQuotaRequest,
    ) -> Result<VpcDescribeVpcNatGatewayNetworkInterfaceQuotaResponse, SdkError> {
        let api_request = ApiRequest {
            action: "VpcDescribeVpcNatGatewayNetworkInterfaceQuota",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateSnatEntry接口在SNAT列表中添加SNAT条目。
    pub async fn create_snat_entry(
        &self,
        request: CreateSnatEntryRequest,
    ) -> Result<CreateSnatEntryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateSnatEntry",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteSnatEntry接口删除指定的SNAT条目。
    pub async fn delete_snat_entry(
        &self,
        request: DeleteSnatEntryRequest,
    ) -> Result<DeleteSnatEntryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteSnatEntry",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifySnatEntry接口修改指定的SNAT条目。
    pub async fn modify_snat_entry(
        &self,
        request: ModifySnatEntryRequest,
    ) -> Result<ModifySnatEntryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifySnatEntry",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeSnatTableEntries接口查询已创建的SNAT条目。
    pub async fn describe_snat_table_entries(
        &self,
        request: DescribeSnatTableEntriesRequest,
    ) -> Result<DescribeSnatTableEntriesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeSnatTableEntries",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateForwardEntry接口在DNAT列表中添加DNAT条目。
    pub async fn create_forward_entry(
        &self,
        request: CreateForwardEntryRequest,
    ) -> Result<CreateForwardEntryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateForwardEntry",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteForwardEntry接口删除指定的DNAT条目。
    pub async fn delete_forward_entry(
        &self,
        request: DeleteForwardEntryRequest,
    ) -> Result<DeleteForwardEntryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteForwardEntry",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyForwardEntry接口修改指定的DNAT条目。
    pub async fn modify_forward_entry(
        &self,
        request: ModifyForwardEntryRequest,
    ) -> Result<ModifyForwardEntryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyForwardEntry",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeForwardTableEntries接口查询已创建的DNAT条目。
    pub async fn describe_forward_table_entries(
        &self,
        request: DescribeForwardTableEntriesRequest,
    ) -> Result<DescribeForwardTableEntriesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeForwardTableEntries",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateFullNatEntry接口在FULLNAT列表中添加FULLNAT条目。
    pub async fn create_full_nat_entry(
        &self,
        request: CreateFullNatEntryRequest,
    ) -> Result<CreateFullNatEntryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateFullNatEntry",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteFullNatEntry接口删除FULLNAT条目。
    pub async fn delete_full_nat_entry(
        &self,
        request: DeleteFullNatEntryRequest,
    ) -> Result<DeleteFullNatEntryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteFullNatEntry",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyFullNatEntryAttribute接口修改FULLNAT条目。
    pub async fn modify_full_nat_entry_attribute(
        &self,
        request: ModifyFullNatEntryAttributeRequest,
    ) -> Result<ModifyFullNatEntryAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyFullNatEntryAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ListFullNatEntries接口查询FULLNAT条目的列表信息。
    pub async fn list_full_nat_entries(
        &self,
        request: ListFullNatEntriesRequest,
    ) -> Result<ListFullNatEntriesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListFullNatEntries",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为VPC NAT网关实例创建NAT IP地址。
    pub async fn create_nat_ip(
        &self,
        request: CreateNatIpRequest,
    ) -> Result<CreateNatIpResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateNatIp",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteNatIp接口删除NAT IP地址。
    pub async fn delete_nat_ip(
        &self,
        request: DeleteNatIpRequest,
    ) -> Result<DeleteNatIpResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteNatIp",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyNatIpAttribute接口修改NAT IP地址的名称和描述。
    pub async fn modify_nat_ip_attribute(
        &self,
        request: ModifyNatIpAttributeRequest,
    ) -> Result<ModifyNatIpAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyNatIpAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ListNatIps接口查询NAT IP地址列表。
    pub async fn list_nat_ips(
        &self,
        request: ListNatIpsRequest,
    ) -> Result<ListNatIpsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListNatIps",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为VPC NAT网关实例创建NAT IP地址段。
    pub async fn create_nat_ip_cidr(
        &self,
        request: CreateNatIpCidrRequest,
    ) -> Result<CreateNatIpCidrResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateNatIpCidr",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteNatIpCidr接口删除NAT IP地址段。
    pub async fn delete_nat_ip_cidr(
        &self,
        request: DeleteNatIpCidrRequest,
    ) -> Result<DeleteNatIpCidrResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteNatIpCidr",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyNatIpCidrAttribute接口修改NAT IP地址段的名称和描述。
    pub async fn modify_nat_ip_cidr_attribute(
        &self,
        request: ModifyNatIpCidrAttributeRequest,
    ) -> Result<ModifyNatIpCidrAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyNatIpCidrAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ListNatIpCidrs接口查询NAT IP地址段列表。
    pub async fn list_nat_ip_cidrs(
        &self,
        request: ListNatIpCidrsRequest,
    ) -> Result<ListNatIpCidrsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListNatIpCidrs",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateIpv4Gateway接口创建IPv4网关。
    pub async fn create_ipv4_gateway(
        &self,
        request: CreateIpv4GatewayRequest,
    ) -> Result<CreateIpv4GatewayResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateIpv4Gateway",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteIpv4Gateway接口删除IPv4网关。
    pub async fn delete_ipv4_gateway(
        &self,
        request: DeleteIpv4GatewayRequest,
    ) -> Result<DeleteIpv4GatewayResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteIpv4Gateway",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用EnableVpcIpv4Gateway接口激活IPv4网关。
    pub async fn enable_vpc_ipv4_gateway(
        &self,
        request: EnableVpcIpv4GatewayRequest,
    ) -> Result<EnableVpcIpv4GatewayResponse, SdkError> {
        let api_request = ApiRequest {
            action: "EnableVpcIpv4Gateway",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用UpdateIpv4GatewayAttribut修改IPv4网关的名称或描述信息。
    pub async fn update_ipv4_gateway_attribute(
        &self,
        request: UpdateIpv4GatewayAttributeRequest,
    ) -> Result<UpdateIpv4GatewayAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateIpv4GatewayAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ListIpv4Gateways接口查询IPv4网关实例的列表信息。
    pub async fn list_ipv4_gateways(
        &self,
        request: ListIpv4GatewaysRequest,
    ) -> Result<ListIpv4GatewaysResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListIpv4Gateways",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用GetIpv4GatewayAttribute接口
    pub async fn get_ipv4_gateway_attribute(
        &self,
        request: GetIpv4GatewayAttributeRequest,
    ) -> Result<GetIpv4GatewayAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetIpv4GatewayAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateVpnGateway接口创建VPN网关。
    pub async fn create_vpn_gateway(
        &self,
        request: CreateVpnGatewayRequest,
    ) -> Result<CreateVpnGatewayResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateVpnGateway",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用AssociateVpnGatewayWithCertificate接口为VPN网关绑定证书。
    pub async fn associate_vpn_gateway_with_certificate(
        &self,
        request: AssociateVpnGatewayWithCertificateRequest,
    ) -> Result<AssociateVpnGatewayWithCertificateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AssociateVpnGatewayWithCertificate",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteVpnGateway接口删除指定的VPN网关。
    pub async fn delete_vpn_gateway(
        &self,
        request: DeleteVpnGatewayRequest,
    ) -> Result<DeleteVpnGatewayResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteVpnGateway",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DissociateVpnGatewayWithCertificate接口解除VPN网关和证书的绑定关系。
    pub async fn dissociate_vpn_gateway_with_certificate(
        &self,
        request: DissociateVpnGatewayWithCertificateRequest,
    ) -> Result<DissociateVpnGatewayWithCertificateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DissociateVpnGatewayWithCertificate",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyVpnGatewayAttribute接口修改VPN网关的名称、描述信息或路由自动传播功能。
    pub async fn modify_vpn_gateway_attribute(
        &self,
        request: ModifyVpnGatewayAttributeRequest,
    ) -> Result<ModifyVpnGatewayAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyVpnGatewayAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改VPN网关资源所属的资源组。
    pub async fn move_vpn_resource_group(
        &self,
        request: MoveVpnResourceGroupRequest,
    ) -> Result<MoveVpnResourceGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "MoveVpnResourceGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeVpnGateway接口查询指定VPN网关的详细信息。
    pub async fn describe_vpn_gateway(
        &self,
        request: DescribeVpnGatewayRequest,
    ) -> Result<DescribeVpnGatewayResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeVpnGateway",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeVpnGateways接口查询指定地域下已创建的VPN网关。
    pub async fn describe_vpn_gateways(
        &self,
        request: DescribeVpnGatewaysRequest,
    ) -> Result<DescribeVpnGatewaysResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeVpnGateways",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ListVpnCertificateAssociations接口查询指定地域下VPN网关实例和证书的绑定关系。
    pub async fn list_vpn_certificate_associations(
        &self,
        request: ListVpnCertificateAssociationsRequest,
    ) -> Result<ListVpnCertificateAssociationsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListVpnCertificateAssociations",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DiagnoseVpnGateway接口一键诊断指定的VPN网关实例。
    pub async fn diagnose_vpn_gateway(
        &self,
        request: DiagnoseVpnGatewayRequest,
    ) -> Result<DiagnoseVpnGatewayResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DiagnoseVpnGateway",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用GetVpnGatewayDiagnoseResult接口查询VPN网关实例的一键诊断结果。
    pub async fn get_vpn_gateway_diagnose_result(
        &self,
        request: GetVpnGatewayDiagnoseResultRequest,
    ) -> Result<GetVpnGatewayDiagnoseResultResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetVpnGatewayDiagnoseResult",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeVpnGatewayAvailableZones接口查询指定地域支持部署IPsec连接的可用区列表。
    pub async fn describe_vpn_gateway_available_zones(
        &self,
        request: DescribeVpnGatewayAvailableZonesRequest,
    ) -> Result<DescribeVpnGatewayAvailableZonesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeVpnGatewayAvailableZones",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateCustomerGateway接口创建用户网关。
    pub async fn create_customer_gateway(
        &self,
        request: CreateCustomerGatewayRequest,
    ) -> Result<CreateCustomerGatewayResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateCustomerGateway",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteCustomerGateway接口删除指定的用户网关。
    pub async fn delete_customer_gateway(
        &self,
        request: DeleteCustomerGatewayRequest,
    ) -> Result<DeleteCustomerGatewayResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteCustomerGateway",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyCustomerGatewayAttribute接口修改用户网关的配置。
    pub async fn modify_customer_gateway_attribute(
        &self,
        request: ModifyCustomerGatewayAttributeRequest,
    ) -> Result<ModifyCustomerGatewayAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyCustomerGatewayAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询已创建的用户网关的详细信息。
    pub async fn describe_customer_gateway(
        &self,
        request: DescribeCustomerGatewayRequest,
    ) -> Result<DescribeCustomerGatewayResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCustomerGateway",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeCustomerGateways接口查询用户网关的信息。
    pub async fn describe_customer_gateways(
        &self,
        request: DescribeCustomerGatewaysRequest,
    ) -> Result<DescribeCustomerGatewaysResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCustomerGateways",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateVpnConnection接口创建IPsec连接。
    pub async fn create_vpn_connection(
        &self,
        request: CreateVpnConnectionRequest,
    ) -> Result<CreateVpnConnectionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateVpnConnection",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteVpnConnection接口删除指定的IPsec连接。
    pub async fn delete_vpn_connection(
        &self,
        request: DeleteVpnConnectionRequest,
    ) -> Result<DeleteVpnConnectionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteVpnConnection",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyVpnConnectionAttribute接口修改IPsec连接的配置信息。
    pub async fn modify_vpn_connection_attribute(
        &self,
        request: ModifyVpnConnectionAttributeRequest,
    ) -> Result<ModifyVpnConnectionAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyVpnConnectionAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyTunnelAttribute接口修改高可用VPN的隧道配置信息。
    pub async fn modify_tunnel_attribute(
        &self,
        request: ModifyTunnelAttributeRequest,
    ) -> Result<ModifyTunnelAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyTunnelAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeVpnConnection接口查询指定的IPsec连接的信息。
    pub async fn describe_vpn_connection(
        &self,
        request: DescribeVpnConnectionRequest,
    ) -> Result<DescribeVpnConnectionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeVpnConnection",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeVpnConnections接口查询IPsec连接信息。
    pub async fn describe_vpn_connections(
        &self,
        request: DescribeVpnConnectionsRequest,
    ) -> Result<DescribeVpnConnectionsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeVpnConnections",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DownloadVpnConnectionConfig接口获取IPsec连接的配置信息。
    pub async fn download_vpn_connection_config(
        &self,
        request: DownloadVpnConnectionConfigRequest,
    ) -> Result<DownloadVpnConnectionConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DownloadVpnConnectionConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeVpnConnections接口查询IPsec连接的日志信息。
    pub async fn describe_vpn_connection_logs(
        &self,
        request: DescribeVpnConnectionLogsRequest,
    ) -> Result<DescribeVpnConnectionLogsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeVpnConnectionLogs",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DiagnoseVpnConnections接口诊断IPsec连接。
    pub async fn diagnose_vpn_connections(
        &self,
        request: DiagnoseVpnConnectionsRequest,
    ) -> Result<DiagnoseVpnConnectionsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DiagnoseVpnConnections",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateVpnRouteEntry接口为VPN网关实例创建目的路由，创建目的路由后，VPN网关实例将基于流量的目的IP地址匹配目的路由，然后根据流量匹配到的目的路由转发流量。
    pub async fn create_vpn_route_entry(
        &self,
        request: CreateVpnRouteEntryRequest,
    ) -> Result<CreateVpnRouteEntryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateVpnRouteEntry",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用PublishVpnRouteEntry接口将目的路由或策略路由发布到VPC的系统路由表中或将已发布的目的路由或策略路由从VPC系统路由表中撤销。
    pub async fn publish_vpn_route_entry(
        &self,
        request: PublishVpnRouteEntryRequest,
    ) -> Result<PublishVpnRouteEntryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PublishVpnRouteEntry",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteVpnRouteEntry接口删除VPN目的路由。
    pub async fn delete_vpn_route_entry(
        &self,
        request: DeleteVpnRouteEntryRequest,
    ) -> Result<DeleteVpnRouteEntryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteVpnRouteEntry",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyVpnRouteEntryWeight接口修改目的路由的权重值。
    pub async fn modify_vpn_route_entry_weight(
        &self,
        request: ModifyVpnRouteEntryWeightRequest,
    ) -> Result<ModifyVpnRouteEntryWeightResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyVpnRouteEntryWeight",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeVpnRouteEntries接口查询VPN网关实例的目的路由条目信息和BGP路由条目信息。
    pub async fn describe_vpn_route_entries(
        &self,
        request: DescribeVpnRouteEntriesRequest,
    ) -> Result<DescribeVpnRouteEntriesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeVpnRouteEntries",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateVpnPbrRouteEntry接口创建VPN策略路由。
    pub async fn create_vpn_pbr_route_entry(
        &self,
        request: CreateVpnPbrRouteEntryRequest,
    ) -> Result<CreateVpnPbrRouteEntryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateVpnPbrRouteEntry",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteVpnPbrRouteEntry接口删除策略路由。
    pub async fn delete_vpn_pbr_route_entry(
        &self,
        request: DeleteVpnPbrRouteEntryRequest,
    ) -> Result<DeleteVpnPbrRouteEntryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteVpnPbrRouteEntry",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyVpnPbrRouteEntryWeight接口修改VPN策略路由的权重值。
    pub async fn modify_vpn_pbr_route_entry_weight(
        &self,
        request: ModifyVpnPbrRouteEntryWeightRequest,
    ) -> Result<ModifyVpnPbrRouteEntryWeightResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyVpnPbrRouteEntryWeight",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyVpnPbrRouteEntryPriority接口修改策略路由的策略优先级。
    pub async fn modify_vpn_pbr_route_entry_priority(
        &self,
        request: ModifyVpnPbrRouteEntryPriorityRequest,
    ) -> Result<ModifyVpnPbrRouteEntryPriorityResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyVpnPbrRouteEntryPriority",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyVpnPbrRouteEntryAttribute接口修改策略路由的权重值和策略优先级。
    pub async fn modify_vpn_pbr_route_entry_attribute(
        &self,
        request: ModifyVpnPbrRouteEntryAttributeRequest,
    ) -> Result<ModifyVpnPbrRouteEntryAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyVpnPbrRouteEntryAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeVpnPbrRouteEntries接口查询VPN网关实例下配置的策略路由的信息。
    pub async fn describe_vpn_pbr_route_entries(
        &self,
        request: DescribeVpnPbrRouteEntriesRequest,
    ) -> Result<DescribeVpnPbrRouteEntriesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeVpnPbrRouteEntries",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateVpnAttachment接口创建IPsec连接，用于绑定转发路由器实例。
    pub async fn create_vpn_attachment(
        &self,
        request: CreateVpnAttachmentRequest,
    ) -> Result<CreateVpnAttachmentResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateVpnAttachment",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateVcoRouteEntry接口为IPsec连接添加目的路由条目。
    pub async fn create_vco_route_entry(
        &self,
        request: CreateVcoRouteEntryRequest,
    ) -> Result<CreateVcoRouteEntryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateVcoRouteEntry",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteVpnAttachment接口删除IPsec连接。
    pub async fn delete_vpn_attachment(
        &self,
        request: DeleteVpnAttachmentRequest,
    ) -> Result<DeleteVpnAttachmentResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteVpnAttachment",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteVcoRouteEntry接口删除IPsec连接下的目的路由条目。
    pub async fn delete_vco_route_entry(
        &self,
        request: DeleteVcoRouteEntryRequest,
    ) -> Result<DeleteVcoRouteEntryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteVcoRouteEntry",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyVpnAttachmentAttribute接口修改IPsec连接的配置。
    pub async fn modify_vpn_attachment_attribute(
        &self,
        request: ModifyVpnAttachmentAttributeRequest,
    ) -> Result<ModifyVpnAttachmentAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyVpnAttachmentAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyVcoRouteEntryWeight接口修改IPsec连接下目的路由条目的权重值。
    pub async fn modify_vco_route_entry_weight(
        &self,
        request: ModifyVcoRouteEntryWeightRequest,
    ) -> Result<ModifyVcoRouteEntryWeightResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyVcoRouteEntryWeight",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeVpnAttachments接口查询已绑定转发路由器实例的IPsec连接的配置信息。
    pub async fn describe_vpn_attachments(
        &self,
        request: DescribeVpnAttachmentsRequest,
    ) -> Result<DescribeVpnAttachmentsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeVpnAttachments",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeVcoRouteEntries接口查询IPsec连接下路由条目的信息。
    pub async fn describe_vco_route_entries(
        &self,
        request: DescribeVcoRouteEntriesRequest,
    ) -> Result<DescribeVcoRouteEntriesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeVcoRouteEntries",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CheckVpnBgpEnabled接口查询IPsec连接所属的地域是否支持BGP功能。
    pub async fn check_vpn_bgp_enabled(
        &self,
        request: CheckVpnBgpEnabledRequest,
    ) -> Result<CheckVpnBgpEnabledResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CheckVpnBgpEnabled",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeVpnCrossAccountAuthorizations接口查询IPsec连接的跨账号授权信息。
    pub async fn describe_vpn_cross_account_authorizations(
        &self,
        request: DescribeVpnCrossAccountAuthorizationsRequest,
    ) -> Result<DescribeVpnCrossAccountAuthorizationsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeVpnCrossAccountAuthorizations",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateSslVpnClientCert接口创建SSL-VPN客户端证书。
    pub async fn create_ssl_vpn_client_cert(
        &self,
        request: CreateSslVpnClientCertRequest,
    ) -> Result<CreateSslVpnClientCertResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateSslVpnClientCert",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteSslVpnClientCert接口删除SSL-VPN客户端证书。
    pub async fn delete_ssl_vpn_client_cert(
        &self,
        request: DeleteSslVpnClientCertRequest,
    ) -> Result<DeleteSslVpnClientCertResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteSslVpnClientCert",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifySslVpnClientCert接口修改SSL-VPN客户端证书的名称。
    pub async fn modify_ssl_vpn_client_cert(
        &self,
        request: ModifySslVpnClientCertRequest,
    ) -> Result<ModifySslVpnClientCertResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifySslVpnClientCert",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeSslVpnClientCerts接口查询已创建的SSL客户端证书。
    pub async fn describe_ssl_vpn_client_certs(
        &self,
        request: DescribeSslVpnClientCertsRequest,
    ) -> Result<DescribeSslVpnClientCertsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeSslVpnClientCerts",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeSslVpnClientCert接口查询指定的SSL客户端证书信息。
    pub async fn describe_ssl_vpn_client_cert(
        &self,
        request: DescribeSslVpnClientCertRequest,
    ) -> Result<DescribeSslVpnClientCertResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeSslVpnClientCert",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeSslVpnClients接口查询指定VPN网关实例下已与阿里云建立SSL-VPN连接的客户端的信息。
    pub async fn describe_ssl_vpn_clients(
        &self,
        request: DescribeSslVpnClientsRequest,
    ) -> Result<DescribeSslVpnClientsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeSslVpnClients",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateSslVpnServer接口创建SSL-VPN服务端。
    pub async fn create_ssl_vpn_server(
        &self,
        request: CreateSslVpnServerRequest,
    ) -> Result<CreateSslVpnServerResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateSslVpnServer",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteSslVpnServer接口删除SSL-VPN服务端，删除SSL-VPN服务端后系统会自动删除SSL-VPN服务端关联的所有SSL客户端证书，安装了这些SSL客户端证书的客户端将...
    pub async fn delete_ssl_vpn_server(
        &self,
        request: DeleteSslVpnServerRequest,
    ) -> Result<DeleteSslVpnServerResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteSslVpnServer",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifySslVpnServer接口修改SSL-VPN服务端的配置信息。
    pub async fn modify_ssl_vpn_server(
        &self,
        request: ModifySslVpnServerRequest,
    ) -> Result<ModifySslVpnServerResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifySslVpnServer",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeSslVpnServers接口查询已创建的SSL-VPN服务端。
    pub async fn describe_ssl_vpn_servers(
        &self,
        request: DescribeSslVpnServersRequest,
    ) -> Result<DescribeSslVpnServersResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeSslVpnServers",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeVpnSslServerLogs接口查看SSL服务端的日志。
    pub async fn describe_vpn_ssl_server_logs(
        &self,
        request: DescribeVpnSslServerLogsRequest,
    ) -> Result<DescribeVpnSslServerLogsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeVpnSslServerLogs",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateIpsecServer接口创建IPsec服务端。
    pub async fn create_ipsec_server(
        &self,
        request: CreateIpsecServerRequest,
    ) -> Result<CreateIpsecServerResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateIpsecServer",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteIpsecServer接口删除IPsec服务端。
    pub async fn delete_ipsec_server(
        &self,
        request: DeleteIpsecServerRequest,
    ) -> Result<DeleteIpsecServerResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteIpsecServer",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用UpdateIpsecServer接口更新IPsec服务端配置。
    pub async fn update_ipsec_server(
        &self,
        request: UpdateIpsecServerRequest,
    ) -> Result<UpdateIpsecServerResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateIpsecServer",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ListIpsecServers接口查看已创建的IPsec服务端。
    pub async fn list_ipsec_servers(
        &self,
        request: ListIpsecServersRequest,
    ) -> Result<ListIpsecServersResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListIpsecServers",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ListIpsecServerLogs接口查看IPsec服务端日志。
    pub async fn list_ipsec_server_logs(
        &self,
        request: ListIpsecServerLogsRequest,
    ) -> Result<ListIpsecServerLogsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListIpsecServerLogs",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 分配一个IPv6地址。
    pub async fn allocate_ipv6_address(
        &self,
        request: AllocateIpv6AddressRequest,
    ) -> Result<AllocateIpv6AddressResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AllocateIpv6Address",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 释放一个未关联实例的IPv6地址。
    pub async fn release_ipv6_address(
        &self,
        request: ReleaseIpv6AddressRequest,
    ) -> Result<ReleaseIpv6AddressResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ReleaseIpv6Address",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// IPv6网关是控制专有网络VPC（Virtual Private Cloud）的IPv6网络流量的网关。您可以通过调用CreateIpv6Gateway接口创建IPv6网关。
    pub async fn create_ipv6_gateway(
        &self,
        request: CreateIpv6GatewayRequest,
    ) -> Result<CreateIpv6GatewayResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateIpv6Gateway",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 您可以通过创建仅主动出规则，使开通了IPv6网段的VPC中的ECS可以主动访问客户端的IPv6终端，但不允许客户端的IPv6终端通过互联网访问VPC网络中的ECS实例，调用CreateIpv6E...
    pub async fn create_ipv6_egress_only_rule(
        &self,
        request: CreateIpv6EgressOnlyRuleRequest,
    ) -> Result<CreateIpv6EgressOnlyRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateIpv6EgressOnlyRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteIpv6Gateway接口删除IPv6网关。
    pub async fn delete_ipv6_gateway(
        &self,
        request: DeleteIpv6GatewayRequest,
    ) -> Result<DeleteIpv6GatewayResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteIpv6Gateway",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteIpv6EgressOnlyRule接口删除仅主动出规则。
    pub async fn delete_ipv6_egress_only_rule(
        &self,
        request: DeleteIpv6EgressOnlyRuleRequest,
    ) -> Result<DeleteIpv6EgressOnlyRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteIpv6EgressOnlyRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteIpv6InternetBandwidth接口删除公网带宽。
    pub async fn delete_ipv6_internet_bandwidth(
        &self,
        request: DeleteIpv6InternetBandwidthRequest,
    ) -> Result<DeleteIpv6InternetBandwidthResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteIpv6InternetBandwidth",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改IPv6地址的公网带宽。
    pub async fn modify_ipv6_internet_bandwidth(
        &self,
        request: ModifyIpv6InternetBandwidthRequest,
    ) -> Result<ModifyIpv6InternetBandwidthResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyIpv6InternetBandwidth",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改IPv6网关的信息。
    pub async fn modify_ipv6_gateway_attribute(
        &self,
        request: ModifyIpv6GatewayAttributeRequest,
    ) -> Result<ModifyIpv6GatewayAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyIpv6GatewayAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改IPv6地址的名称和描述。
    pub async fn modify_ipv6_address_attribute(
        &self,
        request: ModifyIpv6AddressAttributeRequest,
    ) -> Result<ModifyIpv6AddressAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyIpv6AddressAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeIpv6Gateways接口查询某个地域下已创建的IPv6网关。
    pub async fn describe_ipv6_gateways(
        &self,
        request: DescribeIpv6GatewaysRequest,
    ) -> Result<DescribeIpv6GatewaysResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeIpv6Gateways",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询创建的仅主动出规则。
    pub async fn describe_ipv6_egress_only_rules(
        &self,
        request: DescribeIpv6EgressOnlyRulesRequest,
    ) -> Result<DescribeIpv6EgressOnlyRulesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeIpv6EgressOnlyRules",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询IPv6地址列表。
    pub async fn describe_ipv6_addresses(
        &self,
        request: DescribeIpv6AddressesRequest,
    ) -> Result<DescribeIpv6AddressesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeIpv6Addresses",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeIpv6GatewayAttribute接口查询指定IPv6网关所在地域，所属VPC，运行状态，计费方式等详细信息。
    pub async fn describe_ipv6_gateway_attribute(
        &self,
        request: DescribeIpv6GatewayAttributeRequest,
    ) -> Result<DescribeIpv6GatewayAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeIpv6GatewayAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 默认创建的IPv6网关只具备私网通信的能力，您可以调用AllocateIpv6InternetBandwidth接口为IPv6地址购买公网带宽，使VPC网络中的ECS实例可以通过该IPv6地址访...
    pub async fn allocate_ipv6_internet_bandwidth(
        &self,
        request: AllocateIpv6InternetBandwidthRequest,
    ) -> Result<AllocateIpv6InternetBandwidthResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AllocateIpv6InternetBandwidth",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建访问控制策略组。
    pub async fn create_i_pv6_translator_acl_list(
        &self,
        request: CreateIPv6TranslatorAclListRequest,
    ) -> Result<CreateIPv6TranslatorAclListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateIPv6TranslatorAclList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为指定的IPv6转换服务实例添加IPv6转换映射条目。
    pub async fn create_i_pv6_translator_entry(
        &self,
        request: CreateIPv6TranslatorEntryRequest,
    ) -> Result<CreateIPv6TranslatorEntryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateIPv6TranslatorEntry",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 在访问控制策略组中添加IP条目。
    pub async fn add_i_pv6_translator_acl_list_entry(
        &self,
        request: AddIPv6TranslatorAclListEntryRequest,
    ) -> Result<AddIPv6TranslatorAclListEntryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddIPv6TranslatorAclListEntry",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建IPv6转换服务实例。
    pub async fn create_i_pv6_translator(
        &self,
        request: CreateIPv6TranslatorRequest,
    ) -> Result<CreateIPv6TranslatorResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateIPv6Translator",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除访问控制策略组。只有当访问控制策略组没有绑定任何IPv6转换映射时，才可以删除。
    pub async fn delete_i_pv6_translator_acl_list(
        &self,
        request: DeleteIPv6TranslatorAclListRequest,
    ) -> Result<DeleteIPv6TranslatorAclListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteIPv6TranslatorAclList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除IPv6转换映射条目。
    pub async fn delete_i_pv6_translator_entry(
        &self,
        request: DeleteIPv6TranslatorEntryRequest,
    ) -> Result<DeleteIPv6TranslatorEntryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteIPv6TranslatorEntry",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除IPv6转换服务实例。
    pub async fn delete_i_pv6_translator(
        &self,
        request: DeleteIPv6TranslatorRequest,
    ) -> Result<DeleteIPv6TranslatorResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteIPv6Translator",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改访问控制策略组中的IP条目。
    pub async fn modify_i_pv6_translator_acl_list_entry(
        &self,
        request: ModifyIPv6TranslatorAclListEntryRequest,
    ) -> Result<ModifyIPv6TranslatorAclListEntryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyIPv6TranslatorAclListEntry",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改访问控制策略组的名称。
    pub async fn modify_i_pv6_translator_acl_attribute(
        &self,
        request: ModifyIPv6TranslatorAclAttributeRequest,
    ) -> Result<ModifyIPv6TranslatorAclAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyIPv6TranslatorAclAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改IPv6转换服务实例的带宽。
    pub async fn modify_i_pv6_translator_bandwidth(
        &self,
        request: ModifyIPv6TranslatorBandwidthRequest,
    ) -> Result<ModifyIPv6TranslatorBandwidthResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyIPv6TranslatorBandwidth",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改IPv6转换服务实例的名称和描述信息。
    pub async fn modify_i_pv6_translator_attribute(
        &self,
        request: ModifyIPv6TranslatorAttributeRequest,
    ) -> Result<ModifyIPv6TranslatorAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyIPv6TranslatorAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改IPv6转换映射条目。
    pub async fn modify_i_pv6_translator_entry(
        &self,
        request: ModifyIPv6TranslatorEntryRequest,
    ) -> Result<ModifyIPv6TranslatorEntryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyIPv6TranslatorEntry",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询已创建的访问控制策略组。
    pub async fn describe_i_pv6_translator_acl_lists(
        &self,
        request: DescribeIPv6TranslatorAclListsRequest,
    ) -> Result<DescribeIPv6TranslatorAclListsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeIPv6TranslatorAclLists",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询访问控制策略组的详细信息，包括访问控制策略组中的IP和关联的IPv6转换映射条目的具体信息。
    pub async fn describe_i_pv6_translator_acl_list_attributes(
        &self,
        request: DescribeIPv6TranslatorAclListAttributesRequest,
    ) -> Result<DescribeIPv6TranslatorAclListAttributesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeIPv6TranslatorAclListAttributes",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询已创建的IPv6转换服务实例列表。
    pub async fn describe_i_pv6_translators(
        &self,
        request: DescribeIPv6TranslatorsRequest,
    ) -> Result<DescribeIPv6TranslatorsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeIPv6Translators",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询IPv6转换映射条目。
    pub async fn describe_i_pv6_translator_entries(
        &self,
        request: DescribeIPv6TranslatorEntriesRequest,
    ) -> Result<DescribeIPv6TranslatorEntriesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeIPv6TranslatorEntries",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除访问控制策略组中的IP条目。
    pub async fn remove_i_pv6_translator_acl_list_entry(
        &self,
        request: RemoveIPv6TranslatorAclListEntryRequest,
    ) -> Result<RemoveIPv6TranslatorAclListEntryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RemoveIPv6TranslatorAclListEntry",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeRegions接口查询可用的地域。
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

    /// 调用DescribeZones接口查询指定地域中可用区的列表。
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

    /// 为指定的资源统一创建并绑定标签。
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

    /// 调用TagResourcesForExpressConnect接口为指定的高速通道专线实例统一创建并绑定标签。
    pub async fn tag_resources_for_express_connect(
        &self,
        request: TagResourcesForExpressConnectRequest,
    ) -> Result<TagResourcesForExpressConnectResponse, SdkError> {
        let api_request = ApiRequest {
            action: "TagResourcesForExpressConnect",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为指定的资源列表统一解绑标签。
    pub async fn un_tag_resources(
        &self,
        request: UnTagResourcesRequest,
    ) -> Result<UnTagResourcesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UnTagResources",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用UntagResourcesForExpressConnect接口为高速通道指定的资源列表统一解绑标签。
    pub async fn untag_resources_for_express_connect(
        &self,
        request: UntagResourcesForExpressConnectRequest,
    ) -> Result<UntagResourcesForExpressConnectResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UntagResourcesForExpressConnect",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询云资源已经绑定的标签列表。
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

    /// 调用DescribeTags接口查询满足筛选条件的标签列表。
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

    /// 调用DescribeTagKeys接口返回标签键。
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

    /// 调用ListTagResourcesForExpressConnect接口查询高速通道已经绑定的标签列表。
    pub async fn list_tag_resources_for_express_connect(
        &self,
        request: ListTagResourcesForExpressConnectRequest,
    ) -> Result<ListTagResourcesForExpressConnectResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListTagResourcesForExpressConnect",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeTagKeysForExpressConnect接口返回高速通道的标签列表。
    pub async fn describe_tag_keys_for_express_connect(
        &self,
        request: DescribeTagKeysForExpressConnectRequest,
    ) -> Result<DescribeTagKeysForExpressConnectResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeTagKeysForExpressConnect",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateRouterInterface接口创建路由器接口。
    pub async fn create_router_interface(
        &self,
        request: CreateRouterInterfaceRequest,
    ) -> Result<CreateRouterInterfaceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateRouterInterface",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteRouterInterface接口删除路由器接口。
    pub async fn delete_router_interface(
        &self,
        request: DeleteRouterInterfaceRequest,
    ) -> Result<DeleteRouterInterfaceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteRouterInterface",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除高速通道实例，包含发起端和接收端。
    pub async fn delete_express_connect(
        &self,
        request: DeleteExpressConnectRequest,
    ) -> Result<DeleteExpressConnectResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteExpressConnect",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyRouterInterfaceAttribute接口修改路由器接口的配置。
    pub async fn modify_router_interface_attribute(
        &self,
        request: ModifyRouterInterfaceAttributeRequest,
    ) -> Result<ModifyRouterInterfaceAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyRouterInterfaceAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyRouterInterfaceSpec接口修改路由器接口的规格。
    pub async fn modify_router_interface_spec(
        &self,
        request: ModifyRouterInterfaceSpecRequest,
    ) -> Result<ModifyRouterInterfaceSpecResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyRouterInterfaceSpec",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeRouterInterfaces接口查询指定地域内的路由器接口。
    pub async fn describe_router_interfaces(
        &self,
        request: DescribeRouterInterfacesRequest,
    ) -> Result<DescribeRouterInterfacesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeRouterInterfaces",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeEcGrantRelation接口查询跨账号创建VBR上连场景下VPC实例对VBR实例的授权关系。
    pub async fn describe_ec_grant_relation(
        &self,
        request: DescribeEcGrantRelationRequest,
    ) -> Result<DescribeEcGrantRelationResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeEcGrantRelation",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeRouterInterfaceAttribute查看路由器接口配置。
    pub async fn describe_router_interface_attribute(
        &self,
        request: DescribeRouterInterfaceAttributeRequest,
    ) -> Result<DescribeRouterInterfaceAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeRouterInterfaceAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 使用DeactivateRouterInterface冻结路由器接口。
    pub async fn deactivate_router_interface(
        &self,
        request: DeactivateRouterInterfaceRequest,
    ) -> Result<DeactivateRouterInterfaceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeactivateRouterInterface",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用GrantInstanceToVbr接口将跨账号创建VBR上连场景的VPC实例授权给VBR实例。
    pub async fn grant_instance_to_vbr(
        &self,
        request: GrantInstanceToVbrRequest,
    ) -> Result<GrantInstanceToVbrResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GrantInstanceToVbr",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ActivateRouterInterface接口激活处于Inactive状态的路由器接口。
    pub async fn activate_router_interface(
        &self,
        request: ActivateRouterInterfaceRequest,
    ) -> Result<ActivateRouterInterfaceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ActivateRouterInterface",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用RevokeInstanceFromVbr接口取消跨账号创建VBR上连场景下VPC实例对VBR实例的授权。
    pub async fn revoke_instance_from_vbr(
        &self,
        request: RevokeInstanceFromVbrRequest,
    ) -> Result<RevokeInstanceFromVbrResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RevokeInstanceFromVbr",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ConnectRouterInterface接口由发起端路由器接口向接收端发起连接。
    pub async fn connect_router_interface(
        &self,
        request: ConnectRouterInterfaceRequest,
    ) -> Result<ConnectRouterInterfaceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ConnectRouterInterface",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateExpressCloudConnection创建高速上云服务实例。
    pub async fn create_express_cloud_connection(
        &self,
        request: CreateExpressCloudConnectionRequest,
    ) -> Result<CreateExpressCloudConnectionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateExpressCloudConnection",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyExpressCloudConnectionAttribute修改高速上云服务连接。
    pub async fn modify_express_cloud_connection_attribute(
        &self,
        request: ModifyExpressCloudConnectionAttributeRequest,
    ) -> Result<ModifyExpressCloudConnectionAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyExpressCloudConnectionAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyExpressCloudConnectionBandwidth修改高速上云服务带宽。
    pub async fn modify_express_cloud_connection_bandwidth(
        &self,
        request: ModifyExpressCloudConnectionBandwidthRequest,
    ) -> Result<ModifyExpressCloudConnectionBandwidthResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyExpressCloudConnectionBandwidth",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建全球加速实例。
    pub async fn create_global_acceleration_instance(
        &self,
        request: CreateGlobalAccelerationInstanceRequest,
    ) -> Result<CreateGlobalAccelerationInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateGlobalAccelerationInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用AddGlobalAccelerationInstanceIp接口添加EIP到指定的带宽共享实例中。
    pub async fn add_global_acceleration_instance_ip(
        &self,
        request: AddGlobalAccelerationInstanceIpRequest,
    ) -> Result<AddGlobalAccelerationInstanceIpResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddGlobalAccelerationInstanceIp",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除全球加速实例。
    pub async fn delete_global_acceleration_instance(
        &self,
        request: DeleteGlobalAccelerationInstanceRequest,
    ) -> Result<DeleteGlobalAccelerationInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteGlobalAccelerationInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用UnassociateGlobalAccelerationInstance接口解绑与全球加速实例关联的后端服务实例。
    pub async fn unassociate_global_acceleration_instance(
        &self,
        request: UnassociateGlobalAccelerationInstanceRequest,
    ) -> Result<UnassociateGlobalAccelerationInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UnassociateGlobalAccelerationInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改全球加速实例的名称和描述信息。
    pub async fn modify_global_acceleration_instance_attributes(
        &self,
        request: ModifyGlobalAccelerationInstanceAttributesRequest,
    ) -> Result<ModifyGlobalAccelerationInstanceAttributesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyGlobalAccelerationInstanceAttributes",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyGlobalAccelerationInstanceSpec接口修改全球加速实例的带宽。
    pub async fn modify_global_acceleration_instance_spec(
        &self,
        request: ModifyGlobalAccelerationInstanceSpecRequest,
    ) -> Result<ModifyGlobalAccelerationInstanceSpecResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyGlobalAccelerationInstanceSpec",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询已创建的全球加速实例列表。
    pub async fn describe_global_acceleration_instances(
        &self,
        request: DescribeGlobalAccelerationInstancesRequest,
    ) -> Result<DescribeGlobalAccelerationInstancesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeGlobalAccelerationInstances",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定后端服务器绑定的全球加速实例。
    pub async fn describe_server_related_global_acceleration_instances(
        &self,
        request: DescribeServerRelatedGlobalAccelerationInstancesRequest,
    ) -> Result<DescribeServerRelatedGlobalAccelerationInstancesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeServerRelatedGlobalAccelerationInstances",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用RemoveGlobalAccelerationInstanceIp接口从带宽共享实例中移除EIP。
    pub async fn remove_global_acceleration_instance_ip(
        &self,
        request: RemoveGlobalAccelerationInstanceIpRequest,
    ) -> Result<RemoveGlobalAccelerationInstanceIpResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RemoveGlobalAccelerationInstanceIp",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateVpcGatewayEndpoint接口创建网关终端节点。
    pub async fn create_vpc_gateway_endpoint(
        &self,
        request: CreateVpcGatewayEndpointRequest,
    ) -> Result<CreateVpcGatewayEndpointResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateVpcGatewayEndpoint",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteVpcGatewayEndpoint接口删除网关终端节点。
    pub async fn delete_vpc_gateway_endpoint(
        &self,
        request: DeleteVpcGatewayEndpointRequest,
    ) -> Result<DeleteVpcGatewayEndpointResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteVpcGatewayEndpoint",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用UpdateVpcGatewayEndpointAttribute接口修改网关终端节点的配置信息。
    pub async fn update_vpc_gateway_endpoint_attribute(
        &self,
        request: UpdateVpcGatewayEndpointAttributeRequest,
    ) -> Result<UpdateVpcGatewayEndpointAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateVpcGatewayEndpointAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ListVpcEndpointServicesByEndUser查询可使用的终端节点服务。
    pub async fn list_vpc_endpoint_services_by_end_user(
        &self,
        request: ListVpcEndpointServicesByEndUserRequest,
    ) -> Result<ListVpcEndpointServicesByEndUserResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListVpcEndpointServicesByEndUser",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用GetVpcGatewayEndpointAttribute接口查询网关终端节点的属性。
    pub async fn get_vpc_gateway_endpoint_attribute(
        &self,
        request: GetVpcGatewayEndpointAttributeRequest,
    ) -> Result<GetVpcGatewayEndpointAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetVpcGatewayEndpointAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ListVpcGatewayEndpoints接口查询已创建的网关终端节点的列表。
    pub async fn list_vpc_gateway_endpoints(
        &self,
        request: ListVpcGatewayEndpointsRequest,
    ) -> Result<ListVpcGatewayEndpointsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListVpcGatewayEndpoints",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用AssociateRouteTablesWithVpcGatewayEndpoint接口关联路由表与网关终端节点。
    pub async fn associate_route_tables_with_vpc_gateway_endpoint(
        &self,
        request: AssociateRouteTablesWithVpcGatewayEndpointRequest,
    ) -> Result<AssociateRouteTablesWithVpcGatewayEndpointResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AssociateRouteTablesWithVpcGatewayEndpoint",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DissociateRouteTablesFromVpcGatewayEndpoint接口将网关终端节点与路由表解绑。
    pub async fn dissociate_route_tables_from_vpc_gateway_endpoint(
        &self,
        request: DissociateRouteTablesFromVpcGatewayEndpointRequest,
    ) -> Result<DissociateRouteTablesFromVpcGatewayEndpointResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DissociateRouteTablesFromVpcGatewayEndpoint",
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

    /// 调用ChangeResourceGroup接口修改物理专线资源所属的资源组。
    pub async fn change_resource_group(
        &self,
        request: ChangeResourceGroupRequest,
    ) -> Result<ChangeResourceGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ChangeResourceGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定网络实例的专线网关跨账号授权信息。
    pub async fn describe_vpc_grant_rules_to_ecr(
        &self,
        request: DescribeVpcGrantRulesToEcrRequest,
    ) -> Result<DescribeVpcGrantRulesToEcrResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeVpcGrantRulesToEcr",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

}