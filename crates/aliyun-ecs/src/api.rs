//! API 调用 - 自动生成，请勿手动修改

use aliyun_sdk_core::{Client, ClientConfig, ApiRequest, HttpMethod, SdkError};
use super::types::*;

/// Ecs API 版本
pub const API_VERSION: &str = "2014-05-26";

/// Ecs 客户端
#[derive(Debug, Clone)]
pub struct EcsClient {
    client: Client,
}

impl EcsClient {
    /// 创建新客户端
    pub fn new(config: ClientConfig) -> Result<Self, SdkError> {
        Ok(Self {
            client: Client::new(config)?,
        })
    }

    /// 根据计费方式、资源类型等参数查询地域信息列表。
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

    /// 根据地域ID、计费方式等参数查询可用区信息列表。
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

    /// 查询指定可用区的资源库存状态。该接口主要用于在创建实例（RunInstances）或修改实例规格（ModifyInstanceSpec）前，确认目标资源（如实例规格、系统盘类型）在特定可用区是否...
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

    /// 查询您在一个阿里云地域下能创建的ECS资源配额。包括您能创建的安全组数量、弹性网卡数量、按量付费vCPU核数、抢占式实例vCPU核数、按量付费云盘总容量配额、专用宿主机数量、网络类型以及账号是否...
    pub async fn describe_account_attributes(
        &self,
        request: DescribeAccountAttributesRequest,
    ) -> Result<DescribeAccountAttributesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeAccountAttributes",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 变更实例规格或系统盘类型之前，查询某一可用区下实例规格或系统盘的库存情况。
    pub async fn describe_resources_modification(
        &self,
        request: DescribeResourcesModificationRequest,
    ) -> Result<DescribeResourcesModificationResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeResourcesModification",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// （Beta）调用DescribeRecommendInstanceType根据一个指定的实例规格，查找一个或多个备选的实例规格。针对已经或者即将停售的实例规格，DescribeRecommend...
    pub async fn describe_recommend_instance_type(
        &self,
        request: DescribeRecommendInstanceTypeRequest,
    ) -> Result<DescribeRecommendInstanceTypeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeRecommendInstanceType",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于查询指定资源的最新价格，支持根据资源类型查询ECS实例、云盘、专有宿主机、弹性保障服务、容量预定服务的最新价格，包括活动规则、价格、折扣等信息。
    pub async fn describe_price(
        &self,
        request: DescribePriceRequest,
    ) -> Result<DescribePriceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribePrice",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询云服务器ECS资源的续费价格。仅支持查询包年包月资源的续费价格。
    pub async fn describe_renewal_price(
        &self,
        request: DescribeRenewalPriceRequest,
    ) -> Result<DescribeRenewalPriceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeRenewalPrice",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询未到期的包年包月ECS实例升配时目标实例规格的价格信息、新增包年包月数据盘的价格信息。
    pub async fn describe_instance_modification_price(
        &self,
        request: DescribeInstanceModificationPriceRequest,
    ) -> Result<DescribeInstanceModificationPriceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeInstanceModificationPrice",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于批量创建ECS实例，支持自动启动、分配公网IP及设置自动释放时间。
    pub async fn run_instances(
        &self,
        request: RunInstancesRequest,
    ) -> Result<RunInstancesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RunInstances",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口是阿里云 ECS 中用于创建实例的接口，支持创建一台包年包月或按量付费的 ECS 实例。您可通过此接口，依据自身需求灵活配置各类参数，实现实例的个性化创建。
    pub async fn create_instance(
        &self,
        request: CreateInstanceRequest,
    ) -> Result<CreateInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于启动一台ECS实例，您可以通过指定实例ID以及按需设置 InitLocalDisk 等参数启动实例。
    pub async fn start_instance(
        &self,
        request: StartInstanceRequest,
    ) -> Result<StartInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "StartInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于批量启动 ECS 实例，支持通过设置参数来选择不同的批量操作模式，为您提供灵活的启动方式。
    pub async fn start_instances(
        &self,
        request: StartInstancesRequest,
    ) -> Result<StartInstancesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "StartInstances",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于停止一台ECS实例，支持选择不同的停机方式、停机模式以及批量操作模式停止实例。
    pub async fn stop_instance(
        &self,
        request: StopInstanceRequest,
    ) -> Result<StopInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "StopInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于停止一台或多台 ECS 实例，支持选择不同的停机方式、停机模式以及批量操作模式停止实例。
    pub async fn stop_instances(
        &self,
        request: StopInstancesRequest,
    ) -> Result<StopInstancesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "StopInstances",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于重启一台ECS实例。
    pub async fn reboot_instance(
        &self,
        request: RebootInstanceRequest,
    ) -> Result<RebootInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RebootInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于重启一台或多台ECS实例，支持您通过参数来选择是否强制重启和设置不同的批量操作模式。
    pub async fn reboot_instances(
        &self,
        request: RebootInstancesRequest,
    ) -> Result<RebootInstancesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RebootInstances",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除或者释放一台指定的ECS实例。
    pub async fn delete_instance(
        &self,
        request: DeleteInstanceRequest,
    ) -> Result<DeleteInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于批量删除或者释放按量付费实例或者到期的包年包月实例，支持通过参数设置决定云盘是否释放或转换为按量付费保留。
    pub async fn delete_instances(
        &self,
        request: DeleteInstancesRequest,
    ) -> Result<DeleteInstancesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteInstances",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口主要用于查询一台或多台指定ECS实例的状态信息，同时支持查询指定条件下的实例列表。
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

    /// 本接口支持根据不同请求条件查询实例列表，并关联查询实例的详细信息。
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

