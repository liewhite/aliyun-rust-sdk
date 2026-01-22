//! API 调用 - 自动生成，请勿手动修改

use aliyun_sdk_core::{Client, ClientConfig, ApiRequest, HttpMethod, SdkError};
use super::types::*;

/// Ram API 版本
pub const API_VERSION: &str = "2015-05-01";

/// Ram 客户端
#[derive(Debug, Clone)]
pub struct RamClient {
    client: Client,
}

impl RamClient {
    /// 创建新客户端
    pub fn new(config: ClientConfig) -> Result<Self, SdkError> {
        Ok(Self {
            client: Client::new(config)?,
        })
    }

    /// 调用CreateUser接口创建RAM用户。
    pub async fn create_user(
        &self,
        request: CreateUserRequest,
    ) -> Result<CreateUserResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateUser",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用GetUser接口查询RAM用户的详细信息。
    pub async fn get_user(
        &self,
        request: GetUserRequest,
    ) -> Result<GetUserResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetUser",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用UpdateUser接口更新RAM用户的基本信息。
    pub async fn update_user(
        &self,
        request: UpdateUserRequest,
    ) -> Result<UpdateUserResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateUser",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteUser接口删除一个RAM用户。
    pub async fn delete_user(
        &self,
        request: DeleteUserRequest,
    ) -> Result<DeleteUserResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteUser",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询所有RAM用户的详细信息。
    pub async fn list_users(
        &self,
        request: ListUsersRequest,
    ) -> Result<ListUsersResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListUsers",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateLoginProfile接口为一个RAM用户启用Web控制台登录。
    pub async fn create_login_profile(
        &self,
        request: CreateLoginProfileRequest,
    ) -> Result<CreateLoginProfileResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateLoginProfile",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用GetLoginProfile接口查看一个RAM用户的登录配置。
    pub async fn get_login_profile(
        &self,
        request: GetLoginProfileRequest,
    ) -> Result<GetLoginProfileResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetLoginProfile",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用UpdateLoginProfile接口修改用户的登录配置。
    pub async fn update_login_profile(
        &self,
        request: UpdateLoginProfileRequest,
    ) -> Result<UpdateLoginProfileResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateLoginProfile",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 关闭指定RAM用户登录Web控制台的功能。
    pub async fn delete_login_profile(
        &self,
        request: DeleteLoginProfileRequest,
    ) -> Result<DeleteLoginProfileResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteLoginProfile",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// RAM用户调用ChangePassword修改自己的控制台登录密码。
    pub async fn change_password(
        &self,
        request: ChangePasswordRequest,
    ) -> Result<ChangePasswordResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ChangePassword",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateAccessKey接口为RAM用户创建访问密钥。
    pub async fn create_access_key(
        &self,
        request: CreateAccessKeyRequest,
    ) -> Result<CreateAccessKeyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateAccessKey",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用UpdateAccessKey接口更新RAM用户访问密钥的状态。
    pub async fn update_access_key(
        &self,
        request: UpdateAccessKeyRequest,
    ) -> Result<UpdateAccessKeyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateAccessKey",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteAccessKey接口删除RAM用户的访问密钥。
    pub async fn delete_access_key(
        &self,
        request: DeleteAccessKeyRequest,
    ) -> Result<DeleteAccessKeyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteAccessKey",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ListAccessKeys接口列出指定用户的访问密钥。
    pub async fn list_access_keys(
        &self,
        request: ListAccessKeysRequest,
    ) -> Result<ListAccessKeysResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListAccessKeys",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用GetAccessKeyLastUsed查询指定访问密钥的最后使用时间
    pub async fn get_access_key_last_used(
        &self,
        request: GetAccessKeyLastUsedRequest,
    ) -> Result<GetAccessKeyLastUsedResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetAccessKeyLastUsed",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateVirtualMFADevice接口创建多因素认证设备。
    pub async fn create_virtual_mfa_device(
        &self,
        request: CreateVirtualMFADeviceRequest,
    ) -> Result<CreateVirtualMFADeviceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateVirtualMFADevice",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取指定RAM用户的多因素认证设备信息。
    pub async fn get_user_mfa_info(
        &self,
        request: GetUserMFAInfoRequest,
    ) -> Result<GetUserMFAInfoResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetUserMFAInfo",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteVirtualMFADevice接口删除多因素认证设备。
    pub async fn delete_virtual_mfa_device(
        &self,
        request: DeleteVirtualMFADeviceRequest,
    ) -> Result<DeleteVirtualMFADeviceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteVirtualMFADevice",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ListVirtualMFADevices接口列出多因素认证设备。
    pub async fn list_virtual_mfa_devices(
        &self,
        request: ListVirtualMFADevicesRequest,
    ) -> Result<ListVirtualMFADevicesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListVirtualMFADevices",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用BindMFADevice接口绑定多因素认证设备。
    pub async fn bind_mfa_device(
        &self,
        request: BindMFADeviceRequest,
    ) -> Result<BindMFADeviceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "BindMFADevice",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用UnbindMFADevice接口解绑多因素认证设备。
    pub async fn unbind_mfa_device(
        &self,
        request: UnbindMFADeviceRequest,
    ) -> Result<UnbindMFADeviceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UnbindMFADevice",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateGroup接口创建用户组。
    pub async fn create_group(
        &self,
        request: CreateGroupRequest,
    ) -> Result<CreateGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用GetGroup接口查询用户组信息。
    pub async fn get_group(
        &self,
        request: GetGroupRequest,
    ) -> Result<GetGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用UpdateGroup接口修改用户组信息。
    pub async fn update_group(
        &self,
        request: UpdateGroupRequest,
    ) -> Result<UpdateGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteGroup接口删除指定的用户组。
    pub async fn delete_group(
        &self,
        request: DeleteGroupRequest,
    ) -> Result<DeleteGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询用户组列表。
    pub async fn list_groups(
        &self,
        request: ListGroupsRequest,
    ) -> Result<ListGroupsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListGroups",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ListGroupsForUser接口查询指定RAM用户所加入的用户组信息。
    pub async fn list_groups_for_user(
        &self,
        request: ListGroupsForUserRequest,
    ) -> Result<ListGroupsForUserResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListGroupsForUser",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ListUsersForGroup接口列出指定用户组所包含的RAM用户。
    pub async fn list_users_for_group(
        &self,
        request: ListUsersForGroupRequest,
    ) -> Result<ListUsersForGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListUsersForGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用AddUserToGroup接口将RAM用户添加到指定的用户组。
    pub async fn add_user_to_group(
        &self,
        request: AddUserToGroupRequest,
    ) -> Result<AddUserToGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddUserToGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用RemoveUserFromGroup接口将RAM用户从用户组中移除。
    pub async fn remove_user_from_group(
        &self,
        request: RemoveUserFromGroupRequest,
    ) -> Result<RemoveUserFromGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RemoveUserFromGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateRole接口创建RAM角色。
    pub async fn create_role(
        &self,
        request: CreateRoleRequest,
    ) -> Result<CreateRoleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateRole",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteRole接口删除普通RAM角色。
    pub async fn delete_role(
        &self,
        request: DeleteRoleRequest,
    ) -> Result<DeleteRoleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteRole",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用UpdateRole接口更新RAM角色信息。
    pub async fn update_role(
        &self,
        request: UpdateRoleRequest,
    ) -> Result<UpdateRoleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateRole",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用GetRole接口获取角色信息。
    pub async fn get_role(
        &self,
        request: GetRoleRequest,
    ) -> Result<GetRoleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetRole",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 列出角色。
    pub async fn list_roles(
        &self,
        request: ListRolesRequest,
    ) -> Result<ListRolesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListRoles",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreatePolicy接口创建一个自定义权限策略。
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

