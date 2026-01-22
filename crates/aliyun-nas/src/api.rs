//! API 调用 - 自动生成，请勿手动修改

use aliyun_sdk_core::{Client, ClientConfig, ApiRequest, HttpMethod, SdkError};
use super::types::*;

/// NAS API 版本
pub const API_VERSION: &str = "2017-06-26";

/// NAS 客户端
#[derive(Debug, Clone)]
pub struct NasClient {
    client: Client,
}

impl NasClient {
    /// 创建新客户端
    pub fn new(config: ClientConfig) -> Result<Self, SdkError> {
        Ok(Self {
            client: Client::new(config)?,
        })
    }

    /// 开通NAS服务。
    pub async fn open_nas_service(
        &self,
        request: OpenNASServiceRequest,
    ) -> Result<OpenNASServiceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "OpenNASService",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询一个地域下的所有可用区及可用区所支持的文件系统类型。
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

    /// 查询可以使用的阿里云地域。
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

    /// 创建一个文件系统。
    pub async fn create_file_system(
        &self,
        request: CreateFileSystemRequest,
    ) -> Result<CreateFileSystemResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateFileSystem",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除一个文件系统。
    pub async fn delete_file_system(
        &self,
        request: DeleteFileSystemRequest,
    ) -> Result<DeleteFileSystemResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteFileSystem",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改文件系统的描述信息。
    pub async fn modify_file_system(
        &self,
        request: ModifyFileSystemRequest,
    ) -> Result<ModifyFileSystemResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyFileSystem",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询文件系统信息。
    pub async fn describe_file_systems(
        &self,
        request: DescribeFileSystemsRequest,
    ) -> Result<DescribeFileSystemsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeFileSystems",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 扩容极速型NAS文件系统或CPFS文件系统的存储容量。
    pub async fn upgrade_file_system(
        &self,
        request: UpgradeFileSystemRequest,
    ) -> Result<UpgradeFileSystemResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpgradeFileSystem",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建挂载点。
    pub async fn create_mount_target(
        &self,
        request: CreateMountTargetRequest,
    ) -> Result<CreateMountTargetResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateMountTarget",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除已存在挂载点。
    pub async fn delete_mount_target(
        &self,
        request: DeleteMountTargetRequest,
    ) -> Result<DeleteMountTargetResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteMountTarget",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改挂载点信息。
    pub async fn modify_mount_target(
        &self,
        request: ModifyMountTargetRequest,
    ) -> Result<ModifyMountTargetResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyMountTarget",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询挂载点信息。
    pub async fn describe_mount_targets(
        &self,
        request: DescribeMountTargetsRequest,
    ) -> Result<DescribeMountTargetsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeMountTargets",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询已挂载的客户端列表。
    pub async fn describe_mounted_clients(
        &self,
        request: DescribeMountedClientsRequest,
    ) -> Result<DescribeMountedClientsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeMountedClients",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建接入点。
    pub async fn create_access_point(
        &self,
        request: CreateAccessPointRequest,
    ) -> Result<CreateAccessPointResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateAccessPoint",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改接入点信息。
    pub async fn modify_access_point(
        &self,
        request: ModifyAccessPointRequest,
    ) -> Result<ModifyAccessPointResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyAccessPoint",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询接入点信息。
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