    /// 本接口用于查询阿里云 ECS提供的实例规格族列表。通过该接口获取不同系列的实例规格族信息，从而更好地帮助您了解可用的实例规格资源，为您选择合适的实例规格创建 ECS 实例提供参考。
    pub async fn describe_instance_type_families(
        &self,
        request: DescribeInstanceTypeFamiliesRequest,
    ) -> Result<DescribeInstanceTypeFamiliesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeInstanceTypeFamilies",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口主要用于查询云服务器ECS提供的所有实例规格的信息列表并关联获取实例规格的详细信息，也可根据特定条件查询指定实例规格的信息，帮助您了解不同实例规格的配置和性能，以便选择适合自己业务需求的实例。
    pub async fn describe_instance_types(
        &self,
        request: DescribeInstanceTypesRequest,
    ) -> Result<DescribeInstanceTypesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeInstanceTypes",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于查询一台指定ECS实例的属性信息。
    pub async fn describe_instance_attribute(
        &self,
        request: DescribeInstanceAttributeRequest,
    ) -> Result<DescribeInstanceAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeInstanceAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于修改一台ECS实例的部分属性信息，支持修改密码、实例名称、主机名、所属安全组、实例的MTU以及用户自定义数据等。
    pub async fn modify_instance_attribute(
        &self,
        request: ModifyInstanceAttributeRequest,
    ) -> Result<ModifyInstanceAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyInstanceAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 按需修改实例时钟选项。
    pub async fn modify_instance_clock_options(
        &self,
        request: ModifyInstanceClockOptionsRequest,
    ) -> Result<ModifyInstanceClockOptionsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyInstanceClockOptions",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改实例网络选项。
    pub async fn modify_instance_network_options(
        &self,
        request: ModifyInstanceNetworkOptionsRequest,
    ) -> Result<ModifyInstanceNetworkOptionsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyInstanceNetworkOptions",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于修改ECS实例的计费方式，支持批量修改。
    pub async fn modify_instance_charge_type(
        &self,
        request: ModifyInstanceChargeTypeRequest,
    ) -> Result<ModifyInstanceChargeTypeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyInstanceChargeType",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于修改一台按量付费ECS实例的实例规格或者公网带宽大小。
    pub async fn modify_instance_spec(
        &self,
        request: ModifyInstanceSpecRequest,
    ) -> Result<ModifyInstanceSpecResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyInstanceSpec",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于修改一台包年包月ECS实例的实例规格，支持您升级或者降低实例规格，修改后的实例规格在实例整个生命周期内生效。
    pub async fn modify_prepay_instance_spec(
        &self,
        request: ModifyPrepayInstanceSpecRequest,
    ) -> Result<ModifyPrepayInstanceSpecResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyPrepayInstanceSpec",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于修改指定按量付费实例或者抢占式实例的自动释放时间，同时支持取消自动释放功能。
    pub async fn modify_instance_auto_release_time(
        &self,
        request: ModifyInstanceAutoReleaseTimeRequest,
    ) -> Result<ModifyInstanceAutoReleaseTimeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyInstanceAutoReleaseTime",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于为一台或多台ECS实例授予RAM角色。
    pub async fn attach_instance_ram_role(
        &self,
        request: AttachInstanceRamRoleRequest,
    ) -> Result<AttachInstanceRamRoleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AttachInstanceRamRole",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口具备两个主要功能，支持依据实例 ID 来查询一台或多台 ECS 实例所被授予的实RAM角色，同时也允许通过实例 RAM 角色名称来查询被授予了该特定角色的实例情况。
    pub async fn describe_instance_ram_role(
        &self,
        request: DescribeInstanceRamRoleRequest,
    ) -> Result<DescribeInstanceRamRoleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeInstanceRamRole",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于收回一台或多台ECS实例被授予的RAM角色。
    pub async fn detach_instance_ram_role(
        &self,
        request: DetachInstanceRamRoleRequest,
    ) -> Result<DetachInstanceRamRoleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DetachInstanceRamRole",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于查询一台ECS实例的VNC登录地址。
    pub async fn describe_instance_vnc_url(
        &self,
        request: DescribeInstanceVncUrlRequest,
    ) -> Result<DescribeInstanceVncUrlResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeInstanceVncUrl",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于修改一台ECS实例的VNC登录密码。
    pub async fn modify_instance_vnc_passwd(
        &self,
        request: ModifyInstanceVncPasswdRequest,
    ) -> Result<ModifyInstanceVncPasswdResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyInstanceVncPasswd",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于修改指定地域下实例内的元数据访问设置，包括是否启用元数据访问通道和访问元数据时是否启用强制加固模式等设置。
    pub async fn modify_instance_metadata_options(
        &self,
        request: ModifyInstanceMetadataOptionsRequest,
    ) -> Result<ModifyInstanceMetadataOptionsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyInstanceMetadataOptions",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于查询一台ECS实例的自定义数据。
    pub async fn describe_user_data(
        &self,
        request: DescribeUserDataRequest,
    ) -> Result<DescribeUserDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeUserData",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于续费一台包年包月的 ECS 实例，支持您设置续费时长或者续费至统一到期日。
    pub async fn renew_instance(
        &self,
        request: RenewInstanceRequest,
    ) -> Result<RenewInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RenewInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于查询一台或多台包年包月ECS实例的自动续费属性，包括是否开启自动续费、续费周期等信息。
    pub async fn describe_instance_auto_renew_attribute(
        &self,
        request: DescribeInstanceAutoRenewAttributeRequest,
    ) -> Result<DescribeInstanceAutoRenewAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeInstanceAutoRenewAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于修改一台或多台包年包月实例的自动续费属性，可以帮助您减少资源到期的维护成本。
    pub async fn modify_instance_auto_renew_attribute(
        &self,
        request: ModifyInstanceAutoRenewAttributeRequest,
    ) -> Result<ModifyInstanceAutoRenewAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyInstanceAutoRenewAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于重新启动一台已过期或欠费回收的按量付费ECS实例。
    pub async fn re_activate_instances(
        &self,
        request: ReActivateInstancesRequest,
    ) -> Result<ReActivateInstancesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ReActivateInstances",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于查询抢占式实例的历史价格，最多支持获取近30天内的数据，通过历史价格数据可以帮助您合理的设置抢占式实例的单台实例上限价格。
    pub async fn describe_spot_price_history(
        &self,
        request: DescribeSpotPriceHistoryRequest,
    ) -> Result<DescribeSpotPriceHistoryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeSpotPriceHistory",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于查询指定地域下，抢占式实例近30天的平均释放率、平均折扣率等信息。
    pub async fn describe_spot_advice(
        &self,
        request: DescribeSpotAdviceRequest,
    ) -> Result<DescribeSpotAdviceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeSpotAdvice",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建一份自定义镜像。后续您可以使用创建的自定义镜像创建ECS实例（RunInstances），或者更换实例的系统盘（ReplaceSystemDisk）。
    pub async fn create_image(
        &self,
        request: CreateImageRequest,
    ) -> Result<CreateImageResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateImage",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 指定ImageId、镜像被使用场景、Filter过滤等参数，查询您可以使用的镜像资源列表。
    pub async fn describe_images(
        &self,
        request: DescribeImagesRequest,
    ) -> Result<DescribeImagesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeImages",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyImageAttribute接口，并可以指定ImageId、ImageFamily等参数，修改一份自定义镜像的属性，例如镜像族系、名称、启动模式、状态以及是否支持NVMe等。
    pub async fn modify_image_attribute(
        &self,
        request: ModifyImageAttributeRequest,
    ) -> Result<ModifyImageAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyImageAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteImage接口，并可以指定参数ImageId、Force删除一份自定义镜像。
    pub async fn delete_image(
        &self,
        request: DeleteImageRequest,
    ) -> Result<DeleteImageResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteImage",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定镜像族系内最新创建的可用自定义镜像。
    pub async fn describe_image_from_family(
        &self,
        request: DescribeImageFromFamilyRequest,
    ) -> Result<DescribeImageFromFamilyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeImageFromFamily",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定镜像支持的实例规格。
    pub async fn describe_image_support_instance_types(
        &self,
        request: DescribeImageSupportInstanceTypesRequest,
    ) -> Result<DescribeImageSupportInstanceTypesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeImageSupportInstanceTypes",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询一份自定义镜像已经共享的所有用户。持返回结果支分页显示，每页的信息条目默认为10条。
    pub async fn describe_image_share_permission(
        &self,
        request: DescribeImageSharePermissionRequest,
    ) -> Result<DescribeImageSharePermissionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeImageSharePermission",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 管理镜像共享权限。您可以将自己的自定义镜像共享给其他阿里云账号，也可以发布为社区镜像供他人使用。
    pub async fn modify_image_share_permission(
        &self,
        request: ModifyImageSharePermissionRequest,
    ) -> Result<ModifyImageSharePermissionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyImageSharePermission",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改镜像共享组权限
    pub async fn modify_image_share_group_permission(
        &self,
        request: ModifyImageShareGroupPermissionRequest,
    ) -> Result<ModifyImageShareGroupPermissionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyImageShareGroupPermission",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 导入一份您的本地镜像文件到云服务器ECS，作为自定义镜像出现在相应地域中。后续您可以使用导入的镜像创建ECS实例（RunInstances），或者更换实例的系统盘（ReplaceSystemDi...
    pub async fn import_image(
        &self,
        request: ImportImageRequest,
    ) -> Result<ImportImageResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ImportImage",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 导出一份自定义镜像到与自定义镜像同一地域的OSS Bucket里。
    pub async fn export_image(
        &self,
        request: ExportImageRequest,
    ) -> Result<ExportImageResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ExportImage",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 复制一个地域下的自定义镜像到其他地域。复制镜像可以实现跨地域部署ECS实例、跨地域复制ECS实例等目的。
    pub async fn copy_image(
        &self,
        request: CopyImageRequest,
    ) -> Result<CopyImageResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CopyImage",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 取消正在进行中的复制镜像（CopyImage）任务。
    pub async fn cancel_copy_image(
        &self,
        request: CancelCopyImageRequest,
    ) -> Result<CancelCopyImageResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CancelCopyImage",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建一个镜像组件。镜像组件用于存储您在构建镜像时，常用的构建模板命令。
    pub async fn create_image_component(
        &self,
        request: CreateImageComponentRequest,
    ) -> Result<CreateImageComponentResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateImageComponent",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询一个或多个镜像组件的详细信息。
    pub async fn describe_image_components(
        &self,
        request: DescribeImageComponentsRequest,
    ) -> Result<DescribeImageComponentsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeImageComponents",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteImageComponent接口，并指定参数RegionId、ImageComponentId，删除一个镜像组件。
    pub async fn delete_image_component(
        &self,
        request: DeleteImageComponentRequest,
    ) -> Result<DeleteImageComponentResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteImageComponent",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建一个镜像模板。镜像模板可用于构建镜像。
    pub async fn create_image_pipeline(
        &self,
        request: CreateImagePipelineRequest,
    ) -> Result<CreateImagePipelineResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateImagePipeline",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询一个或多个镜像模板的详细信息。
    pub async fn describe_image_pipelines(
        &self,
        request: DescribeImagePipelinesRequest,
    ) -> Result<DescribeImagePipelinesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeImagePipelines",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除一个镜像模板。
    pub async fn delete_image_pipeline(
        &self,
        request: DeleteImagePipelineRequest,
    ) -> Result<DeleteImagePipelineResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteImagePipeline",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 通过一个镜像模板执行构建镜像的任务。
    pub async fn start_image_pipeline_execution(
        &self,
        request: StartImagePipelineExecutionRequest,
    ) -> Result<StartImagePipelineExecutionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "StartImagePipelineExecution",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeImagePipelineExecutions查询一个镜像构建任务的详细信息。
    pub async fn describe_image_pipeline_executions(
        &self,
        request: DescribeImagePipelineExecutionsRequest,
    ) -> Result<DescribeImagePipelineExecutionsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeImagePipelineExecutions",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 取消一个镜像构建任务。
    pub async fn cancel_image_pipeline_execution(
        &self,
        request: CancelImagePipelineExecutionRequest,
    ) -> Result<CancelImagePipelineExecutionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CancelImagePipelineExecution",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建一块按量付费或包年包月的数据盘。
    pub async fn create_disk(
        &self,
        request: CreateDiskRequest,
    ) -> Result<CreateDiskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateDisk",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询一块或多块已创建的块存储（包括云盘、本地盘以及弹性临时盘）信息。
    pub async fn describe_disks(
        &self,
        request: DescribeDisksRequest,
    ) -> Result<DescribeDisksResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDisks",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为一台ECS实例挂载一块按数据盘或系统盘。
    pub async fn attach_disk(
        &self,
        request: AttachDiskRequest,
    ) -> Result<AttachDiskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AttachDisk",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 从一台ECS实例上卸载一块按量付费的数据盘或者系统盘。
    pub async fn detach_disk(
        &self,
        request: DetachDiskRequest,
    ) -> Result<DetachDiskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DetachDisk",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 扩容一块磁盘，支持扩容系统盘和数据盘。
    pub async fn resize_disk(
        &self,
        request: ResizeDiskRequest,
    ) -> Result<ResizeDiskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ResizeDisk",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改一个或多个块存储设备的名称、描述、是否随实例释放、是否随磁盘删除其自动快照、是否启用自动快照策略、是否开启性能突发功能等。
    pub async fn modify_disk_attribute(
        &self,
        request: ModifyDiskAttributeRequest,
    ) -> Result<ModifyDiskAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDiskAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 将包年包月ECS实例上挂载的数据盘的计费方式在按量付费和包年包月之间进行转换。
    pub async fn modify_disk_charge_type(
        &self,
        request: ModifyDiskChargeTypeRequest,
    ) -> Result<ModifyDiskChargeTypeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDiskChargeType",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 变更一块云盘类型，或者修改一块ESSD云盘的性能级别。ESSD同城冗余云盘、普通云盘、弹性临时盘和本地盘均不支持变更云盘类型。
    pub async fn modify_disk_spec(
        &self,
        request: ModifyDiskSpecRequest,
    ) -> Result<ModifyDiskSpecResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDiskSpec",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 通过更换ECS实例系统盘的方式更换操作系统。更换后系统盘的云盘ID将发生变化，原云盘会被释放。
    pub async fn replace_system_disk(
        &self,
        request: ReplaceSystemDiskRequest,
    ) -> Result<ReplaceSystemDiskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ReplaceSystemDisk",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 使用云盘快照，将云盘回滚到之前某一特定的历史状态。
    pub async fn reset_disk(
        &self,
        request: ResetDiskRequest,
    ) -> Result<ResetDiskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ResetDisk",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 通过实例快照回滚一个或多个云盘。
    pub async fn reset_disks(
        &self,
        request: ResetDisksRequest,
    ) -> Result<ResetDisksResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ResetDisks",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 重新初始化一块磁盘至创建时的初始状态。
    pub async fn re_init_disk(
        &self,
        request: ReInitDiskRequest,
    ) -> Result<ReInitDiskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ReInitDisk",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 将云盘迁入、迁出专属块存储集群，或将云盘在不同专属块存储集群之间迁移。
    pub async fn modify_disk_deployment(
        &self,
        request: ModifyDiskDeploymentRequest,
    ) -> Result<ModifyDiskDeploymentResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDiskDeployment",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 释放一块按量付费数据盘。磁盘类型包括普通云盘、高效云盘、SSD云盘和ESSD云盘。
    pub async fn delete_disk(
        &self,
        request: DeleteDiskRequest,
    ) -> Result<DeleteDiskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteDisk",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 开启指定地域块存储账号级默认加密。
    pub async fn enable_disk_encryption_by_default(
        &self,
        request: EnableDiskEncryptionByDefaultRequest,
    ) -> Result<EnableDiskEncryptionByDefaultResponse, SdkError> {
        let api_request = ApiRequest {
            action: "EnableDiskEncryptionByDefault",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定地域块存储账号级默认加密的服务状态。
    pub async fn describe_disk_encryption_by_default_status(
        &self,
        request: DescribeDiskEncryptionByDefaultStatusRequest,
    ) -> Result<DescribeDiskEncryptionByDefaultStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDiskEncryptionByDefaultStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询块存储账号级默认加密使用的密钥。
    pub async fn describe_disk_default_kms_key_id(
        &self,
        request: DescribeDiskDefaultKMSKeyIdRequest,
    ) -> Result<DescribeDiskDefaultKMSKeyIdResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDiskDefaultKMSKeyId",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改指定地域块存储账号级默认加密使用的KMS密钥ID。
    pub async fn modify_disk_default_kms_key_id(
        &self,
        request: ModifyDiskDefaultKMSKeyIdRequest,
    ) -> Result<ModifyDiskDefaultKMSKeyIdResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDiskDefaultKMSKeyId",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 将指定地域块存储账号级默认加密使用的 KMS 密钥 ID 重置为服务密钥的接口。
    pub async fn reset_disk_default_kms_key_id(
        &self,
        request: ResetDiskDefaultKMSKeyIdRequest,
    ) -> Result<ResetDiskDefaultKMSKeyIdResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ResetDiskDefaultKMSKeyId",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 关闭指定地域块存储账号级默认加密。
    pub async fn disable_disk_encryption_by_default(
        &self,
        request: DisableDiskEncryptionByDefaultRequest,
    ) -> Result<DisableDiskEncryptionByDefaultResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DisableDiskEncryptionByDefault",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 开通快照服务。
    pub async fn open_snapshot_service(
        &self,
        request: OpenSnapshotServiceRequest,
    ) -> Result<OpenSnapshotServiceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "OpenSnapshotService",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为一块云盘创建一份快照。
    pub async fn create_snapshot(
        &self,
        request: CreateSnapshotRequest,
    ) -> Result<CreateSnapshotResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateSnapshot",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询云盘的快照列表信息。例如快照状态、正在创建的快照剩余完成时间、自动快照保留天数等。
    pub async fn describe_snapshots(
        &self,
        request: DescribeSnapshotsRequest,
    ) -> Result<DescribeSnapshotsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeSnapshots",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询您在一个地域下的快照数量以及快照容量。
    pub async fn describe_snapshots_usage(
        &self,
        request: DescribeSnapshotsUsageRequest,
    ) -> Result<DescribeSnapshotsUsageResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeSnapshotsUsage",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改一份快照的名称、描述或保留天数。
    pub async fn modify_snapshot_attribute(
        &self,
        request: ModifySnapshotAttributeRequest,
    ) -> Result<ModifySnapshotAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifySnapshotAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改快照类型,可以通过该接口将标准快照转换为归档快照。
    pub async fn modify_snapshot_category(
        &self,
        request: ModifySnapshotCategoryRequest,
    ) -> Result<ModifySnapshotCategoryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifySnapshotCategory",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 将一份标准快照或加密快照从一个地域复制到另一个地域。
    pub async fn copy_snapshot(
        &self,
        request: CopySnapshotRequest,
    ) -> Result<CopySnapshotResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CopySnapshot",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除一份指定的快照。如果需要取消正在创建的快照，也可以调用该接口删除快照，即取消创建快照任务。
    pub async fn delete_snapshot(
        &self,
        request: DeleteSnapshotRequest,
    ) -> Result<DeleteSnapshotResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteSnapshot",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询云盘快照链。快照链是一块云盘所有快照组成的关系链，一块云盘对应一条快照链。
    pub async fn describe_snapshot_links(
        &self,
        request: DescribeSnapshotLinksRequest,
    ) -> Result<DescribeSnapshotLinksResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeSnapshotLinks",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为指定ECS实例中的云盘创建快照一致性组。快照一致性组包含一个或多个云盘对应的快照。
    pub async fn create_snapshot_group(
        &self,
        request: CreateSnapshotGroupRequest,
    ) -> Result<CreateSnapshotGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateSnapshotGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询一个或多个快照一致性组的信息。例如状态、所属的实例ID、快照创建进度等。
    pub async fn describe_snapshot_groups(
        &self,
        request: DescribeSnapshotGroupsRequest,
    ) -> Result<DescribeSnapshotGroupsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeSnapshotGroups",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改快照一致性组的名称与描述信息。
    pub async fn modify_snapshot_group(
        &self,
        request: ModifySnapshotGroupRequest,
    ) -> Result<ModifySnapshotGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifySnapshotGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除快照一致性组。
    pub async fn delete_snapshot_group(
        &self,
        request: DeleteSnapshotGroupRequest,
    ) -> Result<DeleteSnapshotGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteSnapshotGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 在指定地域下创建一条自动快照策略。可以指定自动快照的重复周期、保留时间以及跨地域备份等策略，为系统盘或数据盘创建快照备份数据。
    pub async fn create_auto_snapshot_policy(
        &self,
        request: CreateAutoSnapshotPolicyRequest,
    ) -> Result<CreateAutoSnapshotPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateAutoSnapshotPolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于查询指定地域下已创建的自动快照策略详细信息列表。
    pub async fn describe_auto_snapshot_policy_ex(
        &self,
        request: DescribeAutoSnapshotPolicyExRequest,
    ) -> Result<DescribeAutoSnapshotPolicyExResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeAutoSnapshotPolicyEx",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询自动快照策略的关联关系
    pub async fn describe_auto_snapshot_policy_associations(
        &self,
        request: DescribeAutoSnapshotPolicyAssociationsRequest,
    ) -> Result<DescribeAutoSnapshotPolicyAssociationsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeAutoSnapshotPolicyAssociations",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改一条自动快照策略，例如，快照创建时间点、重复日期、保留时间等。
    pub async fn modify_auto_snapshot_policy_ex(
        &self,
        request: ModifyAutoSnapshotPolicyExRequest,
    ) -> Result<ModifyAutoSnapshotPolicyExResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyAutoSnapshotPolicyEx",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为一块或者多块云盘应用自动快照策略。目标云盘已经应用了自动快照策略时，调用ApplyAutoSnapshotPolicy可以更换云盘当前应用的自动快照策略。
    pub async fn apply_auto_snapshot_policy(
        &self,
        request: ApplyAutoSnapshotPolicyRequest,
    ) -> Result<ApplyAutoSnapshotPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ApplyAutoSnapshotPolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 取消一块或者多块云盘的自动快照策略。
    pub async fn cancel_auto_snapshot_policy(
        &self,
        request: CancelAutoSnapshotPolicyRequest,
    ) -> Result<CancelAutoSnapshotPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CancelAutoSnapshotPolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除一条自动快照策略。如果目标自动快照策略已经被应用到磁盘上，删除自动快照策略后，这些磁盘不再执行该策略。
    pub async fn delete_auto_snapshot_policy(
        &self,
        request: DeleteAutoSnapshotPolicyRequest,
    ) -> Result<DeleteAutoSnapshotPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteAutoSnapshotPolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeSnapshotPackage查询您在一个阿里云地域下已经购买的OSS存储包。存储包可以用于抵扣标准快照存储容量，但不支持抵扣本地快照。
    pub async fn describe_snapshot_package(
        &self,
        request: DescribeSnapshotPackageRequest,
    ) -> Result<DescribeSnapshotPackageResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeSnapshotPackage",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeBandwidthLimitation查询不同实例规格可以购买、升级或降配的公网带宽上限。
    pub async fn describe_bandwidth_limitation(
        &self,
        request: DescribeBandwidthLimitationRequest,
    ) -> Result<DescribeBandwidthLimitationResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeBandwidthLimitation",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改ECS实例的带宽配置、分配公网IP、转换网络计费方式，支持升配和降配。
    pub async fn modify_instance_network_spec(
        &self,
        request: ModifyInstanceNetworkSpecRequest,
    ) -> Result<ModifyInstanceNetworkSpecResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyInstanceNetworkSpec",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为一台ECS实例分配一个公网IP地址。推荐您使用接口ModifyInstanceNetworkSpec分配公网IP。
    pub async fn allocate_public_ip_address(
        &self,
        request: AllocatePublicIpAddressRequest,
    ) -> Result<AllocatePublicIpAddressResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AllocatePublicIpAddress",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 将一台专有网络VPC类型ECS实例的公网IP地址（PublicIP）转化为弹性公网IP（EIP）。
    pub async fn convert_nat_public_ip_to_eip(
        &self,
        request: ConvertNatPublicIpToEipRequest,
    ) -> Result<ConvertNatPublicIpToEipResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ConvertNatPublicIpToEip",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改一台专有网络类型ECS实例的专有网络VPC、私网IP地址、安全组或交换机。
    pub async fn modify_instance_vpc_attribute(
        &self,
        request: ModifyInstanceVpcAttributeRequest,
    ) -> Result<ModifyInstanceVpcAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyInstanceVpcAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询一台或多台与专有网络VPC建立了连接的经典网络类型实例。
    pub async fn describe_classic_link_instances(
        &self,
        request: DescribeClassicLinkInstancesRequest,
    ) -> Result<DescribeClassicLinkInstancesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeClassicLinkInstances",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用AttachClassicLinkVpc将一台经典网络类型实例连接到专有网络VPC中，使经典网络类型实例可以和VPC中的云资源私网互通。
    pub async fn attach_classic_link_vpc(
        &self,
        request: AttachClassicLinkVpcRequest,
    ) -> Result<AttachClassicLinkVpcResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AttachClassicLinkVpc",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DetachClassicLinkVpc取消一台经典网络类型ECS实例与专有网络VPC的连接（ClassicLink）。取消ClassicLink后，经典网络类型实例无法与VPC内的实例互通。
    pub async fn detach_classic_link_vpc(
        &self,
        request: DetachClassicLinkVpcRequest,
    ) -> Result<DetachClassicLinkVpcResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DetachClassicLinkVpc",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建一张弹性网卡（ENI），支持指定主私网IP地址。
    pub async fn create_network_interface(
        &self,
        request: CreateNetworkInterfaceRequest,
    ) -> Result<CreateNetworkInterfaceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateNetworkInterface",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询一个或多个弹性网卡（ENI）的详细信息。
    pub async fn describe_network_interfaces(
        &self,
        request: DescribeNetworkInterfacesRequest,
    ) -> Result<DescribeNetworkInterfacesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeNetworkInterfaces",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 指定弹性网卡ID，查询一个网卡的详细信息。
    pub async fn describe_network_interface_attribute(
        &self,
        request: DescribeNetworkInterfaceAttributeRequest,
    ) -> Result<DescribeNetworkInterfaceAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeNetworkInterfaceAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改一个弹性网卡（ENI）的网卡队列数、所属安全组、队列深度、通讯模式以及释放实例时是否保留网卡等属性。
    pub async fn modify_network_interface_attribute(
        &self,
        request: ModifyNetworkInterfaceAttributeRequest,
    ) -> Result<ModifyNetworkInterfaceAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyNetworkInterfaceAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 异步删除一张弹性网卡（ENI）。
    pub async fn delete_network_interface(
        &self,
        request: DeleteNetworkInterfaceRequest,
    ) -> Result<DeleteNetworkInterfaceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteNetworkInterface",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用AttachNetworkInterface接口，并可以指定NetworkInterfaceId、InstanceId、NetworkCardIndex等参数，附加一个弹性网卡（ENI）到一...
    pub async fn attach_network_interface(
        &self,
        request: AttachNetworkInterfaceRequest,
    ) -> Result<AttachNetworkInterfaceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AttachNetworkInterface",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 从一台ECS实例上分离一张弹性网卡（ENI）。
    pub async fn detach_network_interface(
        &self,
        request: DetachNetworkInterfaceRequest,
    ) -> Result<DetachNetworkInterfaceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DetachNetworkInterface",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为一块弹性网卡分配一个或多个辅助私网IP地址。可以为网卡指定在所属交换机（vSwitch）的空闲私网IP地址，或者通过指定私网地址数量自动分配私网IP地址。
    pub async fn assign_private_ip_addresses(
        &self,
        request: AssignPrivateIpAddressesRequest,
    ) -> Result<AssignPrivateIpAddressesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AssignPrivateIpAddresses",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 从一张弹性网卡删除一个或多个辅助私有IP地址。
    pub async fn unassign_private_ip_addresses(
        &self,
        request: UnassignPrivateIpAddressesRequest,
    ) -> Result<UnassignPrivateIpAddressesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UnassignPrivateIpAddresses",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为弹性网卡分配一个或多个IPv6地址。
    pub async fn assign_ipv6_addresses(
        &self,
        request: AssignIpv6AddressesRequest,
    ) -> Result<AssignIpv6AddressesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AssignIpv6Addresses",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 回收一个或多个弹性网卡IPv6地址。
    pub async fn unassign_ipv6_addresses(
        &self,
        request: UnassignIpv6AddressesRequest,
    ) -> Result<UnassignIpv6AddressesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UnassignIpv6Addresses",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateNetworkInterfacePermission为阿里云合作伙伴（认证ISV）或者个人用户授权弹性网卡权限。
    pub async fn create_network_interface_permission(
        &self,
        request: CreateNetworkInterfacePermissionRequest,
    ) -> Result<CreateNetworkInterfacePermissionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateNetworkInterfacePermission",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeNetworkInterfacePermissions查询您授权给阿里云合作伙伴（认证ISV）或者个人用户的弹性网卡权限列表。
    pub async fn describe_network_interface_permissions(
        &self,
        request: DescribeNetworkInterfacePermissionsRequest,
    ) -> Result<DescribeNetworkInterfacePermissionsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeNetworkInterfacePermissions",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建一个前缀列表。
    pub async fn create_prefix_list(
        &self,
        request: CreatePrefixListRequest,
    ) -> Result<CreatePrefixListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreatePrefixList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribePrefixLists查询一个或多个前缀列表的信息。
    pub async fn describe_prefix_lists(
        &self,
        request: DescribePrefixListsRequest,
    ) -> Result<DescribePrefixListsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribePrefixLists",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定前缀列表的详细信息，包含前缀列表的名称、地址族、最大条目容量以及条目的详细信息等。
    pub async fn describe_prefix_list_attributes(
        &self,
        request: DescribePrefixListAttributesRequest,
    ) -> Result<DescribePrefixListAttributesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribePrefixListAttributes",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定前缀列表已关联的资源信息，例如，资源ID和资源类型。
    pub async fn describe_prefix_list_associations(
        &self,
        request: DescribePrefixListAssociationsRequest,
    ) -> Result<DescribePrefixListAssociationsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribePrefixListAssociations",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改指定前缀列表的名称、描述等，支持增加、修改和删除条目。
    pub async fn modify_prefix_list(
        &self,
        request: ModifyPrefixListRequest,
    ) -> Result<ModifyPrefixListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyPrefixList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeletePrefixList删除指定的前缀列表，同时删除前缀列表中的所有条目。
    pub async fn delete_prefix_list(
        &self,
        request: DeletePrefixListRequest,
    ) -> Result<DeletePrefixListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeletePrefixList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建端口列表，后续可关联资源（例如安全组）使用。
    pub async fn create_port_range_list(
        &self,
        request: CreatePortRangeListRequest,
    ) -> Result<CreatePortRangeListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreatePortRangeList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询端口列表。
    pub async fn describe_port_range_lists(
        &self,
        request: DescribePortRangeListsRequest,
    ) -> Result<DescribePortRangeListsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribePortRangeLists",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定端口列表的条目。
    pub async fn describe_port_range_list_entries(
        &self,
        request: DescribePortRangeListEntriesRequest,
    ) -> Result<DescribePortRangeListEntriesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribePortRangeListEntries",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定端口列表已关联的资源信息，例如，安全组。
    pub async fn describe_port_range_list_associations(
        &self,
        request: DescribePortRangeListAssociationsRequest,
    ) -> Result<DescribePortRangeListAssociationsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribePortRangeListAssociations",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改指定端口列表的名称、条目，支持增加、修改和删除条目。
    pub async fn modify_port_range_list(
        &self,
        request: ModifyPortRangeListRequest,
    ) -> Result<ModifyPortRangeListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyPortRangeList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除指定端口列表，同时端口列表下的端口列表条目都将被删除。
    pub async fn delete_port_range_list(
        &self,
        request: DeletePortRangeListRequest,
    ) -> Result<DeletePortRangeListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeletePortRangeList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于创建一个安全组。
    pub async fn create_security_group(
        &self,
        request: CreateSecurityGroupRequest,
    ) -> Result<CreateSecurityGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateSecurityGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于查询安全组基本信息列表，支持您通过地域、安全组ID、安全组类型等不同参数查询。
    pub async fn describe_security_groups(
        &self,
        request: DescribeSecurityGroupsRequest,
    ) -> Result<DescribeSecurityGroupsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeSecurityGroups",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口主要用于查询一个指定安全组的详细信息，并关联查询安全组规则详细信息列表。
    pub async fn describe_security_group_attribute(
        &self,
        request: DescribeSecurityGroupAttributeRequest,
    ) -> Result<DescribeSecurityGroupAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeSecurityGroupAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于修改一个普通安全组的组内连通策略。
    pub async fn modify_security_group_policy(
        &self,
        request: ModifySecurityGroupPolicyRequest,
    ) -> Result<ModifySecurityGroupPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifySecurityGroupPolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于修改一个指定安全组的名称或者描述信息。
    pub async fn modify_security_group_attribute(
        &self,
        request: ModifySecurityGroupAttributeRequest,
    ) -> Result<ModifySecurityGroupAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifySecurityGroupAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于删除一个安全组，并关联删除组内所有安全组规则。
    pub async fn delete_security_group(
        &self,
        request: DeleteSecurityGroupRequest,
    ) -> Result<DeleteSecurityGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteSecurityGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口是阿里云 ECS 中用于增加一条或多条安全组入方向规则的接口。通过该接口，用户可以指定安全组入方向的访问权限，允许或者拒绝其他设备发送入方向流量到安全组内的实例，从而实现对网络访问的精细控制。
    pub async fn authorize_security_group(
        &self,
        request: AuthorizeSecurityGroupRequest,
    ) -> Result<AuthorizeSecurityGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AuthorizeSecurityGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于修改指定安全组中的一条入方向安全组规则。
    pub async fn modify_security_group_rule(
        &self,
        request: ModifySecurityGroupRuleRequest,
    ) -> Result<ModifySecurityGroupRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifySecurityGroupRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于删除指定安全组内的一条或多条入方向安全组规则。
    pub async fn revoke_security_group(
        &self,
        request: RevokeSecurityGroupRequest,
    ) -> Result<RevokeSecurityGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RevokeSecurityGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口是阿里云 ECS 中用于增加一条或多条安全组出方向规则的接口。通过该接口，用户可以指定安全组出方向的访问权限，允许或拒绝安全组内的实例发送出方向流量到其他设备，从而实现对网络访问的精细控制。
    pub async fn authorize_security_group_egress(
        &self,
        request: AuthorizeSecurityGroupEgressRequest,
    ) -> Result<AuthorizeSecurityGroupEgressResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AuthorizeSecurityGroupEgress",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于修改一条出方向安全组规则。
    pub async fn modify_security_group_egress_rule(
        &self,
        request: ModifySecurityGroupEgressRuleRequest,
    ) -> Result<ModifySecurityGroupEgressRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifySecurityGroupEgressRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于删除指定安全组内的一条或多条出方向安全组规则。
    pub async fn revoke_security_group_egress(
        &self,
        request: RevokeSecurityGroupEgressRequest,
    ) -> Result<RevokeSecurityGroupEgressResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RevokeSecurityGroupEgress",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于查询一个或多个指定安全组已经被授权的其他安全组列表信息。
    pub async fn describe_security_group_references(
        &self,
        request: DescribeSecurityGroupReferencesRequest,
    ) -> Result<DescribeSecurityGroupReferencesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeSecurityGroupReferences",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于将一台ECS实例或一张弹性网卡加入到指定的安全组。
    pub async fn join_security_group(
        &self,
        request: JoinSecurityGroupRequest,
    ) -> Result<JoinSecurityGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "JoinSecurityGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于将一台ECS实例或一张弹性网卡移出指定的安全组。
    pub async fn leave_security_group(
        &self,
        request: LeaveSecurityGroupRequest,
    ) -> Result<LeaveSecurityGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "LeaveSecurityGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateKeyPair创建一对SSH密钥对。系统会为您保管密钥的公钥部分，并返回未加密的PEM编码的PKCS#8格式私钥。您需要自行妥善保管私钥部分。
    pub async fn create_key_pair(
        &self,
        request: CreateKeyPairRequest,
    ) -> Result<CreateKeyPairResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateKeyPair",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 导入由其他工具产生的RSA密钥对的公钥部分。导入密钥对后，阿里云为您保管公钥部分，您需要自行妥善保存密钥对的私钥部分。
    pub async fn import_key_pair(
        &self,
        request: ImportKeyPairRequest,
    ) -> Result<ImportKeyPairResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ImportKeyPair",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeKeyPairs查询一个或多个密钥对。
    pub async fn describe_key_pairs(
        &self,
        request: DescribeKeyPairsRequest,
    ) -> Result<DescribeKeyPairsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeKeyPairs",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 绑定一个SSH密钥对到一台或多台Linux实例。
    pub async fn attach_key_pair(
        &self,
        request: AttachKeyPairRequest,
    ) -> Result<AttachKeyPairResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AttachKeyPair",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为一台或者多台Linux实例解绑SSH密钥对。
    pub async fn detach_key_pair(
        &self,
        request: DetachKeyPairRequest,
    ) -> Result<DetachKeyPairResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DetachKeyPair",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteKeyPairs删除一对或者多对SSH密钥对。
    pub async fn delete_key_pairs(
        &self,
        request: DeleteKeyPairsRequest,
    ) -> Result<DeleteKeyPairsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteKeyPairs",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建一个ECS实例启动模板，简称模板。实例启动模板能免除每次创建实例时都需要填入大量配置参数。
    pub async fn create_launch_template(
        &self,
        request: CreateLaunchTemplateRequest,
    ) -> Result<CreateLaunchTemplateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateLaunchTemplate",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeLaunchTemplates接口，并可以指定TemplateTag、TemplateResourceGroupId、LaunchTemplateId等参数，查询一个或多个实...
    pub async fn describe_launch_templates(
        &self,
        request: DescribeLaunchTemplatesRequest,
    ) -> Result<DescribeLaunchTemplatesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeLaunchTemplates",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 指定参数LaunchTemplateId或LaunchTemplateName删除目标地域下的一个实例启动模板。
    pub async fn delete_launch_template(
        &self,
        request: DeleteLaunchTemplateRequest,
    ) -> Result<DeleteLaunchTemplateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteLaunchTemplate",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 在指定的ECS实例启动模板下创建一个新版本用于后续创建ECS实例、弹性伸缩组或弹性供应组。
    pub async fn create_launch_template_version(
        &self,
        request: CreateLaunchTemplateVersionRequest,
    ) -> Result<CreateLaunchTemplateVersionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateLaunchTemplateVersion",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询ECS实例启动模板版本的信息，例如实例启动模板总数、模板名称、模板版本号等。
    pub async fn describe_launch_template_versions(
        &self,
        request: DescribeLaunchTemplateVersionsRequest,
    ) -> Result<DescribeLaunchTemplateVersionsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeLaunchTemplateVersions",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyLaunchTemplateDefaultVersion接口，并通过指定DefaultVersionNumber参数，切换启动模板的默认版本。如果您在创建实例（RunInstan...
    pub async fn modify_launch_template_default_version(
        &self,
        request: ModifyLaunchTemplateDefaultVersionRequest,
    ) -> Result<ModifyLaunchTemplateDefaultVersionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyLaunchTemplateDefaultVersion",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 指定LaunchTemplateId、DeleteVersion等参数，删除指定实例启动模板的一个或多个版本。
    pub async fn delete_launch_template_version(
        &self,
        request: DeleteLaunchTemplateVersionRequest,
    ) -> Result<DeleteLaunchTemplateVersionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteLaunchTemplateVersion",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建一个弹性供应组。
    pub async fn create_auto_provisioning_group(
        &self,
        request: CreateAutoProvisioningGroupRequest,
    ) -> Result<CreateAutoProvisioningGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateAutoProvisioningGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询一个或多个弹性供应组。
    pub async fn describe_auto_provisioning_groups(
        &self,
        request: DescribeAutoProvisioningGroupsRequest,
    ) -> Result<DescribeAutoProvisioningGroupsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeAutoProvisioningGroups",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询一个弹性供应组内的实例信息。
    pub async fn describe_auto_provisioning_group_instances(
        &self,
        request: DescribeAutoProvisioningGroupInstancesRequest,
    ) -> Result<DescribeAutoProvisioningGroupInstancesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeAutoProvisioningGroupInstances",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改一个弹性供应组的配置信息。
    pub async fn modify_auto_provisioning_group(
        &self,
        request: ModifyAutoProvisioningGroupRequest,
    ) -> Result<ModifyAutoProvisioningGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyAutoProvisioningGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeAutoProvisioningGroupHistory查询弹性供应组的调度任务信息。
    pub async fn describe_auto_provisioning_group_history(
        &self,
        request: DescribeAutoProvisioningGroupHistoryRequest,
    ) -> Result<DescribeAutoProvisioningGroupHistoryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeAutoProvisioningGroupHistory",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 指定参数AutoProvisioningGroupId、TerminateInstances删除一个弹性供应组。
    pub async fn delete_auto_provisioning_group(
        &self,
        request: DeleteAutoProvisioningGroupRequest,
    ) -> Result<DeleteAutoProvisioningGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteAutoProvisioningGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 在指定的地域内创建一个部署集。
    pub async fn create_deployment_set(
        &self,
        request: CreateDeploymentSetRequest,
    ) -> Result<CreateDeploymentSetResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateDeploymentSet",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDeploymentSetSupportedInstanceTypeFamily，并指定RegionId、Strategy参数，查询支持各部署集策略的实例规格族。
    pub async fn describe_deployment_set_supported_instance_type_family(
        &self,
        request: DescribeDeploymentSetSupportedInstanceTypeFamilyRequest,
    ) -> Result<DescribeDeploymentSetSupportedInstanceTypeFamilyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDeploymentSetSupportedInstanceTypeFamily",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询一个或多个部署集的详细信息。
    pub async fn describe_deployment_sets(
        &self,
        request: DescribeDeploymentSetsRequest,
    ) -> Result<DescribeDeploymentSetsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDeploymentSets",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改ECS实例的部署集，或迁移ECS实例至专有宿主机。支持在迁移ECS实例的同时变更实例规格。
    pub async fn modify_instance_deployment(
        &self,
        request: ModifyInstanceDeploymentRequest,
    ) -> Result<ModifyInstanceDeploymentResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyInstanceDeployment",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改一个部署集的名称和描述信息。
    pub async fn modify_deployment_set_attribute(
        &self,
        request: ModifyDeploymentSetAttributeRequest,
    ) -> Result<ModifyDeploymentSetAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDeploymentSetAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除一个部署集。
    pub async fn delete_deployment_set(
        &self,
        request: DeleteDeploymentSetRequest,
    ) -> Result<DeleteDeploymentSetResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteDeploymentSet",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 通过指定弹性保障服务所属地域下的可用区、实例规格、购买时长、是否自动续费等参数创建弹性保障服务。
    pub async fn create_elasticity_assurance(
        &self,
        request: CreateElasticityAssuranceRequest,
    ) -> Result<CreateElasticityAssuranceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateElasticityAssurance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询弹性保障服务的详细信息。例如弹性保障服务的状态、匹配模式、生效方式、失效时间、已使用的实例的数量等。
    pub async fn describe_elasticity_assurances(
        &self,
        request: DescribeElasticityAssurancesRequest,
    ) -> Result<DescribeElasticityAssurancesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeElasticityAssurances",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeElasticityAssuranceInstances查询弹性保障服务已匹配的运行状态的实例列表。
    pub async fn describe_elasticity_assurance_instances(
        &self,
        request: DescribeElasticityAssuranceInstancesRequest,
    ) -> Result<DescribeElasticityAssuranceInstancesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeElasticityAssuranceInstances",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改一个弹性保障服务的部分信息，包含名称、描述、容量（暂时只支持缩容）。
    pub async fn modify_elasticity_assurance(
        &self,
        request: ModifyElasticityAssuranceRequest,
    ) -> Result<ModifyElasticityAssuranceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyElasticityAssurance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 对一个或多个已购买的弹性保障服务进行续费。
    pub async fn renew_elasticity_assurances(
        &self,
        request: RenewElasticityAssurancesRequest,
    ) -> Result<RenewElasticityAssurancesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RenewElasticityAssurances",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改一个或多个弹性保障服务的自动续费属性。
    pub async fn modify_elasticity_assurance_auto_renew_attribute(
        &self,
        request: ModifyElasticityAssuranceAutoRenewAttributeRequest,
    ) -> Result<ModifyElasticityAssuranceAutoRenewAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyElasticityAssuranceAutoRenewAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询一个或多个弹性保障服务的自动续费属性。
    pub async fn describe_elasticity_assurance_auto_renew_attribute(
        &self,
        request: DescribeElasticityAssuranceAutoRenewAttributeRequest,
    ) -> Result<DescribeElasticityAssuranceAutoRenewAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeElasticityAssuranceAutoRenewAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口旨在为用户提供便捷、高效的方式来购买弹性保障服务。当用户存在资源准备完毕，处于未激活状态的弹性保障服务时，可以通过该接口进行购买。
    pub async fn purchase_elasticity_assurance(
        &self,
        request: PurchaseElasticityAssuranceRequest,
    ) -> Result<PurchaseElasticityAssuranceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PurchaseElasticityAssurance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 指定需要预留的实例规格、总数量，生效方式和可用区等，创建容量预定服务。
    pub async fn create_capacity_reservation(
        &self,
        request: CreateCapacityReservationRequest,
    ) -> Result<CreateCapacityReservationResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateCapacityReservation",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询一个或多个容量预定服务的详细信息，例如服务的状态、服务的生效与失效时间、私有池的模式和已使用的实例的数量等。
    pub async fn describe_capacity_reservations(
        &self,
        request: DescribeCapacityReservationsRequest,
    ) -> Result<DescribeCapacityReservationsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCapacityReservations",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询容量预定服务已匹配的实例列表。
    pub async fn describe_capacity_reservation_instances(
        &self,
        request: DescribeCapacityReservationInstancesRequest,
    ) -> Result<DescribeCapacityReservationInstancesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCapacityReservationInstances",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyCapacityReservation修改一个容量预定服务的部分信息，包括容量预定服务的名称、描述信息、失效方式以及预留的实例总数量。
    pub async fn modify_capacity_reservation(
        &self,
        request: ModifyCapacityReservationRequest,
    ) -> Result<ModifyCapacityReservationResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyCapacityReservation",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询实例匹配的私有池信息，例如匹配模式、私有池ID等。
    pub async fn describe_instance_attachment_attributes(
        &self,
        request: DescribeInstanceAttachmentAttributesRequest,
    ) -> Result<DescribeInstanceAttachmentAttributesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeInstanceAttachmentAttributes",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改实例的私有池匹配模式。
    pub async fn modify_instance_attachment_attributes(
        &self,
        request: ModifyInstanceAttachmentAttributesRequest,
    ) -> Result<ModifyInstanceAttachmentAttributesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyInstanceAttachmentAttributes",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ReleaseCapacityReservation释放容量预定服务。
    pub async fn release_capacity_reservation(
        &self,
        request: ReleaseCapacityReservationRequest,
    ) -> Result<ReleaseCapacityReservationResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ReleaseCapacityReservation",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于购买一张预留实例券，支持您购买一张地域级或者可用区级的预留实例券来抵扣对应规格的按量付费实例账单。
    pub async fn purchase_reserved_instances_offering(
        &self,
        request: PurchaseReservedInstancesOfferingRequest,
    ) -> Result<PurchaseReservedInstancesOfferingResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PurchaseReservedInstancesOffering",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口主要用于查询指定地域下您已经购买的预留实例券信息列表，支持您根据标签、预留实例券属性等参数获取预留实例券详细信息列表。
    pub async fn describe_reserved_instances(
        &self,
        request: DescribeReservedInstancesRequest,
    ) -> Result<DescribeReservedInstancesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeReservedInstances",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口的主要功能是通过修改实例规格、数量、可用区等预留实例券的配置来对预留实例券进行拆分、合并或范围变更操作。
    pub async fn modify_reserved_instances(
        &self,
        request: ModifyReservedInstancesRequest,
    ) -> Result<ModifyReservedInstancesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyReservedInstances",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于修改一张预留实例券的属性信息，包括名称和描述。
    pub async fn modify_reserved_instance_attribute(
        &self,
        request: ModifyReservedInstanceAttributeRequest,
    ) -> Result<ModifyReservedInstanceAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyReservedInstanceAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于对一张或多张预留实例券进行续费，支持您在续费时设置购买时长和自动续费属性。
    pub async fn renew_reserved_instances(
        &self,
        request: RenewReservedInstancesRequest,
    ) -> Result<RenewReservedInstancesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RenewReservedInstances",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于查询一个或多个预留实例券自动续费属性，包括自动续费时长和自动续费状态。
    pub async fn describe_reserved_instance_auto_renew_attribute(
        &self,
        request: DescribeReservedInstanceAutoRenewAttributeRequest,
    ) -> Result<DescribeReservedInstanceAutoRenewAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeReservedInstanceAutoRenewAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口主要用于修改一个或多个预留实例券的自动续费属性，支持您取消或者关闭自动续费。
    pub async fn modify_reserved_instance_auto_renew_attribute(
        &self,
        request: ModifyReservedInstanceAutoRenewAttributeRequest,
    ) -> Result<ModifyReservedInstanceAutoRenewAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyReservedInstanceAutoRenewAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于购买一个或多个存储容量单位包SCU（Storage Capacity Unit）。
    pub async fn purchase_storage_capacity_unit(
        &self,
        request: PurchaseStorageCapacityUnitRequest,
    ) -> Result<PurchaseStorageCapacityUnitResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PurchaseStorageCapacityUnit",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于查询存储容量单位包SCU（Storage Capacity Unit）的详细信息列表，支持根据名称、状态以及容量大小等条件进行查询。
    pub async fn describe_storage_capacity_units(
        &self,
        request: DescribeStorageCapacityUnitsRequest,
    ) -> Result<DescribeStorageCapacityUnitsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeStorageCapacityUnits",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于修改一个存储容量单位包SCU（Storage Capacity Unit）的名称或者描述信息。
    pub async fn modify_storage_capacity_unit_attribute(
        &self,
        request: ModifyStorageCapacityUnitAttributeRequest,
    ) -> Result<ModifyStorageCapacityUnitAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyStorageCapacityUnitAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于在一台或多台ECS实例中创建并执行云助手命令，支持Shell、PowerShell或者Bat类型的脚本，支持定时执行、自定义参数和实例内容器执行等功能。
    pub async fn run_command(
        &self,
        request: RunCommandRequest,
    ) -> Result<RunCommandResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RunCommand",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 新建一条Shell、PowerShell或者Bat脚本类型的云助手命令。
    pub async fn create_command(
        &self,
        request: CreateCommandRequest,
    ) -> Result<CreateCommandResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateCommand",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 指定CommandId、InstanceId、ResourceGroupId等参数，为一台或多台ECS实例触发一条云助手命令。
    pub async fn invoke_command(
        &self,
        request: InvokeCommandRequest,
    ) -> Result<InvokeCommandResponse, SdkError> {
        let api_request = ApiRequest {
            action: "InvokeCommand",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeInvocations查询云助手命令的执行列表和状态。
    pub async fn describe_invocations(
        &self,
        request: DescribeInvocationsRequest,
    ) -> Result<DescribeInvocationsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeInvocations",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeInvocationResults查看一条或多条云助手命令的执行结果，即在ECS实例中的实际执行结果。
    pub async fn describe_invocation_results(
        &self,
        request: DescribeInvocationResultsRequest,
    ) -> Result<DescribeInvocationResultsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeInvocationResults",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改云助手定时任务的执行信息，包括命令内容、定时执行方式、添加ECS实例或托管实例到任务。
    pub async fn modify_invocation_attribute(
        &self,
        request: ModifyInvocationAttributeRequest,
    ) -> Result<ModifyInvocationAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyInvocationAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用StopInvocation停止一台或多台ECS实例中一条正在进行中（Running）的云助手命令进程。
    pub async fn stop_invocation(
        &self,
        request: StopInvocationRequest,
    ) -> Result<StopInvocationResponse, SdkError> {
        let api_request = ApiRequest {
            action: "StopInvocation",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询您手动创建的云助手命令或者阿里云提供的公共命令。
    pub async fn describe_commands(
        &self,
        request: DescribeCommandsRequest,
    ) -> Result<DescribeCommandsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCommands",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyCommand修改一条云助手命令相关参数。
    pub async fn modify_command(
        &self,
        request: ModifyCommandRequest,
    ) -> Result<ModifyCommandResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyCommand",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteCommand接口，并指定参数RegionId、CommandId删除一条云助手命令。它无法删除正在执行中的命令。
    pub async fn delete_command(
        &self,
        request: DeleteCommandRequest,
    ) -> Result<DeleteCommandResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteCommand",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用SendFile向一台或多台ECS实例下发远程文件。
    pub async fn send_file(
        &self,
        request: SendFileRequest,
    ) -> Result<SendFileResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SendFile",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeSendFileResults查询云助手下发文件列表及状态。
    pub async fn describe_send_file_results(
        &self,
        request: DescribeSendFileResultsRequest,
    ) -> Result<DescribeSendFileResultsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeSendFileResults",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeCloudAssistantStatus查询一台或者多台实例是否安装了云助手Agent。如果已安装了云助手，还将查询云助手命令执行的总数量、正在执行的数量以及最近一次命令执行...
    pub async fn describe_cloud_assistant_status(
        &self,
        request: DescribeCloudAssistantStatusRequest,
    ) -> Result<DescribeCloudAssistantStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCloudAssistantStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用InstallCloudAssistant为一台或多台ECS实例安装云助手Agent。需要重启实例来完成安装云助手Agent的操作。
    pub async fn install_cloud_assistant(
        &self,
        request: InstallCloudAssistantRequest,
    ) -> Result<InstallCloudAssistantResponse, SdkError> {
        let api_request = ApiRequest {
            action: "InstallCloudAssistant",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用StartTerminalSession基于会话管理功能创建一个会话。您可以通过指定ECS实例ID与该实例建立一个WebSocket会话，通过接口返回的WebSocketUrl可以远程连接到...
    pub async fn start_terminal_session(
        &self,
        request: StartTerminalSessionRequest,
    ) -> Result<StartTerminalSessionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "StartTerminalSession",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 永久关闭指定Session的数据连接。
    pub async fn end_terminal_session(
        &self,
        request: EndTerminalSessionRequest,
    ) -> Result<EndTerminalSessionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "EndTerminalSession",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查看Session Manager会话历史记录。
    pub async fn describe_terminal_sessions(
        &self,
        request: DescribeTerminalSessionsRequest,
    ) -> Result<DescribeTerminalSessionsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeTerminalSessions",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改云助手服务配置。
    pub async fn modify_cloud_assistant_settings(
        &self,
        request: ModifyCloudAssistantSettingsRequest,
    ) -> Result<ModifyCloudAssistantSettingsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyCloudAssistantSettings",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询云助手服务配置。
    pub async fn describe_cloud_assistant_settings(
        &self,
        request: DescribeCloudAssistantSettingsRequest,
    ) -> Result<DescribeCloudAssistantSettingsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCloudAssistantSettings",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于创建一个激活码，该激活码用于将非阿里云服务器注册为阿里云托管实例。
    pub async fn create_activation(
        &self,
        request: CreateActivationRequest,
    ) -> Result<CreateActivationResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateActivation",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeActivations查询已创建的激活码以及激活码的使用情况。
    pub async fn describe_activations(
        &self,
        request: DescribeActivationsRequest,
    ) -> Result<DescribeActivationsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeActivations",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DisableActivation手动禁用指定的激活码。
    pub async fn disable_activation(
        &self,
        request: DisableActivationRequest,
    ) -> Result<DisableActivationResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DisableActivation",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteActivation删除一个未被使用的激活码。
    pub async fn delete_activation(
        &self,
        request: DeleteActivationRequest,
    ) -> Result<DeleteActivationResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteActivation",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeManagedInstances查询托管实例列表。
    pub async fn describe_managed_instances(
        &self,
        request: DescribeManagedInstancesRequest,
    ) -> Result<DescribeManagedInstancesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeManagedInstances",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyManagedInstance修改一台托管实例的相关信息。
    pub async fn modify_managed_instance(
        &self,
        request: ModifyManagedInstanceRequest,
    ) -> Result<ModifyManagedInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyManagedInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeregisterManagedInstance注销一个托管实例。注销后您将无法再使用云助手向实例发送命令或文件。
    pub async fn deregister_managed_instance(
        &self,
        request: DeregisterManagedInstanceRequest,
    ) -> Result<DeregisterManagedInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeregisterManagedInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ListPluginStatus查询实例中云助手插件的状态。
    pub async fn list_plugin_status(
        &self,
        request: ListPluginStatusRequest,
    ) -> Result<ListPluginStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListPluginStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeInstancesFullStatus查询一台或多台ECS实例的全状态信息。全状态信息包括实例状态和实例系统事件状态，其中，实例状态为实例的生命周期状态，实例系统事件为维护事...
    pub async fn describe_instances_full_status(
        &self,
        request: DescribeInstancesFullStatusRequest,
    ) -> Result<DescribeInstancesFullStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeInstancesFullStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDisksFullStatus查询一块或多块块存储的全部状态信息。
    pub async fn describe_disks_full_status(
        &self,
        request: DescribeDisksFullStatusRequest,
    ) -> Result<DescribeDisksFullStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDisksFullStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeInstanceHistoryEvents接口，并可以指定InstanceId、EventType等参数，查询指定实例系统事件信息，默认查询处于非活跃状态的历史系统事件。
    pub async fn describe_instance_history_events(
        &self,
        request: DescribeInstanceHistoryEventsRequest,
    ) -> Result<DescribeInstanceHistoryEventsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeInstanceHistoryEvents",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为一台或多台ECS实例预约模拟系统事件。模拟系统事件相当于事件演习，不会真正执行事件，也不会对ECS实例产生影响。
    pub async fn create_simulated_system_events(
        &self,
        request: CreateSimulatedSystemEventsRequest,
    ) -> Result<CreateSimulatedSystemEventsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateSimulatedSystemEvents",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CancelSimulatedSystemEvents取消一件或多件处于Scheduled（计划中）或Executing（执行中）状态的模拟系统事件。取消系统事件后，模拟事件变为Cancel...
    pub async fn cancel_simulated_system_events(
        &self,
        request: CancelSimulatedSystemEventsRequest,
    ) -> Result<CancelSimulatedSystemEventsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CancelSimulatedSystemEvents",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用AcceptInquiredSystemEvent接受并授权执行系统事件操作。对问询中（Inquiring）状态的系统事件，接受系统事件的默认操作，授权系统执行默认操作。
    pub async fn accept_inquired_system_event(
        &self,
        request: AcceptInquiredSystemEventRequest,
    ) -> Result<AcceptInquiredSystemEventResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AcceptInquiredSystemEvent",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDiagnosticMetrics查询诊断指标列表。
    pub async fn describe_diagnostic_metrics(
        &self,
        request: DescribeDiagnosticMetricsRequest,
    ) -> Result<DescribeDiagnosticMetricsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDiagnosticMetrics",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateDiagnosticMetricSet创建资源诊断指标集合。您可以根据需要，灵活组合诊断指标。
    pub async fn create_diagnostic_metric_set(
        &self,
        request: CreateDiagnosticMetricSetRequest,
    ) -> Result<CreateDiagnosticMetricSetResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateDiagnosticMetricSet",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDiagnosticMetricSets查询资源诊断集合列表。
    pub async fn describe_diagnostic_metric_sets(
        &self,
        request: DescribeDiagnosticMetricSetsRequest,
    ) -> Result<DescribeDiagnosticMetricSetsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDiagnosticMetricSets",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改资源诊断指标集合。
    pub async fn modify_diagnostic_metric_set(
        &self,
        request: ModifyDiagnosticMetricSetRequest,
    ) -> Result<ModifyDiagnosticMetricSetResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDiagnosticMetricSet",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteDiagnosticMetricSets删除资源诊断指标集合。
    pub async fn delete_diagnostic_metric_sets(
        &self,
        request: DeleteDiagnosticMetricSetsRequest,
    ) -> Result<DeleteDiagnosticMetricSetsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteDiagnosticMetricSets",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateDiagnosticReport创建资源诊断报告。根据您传入诊断指标集合ID，生成多个诊断指标的诊断报告。您可以根据返回的诊断报告ID，调用DescribeDiagnosticR...
    pub async fn create_diagnostic_report(
        &self,
        request: CreateDiagnosticReportRequest,
    ) -> Result<CreateDiagnosticReportResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateDiagnosticReport",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDiagnosticReports查询资源诊断报告列表。
    pub async fn describe_diagnostic_reports(
        &self,
        request: DescribeDiagnosticReportsRequest,
    ) -> Result<DescribeDiagnosticReportsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDiagnosticReports",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDiagnosticReportAttributes查询资源诊断详情。
    pub async fn describe_diagnostic_report_attributes(
        &self,
        request: DescribeDiagnosticReportAttributesRequest,
    ) -> Result<DescribeDiagnosticReportAttributesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDiagnosticReportAttributes",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteDiagnosticReports删除资源诊断报告。
    pub async fn delete_diagnostic_reports(
        &self,
        request: DeleteDiagnosticReportsRequest,
    ) -> Result<DeleteDiagnosticReportsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteDiagnosticReports",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用GetInstanceScreenshot获取实例的截屏信息。
    pub async fn get_instance_screenshot(
        &self,
        request: GetInstanceScreenshotRequest,
    ) -> Result<GetInstanceScreenshotResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetInstanceScreenshot",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取一台实例的系统命令行输出，数据以Base64编码后返回。
    pub async fn get_instance_console_output(
        &self,
        request: GetInstanceConsoleOutputRequest,
    ) -> Result<GetInstanceConsoleOutputResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetInstanceConsoleOutput",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询一块云盘在指定时间内的使用信息，例如，云盘读IOPS、写IOPS、读带宽（B/s）、写带宽（B/s）、读时延（μs）以及写时延（μs）。
    pub async fn describe_disk_monitor_data(
        &self,
        request: DescribeDiskMonitorDataRequest,
    ) -> Result<DescribeDiskMonitorDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDiskMonitorData",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeInstanceMonitorData查询一台ECS实例的监控信息。可查询的指标包括ECS实例的vCPU使用率、突发性能实例积分、接收的数据流量、发送的数据流量、平均带宽等。
    pub async fn describe_instance_monitor_data(
        &self,
        request: DescribeInstanceMonitorDataRequest,
    ) -> Result<DescribeInstanceMonitorDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeInstanceMonitorData",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeEniMonitorData查询一块辅助网卡在指定时间段内使用的流量信息。
    pub async fn describe_eni_monitor_data(
        &self,
        request: DescribeEniMonitorDataRequest,
    ) -> Result<DescribeEniMonitorDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeEniMonitorData",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询一个地域下近30天内的快照容量变化监控数据。
    pub async fn describe_snapshot_monitor_data(
        &self,
        request: DescribeSnapshotMonitorDataRequest,
    ) -> Result<DescribeSnapshotMonitorDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeSnapshotMonitorData",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeInstanceMaintenanceAttributes查询实例的维护属性。
    pub async fn describe_instance_maintenance_attributes(
        &self,
        request: DescribeInstanceMaintenanceAttributesRequest,
    ) -> Result<DescribeInstanceMaintenanceAttributesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeInstanceMaintenanceAttributes",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyInstanceMaintenanceAttributes修改实例的维护属性。
    pub async fn modify_instance_maintenance_attributes(
        &self,
        request: ModifyInstanceMaintenanceAttributesRequest,
    ) -> Result<ModifyInstanceMaintenanceAttributesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyInstanceMaintenanceAttributes",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 当ECS实例收到系统事件通知时，调用RedeployInstance可以重新部署这台ECS实例。
    pub async fn redeploy_instance(
        &self,
        request: RedeployInstanceRequest,
    ) -> Result<RedeployInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RedeployInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ReportInstancesStatus反馈一台或者多台ECS实例的异常问题。您可以反馈多台ECS实例发生的相同问题，也可以反馈一台ECS实例的多块磁盘发生的相同问题。
    pub async fn report_instances_status(
        &self,
        request: ReportInstancesStatusRequest,
    ) -> Result<ReportInstancesStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ReportInstancesStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为指定的ECS资源列表统一创建并绑定标签。
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

    /// 查询一个或多个ECS资源已经绑定的标签列表。
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

    /// 调用UntagResources为指定的ECS资源列表统一解绑标签。解绑后，如果该标签没有绑定其他任何资源，会被自动删除。
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

    /// 调用JoinResourceGroup将一个ECS资源或者服务加入一个资源组。
    pub async fn join_resource_group(
        &self,
        request: JoinResourceGroupRequest,
    ) -> Result<JoinResourceGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "JoinResourceGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用AllocateDedicatedHosts创建一台或多台按量付费或者包年包月专有宿主机。专有宿主机是单租户独享的物理机资源，您可以在专有宿主机上自行创建ECS实例和获取物理服务器属性等信息。
    pub async fn allocate_dedicated_hosts(
        &self,
        request: AllocateDedicatedHostsRequest,
    ) -> Result<AllocateDedicatedHostsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AllocateDedicatedHosts",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于查询一台或多台专有宿主机的详细信息。通过该接口，您可以获取包括专有宿主机的物理性能指标、机器码、使用状态以及已创建的ECS实例列表等信息。您能够根据具体需求，通过指定相关参数，如专有宿...
    pub async fn describe_dedicated_hosts(
        &self,
        request: DescribeDedicatedHostsRequest,
    ) -> Result<DescribeDedicatedHostsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDedicatedHosts",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDedicatedHostTypes查询指定地域下支持的专有宿主机规格详细参数，或者查询专有宿主机支持的ECS实例规格族。
    pub async fn describe_dedicated_host_types(
        &self,
        request: DescribeDedicatedHostTypesRequest,
    ) -> Result<DescribeDedicatedHostTypesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDedicatedHostTypes",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyDedicatedHostAttribute修改一台专有宿主机的部分信息，包括专有宿主机的名称、描述和服务不可用属性等。
    pub async fn modify_dedicated_host_attribute(
        &self,
        request: ModifyDedicatedHostAttributeRequest,
    ) -> Result<ModifyDedicatedHostAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDedicatedHostAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyDedicatedHostsChargeType修改专有宿主机的付费类型。
    pub async fn modify_dedicated_hosts_charge_type(
        &self,
        request: ModifyDedicatedHostsChargeTypeRequest,
    ) -> Result<ModifyDedicatedHostsChargeTypeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDedicatedHostsChargeType",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDedicatedHostAutoRenew查询一台或多台包年包月专有宿主机自动续费状态。
    pub async fn describe_dedicated_host_auto_renew(
        &self,
        request: DescribeDedicatedHostAutoRenewRequest,
    ) -> Result<DescribeDedicatedHostAutoRenewResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDedicatedHostAutoRenew",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyDedicatedHostAutoRenewAttribute为一台或多台包年包月专有宿主机设置自动续费，也可以取消已设定的自动续费。
    pub async fn modify_dedicated_host_auto_renew_attribute(
        &self,
        request: ModifyDedicatedHostAutoRenewAttributeRequest,
    ) -> Result<ModifyDedicatedHostAutoRenewAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDedicatedHostAutoRenewAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 续费一台或者多台包年包月的专有宿主机。
    pub async fn renew_dedicated_hosts(
        &self,
        request: RenewDedicatedHostsRequest,
    ) -> Result<RenewDedicatedHostsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RenewDedicatedHosts",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyDedicatedHostAutoReleaseTime为一台按量付费专有宿主机设定自动释放时间，或者取消自动释放一台按量付费专有宿主机。
    pub async fn modify_dedicated_host_auto_release_time(
        &self,
        request: ModifyDedicatedHostAutoReleaseTimeRequest,
    ) -> Result<ModifyDedicatedHostAutoReleaseTimeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDedicatedHostAutoReleaseTime",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 执行专有宿主机的故障迁移。
    pub async fn redeploy_dedicated_host(
        &self,
        request: RedeployDedicatedHostRequest,
    ) -> Result<RedeployDedicatedHostResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RedeployDedicatedHost",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于释放一台按量付费专有宿主机或者到期的包年包月专有宿主机
    pub async fn release_dedicated_host(
        &self,
        request: ReleaseDedicatedHostRequest,
    ) -> Result<ReleaseDedicatedHostResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ReleaseDedicatedHost",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建一个专有宿主机组。
    pub async fn create_dedicated_host_cluster(
        &self,
        request: CreateDedicatedHostClusterRequest,
    ) -> Result<CreateDedicatedHostClusterResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateDedicatedHostCluster",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改一台专有宿主机组的部分信息，包括专有宿主机组的名称、描述信息、属性等。
    pub async fn modify_dedicated_host_cluster_attribute(
        &self,
        request: ModifyDedicatedHostClusterAttributeRequest,
    ) -> Result<ModifyDedicatedHostClusterAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDedicatedHostClusterAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询一个或多个专有宿主机组的详细信息。
    pub async fn describe_dedicated_host_clusters(
        &self,
        request: DescribeDedicatedHostClustersRequest,
    ) -> Result<DescribeDedicatedHostClustersResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDedicatedHostClusters",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除一个专有宿主机组，操作前请先将该专有宿主机组下的专有宿主机迁移至其他专有宿主机组。
    pub async fn delete_dedicated_host_cluster(
        &self,
        request: DeleteDedicatedHostClusterRequest,
    ) -> Result<DeleteDedicatedHostClusterResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteDedicatedHostCluster",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateHpcCluster创建一个HPC集群。
    pub async fn create_hpc_cluster(
        &self,
        request: CreateHpcClusterRequest,
    ) -> Result<CreateHpcClusterResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateHpcCluster",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeHpcClusters查询您可用的HPC集群。请求参数作为筛选器（Filter）使用，筛选关系为逻辑与关系，参数之间无依赖关系。
    pub async fn describe_hpc_clusters(
        &self,
        request: DescribeHpcClustersRequest,
    ) -> Result<DescribeHpcClustersResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeHpcClusters",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyHpcClusterAttribute修改一个HPC集群的描述信息。
    pub async fn modify_hpc_cluster_attribute(
        &self,
        request: ModifyHpcClusterAttributeRequest,
    ) -> Result<ModifyHpcClusterAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyHpcClusterAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteHpcCluster删除一个HPC集群。
    pub async fn delete_hpc_cluster(
        &self,
        request: DeleteHpcClusterRequest,
    ) -> Result<DeleteHpcClusterResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteHpcCluster",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeTasks查询一个或多个异步请求的进度。
    pub async fn describe_tasks(
        &self,
        request: DescribeTasksRequest,
    ) -> Result<DescribeTasksResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeTasks",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeTaskAttribute查询异步任务的详细信息。目前，可以查询的异步任务有导入镜像（ImportImage）、导出镜像（ExportImage）及变更云盘类型（Modify...
    pub async fn describe_task_attribute(
        &self,
        request: DescribeTaskAttributeRequest,
    ) -> Result<DescribeTaskAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeTaskAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CancelTask取消一件正在运行的任务。目前，您能取消正在运行的导入镜像任务（ImportImage）和导出镜像任务（ExportImage）。
    pub async fn cancel_task(
        &self,
        request: CancelTaskRequest,
    ) -> Result<CancelTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CancelTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取用户级别默认属性
    pub async fn describe_user_business_behavior(
        &self,
        request: DescribeUserBusinessBehaviorRequest,
    ) -> Result<DescribeUserBusinessBehaviorResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeUserBusinessBehavior",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置用户级别默认属性
    pub async fn modify_user_business_behavior(
        &self,
        request: ModifyUserBusinessBehaviorRequest,
    ) -> Result<ModifyUserBusinessBehaviorResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyUserBusinessBehavior",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询账号限制
    pub async fn describe_limitation(
        &self,
        request: DescribeLimitationRequest,
    ) -> Result<DescribeLimitationResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeLimitation",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询集群
    pub async fn describe_clusters(
        &self,
        request: DescribeClustersRequest,
    ) -> Result<DescribeClustersResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeClusters",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除操作弹性网卡的权限
    pub async fn delete_network_interface_permission(
        &self,
        request: DeleteNetworkInterfacePermissionRequest,
    ) -> Result<DeleteNetworkInterfacePermissionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteNetworkInterfacePermission",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// DeleteBandwidthPackage
    pub async fn delete_bandwidth_package(
        &self,
        request: DeleteBandwidthPackageRequest,
    ) -> Result<DeleteBandwidthPackageResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteBandwidthPackage",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// ModifyBandwidthPackageSpec
    pub async fn modify_bandwidth_package_spec(
        &self,
        request: ModifyBandwidthPackageSpecRequest,
    ) -> Result<ModifyBandwidthPackageSpecResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyBandwidthPackageSpec",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// DescribeBandwidthPackages
    pub async fn describe_bandwidth_packages(
        &self,
        request: DescribeBandwidthPackagesRequest,
    ) -> Result<DescribeBandwidthPackagesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeBandwidthPackages",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// CreateVSwitch
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

    /// DeleteVSwitch
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

    /// ModifyVSwitchAttribute
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

    /// DescribeVSwitches
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

    /// CreatePhysicalConnection
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

    /// DeletePhysicalConnection
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

    /// ModifyPhysicalConnectionAttribute
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

    /// EnablePhysicalConnection
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

    /// DescribePhysicalConnections
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

    /// CancelPhysicalConnection
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

    /// TerminatePhysicalConnection
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

    /// CreateVpc
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

    /// 删除VPC
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

    /// 查询VPC
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

    /// ModifyVpcAttribute
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

    /// RemoveBandwidthPackageIps
    pub async fn remove_bandwidth_package_ips(
        &self,
        request: RemoveBandwidthPackageIpsRequest,
    ) -> Result<RemoveBandwidthPackageIpsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RemoveBandwidthPackageIps",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// CreateHaVip
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

    /// DeleteHaVip
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

    /// AssociateHaVip
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

    /// ModifyHaVipAttribute
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

    /// DescribeHaVips
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

    /// UnassociateHaVip
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

    /// CreateVirtualBorderRouter
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

    /// DeleteVirtualBorderRouter
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

    /// ModifyVirtualBorderRouterAttribute
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

    /// RecoverVirtualBorderRouter
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

    /// TerminateVirtualBorderRouter
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

    /// DescribeVirtualBorderRouters
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

    /// DescribeVirtualBorderRoutersForPhysicalConnection
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

    /// CreateRouterInterface
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

    /// DeleteRouterInterface
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

    /// ActivateRouterInterface
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

    /// DeactivateRouterInterface
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

    /// ModifyRouterInterfaceAttribute
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

    /// ModifyRouterInterfaceSpec
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

    /// ModifyVRouterAttribute
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

    /// DescribeRouterInterfaces
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

    /// UnassociateEipAddress
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

    /// AllocateEipAddress
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

    /// ModifyEipAddressAttribute
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

    /// ReleaseEipAddress
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

    /// AssociateEipAddress
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

    /// DescribeEipAddresses
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

    /// DescribeEipMonitorData
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

    /// CreateNatGateway
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

    /// DeleteNatGateway
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

    /// DescribeNatGateways
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

    /// DescribeNewProjectEipMonitorData
    pub async fn describe_new_project_eip_monitor_data(
        &self,
        request: DescribeNewProjectEipMonitorDataRequest,
    ) -> Result<DescribeNewProjectEipMonitorDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeNewProjectEipMonitorData",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// DeleteRouteEntry
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

    /// DeleteForwardEntry
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

    /// CreateForwardEntry
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

    /// AddBandwidthPackageIps
    pub async fn add_bandwidth_package_ips(
        &self,
        request: AddBandwidthPackageIpsRequest,
    ) -> Result<AddBandwidthPackageIpsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddBandwidthPackageIps",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询路由器列表
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

    /// CreateRouteEntry
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

    /// DescribeForwardTableEntries
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

    /// ModifyForwardEntry
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

    /// DescribeAccessPoints
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

    /// 查询路由表信息列表
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

    /// ConnectRouterInterface
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

    /// 将快照导出到指定的对象存储
    pub async fn export_snapshot(
        &self,
        request: ExportSnapshotRequest,
    ) -> Result<ExportSnapshotResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ExportSnapshot",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 释放公网IP。
    pub async fn release_public_ip_address(
        &self,
        request: ReleasePublicIpAddressRequest,
    ) -> Result<ReleasePublicIpAddressResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ReleasePublicIpAddress",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 添加或者覆盖一个或者多个标签到云服务器ECS的各项资源上。您可以添加标签到实例、磁盘、快照、镜像、安全组等，便于管理资源。
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

    /// 调用DescribeResourceByTags根据标签检索资源。支持根据标签检索，也支持根据资源类型检索。
    pub async fn describe_resource_by_tags(
        &self,
        request: DescribeResourceByTagsRequest,
    ) -> Result<DescribeResourceByTagsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeResourceByTags",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询可以供您使用的标签。您可以根据资源类型、资源ID、标签键或标签值等条件查询标签，筛选条件之间为逻辑与（&amp;&amp;）关系，返回满足所有筛选条件的标签。
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

    /// 调用RemoveTags从实例、磁盘、快照、镜像或者安全组等解绑一个或多个标签。
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

    /// 启用或修改弹性网卡QoS限速设置
    pub async fn enable_network_interface_qo_s(
        &self,
        request: EnableNetworkInterfaceQoSRequest,
    ) -> Result<EnableNetworkInterfaceQoSResponse, SdkError> {
        let api_request = ApiRequest {
            action: "EnableNetworkInterfaceQoS",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 禁用弹性网卡QoS限速设置
    pub async fn disable_network_interface_qo_s(
        &self,
        request: DisableNetworkInterfaceQoSRequest,
    ) -> Result<DisableNetworkInterfaceQoSResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DisableNetworkInterfaceQoS",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

}