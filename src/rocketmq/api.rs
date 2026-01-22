//! API 调用 - 自动生成，请勿手动修改

use crate::client::{Client, ClientConfig, ApiRequest, HttpMethod};
use crate::error::SdkError;
use super::types::*;

/// RocketMQ API 版本
pub const API_VERSION: &str = "2022-08-01";

/// RocketMQ 客户端
#[derive(Debug, Clone)]
pub struct RocketmqClient {
    client: Client,
}

impl RocketmqClient {
    /// 创建新客户端
    pub fn new(config: ClientConfig) -> Result<Self, SdkError> {
        Ok(Self {
            client: Client::new(config)?,
        })
    }

    /// 创建云消息队列 RocketMQ 版5.x系列实例。
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

    /// 更新云消息队列 RocketMQ 版实例的基本信息和规格配置。
    pub async fn update_instance(
        &self,
        request: UpdateInstanceRequest,
    ) -> Result<UpdateInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateInstance",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 释放实例。
    pub async fn delete_instance(
        &self,
        request: DeleteInstanceRequest,
    ) -> Result<DeleteInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteInstance",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定地域下，所有的云消息队列 RocketMQ 版实例的列表信息。
    pub async fn list_instances(
        &self,
        request: ListInstancesRequest,
    ) -> Result<ListInstancesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListInstances",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取某个指定实例的详细信息。
    pub async fn get_instance(
        &self,
        request: GetInstanceRequest,
    ) -> Result<GetInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetInstance",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建云消息队列 RocketMQ 版的主题（Topic）。主题是云消息队列 RocketMQ 版中消息传输和存储的顶层容器，用于标识同一类业务逻辑的消息。在消息收发模型中，生产者将消息发送至主题...
    pub async fn create_topic(
        &self,
        request: CreateTopicRequest,
    ) -> Result<CreateTopicResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateTopic",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新主题的备注信息。
    pub async fn update_topic(
        &self,
        request: UpdateTopicRequest,
    ) -> Result<UpdateTopicResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateTopic",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除指定主题。
    pub async fn delete_topic(
        &self,
        request: DeleteTopicRequest,
    ) -> Result<DeleteTopicResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteTopic",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定实例下的主题列表信息。
    pub async fn list_topics(
        &self,
        request: ListTopicsRequest,
    ) -> Result<ListTopicsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListTopics",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取某个指定主题的详细信息。
    pub async fn get_topic(
        &self,
        request: GetTopicRequest,
    ) -> Result<GetTopicResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetTopic",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建云消息队列 RocketMQ 版的消费者分组（ConsumerGroup）。消费者分组是云消息队列 RocketMQ 版系统中承载多个消费行为一致的消费者的负载均衡分组。消费者需要通过指定消...
    pub async fn create_consumer_group(
        &self,
        request: CreateConsumerGroupRequest,
    ) -> Result<CreateConsumerGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateConsumerGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新消费者分组的基础信息和消费重试策略。
    pub async fn update_consumer_group(
        &self,
        request: UpdateConsumerGroupRequest,
    ) -> Result<UpdateConsumerGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateConsumerGroup",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除指定的消费者分组。
    pub async fn delete_consumer_group(
        &self,
        request: DeleteConsumerGroupRequest,
    ) -> Result<DeleteConsumerGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteConsumerGroup",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定实例下消费者分组的列表信息。
    pub async fn list_consumer_groups(
        &self,
        request: ListConsumerGroupsRequest,
    ) -> Result<ListConsumerGroupsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListConsumerGroups",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取某个指定消费者分组的详细信息。
    pub async fn get_consumer_group(
        &self,
        request: GetConsumerGroupRequest,
    ) -> Result<GetConsumerGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetConsumerGroup",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询主题订阅关系列表。
    pub async fn list_topic_subscriptions(
        &self,
        request: ListTopicSubscriptionsRequest,
    ) -> Result<ListTopicSubscriptionsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListTopicSubscriptions",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询消费组订阅关系列表。
    pub async fn list_consumer_group_subscriptions(
        &self,
        request: ListConsumerGroupSubscriptionsRequest,
    ) -> Result<ListConsumerGroupSubscriptionsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListConsumerGroupSubscriptions",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取消费组订阅关系详情，包括客户端分布等信息。
    pub async fn get_consumer_group_subscription(
        &self,
        request: GetConsumerGroupSubscriptionRequest,
    ) -> Result<GetConsumerGroupSubscriptionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetConsumerGroupSubscription",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除消费者组订阅关系。
    pub async fn delete_consumer_group_subscription(
        &self,
        request: DeleteConsumerGroupSubscriptionRequest,
    ) -> Result<DeleteConsumerGroupSubscriptionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteConsumerGroupSubscription",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取指定消费者客户端的连接信息。
    pub async fn list_consumer_connections(
        &self,
        request: ListConsumerConnectionsRequest,
    ) -> Result<ListConsumerConnectionsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListConsumerConnections",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询消费者组堆积信息。
    pub async fn get_consumer_group_lag(
        &self,
        request: GetConsumerGroupLagRequest,
    ) -> Result<GetConsumerGroupLagResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetConsumerGroupLag",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取消费者堆栈信息。
    pub async fn get_consumer_stack(
        &self,
        request: GetConsumerStackRequest,
    ) -> Result<GetConsumerStackResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetConsumerStack",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 重置指定消费者分组的消费位点。重置消费位点是指改变订阅者当前的消费位置。当消费者出现故障或者消费错误数据时，您可通过重置消费位点将消费位置回滚到之前的某个位点重新开始消费，您也可以将消费位置移动...
    pub async fn reset_consume_offset(
        &self,
        request: ResetConsumeOffsetRequest,
    ) -> Result<ResetConsumeOffsetResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ResetConsumeOffset",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询消息列表。
    pub async fn list_messages(
        &self,
        request: ListMessagesRequest,
    ) -> Result<ListMessagesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListMessages",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取指定消息的详情。
    pub async fn get_message_detail(
        &self,
        request: GetMessageDetailRequest,
    ) -> Result<GetMessageDetailResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetMessageDetail",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 验证指定实例下指定主题的消息发送功能。
    pub async fn verify_send_message(
        &self,
        request: VerifySendMessageRequest,
    ) -> Result<VerifySendMessageResponse, SdkError> {
        let api_request = ApiRequest {
            action: "VerifySendMessage",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 验证指定实例下指定主题的消息消费状态。
    pub async fn verify_consume_message(
        &self,
        request: VerifyConsumeMessageRequest,
    ) -> Result<VerifyConsumeMessageResponse, SdkError> {
        let api_request = ApiRequest {
            action: "VerifyConsumeMessage",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定主题下的轨迹消息列表。
    pub async fn list_traces(
        &self,
        request: ListTracesRequest,
    ) -> Result<ListTracesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListTraces",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取指定主题下的指定消息轨迹。
    pub async fn get_trace(
        &self,
        request: GetTraceRequest,
    ) -> Result<GetTraceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetTrace",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建实例的访问账号。
    pub async fn create_instance_account(
        &self,
        request: CreateInstanceAccountRequest,
    ) -> Result<CreateInstanceAccountResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateInstanceAccount",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新指定实例下指定账号的信息。
    pub async fn update_instance_account(
        &self,
        request: UpdateInstanceAccountRequest,
    ) -> Result<UpdateInstanceAccountResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateInstanceAccount",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除指定实例下的访问账号信息。
    pub async fn delete_instance_account(
        &self,
        request: DeleteInstanceAccountRequest,
    ) -> Result<DeleteInstanceAccountResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteInstanceAccount",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定实例下访问账号的列表。
    pub async fn list_instance_account(
        &self,
        request: ListInstanceAccountRequest,
    ) -> Result<ListInstanceAccountResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListInstanceAccount",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取指定实例的账号。
    pub async fn get_instance_account(
        &self,
        request: GetInstanceAccountRequest,
    ) -> Result<GetInstanceAccountResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetInstanceAccount",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 在指定实例下创建访问的权限信息。
    pub async fn create_instance_acl(
        &self,
        request: CreateInstanceAclRequest,
    ) -> Result<CreateInstanceAclResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateInstanceAcl",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新指定实例下，指定用户和资源的访问权限。
    pub async fn update_instance_acl(
        &self,
        request: UpdateInstanceAclRequest,
    ) -> Result<UpdateInstanceAclResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateInstanceAcl",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除指定实例下，指定账号和资源的权限。
    pub async fn delete_instance_acl(
        &self,
        request: DeleteInstanceAclRequest,
    ) -> Result<DeleteInstanceAclResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteInstanceAcl",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定实例下的访问权限列表。
    pub async fn list_instance_acl(
        &self,
        request: ListInstanceAclRequest,
    ) -> Result<ListInstanceAclResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListInstanceAcl",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询访问权限信息详情。
    pub async fn get_instance_acl(
        &self,
        request: GetInstanceAclRequest,
    ) -> Result<GetInstanceAclResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetInstanceAcl",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建访问控制IP白名单。
    pub async fn create_instance_ip_whitelist(
        &self,
        request: CreateInstanceIpWhitelistRequest,
    ) -> Result<CreateInstanceIpWhitelistResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateInstanceIpWhitelist",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除指定实例下的指定全局IP白名单。
    pub async fn delete_instance_ip_whitelist(
        &self,
        request: DeleteInstanceIpWhitelistRequest,
    ) -> Result<DeleteInstanceIpWhitelistResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteInstanceIpWhitelist",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询访问控制IP白名单列表。
    pub async fn list_instance_ip_whitelist(
        &self,
        request: ListInstanceIpWhitelistRequest,
    ) -> Result<ListInstanceIpWhitelistResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListInstanceIpWhitelist",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询访问控制IP白名单详情。
    pub async fn get_instance_ip_whitelist(
        &self,
        request: GetInstanceIpWhitelistRequest,
    ) -> Result<GetInstanceIpWhitelistResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetInstanceIpWhitelist",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询迁移任务列表。
    pub async fn list_migrations(
        &self,
        request: ListMigrationsRequest,
    ) -> Result<ListMigrationsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListMigrations",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询迁移操作列表。
    pub async fn list_migration_operations(
        &self,
        request: ListMigrationOperationsRequest,
    ) -> Result<ListMigrationOperationsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListMigrationOperations",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 指定的迁移任务执行指定的操作。
    pub async fn execute_migration_operation(
        &self,
        request: ExecuteMigrationOperationRequest,
    ) -> Result<ExecuteMigrationOperationResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ExecuteMigrationOperation",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 指定的迁移任务完成当前迁移阶段。
    pub async fn finish_migration_stage(
        &self,
        request: FinishMigrationStageRequest,
    ) -> Result<FinishMigrationStageResponse, SdkError> {
        let api_request = ApiRequest {
            action: "FinishMigrationStage",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建备份计划。
    pub async fn create_disaster_recovery_plan(
        &self,
        request: CreateDisasterRecoveryPlanRequest,
    ) -> Result<CreateDisasterRecoveryPlanResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateDisasterRecoveryPlan",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除备份计划。
    pub async fn delete_disaster_recovery_plan(
        &self,
        request: DeleteDisasterRecoveryPlanRequest,
    ) -> Result<DeleteDisasterRecoveryPlanResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteDisasterRecoveryPlan",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改备份计划。
    pub async fn update_disaster_recovery_plan(
        &self,
        request: UpdateDisasterRecoveryPlanRequest,
    ) -> Result<UpdateDisasterRecoveryPlanResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateDisasterRecoveryPlan",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询备份计划列表。
    pub async fn list_disaster_recovery_plans(
        &self,
        request: ListDisasterRecoveryPlansRequest,
    ) -> Result<ListDisasterRecoveryPlansResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListDisasterRecoveryPlans",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询备份计划详情。
    pub async fn get_disaster_recovery_plan(
        &self,
        request: GetDisasterRecoveryPlanRequest,
    ) -> Result<GetDisasterRecoveryPlanResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetDisasterRecoveryPlan",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 添加备份映射任务
    pub async fn add_disaster_recovery_item(
        &self,
        request: AddDisasterRecoveryItemRequest,
    ) -> Result<AddDisasterRecoveryItemResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddDisasterRecoveryItem",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新备份映射任务。
    pub async fn update_disaster_recovery_item(
        &self,
        request: UpdateDisasterRecoveryItemRequest,
    ) -> Result<UpdateDisasterRecoveryItemResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateDisasterRecoveryItem",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除备份映射任务。
    pub async fn delete_disaster_recovery_item(
        &self,
        request: DeleteDisasterRecoveryItemRequest,
    ) -> Result<DeleteDisasterRecoveryItemResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteDisasterRecoveryItem",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 启用备份映射任务。
    pub async fn start_disaster_recovery_item(
        &self,
        request: StartDisasterRecoveryItemRequest,
    ) -> Result<StartDisasterRecoveryItemResponse, SdkError> {
        let api_request = ApiRequest {
            action: "StartDisasterRecoveryItem",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 停用备份映射任务。
    pub async fn stop_disaster_recovery_item(
        &self,
        request: StopDisasterRecoveryItemRequest,
    ) -> Result<StopDisasterRecoveryItemResponse, SdkError> {
        let api_request = ApiRequest {
            action: "StopDisasterRecoveryItem",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询备份映射任务列表。
    pub async fn list_disaster_recovery_items(
        &self,
        request: ListDisasterRecoveryItemsRequest,
    ) -> Result<ListDisasterRecoveryItemsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListDisasterRecoveryItems",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询备份计划映射详情。
    pub async fn get_disaster_recovery_item(
        &self,
        request: GetDisasterRecoveryItemRequest,
    ) -> Result<GetDisasterRecoveryItemResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetDisasterRecoveryItem",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询消费进度同步列表。
    pub async fn list_disaster_recovery_checkpoints(
        &self,
        request: ListDisasterRecoveryCheckpointsRequest,
    ) -> Result<ListDisasterRecoveryCheckpointsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListDisasterRecoveryCheckpoints",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 同步消费进度信息到目标实例。
    pub async fn sync_disaster_recovery_checkpoint(
        &self,
        request: SyncDisasterRecoveryCheckpointRequest,
    ) -> Result<SyncDisasterRecoveryCheckpointResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SyncDisasterRecoveryCheckpoint",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询可见的资源标签关系。
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

    /// 用户创建标签资源关系。
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

    /// 用户删除标签资源关系。
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

    /// 修改实例所属的资源组。
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

    /// 查询topic可重置时间范围
    pub async fn get_consume_timespan(
        &self,
        request: GetConsumeTimespanRequest,
    ) -> Result<GetConsumeTimespanResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetConsumeTimespan",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询地域列表。
    pub async fn list_regions(
        &self,
        request: ListRegionsRequest,
    ) -> Result<ListRegionsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListRegions",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询支持的可用区。
    pub async fn list_available_zones(
        &self,
        request: ListAvailableZonesRequest,
    ) -> Result<ListAvailableZonesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListAvailableZones",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询监控项列表。
    pub async fn list_metric_meta(
        &self,
        request: ListMetricMetaRequest,
    ) -> Result<ListMetricMetaResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListMetricMeta",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

}