    /// 查询接入点详情。
    pub async fn describe_access_point(
        &self,
        request: DescribeAccessPointRequest,
    ) -> Result<DescribeAccessPointResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeAccessPoint",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除接入点。
    pub async fn delete_access_point(
        &self,
        request: DeleteAccessPointRequest,
    ) -> Result<DeleteAccessPointResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteAccessPoint",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 在文件系统中创建目录。
    pub async fn create_dir(
        &self,
        request: CreateDirRequest,
    ) -> Result<CreateDirResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateDir",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建权限组。
    pub async fn create_access_group(
        &self,
        request: CreateAccessGroupRequest,
    ) -> Result<CreateAccessGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateAccessGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除已有的权限组。
    pub async fn delete_access_group(
        &self,
        request: DeleteAccessGroupRequest,
    ) -> Result<DeleteAccessGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteAccessGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改权限组。
    pub async fn modify_access_group(
        &self,
        request: ModifyAccessGroupRequest,
    ) -> Result<ModifyAccessGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyAccessGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询权限组信息。
    pub async fn describe_access_groups(
        &self,
        request: DescribeAccessGroupsRequest,
    ) -> Result<DescribeAccessGroupsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeAccessGroups",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建权限组规则。
    pub async fn create_access_rule(
        &self,
        request: CreateAccessRuleRequest,
    ) -> Result<CreateAccessRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateAccessRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除已创建的权限组规则。
    pub async fn delete_access_rule(
        &self,
        request: DeleteAccessRuleRequest,
    ) -> Result<DeleteAccessRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteAccessRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改一个权限组规则。
    pub async fn modify_access_rule(
        &self,
        request: ModifyAccessRuleRequest,
    ) -> Result<ModifyAccessRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyAccessRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询权限规则描述。
    pub async fn describe_access_rules(
        &self,
        request: DescribeAccessRulesRequest,
    ) -> Result<DescribeAccessRulesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeAccessRules",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建一个快照。
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

    /// 删除指定的快照或取消正在创建的快照任务。
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

    /// 查询指定文件系统一个或多个快照的信息。
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

    /// 创建一条自动快照策略。
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

    /// 删除一条自动快照策略。
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

    /// 修改一条自动快照策略。修改自动快照策略后，之前已应用该策略的文件系统随即执行修改后的自动快照策略。
    pub async fn modify_auto_snapshot_policy(
        &self,
        request: ModifyAutoSnapshotPolicyRequest,
    ) -> Result<ModifyAutoSnapshotPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyAutoSnapshotPolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为一个或者多个文件系统应用自动快照策略。
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

    /// 取消一个或者多个文件系统的自动快照策略。
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

    /// 查询已创建的自动快照策略。
    pub async fn describe_auto_snapshot_policies(
        &self,
        request: DescribeAutoSnapshotPoliciesRequest,
    ) -> Result<DescribeAutoSnapshotPoliciesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeAutoSnapshotPolicies",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询自动快照的任务列表。
    pub async fn describe_auto_snapshot_tasks(
        &self,
        request: DescribeAutoSnapshotTasksRequest,
    ) -> Result<DescribeAutoSnapshotTasksResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeAutoSnapshotTasks",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 使文件系统回滚至某一历史快照的文件系统状态。
    pub async fn reset_file_system(
        &self,
        request: ResetFileSystemRequest,
    ) -> Result<ResetFileSystemResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ResetFileSystem",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为指定资源创建并绑定标签。支持文件系统和接入点。
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

    /// 删除指定资源标签。
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

    /// 查询标签列表。
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

