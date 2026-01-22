//! API 调用 - 自动生成，请勿手动修改

use aliyun_sdk_core::{Client, ClientConfig, ApiRequest, HttpMethod, SdkError};
use super::types::*;

/// CS API 版本
pub const API_VERSION: &str = "2015-12-15";

/// CS 客户端
#[derive(Debug, Clone)]
pub struct CsClient {
    client: Client,
}

impl CsClient {
    /// 创建新客户端
    pub fn new(config: ClientConfig) -> Result<Self, SdkError> {
        Ok(Self {
            client: Client::new(config)?,
        })
    }

    /// 首次使用容器服务Kubernetes版（Alibaba Cloud Container Service for Kubernetes，简称容器服务ACK）时，您需要调用OpenAckServic...
    pub async fn open_ack_service(
        &self,
        request: OpenAckServiceRequest,
    ) -> Result<OpenAckServiceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "OpenAckService",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 您可以通过OpenAPI创建ACK集群，包含ACK托管集群、ACK Serverless集群，ACK Edge集群以及注册集群。创建集群时，您将完成集群信息、集群组件以及ACK相关云资源的配置。
    pub async fn create_cluster(
        &self,
        request: CreateClusterRequest,
    ) -> Result<CreateClusterResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateCluster",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 如果您的集群无需使用，请删除集群并选择是否删除或保留集群的相关资源。删除集群前，请手动清理工作负载（无状态、有状态、任务和定时任务），否则可能导致集群删除失败。
    pub async fn delete_cluster(
        &self,
        request: DeleteClusterRequest,
    ) -> Result<DeleteClusterResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteCluster",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 您可以调用ModifyCluster接口修改ACK集群的配置。
    pub async fn modify_cluster(
        &self,
        request: ModifyClusterRequest,
    ) -> Result<ModifyClusterResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyCluster",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为避免过期版本集群潜在的安全和稳定性风险，同时享用新集群版本的新功能，请随ACK集群的版本发布节奏升级集群。您可以调用UpgradeCluster接口手动升级集群。
    pub async fn upgrade_cluster(
        &self,
        request: UpgradeClusterRequest,
    ) -> Result<UpgradeClusterResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpgradeCluster",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 您可以调用DescribeClustersV1接口查看本账号下符合条件的ACK集群（例如指定集群类型、集群规格）列表信息。
    pub async fn describe_clusters_v1(
        &self,
        request: DescribeClustersV1Request,
    ) -> Result<DescribeClustersV1Response, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeClustersV1",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定地域内全部集群列表。
    pub async fn describe_clusters_for_region(
        &self,
        request: DescribeClustersForRegionRequest,
    ) -> Result<DescribeClustersForRegionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeClustersForRegion",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 您可以调用DescribeClusterDetail接口，根据集群ID查询指定集群的详细信息。
    pub async fn describe_cluster_detail(
        &self,
        request: DescribeClusterDetailRequest,
    ) -> Result<DescribeClusterDetailResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeClusterDetail",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 您在使用容器服务 Kubernetes 版过程中，会同时使用其他关联的阿里云云产品资源。您可以调用DescribeClusterResources接口查询指定集群的关联资源，例如VPC、SLB等...
    pub async fn describe_cluster_resources(
        &self,
        request: DescribeClusterResourcesRequest,
    ) -> Result<DescribeClusterResourcesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeClusterResources",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 您可以调用DescribeKubernetesVersionMetadata接口查询Kubernetes版本的详细信息，包括Kubernetes版本信息、版本的发布日期和过期时间、兼容的操作系统...
    pub async fn describe_kubernetes_version_metadata(
        &self,
        request: DescribeKubernetesVersionMetadataRequest,
    ) -> Result<DescribeKubernetesVersionMetadataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeKubernetesVersionMetadata",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 在ACK集群中，可以通过Kubernetes命名空间对集群访问者进行权限和资源的逻辑隔离，只被授予指定命名空间下RBAC权限的用户将无法访问集群其他命名空间内的资源。您可以调用DescribeU...
    pub async fn describe_user_cluster_namespaces(
        &self,
        request: DescribeUserClusterNamespacesRequest,
    ) -> Result<DescribeUserClusterNamespacesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeUserClusterNamespaces",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 您可以调用DescribeClusterLogs接口检索集群的日志数据，以便在出现集群问题时进行根因分析和溯源。
    pub async fn describe_cluster_logs(
        &self,
        request: DescribeClusterLogsRequest,
    ) -> Result<DescribeClusterLogsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeClusterLogs",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 您可以调用DescribeUserQuota接口，查询ACK集群、节点池、节点的相关配额。如需扩大配额，请前往配额中心申请。
    pub async fn describe_user_quota(
        &self,
        request: DescribeUserQuotaRequest,
    ) -> Result<DescribeUserQuotaResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeUserQuota",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// ACK集群Pro版是在ACK集群基础版的基础上发展而来的集群类型，继承了原托管版集群的所有优势，例如控制面托管、控制面高可用等。同时，ACK集群Pro版进一步增强了集群的可靠性、安全性和调度性，...
    pub async fn migrate_cluster(
        &self,
        request: MigrateClusterRequest,
    ) -> Result<MigrateClusterResponse, SdkError> {
        let api_request = ApiRequest {
            action: "MigrateCluster",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// KubeConfig用于在客户端配置ACK集群的访问凭据，包含访问目标集群的身份和认证数据等信息。使用kubectl管理集群时，需要先通过KubeConfig来连接集群，您可以调用Describ...
    pub async fn describe_cluster_user_kubeconfig(
        &self,
        request: DescribeClusterUserKubeconfigRequest,
    ) -> Result<DescribeClusterUserKubeconfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeClusterUserKubeconfig",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 作为集群权限管理者，您可以使用阿里云账号（主账号）为账号内指定RAM用户或RAM角色签发包含其身份信息的KubeConfig凭证，用于连接ACK集群。您可以调用DescribeSubaccoun...
    pub async fn describe_subaccount_k8s_cluster_user_config(
        &self,
        request: DescribeSubaccountK8sClusterUserConfigRequest,
    ) -> Result<DescribeSubaccountK8sClusterUserConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeSubaccountK8sClusterUserConfig",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 如果您需要查看当前用户各个集群KubeConfig的下发情况，您可以调用ListUserKubeConfigStates接口来获取当前用户所有集群的KubeConfig状态列表。
    pub async fn list_user_kube_config_states(
        &self,
        request: ListUserKubeConfigStatesRequest,
    ) -> Result<ListUserKubeConfigStatesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListUserKubeConfigStates",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 如果您需要查看当前集群的访问控制情况，您可以调用ListClusterKubeconfigStates接口来获取当前集群已下发用户KubeConfig的列表以及状态。
    pub async fn list_cluster_kubeconfig_states(
        &self,
        request: ListClusterKubeconfigStatesRequest,
    ) -> Result<ListClusterKubeconfigStatesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListClusterKubeconfigStates",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// ACK集群下发的KubeConfig过期时间默认为3年。您可以使用阿里云账号（主账号）自定义配置，调用UpdateK8sClusterUserConfigExpire接口指定RAM用户或角色在A...
    pub async fn update_k8s_cluster_user_config_expire(
        &self,
        request: UpdateK8sClusterUserConfigExpireRequest,
    ) -> Result<UpdateK8sClusterUserConfigExpireResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateK8sClusterUserConfigExpire",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 如果您希望吊销当前登录的阿里云账号/RAM用户所拥有的集群KubeConfig凭证，您可以调用RevokeK8sClusterKubeConfig接口进行吊销。吊销成功后，集群会生成新的Kube...
    pub async fn revoke_k8s_cluster_kube_config(
        &self,
        request: RevokeK8sClusterKubeConfigRequest,
    ) -> Result<RevokeK8sClusterKubeConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RevokeK8sClusterKubeConfig",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 如果您想针对部分有安全风险的KubeConfig进行清理和解除授权，您可以调用CleanClusterUserPermissions清理指定用户在指定集群已下发的KubeConfig凭据和RBA...
    pub async fn clean_cluster_user_permissions(
        &self,
        request: CleanClusterUserPermissionsRequest,
    ) -> Result<CleanClusterUserPermissionsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CleanClusterUserPermissions",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 如果您想针对部分风险用户（已离职、账号冻结等）的KubeConfig进行清理并解除授权，您可以调用CleanUserPermissions清理指定用户已下发的KubeConfig凭据和RBAC权限。
    pub async fn clean_user_permissions(
        &self,
        request: CleanUserPermissionsRequest,
    ) -> Result<CleanUserPermissionsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CleanUserPermissions",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 节点池是具有相同属性的一组节点的逻辑集合，允许对节点进行统一的管理和运维，例如节点升级、弹性伸缩等。您可以进一步使用节点池的自动化运维能力，使用OS CVE漏洞自动修复、故障节点自动恢复、kub...
    pub async fn create_cluster_node_pool(
        &self,
        request: CreateClusterNodePoolRequest,
    ) -> Result<CreateClusterNodePoolResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateClusterNodePool",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 您可以调用DeleteClusterNodepool接口删除不再使用的节点池。节点池删除时所有节点上的Pod将会被删除，并可能触发重调度，如无法调度可能影响业务，请确保集群有足够的资源进行重调度。
    pub async fn delete_cluster_nodepool(
        &self,
        request: DeleteClusterNodepoolRequest,
    ) -> Result<DeleteClusterNodepoolResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteClusterNodepool",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 您可以根据节点池ID，调用DescribeClusterNodePoolDetail接口查询集群中目标节点池的配置。
    pub async fn describe_cluster_node_pool_detail(
        &self,
        request: DescribeClusterNodePoolDetailRequest,
    ) -> Result<DescribeClusterNodePoolDetailResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeClusterNodePoolDetail",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询集群内所有节点池列表。
    pub async fn describe_cluster_node_pools(
        &self,
        request: DescribeClusterNodePoolsRequest,
    ) -> Result<DescribeClusterNodePoolsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeClusterNodePools",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 您可以根据节点池ID调用DescribeNodePoolVuls接口查看节点池的安全漏洞详细信息，包括漏洞名称、漏洞等级等。建议您定期扫描节点池的安全漏洞，提高集群安全性。
    pub async fn describe_node_pool_vuls(
        &self,
        request: DescribeNodePoolVulsRequest,
    ) -> Result<DescribeNodePoolVulsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeNodePoolVuls",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 您可以根据节点池ID，调用ModifyClusterNodePool接口更新目标节点池的配置。
    pub async fn modify_cluster_node_pool(
        &self,
        request: ModifyClusterNodePoolRequest,
    ) -> Result<ModifyClusterNodePoolResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyClusterNodePool",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 您可以调用ScaleClusterNodePool对节点进行扩容，以保证节点数量足够支撑业务运行。
    pub async fn scale_cluster_node_pool(
        &self,
        request: ScaleClusterNodePoolRequest,
    ) -> Result<ScaleClusterNodePoolResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ScaleClusterNodePool",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 如果需要将已有ECS实例添加到ACK集群中作为Worker节点，或移除Worker节点后需将节点实例重新加入节点池，您可以调用AttachInstancesToNodePool将已有节点添加到节点池。
    pub async fn attach_instances_to_node_pool(
        &self,
        request: AttachInstancesToNodePoolRequest,
    ) -> Result<AttachInstancesToNodePoolResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AttachInstancesToNodePool",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 当您不再需要集群节点继续工作时，可以调用RemoveNodePoolNodes接口将集群中的节点移出节点池，并同时调整期望节点数。移除节点时，您可以选择是否同时释放ECS、是否自动排空节点。移除...
    pub async fn remove_node_pool_nodes(
        &self,
        request: RemoveNodePoolNodesRequest,
    ) -> Result<RemoveNodePoolNodesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RemoveNodePoolNodes",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 您可以调用UpgradeClusterNodepool接口升级指定集群节点池的kubelet版本（建议与控制面版本一致）、操作系统版本或容器运行时版本。
    pub async fn upgrade_cluster_nodepool(
        &self,
        request: UpgradeClusterNodepoolRequest,
    ) -> Result<UpgradeClusterNodepoolResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpgradeClusterNodepool",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修复集群节点池。
    pub async fn repair_cluster_node_pool(
        &self,
        request: RepairClusterNodePoolRequest,
    ) -> Result<RepairClusterNodePoolResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RepairClusterNodePool",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 阿里云云安全产品会定期扫描ECS节点上的安全漏洞，并提供对应的修复建议与方法。某些CVE的修复可能需要重启节点，请确保集群有充足的节点用于排水操作。您可以调用FixNodePoolVuls接口修...
    pub async fn fix_node_pool_vuls(
        &self,
        request: FixNodePoolVulsRequest,
    ) -> Result<FixNodePoolVulsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "FixNodePoolVuls",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 您可以调用ModifyNodePoolNodeConfig接口修改集群节点池中节点配置信息，例如kubelet配置、节点轮转配置等。修改节点配置会按批次变更节点配置并重启kubelet, 可能会...
    pub async fn modify_node_pool_node_config(
        &self,
        request: ModifyNodePoolNodeConfigRequest,
    ) -> Result<ModifyNodePoolNodeConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyNodePoolNodeConfig",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 您可以调用SyncClusterNodePool接口同步集群节点池，包含节点池元数据、节点池内节点信息等。
    pub async fn sync_cluster_node_pool(
        &self,
        request: SyncClusterNodePoolRequest,
    ) -> Result<SyncClusterNodePoolResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SyncClusterNodePool",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 如果需要将已有ECS实例添加到ACK集群中作为Worker节点，或移除Worker节点后需将节点实例重新加入节点池，ACK支持手动将已有节点到节点池。您可以调用DescribeClusterAt...
    pub async fn describe_cluster_attach_scripts(
        &self,
        request: DescribeClusterAttachScriptsRequest,
    ) -> Result<DescribeClusterAttachScriptsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeClusterAttachScripts",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 您可以创建自动弹性伸缩配置，让系统能够按照配置的伸缩规则自动增加或减少计算资源，满足您的集群工作负载的需求。创建过程中，您可以指定扩缩容的度量指标和阈值、扩容顺序、静默时间等。
    pub async fn create_autoscaling_config(
        &self,
        request: CreateAutoscalingConfigRequest,
    ) -> Result<CreateAutoscalingConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateAutoscalingConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 您可以调用DescribeClusterNodes接口，查询符合条件的节点列表信息。
    pub async fn describe_cluster_nodes(
        &self,
        request: DescribeClusterNodesRequest,
    ) -> Result<DescribeClusterNodesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeClusterNodes",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 当您不再需要集群节点继续工作时，可以调用DeleteClusterNodes接口将集群中的节点移出集群。移除节点时，您可以选择是否同时释放ECS、是否自动排空节点。
    pub async fn delete_cluster_nodes(
        &self,
        request: DeleteClusterNodesRequest,
    ) -> Result<DeleteClusterNodesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteClusterNodes",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 如果购买ECS实例后需要将该云服务器添加到ACK集群中作为Worker节点，或移除Worker节点后需将节点实例重新加入节点池，您可以调用AttachInstances接口，将已有ECS实例添加...
    pub async fn attach_instances(
        &self,
        request: AttachInstancesRequest,
    ) -> Result<AttachInstancesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AttachInstances",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为了增强Kubernetes能力，ACK集群支持了多种组件，例如托管的核心组件，应用、日志和监控、网络、存储、安全组件等。您可以调用InstallClusterAddons接口，通过组件名称和版...
    pub async fn install_cluster_addons(
        &self,
        request: InstallClusterAddonsRequest,
    ) -> Result<InstallClusterAddonsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "InstallClusterAddons",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 当您无需使用集群某些组件时，您可以调用UnInstallClusterAddons接口指定组件名称，删除组件，并选择是否删除关联的阿里云云资源。
    pub async fn un_install_cluster_addons(
        &self,
        request: UnInstallClusterAddonsRequest,
    ) -> Result<UnInstallClusterAddonsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UnInstallClusterAddons",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 您可以调用ListAddons接口，根据指定地域、集群类型、集群子类型（profile）、集群版本等参数获取可用组件的列表，并查询组件的详细信息，包括组件托管与否、支持的自定义参数Schema、...
    pub async fn list_addons(
        &self,
        request: ListAddonsRequest,
    ) -> Result<ListAddonsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListAddons",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 您可以调用ListClusterAddonInstances接口获取目标集群已安装的组件实例列表并查询组件实例的相关信息，包括组件版本、状态等。
    pub async fn list_cluster_addon_instances(
        &self,
        request: ListClusterAddonInstancesRequest,
    ) -> Result<ListClusterAddonInstancesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListClusterAddonInstances",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 您可以调用GetClusterAddonInstance接口查询集群中目标组件的详细信息，包括目标组件实例的版本、参数配置、日志功能启用状态等。
    pub async fn get_cluster_addon_instance(
        &self,
        request: GetClusterAddonInstanceRequest,
    ) -> Result<GetClusterAddonInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetClusterAddonInstance",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 您可以调用DescribeAddon接口，根据指定地域、集群类型、集群子类型（profile）、集群版本、组件名称等参数查询目标组件的信息，包括组件托管与否、组件分类、支持的自定义参数Schem...
    pub async fn describe_addon(
        &self,
        request: DescribeAddonRequest,
    ) -> Result<DescribeAddonResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeAddon",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 您可以调用ModifyClusterAddon配置修改已安装的集群组件实例的配置。修改配置可能会影响业务，请评估影响后在业务低峰期操作并提前做好相关的数据备份。
    pub async fn modify_cluster_addon(
        &self,
        request: ModifyClusterAddonRequest,
    ) -> Result<ModifyClusterAddonResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyClusterAddon",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 您可以调用UpgradeClusterAddons接口升级集群组件实例的版本，从而享受新版本带来的功能优化体验。
    pub async fn upgrade_cluster_addons(
        &self,
        request: UpgradeClusterAddonsRequest,
    ) -> Result<UpgradeClusterAddonsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpgradeClusterAddons",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询已安装的集群组件实例所包含的资源列表，包括Kubernetes集群资源、Helm发布实例等。
    pub async fn list_cluster_addon_instance_resources(
        &self,
        request: ListClusterAddonInstanceResourcesRequest,
    ) -> Result<ListClusterAddonInstanceResourcesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListClusterAddonInstanceResources",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 在ACK集群中，您可以创建并指定不同RAM用户或角色拥有不同的访问权限，从而确保安全访问控制和资源隔离。您可以调用DescribeUserPermission接口查询RAM用户或角色拥有的集群权...
    pub async fn describe_user_permission(
        &self,
        request: DescribeUserPermissionRequest,
    ) -> Result<DescribeUserPermissionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeUserPermission",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 非集群创建者、没有被授予过所有集群维度权限的RAM用户或角色在集群中默认没有任何RBAC权限。您可以调用GrantPermissions接口，更新RAM用户或角色拥有的RBAC访问权限，包括可访...
    pub async fn grant_permissions(
        &self,
        request: GrantPermissionsRequest,
    ) -> Result<GrantPermissionsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GrantPermissions",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 在ACK集群中，非集群创建者、RAM用户以及RAM角色在集群中默认没有任何RBAC权限。您可以调用UpdateUserPermissions接口，更新RAM用户或角色拥有的RBAC访问权限，包括...
    pub async fn update_user_permissions(
        &self,
        request: UpdateUserPermissionsRequest,
    ) -> Result<UpdateUserPermissionsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateUserPermissions",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 检查当前服务账号是否已授予指定的服务角色权限。只有授予对应的角色权限后，容器服务才能正常调用服务角色相关的其他云服务（ECS、OSS、NAS、SLB等）。
    pub async fn check_service_role(
        &self,
        request: CheckServiceRoleRequest,
    ) -> Result<CheckServiceRoleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CheckServiceRole",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 您可以调用ScanClusterVuls接口扫描ACK集群中潜在的安全漏洞，包括容器工作负载漏洞、三方软件漏洞、CVE漏洞、WebCMS漏洞、Windows操作系统漏洞等。建议您定期扫描集群的安...
    pub async fn scan_cluster_vuls(
        &self,
        request: ScanClusterVulsRequest,
    ) -> Result<ScanClusterVulsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ScanClusterVuls",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 您可以根据集群ID调用DescribeClusterVuls接口查看集群的安全漏洞详细信息，包括漏洞名称、漏洞类型、漏洞等级等。建议您定期扫描集群的安全漏洞，提高集群安全性。
    pub async fn describe_cluster_vuls(
        &self,
        request: DescribeClusterVulsRequest,
    ) -> Result<DescribeClusterVulsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeClusterVuls",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新指定资源的删除保护状态，当前支持的资源类型有命名空间和服务。
    pub async fn update_resources_delete_protection(
        &self,
        request: UpdateResourcesDeleteProtectionRequest,
    ) -> Result<UpdateResourcesDeleteProtectionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateResourcesDeleteProtection",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询集群内的指定资源是否开启删除保护，当前支持删除保护的资源包括命名空间和服务。
    pub async fn describe_resources_delete_protection(
        &self,
        request: DescribeResourcesDeleteProtectionRequest,
    ) -> Result<DescribeResourcesDeleteProtectionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeResourcesDeleteProtection",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 您可以在ACK集群中选择安全策略类型，配置策略实例的治理动作（告警或拦截）、作用的命名空间范围等，以创建并部署一个策略实例。您可以调用DeployPolicyInstance接口，在指定集群的命...
    pub async fn deploy_policy_instance(
        &self,
        request: DeployPolicyInstanceRequest,
    ) -> Result<DeployPolicyInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeployPolicyInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 您可以调用ModifyPolicyInstance接口，在指定集群中更新策略规则实例，调整策略实例的治理动作（告警或拦截）、作用的命名空间范围等。
    pub async fn modify_policy_instance(
        &self,
        request: ModifyPolicyInstanceRequest,
    ) -> Result<ModifyPolicyInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyPolicyInstance",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 您可以调用DeletePolicyInstance接口，在指定集群中删除策略规则实例。
    pub async fn delete_policy_instance(
        &self,
        request: DeletePolicyInstanceRequest,
    ) -> Result<DeletePolicyInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeletePolicyInstance",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// ACK集群容器安全策略供了种类丰富的内置规则库，包括Compliance、Infra、K8s-general和PSP，旨在确保容器在生产环境中的安全运行。您可以调用DescribePolicie...
    pub async fn describe_policies(
        &self,
        request: DescribePoliciesRequest,
    ) -> Result<DescribePoliciesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribePolicies",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// ACK集群容器安全策略供了种类丰富的内置规则库，包括Compliance、Infra、K8s-general和PSP，旨在确保容器在生产环境中的安全运行。您可以调用DescribePolicyD...
    pub async fn describe_policy_details(
        &self,
        request: DescribePolicyDetailsRequest,
    ) -> Result<DescribePolicyDetailsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribePolicyDetails",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// ACK集群容器安全策略供了种类丰富的内置规则库，包括Compliance、Infra、K8s-general和PSP，旨在确保容器在生产环境中的安全运行。您可以调用DescribePolicyG...
    pub async fn describe_policy_governance_in_cluster(
        &self,
        request: DescribePolicyGovernanceInClusterRequest,
    ) -> Result<DescribePolicyGovernanceInClusterResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribePolicyGovernanceInCluster",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 您可以在ACK集群中选择安全策略类型，配置相关实施动作、作用范围等，创建并部署一个策略实例。您可以调用DescribePolicyInstances接口，获取集群中指定策略规则实例的详细信息，例...
    pub async fn describe_policy_instances(
        &self,
        request: DescribePolicyInstancesRequest,
    ) -> Result<DescribePolicyInstancesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribePolicyInstances",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 您可以调用DescribePolicyInstancesStatus接口，查询集群当前不同策略类型对应的实例部署状态，包括每种策略规则对应开启的实例计数，以及不同治理等级下开启的策略种类计数。
    pub async fn describe_policy_instances_status(
        &self,
        request: DescribePolicyInstancesStatusRequest,
    ) -> Result<DescribePolicyInstancesStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribePolicyInstancesStatus",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 容器智能运维平台提供丰富的Kubernetes集群检查能力，包括集群升级检查、集群迁移检查、组件安装检查、组件升级检查、节点池检查等。在正式执行升级、迁移或安装操作前，容器智能运维平台会自动触发...
    pub async fn run_cluster_check(
        &self,
        request: RunClusterCheckRequest,
    ) -> Result<RunClusterCheckResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RunClusterCheck",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 容器智能运维平台提供丰富的Kubernetes集群检查能力，包括集群升级检查、集群迁移检查、组件安装检查、组件升级检查、节点池检查等。您可以调用ListClusterChecks接口，根据集群I...
    pub async fn list_cluster_checks(
        &self,
        request: ListClusterChecksRequest,
    ) -> Result<ListClusterChecksResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListClusterChecks",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 容器智能运维平台提供丰富的Kubernetes集群检查能力，包括集群升级检查、集群迁移检查、组件安装检查、组件升级检查、节点池检查等。您可以调用GetClusterCheck接口，根据集群ID和...
    pub async fn get_cluster_check(
        &self,
        request: GetClusterCheckRequest,
    ) -> Result<GetClusterCheckResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetClusterCheck",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建集群巡检配置。
    pub async fn create_cluster_inspect_config(
        &self,
        request: CreateClusterInspectConfigRequest,
    ) -> Result<CreateClusterInspectConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateClusterInspectConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新集群巡检配置
    pub async fn update_cluster_inspect_config(
        &self,
        request: UpdateClusterInspectConfigRequest,
    ) -> Result<UpdateClusterInspectConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateClusterInspectConfig",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取集群巡检配置
    pub async fn get_cluster_inspect_config(
        &self,
        request: GetClusterInspectConfigRequest,
    ) -> Result<GetClusterInspectConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetClusterInspectConfig",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 发起集群巡检，创建巡检报告。
    pub async fn run_cluster_inspect(
        &self,
        request: RunClusterInspectRequest,
    ) -> Result<RunClusterInspectResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RunClusterInspect",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取集群巡检报告列表。
    pub async fn list_cluster_inspect_reports(
        &self,
        request: ListClusterInspectReportsRequest,
    ) -> Result<ListClusterInspectReportsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListClusterInspectReports",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取集群巡检报告详情
    pub async fn get_cluster_inspect_report_detail(
        &self,
        request: GetClusterInspectReportDetailRequest,
    ) -> Result<GetClusterInspectReportDetailResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetClusterInspectReportDetail",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除集群巡检配置
    pub async fn delete_cluster_inspect_config(
        &self,
        request: DeleteClusterInspectConfigRequest,
    ) -> Result<DeleteClusterInspectConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteClusterInspectConfig",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 发起集群诊断。
    pub async fn create_cluster_diagnosis(
        &self,
        request: CreateClusterDiagnosisRequest,
    ) -> Result<CreateClusterDiagnosisResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateClusterDiagnosis",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取集群诊断结果。
    pub async fn get_cluster_diagnosis_result(
        &self,
        request: GetClusterDiagnosisResultRequest,
    ) -> Result<GetClusterDiagnosisResultResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetClusterDiagnosisResult",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取集群诊断检查项。
    pub async fn get_cluster_diagnosis_check_items(
        &self,
        request: GetClusterDiagnosisCheckItemsRequest,
    ) -> Result<GetClusterDiagnosisCheckItemsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetClusterDiagnosisCheckItems",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 编排模板是对一组Kubernetes集群资源的定义和描述，以声明性方式描述应用应该如何运行或者配置。您可以基于这些模板自动化部署和管理集群中的资源，例如Pods、Services、Deploym...
    pub async fn create_template(
        &self,
        request: CreateTemplateRequest,
    ) -> Result<CreateTemplateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateTemplate",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 编排模板是对一组Kubernetes集群资源的定义和描述，以声明性方式描述应用应该如何运行或者配置。您可以调用DescribeTemplateAttribute接口，查询目标编排模板的详细信息，...
    pub async fn describe_template_attribute(
        &self,
        request: DescribeTemplateAttributeRequest,
    ) -> Result<DescribeTemplateAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeTemplateAttribute",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 编排模板是对一组Kubernetes集群资源的定义和描述，以声明性方式描述应用应该如何运行或者配置。您可以调用DescribeTemplates接口获取已创建的编排模板的列表，并查询编排模板的详...
    pub async fn describe_templates(
        &self,
        request: DescribeTemplatesRequest,
    ) -> Result<DescribeTemplatesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeTemplates",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 编排模板是对一组Kubernetes集群资源的定义和描述，以声明性方式描述应用应该如何运行或者配置。您可以调用UpdateTemplate接口更新编排模板配置。
    pub async fn update_template(
        &self,
        request: UpdateTemplateRequest,
    ) -> Result<UpdateTemplateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateTemplate",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 当您无需使用某些编排模板时，您可以调用DeleteTemplate接口删除编排模板。
    pub async fn delete_template(
        &self,
        request: DeleteTemplateRequest,
    ) -> Result<DeleteTemplateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteTemplate",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 您可以为集群应用配置触发器的功能，指定触发器在接收到某些条件时重新部署Pod。
    pub async fn create_trigger(
        &self,
        request: CreateTriggerRequest,
    ) -> Result<CreateTriggerResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateTrigger",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 当您不再需要触发器时，可以调用DeleteTrigger删除应用触发器。
    pub async fn delete_trigger(
        &self,
        request: DeleteTriggerRequest,
    ) -> Result<DeleteTriggerResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteTrigger",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 您可以调用DescribeTrigger接口，查询符合条件的触发器。
    pub async fn describe_trigger(
        &self,
        request: DescribeTriggerRequest,
    ) -> Result<DescribeTriggerResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeTrigger",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 您可以为集群添加标签键值对，让集群开发或运维人员能够更灵活地对集群进行分类管理，更好的支持监控、成本分析、租户隔离等需求。您可以调用ListTagResources接口，获取资源标签的列表并查询...
    pub async fn list_tag_resources(
        &self,
        request: ListTagResourcesRequest,
    ) -> Result<ListTagResourcesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListTagResources",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 您可以为集群添加标签键值对，让集群开发或运维人员能够更灵活地对集群进行分类管理，更好的支持监控、成本分析、租户隔离等需求。您可以调用TagResources接口，为集群绑定标签。
    pub async fn tag_resources(
        &self,
        request: TagResourcesRequest,
    ) -> Result<TagResourcesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "TagResources",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 当您无需使用集群的标签键值对时，您可以调用UntagResources接口，删除资源标签。
    pub async fn untag_resources(
        &self,
        request: UntagResourcesRequest,
    ) -> Result<UntagResourcesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UntagResources",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 您可以为集群添加标签键值对，让集群开发或运维人员能够更灵活地对集群进行分类管理，更好的支持监控、成本分析、租户隔离等需求。您可以调用ModifyClusterTags接口，修改集群标签。
    pub async fn modify_cluster_tags(
        &self,
        request: ModifyClusterTagsRequest,
    ) -> Result<ModifyClusterTagsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyClusterTags",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 启动指定的报警规则。
    pub async fn start_alert(
        &self,
        request: StartAlertRequest,
    ) -> Result<StartAlertResponse, SdkError> {
        let api_request = ApiRequest {
            action: "StartAlert",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 您可以调用UpdateContactGroupForAlert接口，为ACK集群中的报警规则集设置指定的联系人分组。
    pub async fn update_contact_group_for_alert(
        &self,
        request: UpdateContactGroupForAlertRequest,
    ) -> Result<UpdateContactGroupForAlertResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateContactGroupForAlert",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 停止ACK报警中心中报警规则，停止整个报警规则集分组、单个报警规则。
    pub async fn stop_alert(
        &self,
        request: StopAlertRequest,
    ) -> Result<StopAlertResponse, SdkError> {
        let api_request = ApiRequest {
            action: "StopAlert",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除ACK报警联系人
    pub async fn delete_alert_contact(
        &self,
        request: DeleteAlertContactRequest,
    ) -> Result<DeleteAlertContactResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteAlertContact",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除ACK报警联系人分组
    pub async fn delete_alert_contact_group(
        &self,
        request: DeleteAlertContactGroupRequest,
    ) -> Result<DeleteAlertContactGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteAlertContactGroup",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// ACK托管集群支持收集控制面组件日志并投递到您的SLS Log Project中。控制面组件包括Kube API Server、Kube Scheduler、Kube Controller Ma...
    pub async fn update_control_plane_log(
        &self,
        request: UpdateControlPlaneLogRequest,
    ) -> Result<UpdateControlPlaneLogResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateControlPlaneLog",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// ACK托管集群支持收集控制面组件日志并投递到您的SLS Log Project中。控制面组件包括Kube API Server、Kube Scheduler、Kube Controller Ma...
    pub async fn check_control_plane_log_enable(
        &self,
        request: CheckControlPlaneLogEnableRequest,
    ) -> Result<CheckControlPlaneLogEnableResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CheckControlPlaneLogEnable",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 您可以调用GetClusterAuditProject查看集群是否开启API Server审计功能以及API Server审计日志对应的SLS Project。
    pub async fn get_cluster_audit_project(
        &self,
        request: GetClusterAuditProjectRequest,
    ) -> Result<GetClusterAuditProjectResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetClusterAuditProject",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 当您需要记录对Kubernetes API的请求以及请求结果来用于追溯集群操作历史、排查集群故障时，您可以调用UpdateClusterAuditLogConfig接口来开启、关闭指定ACK集群...
    pub async fn update_cluster_audit_log_config(
        &self,
        request: UpdateClusterAuditLogConfigRequest,
    ) -> Result<UpdateClusterAuditLogConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateClusterAuditLogConfig",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定地域内全部事件列表。
    pub async fn describe_events_for_region(
        &self,
        request: DescribeEventsForRegionRequest,
    ) -> Result<DescribeEventsForRegionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeEventsForRegion",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 集群操作事件包括集群创建、集群变更、节点池创建、节点池扩容、Addon安装、集群升级等。您可以调用DescribeEvents接口，查询某类事件的详细信息，包括事件级别、时间状态、事件发生时间等。
    pub async fn describe_events(
        &self,
        request: DescribeEventsRequest,
    ) -> Result<DescribeEventsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeEvents",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 集群操作事件包括集群创建、集群变更、节点池创建、节点池扩容、Addon安装、集群升级等。您可以调用DescribeClusterEvents接口，获取指定集群中发生的事件列表，并查询事件的详细信...
    pub async fn describe_cluster_events(
        &self,
        request: DescribeClusterEventsRequest,
    ) -> Result<DescribeClusterEventsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeClusterEvents",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 您可以调用DescribeTaskInfo接口，查询集群任务的详细信息，例如任务类型、运行状态、运行阶段等。
    pub async fn describe_task_info(
        &self,
        request: DescribeTaskInfoRequest,
    ) -> Result<DescribeTaskInfoResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeTaskInfo",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 您可以调用PauseTask接口，暂停执行中的集群任务。
    pub async fn pause_task(
        &self,
        request: PauseTaskRequest,
    ) -> Result<PauseTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PauseTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 您可以调用ResumeTask接口，恢复已暂停的集群任务。
    pub async fn resume_task(
        &self,
        request: ResumeTaskRequest,
    ) -> Result<ResumeTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ResumeTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 您可以调用CancelTask接口，取消集群任务的执行。
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

    /// 您可以调用DescribeClusterTasks接口，查询集群下的任务列表。
    pub async fn describe_cluster_tasks(
        &self,
        request: DescribeClusterTasksRequest,
    ) -> Result<DescribeClusterTasksResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeClusterTasks",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定地域内最近100条自动运维执行计划列表。开启集群智能托管模式Auto Mode、集群自动升级、节点池自动化运维等功能时，您可以通过该接口查询由系统自动生成的运维计划及其执行情况，例如集群...
    pub async fn list_operation_plans_for_region(
        &self,
        request: ListOperationPlansForRegionRequest,
    ) -> Result<ListOperationPlansForRegionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListOperationPlansForRegion",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取自动运维执行计划列表。
    pub async fn list_operation_plans(
        &self,
        request: ListOperationPlansRequest,
    ) -> Result<ListOperationPlansResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListOperationPlans",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 使用CancelOperationPlan取消已存在但未执行的自动运维任务执行计划。
    pub async fn cancel_operation_plan(
        &self,
        request: CancelOperationPlanRequest,
    ) -> Result<CancelOperationPlanResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CancelOperationPlan",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据集群ID，查询该集群的升级状态。
    pub async fn get_upgrade_status(
        &self,
        request: GetUpgradeStatusRequest,
    ) -> Result<GetUpgradeStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetUpgradeStatus",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 暂停用户集群升级。
    pub async fn pause_cluster_upgrade(
        &self,
        request: PauseClusterUpgradeRequest,
    ) -> Result<PauseClusterUpgradeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PauseClusterUpgrade",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 取消处于升级状态的ACK集群。
    pub async fn cancel_cluster_upgrade(
        &self,
        request: CancelClusterUpgradeRequest,
    ) -> Result<CancelClusterUpgradeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CancelClusterUpgrade",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据集群ID，恢复升级处于升级暂停状态的集群。
    pub async fn resume_upgrade_cluster(
        &self,
        request: ResumeUpgradeClusterRequest,
    ) -> Result<ResumeUpgradeClusterResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ResumeUpgradeCluster",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询平台支持的所有组件的详情。
    pub async fn describe_addons(
        &self,
        request: DescribeAddonsRequest,
    ) -> Result<DescribeAddonsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeAddons",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据集群ID，查询集群中已安装的所有组件的详情。
    pub async fn describe_cluster_addons_version(
        &self,
        request: DescribeClusterAddonsVersionRequest,
    ) -> Result<DescribeClusterAddonsVersionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeClusterAddonsVersion",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeClusterAddonInstance查询已安装的集群组件的版本、状态和配置等信息。
    pub async fn describe_cluster_addon_instance(
        &self,
        request: DescribeClusterAddonInstanceRequest,
    ) -> Result<DescribeClusterAddonInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeClusterAddonInstance",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeClusterAddonMetadata查询特定集群可使用的指定组件版本信息，包括组件版本、可配置参数等。
    pub async fn describe_cluster_addon_metadata(
        &self,
        request: DescribeClusterAddonMetadataRequest,
    ) -> Result<DescribeClusterAddonMetadataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeClusterAddonMetadata",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据组件名称查询该组件升级状态。
    pub async fn describe_cluster_addons_upgrade_status(
        &self,
        request: DescribeClusterAddonsUpgradeStatusRequest,
    ) -> Result<DescribeClusterAddonsUpgradeStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeClusterAddonsUpgradeStatus",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据集群ID查询集群中部署注册集群的代理配置。
    pub async fn describe_external_agent(
        &self,
        request: DescribeExternalAgentRequest,
    ) -> Result<DescribeExternalAgentResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeExternalAgent",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为应用创建触发器。
    pub async fn create_kubernetes_trigger(
        &self,
        request: CreateKubernetesTriggerRequest,
    ) -> Result<CreateKubernetesTriggerResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateKubernetesTrigger",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ResumeComponentUpgrade重新启动被暂停的组件升级任务。
    pub async fn resume_component_upgrade(
        &self,
        request: ResumeComponentUpgradeRequest,
    ) -> Result<ResumeComponentUpgradeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ResumeComponentUpgrade",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用PauseComponentUpgrade暂停组件升级。
    pub async fn pause_component_upgrade(
        &self,
        request: PauseComponentUpgradeRequest,
    ) -> Result<PauseComponentUpgradeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PauseComponentUpgrade",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据应用名称查询该应用的触发器。
    pub async fn get_kubernetes_trigger(
        &self,
        request: GetKubernetesTriggerRequest,
    ) -> Result<GetKubernetesTriggerResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetKubernetesTrigger",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查看容器服务中创建的所有集群（包括Swarm和Kubernetes集群）。
    pub async fn describe_clusters(
        &self,
        request: DescribeClustersRequest,
    ) -> Result<DescribeClustersResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeClusters",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取集群kubeconfig接口
    pub async fn describe_cluster_v2_user_kubeconfig(
        &self,
        request: DescribeClusterV2UserKubeconfigRequest,
    ) -> Result<DescribeClusterV2UserKubeconfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeClusterV2UserKubeconfig",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeClusterAddonUpgradeStatus查询集群组件升级状态。
    pub async fn describe_cluster_addon_upgrade_status(
        &self,
        request: DescribeClusterAddonUpgradeStatusRequest,
    ) -> Result<DescribeClusterAddonUpgradeStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeClusterAddonUpgradeStatus",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 取消集群组件升级。
    pub async fn cancel_component_upgrade(
        &self,
        request: CancelComponentUpgradeRequest,
    ) -> Result<CancelComponentUpgradeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CancelComponentUpgrade",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据应用触发器ID，删除应用触发器。
    pub async fn delete_kubernetes_trigger(
        &self,
        request: DeleteKubernetesTriggerRequest,
    ) -> Result<DeleteKubernetesTriggerResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteKubernetesTrigger",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新节点组件。
    pub async fn update_node_pool_component(
        &self,
        request: UpdateNodePoolComponentRequest,
    ) -> Result<UpdateNodePoolComponentResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateNodePoolComponent",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 安装节点组件，支持配置并指定节点安装节点组件。
    pub async fn install_node_pool_components(
        &self,
        request: InstallNodePoolComponentsRequest,
    ) -> Result<InstallNodePoolComponentsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "InstallNodePoolComponents",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

}