    /// 调用GetPolicy接口获取指定的权限策略信息。
    pub async fn get_policy(
        &self,
        request: GetPolicyRequest,
    ) -> Result<GetPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetPolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用UpdatePolicyDescription更新自定义策略的描述信息。
    pub async fn update_policy_description(
        &self,
        request: UpdatePolicyDescriptionRequest,
    ) -> Result<UpdatePolicyDescriptionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdatePolicyDescription",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeletePolicy接口删除指定的权限策略。
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

    /// 调用ListPolicies接口列出权限策略。
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

    /// 调用CreatePolicyVersion接口为权限策略创建新的版本。
    pub async fn create_policy_version(
        &self,
        request: CreatePolicyVersionRequest,
    ) -> Result<CreatePolicyVersionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreatePolicyVersion",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用GetPolicyVersion接口获取某个权限策略的版本。
    pub async fn get_policy_version(
        &self,
        request: GetPolicyVersionRequest,
    ) -> Result<GetPolicyVersionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetPolicyVersion",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeletePolicyVersion接口删除指定的权限策略的某个版本。
    pub async fn delete_policy_version(
        &self,
        request: DeletePolicyVersionRequest,
    ) -> Result<DeletePolicyVersionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeletePolicyVersion",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ListPolicyVersions接口列出权限策略版本。
    pub async fn list_policy_versions(
        &self,
        request: ListPolicyVersionsRequest,
    ) -> Result<ListPolicyVersionsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListPolicyVersions",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用SetDefaultPolicyVersion接口设置权限策略默认版本。
    pub async fn set_default_policy_version(
        &self,
        request: SetDefaultPolicyVersionRequest,
    ) -> Result<SetDefaultPolicyVersionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SetDefaultPolicyVersion",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用AttachPolicyToUser接口为指定用户添加权限。
    pub async fn attach_policy_to_user(
        &self,
        request: AttachPolicyToUserRequest,
    ) -> Result<AttachPolicyToUserResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AttachPolicyToUser",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DetachPolicyFromUser接口为用户撤销指定的权限。
    pub async fn detach_policy_from_user(
        &self,
        request: DetachPolicyFromUserRequest,
    ) -> Result<DetachPolicyFromUserResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DetachPolicyFromUser",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用AttachPolicyToGroup接口为指定用户组添加权限。
    pub async fn attach_policy_to_group(
        &self,
        request: AttachPolicyToGroupRequest,
    ) -> Result<AttachPolicyToGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AttachPolicyToGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DetachPolicyFromGroup接口为用户组撤销指定的权限。
    pub async fn detach_policy_from_group(
        &self,
        request: DetachPolicyFromGroupRequest,
    ) -> Result<DetachPolicyFromGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DetachPolicyFromGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用AttachPolicyToRole接口为指定角色添加权限。
    pub async fn attach_policy_to_role(
        &self,
        request: AttachPolicyToRoleRequest,
    ) -> Result<AttachPolicyToRoleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AttachPolicyToRole",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DetachPolicyFromRole接口为角色撤销指定的权限。
    pub async fn detach_policy_from_role(
        &self,
        request: DetachPolicyFromRoleRequest,
    ) -> Result<DetachPolicyFromRoleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DetachPolicyFromRole",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ListPoliciesForUser接口查询RAM用户的授权列表。
    pub async fn list_policies_for_user(
        &self,
        request: ListPoliciesForUserRequest,
    ) -> Result<ListPoliciesForUserResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListPoliciesForUser",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ListPoliciesForGroup接口列出用户组的权限策略。
    pub async fn list_policies_for_group(
        &self,
        request: ListPoliciesForGroupRequest,
    ) -> Result<ListPoliciesForGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListPoliciesForGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ListPoliciesForRole接口列出角色的权限策略。
    pub async fn list_policies_for_role(
        &self,
        request: ListPoliciesForRoleRequest,
    ) -> Result<ListPoliciesForRoleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListPoliciesForRole",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ListEntitiesForPolicy接口列出引用权限策略的实体。
    pub async fn list_entities_for_policy(
        &self,
        request: ListEntitiesForPolicyRequest,
    ) -> Result<ListEntitiesForPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListEntitiesForPolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用SetAccountAlias接口设置云账号别名。
    pub async fn set_account_alias(
        &self,
        request: SetAccountAliasRequest,
    ) -> Result<SetAccountAliasResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SetAccountAlias",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用GetAccountAlias接口查看云账号别名。
    pub async fn get_account_alias(
        &self,
        request: GetAccountAliasRequest,
    ) -> Result<GetAccountAliasResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetAccountAlias",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ClearAccountAlias接口清除云账号别名。
    pub async fn clear_account_alias(
        &self,
        request: ClearAccountAliasRequest,
    ) -> Result<ClearAccountAliasResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ClearAccountAlias",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用SetPasswordPolicy接口设置RAM用户密码强度等策略信息。
    pub async fn set_password_policy(
        &self,
        request: SetPasswordPolicyRequest,
    ) -> Result<SetPasswordPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SetPasswordPolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用GetPasswordPolicy接口获取RAM用户密码强度等策略信息。
    pub async fn get_password_policy(
        &self,
        request: GetPasswordPolicyRequest,
    ) -> Result<GetPasswordPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetPasswordPolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用SetSecurityPreference接口设置全局安全首选项。
    pub async fn set_security_preference(
        &self,
        request: SetSecurityPreferenceRequest,
    ) -> Result<SetSecurityPreferenceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SetSecurityPreference",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用GetSecurityPreference接口获取全局安全首选项详情。
    pub async fn get_security_preference(
        &self,
        request: GetSecurityPreferenceRequest,
    ) -> Result<GetSecurityPreferenceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetSecurityPreference",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 从因无RAM权限导致的请求被拒绝访问的响应体中解码无权限诊断信息。
    pub async fn decode_diagnostic_message(
        &self,
        request: DecodeDiagnosticMessageRequest,
    ) -> Result<DecodeDiagnosticMessageResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DecodeDiagnosticMessage",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用TagResources为云资源（RAM角色、权限策略）创建并绑定标签。
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

    /// 调用UntagResources为云资源（RAM角色、权限策略）解绑标签。
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

    /// 查询云资源（RAM角色、权限策略）绑定的标签列表。
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

}