    /// 设置文件系统的目录配额。
    pub async fn set_dir_quota(
        &self,
        request: SetDirQuotaRequest,
    ) -> Result<SetDirQuotaResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SetDirQuota",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 取消文件系统的目录配额。
    pub async fn cancel_dir_quota(
        &self,
        request: CancelDirQuotaRequest,
    ) -> Result<CancelDirQuotaResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CancelDirQuota",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取文件系统每个目录配额的详细信息。
    pub async fn describe_dir_quotas(
        &self,
        request: DescribeDirQuotasRequest,
    ) -> Result<DescribeDirQuotasResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDirQuotas",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建一个生命周期管理策略。
    pub async fn create_lifecycle_policy(
        &self,
        request: CreateLifecyclePolicyRequest,
    ) -> Result<CreateLifecyclePolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateLifecyclePolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除一个生命周期管理策略。
    pub async fn delete_lifecycle_policy(
        &self,
        request: DeleteLifecyclePolicyRequest,
    ) -> Result<DeleteLifecyclePolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteLifecyclePolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改一个生命周期管理策略。
    pub async fn modify_lifecycle_policy(
        &self,
        request: ModifyLifecyclePolicyRequest,
    ) -> Result<ModifyLifecyclePolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyLifecyclePolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定目录下是否包含低频存储和归档存储文件，或者查询指定文件是否为低频存储或归档存储文件。
    pub async fn get_directory_or_file_properties(
        &self,
        request: GetDirectoryOrFilePropertiesRequest,
    ) -> Result<GetDirectoryOrFilePropertiesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetDirectoryOrFileProperties",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取生命周期管理策略列表。
    pub async fn describe_lifecycle_policies(
        &self,
        request: DescribeLifecyclePoliciesRequest,
    ) -> Result<DescribeLifecyclePoliciesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeLifecyclePolicies",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建一个数据取回任务。
    pub async fn create_lifecycle_retrieve_job(
        &self,
        request: CreateLifecycleRetrieveJobRequest,
    ) -> Result<CreateLifecycleRetrieveJobResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateLifecycleRetrieveJob",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 取消任务状态为运行中（active）的一个数据取回任务。
    pub async fn cancel_lifecycle_retrieve_job(
        &self,
        request: CancelLifecycleRetrieveJobRequest,
    ) -> Result<CancelLifecycleRetrieveJobResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CancelLifecycleRetrieveJob",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 重试任务状态为已失败（failed）的数据取回任务。
    pub async fn retry_lifecycle_retrieve_job(
        &self,
        request: RetryLifecycleRetrieveJobRequest,
    ) -> Result<RetryLifecycleRetrieveJobResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RetryLifecycleRetrieveJob",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取已创建的数据取回任务列表。
    pub async fn list_lifecycle_retrieve_jobs(
        &self,
        request: ListLifecycleRetrieveJobsRequest,
    ) -> Result<ListLifecycleRetrieveJobsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListLifecycleRetrieveJobs",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取通用型NAS指定目录下的低频存储文件和包含低频存储文件的子目录列表。
    pub async fn list_directories_and_files(
        &self,
        request: ListDirectoriesAndFilesRequest,
    ) -> Result<ListDirectoriesAndFilesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListDirectoriesAndFiles",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 开启回收站功能。
    pub async fn enable_recycle_bin(
        &self,
        request: EnableRecycleBinRequest,
    ) -> Result<EnableRecycleBinResponse, SdkError> {
        let api_request = ApiRequest {
            action: "EnableRecycleBin",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 关闭通用型NAS文件系统的回收站功能，并清空回收站中的数据。
    pub async fn disable_and_clean_recycle_bin(
        &self,
        request: DisableAndCleanRecycleBinRequest,
    ) -> Result<DisableAndCleanRecycleBinResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DisableAndCleanRecycleBin",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改一个回收站中数据的保留时间。
    pub async fn update_recycle_bin_attribute(
        &self,
        request: UpdateRecycleBinAttributeRequest,
    ) -> Result<UpdateRecycleBinAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateRecycleBinAttribute",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定通用型NAS文件系统的回收站配置。
    pub async fn get_recycle_bin_attribute(
        &self,
        request: GetRecycleBinAttributeRequest,
    ) -> Result<GetRecycleBinAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetRecycleBinAttribute",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 恢复一个暂存回收站中的文件。
    pub async fn create_recycle_bin_restore_job(
        &self,
        request: CreateRecycleBinRestoreJobRequest,
    ) -> Result<CreateRecycleBinRestoreJobResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateRecycleBinRestoreJob",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建一个彻底删除回收站中暂存的文件或目录的任务。
    pub async fn create_recycle_bin_delete_job(
        &self,
        request: CreateRecycleBinDeleteJobRequest,
    ) -> Result<CreateRecycleBinDeleteJobResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateRecycleBinDeleteJob",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 取消一个回收站中正在运行中的任务。
    pub async fn cancel_recycle_bin_job(
        &self,
        request: CancelRecycleBinJobRequest,
    ) -> Result<CancelRecycleBinJobResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CancelRecycleBinJob",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询回收站中的一个或多个任务信息。
    pub async fn list_recycle_bin_jobs(
        &self,
        request: ListRecycleBinJobsRequest,
    ) -> Result<ListRecycleBinJobsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListRecycleBinJobs",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询最近执行过删除操作的目录。
    pub async fn list_recently_recycled_directories(
        &self,
        request: ListRecentlyRecycledDirectoriesRequest,
    ) -> Result<ListRecentlyRecycledDirectoriesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListRecentlyRecycledDirectories",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询已删除的文件或目录。
    pub async fn list_recycled_directories_and_files(
        &self,
        request: ListRecycledDirectoriesAndFilesRequest,
    ) -> Result<ListRecycledDirectoriesAndFilesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListRecycledDirectoriesAndFiles",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 开启SMB AD ACL功能。
    pub async fn enable_smb_acl(
        &self,
        request: EnableSmbAclRequest,
    ) -> Result<EnableSmbAclResponse, SdkError> {
        let api_request = ApiRequest {
            action: "EnableSmbAcl",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 关闭SMB AD ACL功能。
    pub async fn disable_smb_acl(
        &self,
        request: DisableSmbAclRequest,
    ) -> Result<DisableSmbAclResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DisableSmbAcl",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新SMB AD ACL功能。
    pub async fn modify_smb_acl(
        &self,
        request: ModifySmbAclRequest,
    ) -> Result<ModifySmbAclResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifySmbAcl",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查看SMB AD ACL功能。
    pub async fn describe_smb_acl(
        &self,
        request: DescribeSmbAclRequest,
    ) -> Result<DescribeSmbAclResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeSmbAcl",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建目录或者文件。
    pub async fn create_file(
        &self,
        request: CreateFileRequest,
    ) -> Result<CreateFileResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateFile",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 开启NFS ACL功能。
    pub async fn enable_nfs_acl(
        &self,
        request: EnableNfsAclRequest,
    ) -> Result<EnableNfsAclResponse, SdkError> {
        let api_request = ApiRequest {
            action: "EnableNfsAcl",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 关闭NFS ACL功能。
    pub async fn disable_nfs_acl(
        &self,
        request: DisableNfsAclRequest,
    ) -> Result<DisableNfsAclResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DisableNfsAcl",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定文件系统是否启用了NFS ACL功能。
    pub async fn describe_nfs_acl(
        &self,
        request: DescribeNfsAclRequest,
    ) -> Result<DescribeNfsAclResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeNfsAcl",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 将通用型NAS文件系统日志转储到日志服务中。
    pub async fn create_log_analysis(
        &self,
        request: CreateLogAnalysisRequest,
    ) -> Result<CreateLogAnalysisResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateLogAnalysis",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 停止通用型NAS文件系统的日志转储。
    pub async fn delete_log_analysis(
        &self,
        request: DeleteLogAnalysisRequest,
    ) -> Result<DeleteLogAnalysisResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteLogAnalysis",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 列出日志分析中配置的日志转储信息。
    pub async fn describe_log_analysis(
        &self,
        request: DescribeLogAnalysisRequest,
    ) -> Result<DescribeLogAnalysisResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeLogAnalysis",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建一个Fileset。
    pub async fn create_fileset(
        &self,
        request: CreateFilesetRequest,
    ) -> Result<CreateFilesetResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateFileset",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除一个Fileset。
    pub async fn delete_fileset(
        &self,
        request: DeleteFilesetRequest,
    ) -> Result<DeleteFilesetResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteFileset",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改一个Fileset信息。
    pub async fn modify_fileset(
        &self,
        request: ModifyFilesetRequest,
    ) -> Result<ModifyFilesetResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyFileset",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询创建的Fileset信息。
    pub async fn get_fileset(
        &self,
        request: GetFilesetRequest,
    ) -> Result<GetFilesetResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetFileset",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询已创建的Fileset列表信息。
    pub async fn describe_filesets(
        &self,
        request: DescribeFilesetsRequest,
    ) -> Result<DescribeFilesetsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeFilesets",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置文件集配额。
    pub async fn set_fileset_quota(
        &self,
        request: SetFilesetQuotaRequest,
    ) -> Result<SetFilesetQuotaResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SetFilesetQuota",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 取消文件集设置的配额。
    pub async fn cancel_fileset_quota(
        &self,
        request: CancelFilesetQuotaRequest,
    ) -> Result<CancelFilesetQuotaResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CancelFilesetQuota",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建一个CPFS文件系统与源端存储的数据流动。
    pub async fn create_data_flow(
        &self,
        request: CreateDataFlowRequest,
    ) -> Result<CreateDataFlowResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateDataFlow",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除一个数据流动。
    pub async fn delete_data_flow(
        &self,
        request: DeleteDataFlowRequest,
    ) -> Result<DeleteDataFlowResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteDataFlow",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改数据流动的属性。
    pub async fn modify_data_flow(
        &self,
        request: ModifyDataFlowRequest,
    ) -> Result<ModifyDataFlowResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDataFlow",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询一个CPFS文件系统的数据流动信息。
    pub async fn describe_data_flows(
        &self,
        request: DescribeDataFlowsRequest,
    ) -> Result<DescribeDataFlowsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDataFlows",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 停用一个数据流动。
    pub async fn stop_data_flow(
        &self,
        request: StopDataFlowRequest,
    ) -> Result<StopDataFlowResponse, SdkError> {
        let api_request = ApiRequest {
            action: "StopDataFlow",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 启用一个数据流动。
    pub async fn start_data_flow(
        &self,
        request: StartDataFlowRequest,
    ) -> Result<StartDataFlowResponse, SdkError> {
        let api_request = ApiRequest {
            action: "StartDataFlow",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建一个数据流动任务。
    pub async fn create_data_flow_task(
        &self,
        request: CreateDataFlowTaskRequest,
    ) -> Result<CreateDataFlowTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateDataFlowTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建数据流动流式任务的子任务。
    pub async fn create_data_flow_sub_task(
        &self,
        request: CreateDataFlowSubTaskRequest,
    ) -> Result<CreateDataFlowSubTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateDataFlowSubTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 取消一个Pending（等待）或Executing（运行中）状态的数据流动批式任务或流式任务。
    pub async fn cancel_data_flow_task(
        &self,
        request: CancelDataFlowTaskRequest,
    ) -> Result<CancelDataFlowTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CancelDataFlowTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 取消数据流动流式任务。
    pub async fn cancel_data_flow_sub_task(
        &self,
        request: CancelDataFlowSubTaskRequest,
    ) -> Result<CancelDataFlowSubTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CancelDataFlowSubTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询数据流动任务详细信息。
    pub async fn describe_data_flow_tasks(
        &self,
        request: DescribeDataFlowTasksRequest,
    ) -> Result<DescribeDataFlowTasksResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDataFlowTasks",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询数据流动批量子任务。
    pub async fn describe_data_flow_sub_tasks(
        &self,
        request: DescribeDataFlowSubTasksRequest,
    ) -> Result<DescribeDataFlowSubTasksResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDataFlowSubTasks",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为指定数据流动配置自动更新。
    pub async fn apply_data_flow_auto_refresh(
        &self,
        request: ApplyDataFlowAutoRefreshRequest,
    ) -> Result<ApplyDataFlowAutoRefreshResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ApplyDataFlowAutoRefresh",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 取消指定数据流动上的自动更新配置。
    pub async fn cancel_data_flow_auto_refresh(
        &self,
        request: CancelDataFlowAutoRefreshRequest,
    ) -> Result<CancelDataFlowAutoRefreshResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CancelDataFlowAutoRefresh",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改数据流动中的自动更新配置。
    pub async fn modify_data_flow_auto_refresh(
        &self,
        request: ModifyDataFlowAutoRefreshRequest,
    ) -> Result<ModifyDataFlowAutoRefreshResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDataFlowAutoRefresh",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改文件系统实例所属的资源组。
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

    /// 创建CPFS文件系统的协议服务，创建协议服务过程大约需要5～10分钟。
    pub async fn create_protocol_service(
        &self,
        request: CreateProtocolServiceRequest,
    ) -> Result<CreateProtocolServiceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateProtocolService",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除CPFS文件系统的协议服务。
    pub async fn delete_protocol_service(
        &self,
        request: DeleteProtocolServiceRequest,
    ) -> Result<DeleteProtocolServiceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteProtocolService",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改一个协议服务，支持修改协议服务描述信息。
    pub async fn modify_protocol_service(
        &self,
        request: ModifyProtocolServiceRequest,
    ) -> Result<ModifyProtocolServiceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyProtocolService",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询协议服务信息。
    pub async fn describe_protocol_service(
        &self,
        request: DescribeProtocolServiceRequest,
    ) -> Result<DescribeProtocolServiceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeProtocolService",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建一个协议服务导出目录。
    pub async fn create_protocol_mount_target(
        &self,
        request: CreateProtocolMountTargetRequest,
    ) -> Result<CreateProtocolMountTargetResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateProtocolMountTarget",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除一个协议服务导出目录。
    pub async fn delete_protocol_mount_target(
        &self,
        request: DeleteProtocolMountTargetRequest,
    ) -> Result<DeleteProtocolMountTargetResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteProtocolMountTarget",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改协议服务的导出目录参数，只能修改权限组和描述信息。VPC ID和vSwitch ID不允许修改，如果需要修改，需要删除导出目录再新建。
    pub async fn modify_protocol_mount_target(
        &self,
        request: ModifyProtocolMountTargetRequest,
    ) -> Result<ModifyProtocolMountTargetResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyProtocolMountTarget",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询协议服务导出目录。
    pub async fn describe_protocol_mount_target(
        &self,
        request: DescribeProtocolMountTargetRequest,
    ) -> Result<DescribeProtocolMountTargetResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeProtocolMountTarget",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询协议服务导出目录信息。
    pub async fn get_protocol_mount_target(
        &self,
        request: GetProtocolMountTargetRequest,
    ) -> Result<GetProtocolMountTargetResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetProtocolMountTarget",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 取消 VSC 设备关联到文件系统。
    pub async fn detach_vsc_from_filesystems(
        &self,
        request: DetachVscFromFilesystemsRequest,
    ) -> Result<DetachVscFromFilesystemsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DetachVscFromFilesystems",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 关联 VSC 设备到文件系统。
    pub async fn attach_vsc_to_filesystems(
        &self,
        request: AttachVscToFilesystemsRequest,
    ) -> Result<AttachVscToFilesystemsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AttachVscToFilesystems",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询文件系统关联的虚拟存储通道信息。
    pub async fn describe_filesystems_vsc_attach_info(
        &self,
        request: DescribeFilesystemsVscAttachInfoRequest,
    ) -> Result<DescribeFilesystemsVscAttachInfoResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeFilesystemsVscAttachInfo",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 从CPFS服务中，将客户端加入黑名单，逐出其写入请求，起到IO Fence作用。
    pub async fn add_client_to_black_list(
        &self,
        request: AddClientToBlackListRequest,
    ) -> Result<AddClientToBlackListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddClientToBlackList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取CPFS服务中黑名单客户端的状态。
    pub async fn describe_black_list_clients(
        &self,
        request: DescribeBlackListClientsRequest,
    ) -> Result<DescribeBlackListClientsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeBlackListClients",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 从CPFS服务中，将客户端移出黑名单，恢复其写入请求。
    pub async fn remove_client_from_black_list(
        &self,
        request: RemoveClientFromBlackListRequest,
    ) -> Result<RemoveClientFromBlackListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RemoveClientFromBlackList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 添加LDAP配置。
    pub async fn create_ldap_config(
        &self,
        request: CreateLDAPConfigRequest,
    ) -> Result<CreateLDAPConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateLDAPConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 用于删除LDAP设置。
    pub async fn delete_ldap_config(
        &self,
        request: DeleteLDAPConfigRequest,
    ) -> Result<DeleteLDAPConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteLDAPConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 用于修改LDAP配置。
    pub async fn modify_ldap_config(
        &self,
        request: ModifyLDAPConfigRequest,
    ) -> Result<ModifyLDAPConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyLDAPConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 列出当前账号下文件系统的统计信息。
    pub async fn describe_file_system_statistics(
        &self,
        request: DescribeFileSystemStatisticsRequest,
    ) -> Result<DescribeFileSystemStatisticsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeFileSystemStatistics",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询存储包列表信息。
    pub async fn describe_storage_packages(
        &self,
        request: DescribeStoragePackagesRequest,
    ) -> Result<DescribeStoragePackagesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeStoragePackages",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

}