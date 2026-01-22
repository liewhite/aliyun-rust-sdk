//! API 调用 - 自动生成，请勿手动修改

use crate::client::{Client, ClientConfig, ApiRequest, HttpMethod};
use crate::error::SdkError;
use super::types::*;

/// Cdn API 版本
pub const API_VERSION: &str = "2018-05-10";

/// Cdn 客户端
#[derive(Debug, Clone)]
pub struct CdnClient {
    client: Client,
}

impl CdnClient {
    /// 创建新客户端
    pub fn new(config: ClientConfig) -> Result<Self, SdkError> {
        Ok(Self {
            client: Client::new(config)?,
        })
    }

    /// 调用AddCdnDomain添加加速域名。
    pub async fn add_cdn_domain(
        &self,
        request: AddCdnDomainRequest,
    ) -> Result<AddCdnDomainResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddCdnDomain",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用BatchAddCdnDomain可以实现批量添加加速域名，一次最多可以添加50个加速域名。
    pub async fn batch_add_cdn_domain(
        &self,
        request: BatchAddCdnDomainRequest,
    ) -> Result<BatchAddCdnDomainResponse, SdkError> {
        let api_request = ApiRequest {
            action: "BatchAddCdnDomain",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteCdnDomain删除已添加的加速域名。
    pub async fn delete_cdn_domain(
        &self,
        request: DeleteCdnDomainRequest,
    ) -> Result<DeleteCdnDomainResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteCdnDomain",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeCdnDeletedDomains查询您名下已删除的域名。
    pub async fn describe_cdn_deleted_domains(
        &self,
        request: DescribeCdnDeletedDomainsRequest,
    ) -> Result<DescribeCdnDeletedDomainsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCdnDeletedDomains",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用VerifyDomainOwner对域名归属权进行校验。
    pub async fn verify_domain_owner(
        &self,
        request: VerifyDomainOwnerRequest,
    ) -> Result<VerifyDomainOwnerResponse, SdkError> {
        let api_request = ApiRequest {
            action: "VerifyDomainOwner",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeVerifyContent查询归属校验内容。
    pub async fn describe_verify_content(
        &self,
        request: DescribeVerifyContentRequest,
    ) -> Result<DescribeVerifyContentResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeVerifyContent",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDomainVerifyData，根据加速域名是否开启全球资源计划，返回对应的校验内容。
    pub async fn describe_domain_verify_data(
        &self,
        request: DescribeDomainVerifyDataRequest,
    ) -> Result<DescribeDomainVerifyDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainVerifyData",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用StartCdnDomain启用状态为停用的加速域名，将DomainStatus变更为Online。
    pub async fn start_cdn_domain(
        &self,
        request: StartCdnDomainRequest,
    ) -> Result<StartCdnDomainResponse, SdkError> {
        let api_request = ApiRequest {
            action: "StartCdnDomain",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用BatchStartCdnDomain批量启用状态为停用的加速域名，将DomainStatus变更为Online。
    pub async fn batch_start_cdn_domain(
        &self,
        request: BatchStartCdnDomainRequest,
    ) -> Result<BatchStartCdnDomainResponse, SdkError> {
        let api_request = ApiRequest {
            action: "BatchStartCdnDomain",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用StopCdnDomain停用某个加速域名，将DomainStatus变更为Offline。
    pub async fn stop_cdn_domain(
        &self,
        request: StopCdnDomainRequest,
    ) -> Result<StopCdnDomainResponse, SdkError> {
        let api_request = ApiRequest {
            action: "StopCdnDomain",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用BatchStopCdnDomain批量停用加速域名，将DomainStatus变更为Offline。
    pub async fn batch_stop_cdn_domain(
        &self,
        request: BatchStopCdnDomainRequest,
    ) -> Result<BatchStopCdnDomainResponse, SdkError> {
        let api_request = ApiRequest {
            action: "BatchStopCdnDomain",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeUserDomains查询用户名下所有的域名与状态，支持域名模糊匹配过滤和域名状态过滤。
    pub async fn describe_user_domains(
        &self,
        request: DescribeUserDomainsRequest,
    ) -> Result<DescribeUserDomainsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeUserDomains",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用接口对加速域名 Cname 进行解析检测，查看解析结果判断是否完成Cname配置。
    pub async fn describe_domain_cname(
        &self,
        request: DescribeDomainCnameRequest,
    ) -> Result<DescribeDomainCnameResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainCname",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDomainsBySource按源站查询加速域名。
    pub async fn describe_domains_by_source(
        &self,
        request: DescribeDomainsBySourceRequest,
    ) -> Result<DescribeDomainsBySourceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainsBySource",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeCdnUserDomainsByFunc查询用户名下配置了指定功能的所有的域名及其状态。
    pub async fn describe_cdn_user_domains_by_func(
        &self,
        request: DescribeCdnUserDomainsByFuncRequest,
    ) -> Result<DescribeCdnUserDomainsByFuncResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCdnUserDomainsByFunc",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeCdnDomainDetail查询指定加速域名的基本配置。
    pub async fn describe_cdn_domain_detail(
        &self,
        request: DescribeCdnDomainDetailRequest,
    ) -> Result<DescribeCdnDomainDetailResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCdnDomainDetail",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeCdnDomainConfigs获取加速域名的配置信息，一次可查询多个功能配置信息。
    pub async fn describe_cdn_domain_configs(
        &self,
        request: DescribeCdnDomainConfigsRequest,
    ) -> Result<DescribeCdnDomainConfigsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCdnDomainConfigs",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用BatchSetCdnDomainConfig进行域名批量配置。
    pub async fn batch_set_cdn_domain_config(
        &self,
        request: BatchSetCdnDomainConfigRequest,
    ) -> Result<BatchSetCdnDomainConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "BatchSetCdnDomainConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用BatchDeleteCdnDomainConfig批量删除加速域名的配置。
    pub async fn batch_delete_cdn_domain_config(
        &self,
        request: BatchDeleteCdnDomainConfigRequest,
    ) -> Result<BatchDeleteCdnDomainConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "BatchDeleteCdnDomainConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用BatchUpdateCdnDomain批量更新加速域名基本信息。
    pub async fn batch_update_cdn_domain(
        &self,
        request: BatchUpdateCdnDomainRequest,
    ) -> Result<BatchUpdateCdnDomainResponse, SdkError> {
        let api_request = ApiRequest {
            action: "BatchUpdateCdnDomain",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteSpecificConfig删除域名的指定配置。
    pub async fn delete_specific_config(
        &self,
        request: DeleteSpecificConfigRequest,
    ) -> Result<DeleteSpecificConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteSpecificConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyCdnDomainSchdmByProperty通过配置组修改域名的加速区域。
    pub async fn modify_cdn_domain_schdm_by_property(
        &self,
        request: ModifyCdnDomainSchdmByPropertyRequest,
    ) -> Result<ModifyCdnDomainSchdmByPropertyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyCdnDomainSchdmByProperty",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyCdnDomain修改加速域名基础信息。
    pub async fn modify_cdn_domain(
        &self,
        request: ModifyCdnDomainRequest,
    ) -> Result<ModifyCdnDomainResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyCdnDomain",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeCdnDomainStagingConfig查询模拟环境配置信息，一次可查询多个功能配置。
    pub async fn describe_cdn_domain_staging_config(
        &self,
        request: DescribeCdnDomainStagingConfigRequest,
    ) -> Result<DescribeCdnDomainStagingConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCdnDomainStagingConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用SetCdnDomainStagingConfig设置模拟环境下的加速域名配置。
    pub async fn set_cdn_domain_staging_config(
        &self,
        request: SetCdnDomainStagingConfigRequest,
    ) -> Result<SetCdnDomainStagingConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SetCdnDomainStagingConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用RollbackStagingConfig可以起到回滚模拟环境配置的作用，配置回滚后的模拟环境将被还原为初始状态，模拟环境下的所有配置都将被清空。
    pub async fn rollback_staging_config(
        &self,
        request: RollbackStagingConfigRequest,
    ) -> Result<RollbackStagingConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RollbackStagingConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用PublishStagingConfigToProduction将模拟环境配置发布为生产环境配置。
    pub async fn publish_staging_config_to_production(
        &self,
        request: PublishStagingConfigToProductionRequest,
    ) -> Result<PublishStagingConfigToProductionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PublishStagingConfigToProduction",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteSpecificStagingConfig删除模拟环境中的指定配置。
    pub async fn delete_specific_staging_config(
        &self,
        request: DeleteSpecificStagingConfigRequest,
    ) -> Result<DeleteSpecificStagingConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteSpecificStagingConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用SetWaitingRoomConfig设置waiting_room功能，只支持全站加速类型域名。
    pub async fn set_waiting_room_config(
        &self,
        request: SetWaitingRoomConfigRequest,
    ) -> Result<SetWaitingRoomConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SetWaitingRoomConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeCdnUserConfigs查询安全功能相关的配置。
    pub async fn describe_cdn_user_configs(
        &self,
        request: DescribeCdnUserConfigsRequest,
    ) -> Result<DescribeCdnUserConfigsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCdnUserConfigs",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeCdnWafDomain查询WAF全量域名列表。
    pub async fn describe_cdn_waf_domain(
        &self,
        request: DescribeCdnWafDomainRequest,
    ) -> Result<DescribeCdnWafDomainResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCdnWafDomain",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeBlockedRegions查询区域封禁支持的国家和地区。
    pub async fn describe_blocked_regions(
        &self,
        request: DescribeBlockedRegionsRequest,
    ) -> Result<DescribeBlockedRegionsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeBlockedRegions",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 当用户设置了缓存标签功能后，通过调用RefreshObjectCacheByCacheTag指定缓存标签对缓存进行刷新。
    pub async fn refresh_object_cache_by_cache_tag(
        &self,
        request: RefreshObjectCacheByCacheTagRequest,
    ) -> Result<RefreshObjectCacheByCacheTagResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RefreshObjectCacheByCacheTag",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeRefreshQuota查询当天URL刷新、目录刷新、预热及封禁的最大限制数量和剩余量。
    pub async fn describe_refresh_quota(
        &self,
        request: DescribeRefreshQuotaRequest,
    ) -> Result<DescribeRefreshQuotaResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeRefreshQuota",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 将源站的内容主动预热到缓存节点上。您首次访问可直接命中缓存，缓解源站压力。
    pub async fn push_object_cache(
        &self,
        request: PushObjectCacheRequest,
    ) -> Result<PushObjectCacheResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PushObjectCache",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 刷新节点上的文件内容。被刷新的文件缓存将立即失效，新的请求将回源获取最新的文件，支持URL批量刷新。
    pub async fn refresh_object_caches(
        &self,
        request: RefreshObjectCachesRequest,
    ) -> Result<RefreshObjectCachesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RefreshObjectCaches",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeRefreshTasks查询刷新、预热状态是否在全网生效。
    pub async fn describe_refresh_tasks(
        &self,
        request: DescribeRefreshTasksRequest,
    ) -> Result<DescribeRefreshTasksResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeRefreshTasks",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeRefreshTaskById查询刷新状态和预热状态是否在全网生效。
    pub async fn describe_refresh_task_by_id(
        &self,
        request: DescribeRefreshTaskByIdRequest,
    ) -> Result<DescribeRefreshTaskByIdResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeRefreshTaskById",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询任务预热详情，包括任务下所有资源的预热进度。该接口需要申请白名单后才可以使用，带宽大于100Gbps的情况下，可以通过您的商务经理申请白名单。
    pub async fn describe_preload_detail_by_id(
        &self,
        request: DescribePreloadDetailByIdRequest,
    ) -> Result<DescribePreloadDetailByIdResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribePreloadDetailById",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeCdnUserQuota查询用户配额上限和余量信息。
    pub async fn describe_cdn_user_quota(
        &self,
        request: DescribeCdnUserQuotaRequest,
    ) -> Result<DescribeCdnUserQuotaResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCdnUserQuota",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDomainPathData按目录维度获取监控数据，包括流量和访问次数。
    pub async fn describe_domain_path_data(
        &self,
        request: DescribeDomainPathDataRequest,
    ) -> Result<DescribeDomainPathDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainPathData",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDomainQpsData获取5分钟计算粒度加速域名的每秒访问次数QPS，支持获取最近90天的数据。
    pub async fn describe_domain_qps_data(
        &self,
        request: DescribeDomainQpsDataRequest,
    ) -> Result<DescribeDomainQpsDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainQpsData",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDomainQpsDataByLayer查询加速域名的每秒访问次数QPS，支持获取最近90天的数据。
    pub async fn describe_domain_qps_data_by_layer(
        &self,
        request: DescribeDomainQpsDataByLayerRequest,
    ) -> Result<DescribeDomainQpsDataByLayerResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainQpsDataByLayer",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDomainBpsData查询带宽数据。
    pub async fn describe_domain_bps_data(
        &self,
        request: DescribeDomainBpsDataRequest,
    ) -> Result<DescribeDomainBpsDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainBpsData",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDomainBpsDataByLayer按协议分类获取带宽数据。
    pub async fn describe_domain_bps_data_by_layer(
        &self,
        request: DescribeDomainBpsDataByLayerRequest,
    ) -> Result<DescribeDomainBpsDataByLayerResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainBpsDataByLayer",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDomainBpsDataByTimeStamp查询加速域名的在某个时刻不同运营商和区域的带宽数据。
    pub async fn describe_domain_bps_data_by_time_stamp(
        &self,
        request: DescribeDomainBpsDataByTimeStampRequest,
    ) -> Result<DescribeDomainBpsDataByTimeStampResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainBpsDataByTimeStamp",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDomainTrafficData查询加速域名的网络流量监控数据，支持获取最近90天的数据。
    pub async fn describe_domain_traffic_data(
        &self,
        request: DescribeDomainTrafficDataRequest,
    ) -> Result<DescribeDomainTrafficDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainTrafficData",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDomainHttpCodeData查询加速域名5分钟粒度的HTTP返回码总数和占比数据。
    pub async fn describe_domain_http_code_data(
        &self,
        request: DescribeDomainHttpCodeDataRequest,
    ) -> Result<DescribeDomainHttpCodeDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainHttpCodeData",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDomainHttpCodeDataByLayer按协议分类获取HTTP状态码数据。
    pub async fn describe_domain_http_code_data_by_layer(
        &self,
        request: DescribeDomainHttpCodeDataByLayerRequest,
    ) -> Result<DescribeDomainHttpCodeDataByLayerResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainHttpCodeDataByLayer",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDomainHitRateData查询字节命中率（命中字节百分比）。
    pub async fn describe_domain_hit_rate_data(
        &self,
        request: DescribeDomainHitRateDataRequest,
    ) -> Result<DescribeDomainHitRateDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainHitRateData",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDomainReqHitRateData查询请求命中率（命中请求百分比）。
    pub async fn describe_domain_req_hit_rate_data(
        &self,
        request: DescribeDomainReqHitRateDataRequest,
    ) -> Result<DescribeDomainReqHitRateDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainReqHitRateData",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDomainsUsageByDay查询加速域名天粒度的监控统计数据，支持获取最近90天的数据。
    pub async fn describe_domains_usage_by_day(
        &self,
        request: DescribeDomainsUsageByDayRequest,
    ) -> Result<DescribeDomainsUsageByDayResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainsUsageByDay",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDomainDetailDataByLayer按协议类型获取加速域名的数据明细（域名维度）。
    pub async fn describe_domain_detail_data_by_layer(
        &self,
        request: DescribeDomainDetailDataByLayerRequest,
    ) -> Result<DescribeDomainDetailDataByLayerResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainDetailDataByLayer",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeRangeDataByLocateAndIspService获取按运营商和地域获取带宽数据。
    pub async fn describe_range_data_by_locate_and_isp_service(
        &self,
        request: DescribeRangeDataByLocateAndIspServiceRequest,
    ) -> Result<DescribeRangeDataByLocateAndIspServiceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeRangeDataByLocateAndIspService",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDomainSrcBpsData查询加速域名的回源带宽监控数据。
    pub async fn describe_domain_src_bps_data(
        &self,
        request: DescribeDomainSrcBpsDataRequest,
    ) -> Result<DescribeDomainSrcBpsDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainSrcBpsData",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDomainSrcHttpCodeData查询加速域名回源HTTP返回码的总数和占比数据。
    pub async fn describe_domain_src_http_code_data(
        &self,
        request: DescribeDomainSrcHttpCodeDataRequest,
    ) -> Result<DescribeDomainSrcHttpCodeDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainSrcHttpCodeData",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDomainSrcTrafficData查询加速域名的回源流量监控数据。
    pub async fn describe_domain_src_traffic_data(
        &self,
        request: DescribeDomainSrcTrafficDataRequest,
    ) -> Result<DescribeDomainSrcTrafficDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainSrcTrafficData",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDomainSrcQpsData查询回源QPS数据，支持获取最近90天的数据。
    pub async fn describe_domain_src_qps_data(
        &self,
        request: DescribeDomainSrcQpsDataRequest,
    ) -> Result<DescribeDomainSrcQpsDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainSrcQpsData",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDomainRealTimeTrafficData查询加速域名的流量监控数据。
    pub async fn describe_domain_real_time_traffic_data(
        &self,
        request: DescribeDomainRealTimeTrafficDataRequest,
    ) -> Result<DescribeDomainRealTimeTrafficDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainRealTimeTrafficData",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDomainRealTimeBpsData查询加速域名的带宽数据。
    pub async fn describe_domain_real_time_bps_data(
        &self,
        request: DescribeDomainRealTimeBpsDataRequest,
    ) -> Result<DescribeDomainRealTimeBpsDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainRealTimeBpsData",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDomainRealTimeHttpCodeData查询加速域名HTTP返回码的总数和占比数据。
    pub async fn describe_domain_real_time_http_code_data(
        &self,
        request: DescribeDomainRealTimeHttpCodeDataRequest,
    ) -> Result<DescribeDomainRealTimeHttpCodeDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainRealTimeHttpCodeData",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDomainRealTimeQpsData查询加速域名每秒访问次数数据。
    pub async fn describe_domain_real_time_qps_data(
        &self,
        request: DescribeDomainRealTimeQpsDataRequest,
    ) -> Result<DescribeDomainRealTimeQpsDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainRealTimeQpsData",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDomainRealTimeReqHitRateData查询加速域名请求命中率数据。
    pub async fn describe_domain_real_time_req_hit_rate_data(
        &self,
        request: DescribeDomainRealTimeReqHitRateDataRequest,
    ) -> Result<DescribeDomainRealTimeReqHitRateDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainRealTimeReqHitRateData",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDomainRealTimeByteHitRateData查询域名字节命中率数据。
    pub async fn describe_domain_real_time_byte_hit_rate_data(
        &self,
        request: DescribeDomainRealTimeByteHitRateDataRequest,
    ) -> Result<DescribeDomainRealTimeByteHitRateDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainRealTimeByteHitRateData",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDomainRealTimeSrcBpsData查询域名回源带宽数据。
    pub async fn describe_domain_real_time_src_bps_data(
        &self,
        request: DescribeDomainRealTimeSrcBpsDataRequest,
    ) -> Result<DescribeDomainRealTimeSrcBpsDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainRealTimeSrcBpsData",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDomainRealTimeSrcHttpCodeData查询加速域名回源HTTP返回码的总数和占比数据。
    pub async fn describe_domain_real_time_src_http_code_data(
        &self,
        request: DescribeDomainRealTimeSrcHttpCodeDataRequest,
    ) -> Result<DescribeDomainRealTimeSrcHttpCodeDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainRealTimeSrcHttpCodeData",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDomainRealTimeSrcTrafficData查询加速域名1分钟粒度的回源流量监控数据。
    pub async fn describe_domain_real_time_src_traffic_data(
        &self,
        request: DescribeDomainRealTimeSrcTrafficDataRequest,
    ) -> Result<DescribeDomainRealTimeSrcTrafficDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainRealTimeSrcTrafficData",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeEsExecuteData查询ES规则的运行情况。
    pub async fn describe_es_execute_data(
        &self,
        request: DescribeEsExecuteDataRequest,
    ) -> Result<DescribeEsExecuteDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeEsExecuteData",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeEsExceptionData查询ES规则执行的异常情况。
    pub async fn describe_es_exception_data(
        &self,
        request: DescribeEsExceptionDataRequest,
    ) -> Result<DescribeEsExceptionDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeEsExceptionData",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeCdnUserBillHistory查询用户历史账单。
    pub async fn describe_cdn_user_bill_history(
        &self,
        request: DescribeCdnUserBillHistoryRequest,
    ) -> Result<DescribeCdnUserBillHistoryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCdnUserBillHistory",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeCdnUserBillType查询用户计费类型，最多查询一个月的数据。
    pub async fn describe_cdn_user_bill_type(
        &self,
        request: DescribeCdnUserBillTypeRequest,
    ) -> Result<DescribeCdnUserBillTypeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCdnUserBillType",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeCdnUserBillPrediction预测用户当月的用量数据。
    pub async fn describe_cdn_user_bill_prediction(
        &self,
        request: DescribeCdnUserBillPredictionRequest,
    ) -> Result<DescribeCdnUserBillPredictionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCdnUserBillPrediction",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateUserUsageDataExportTask创建账号历史用量数据导出任务，将历史用量生成PDF文件用于下载。
    pub async fn create_user_usage_data_export_task(
        &self,
        request: CreateUserUsageDataExportTaskRequest,
    ) -> Result<CreateUserUsageDataExportTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateUserUsageDataExportTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeUserUsageDataExportTask查询用户最近三个月的用量导出任务信息。
    pub async fn describe_user_usage_data_export_task(
        &self,
        request: DescribeUserUsageDataExportTaskRequest,
    ) -> Result<DescribeUserUsageDataExportTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeUserUsageDataExportTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteUserUsageDataExportTask删除账号历史用量数据导出任务。
    pub async fn delete_user_usage_data_export_task(
        &self,
        request: DeleteUserUsageDataExportTaskRequest,
    ) -> Result<DeleteUserUsageDataExportTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteUserUsageDataExportTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateUsageDetailDataExportTask创建用量明细数据导出任务，将详细用量生成Excel文件用于下载。
    pub async fn create_usage_detail_data_export_task(
        &self,
        request: CreateUsageDetailDataExportTaskRequest,
    ) -> Result<CreateUsageDetailDataExportTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateUsageDetailDataExportTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeUserUsageDetailDataExportTask查询您账户下单个或多个域名5分钟明细数据的导出任务。
    pub async fn describe_user_usage_detail_data_export_task(
        &self,
        request: DescribeUserUsageDetailDataExportTaskRequest,
    ) -> Result<DescribeUserUsageDetailDataExportTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeUserUsageDetailDataExportTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteUsageDetailDataExportTask删除用量明细数据导出任务。
    pub async fn delete_usage_detail_data_export_task(
        &self,
        request: DeleteUsageDetailDataExportTaskRequest,
    ) -> Result<DeleteUsageDetailDataExportTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteUsageDetailDataExportTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDomainMax95BpsData查询加速域名95带宽峰值监控数据。
    pub async fn describe_domain_max95_bps_data(
        &self,
        request: DescribeDomainMax95BpsDataRequest,
    ) -> Result<DescribeDomainMax95BpsDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainMax95BpsData",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDomainUsageData查询域名在特定计费区域的用量数据。
    pub async fn describe_domain_usage_data(
        &self,
        request: DescribeDomainUsageDataRequest,
    ) -> Result<DescribeDomainUsageDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainUsageData",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeCdnUserResourcePackage查询CDN用户当前资源包信息。
    pub async fn describe_cdn_user_resource_package(
        &self,
        request: DescribeCdnUserResourcePackageRequest,
    ) -> Result<DescribeCdnUserResourcePackageResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCdnUserResourcePackage",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteRealTimeLogLogstore删除实时日志推送的Logstore。
    pub async fn delete_real_time_log_logstore(
        &self,
        request: DeleteRealTimeLogLogstoreRequest,
    ) -> Result<DeleteRealTimeLogLogstoreResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteRealTimeLogLogstore",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateRealTimeLogDelivery创建域名实时日志投递。
    pub async fn create_real_time_log_delivery(
        &self,
        request: CreateRealTimeLogDeliveryRequest,
    ) -> Result<CreateRealTimeLogDeliveryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateRealTimeLogDelivery",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyRealtimeLogDelivery更改域名实时日志投递，一个域名仅支持投递单个Logstore。
    pub async fn modify_realtime_log_delivery(
        &self,
        request: ModifyRealtimeLogDeliveryRequest,
    ) -> Result<ModifyRealtimeLogDeliveryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyRealtimeLogDelivery",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteRealtimeLogDelivery删除实时日志推送域名。
    pub async fn delete_realtime_log_delivery(
        &self,
        request: DeleteRealtimeLogDeliveryRequest,
    ) -> Result<DeleteRealtimeLogDeliveryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteRealtimeLogDelivery",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DisableRealtimeLogDelivery暂停域名实时日志投递。
    pub async fn disable_realtime_log_delivery(
        &self,
        request: DisableRealtimeLogDeliveryRequest,
    ) -> Result<DisableRealtimeLogDeliveryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DisableRealtimeLogDelivery",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用EnableRealtimeLogDelivery开启域名实时日志投递。
    pub async fn enable_realtime_log_delivery(
        &self,
        request: EnableRealtimeLogDeliveryRequest,
    ) -> Result<EnableRealtimeLogDeliveryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "EnableRealtimeLogDelivery",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询用户下所有实时日志投递。
    pub async fn list_realtime_log_delivery(
        &self,
        request: ListRealtimeLogDeliveryRequest,
    ) -> Result<ListRealtimeLogDeliveryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListRealtimeLogDelivery",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDomainRealtimeLogDelivery查询域名实时日志投递信息。
    pub async fn describe_domain_realtime_log_delivery(
        &self,
        request: DescribeDomainRealtimeLogDeliveryRequest,
    ) -> Result<DescribeDomainRealtimeLogDeliveryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainRealtimeLogDelivery",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeRealtimeDeliveryAcc查询实时日志投递次数。
    pub async fn describe_realtime_delivery_acc(
        &self,
        request: DescribeRealtimeDeliveryAccRequest,
    ) -> Result<DescribeRealtimeDeliveryAccResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeRealtimeDeliveryAcc",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ListRealtimeLogDeliveryInfos查询所有实时日志投递服务信息。
    pub async fn list_realtime_log_delivery_infos(
        &self,
        request: ListRealtimeLogDeliveryInfosRequest,
    ) -> Result<ListRealtimeLogDeliveryInfosResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListRealtimeLogDeliveryInfos",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ListRealtimeLogDeliveryDomains查询实时日志投递服务下所有域名。
    pub async fn list_realtime_log_delivery_domains(
        &self,
        request: ListRealtimeLogDeliveryDomainsRequest,
    ) -> Result<ListRealtimeLogDeliveryDomainsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListRealtimeLogDeliveryDomains",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDomainCustomLogConfig查询域名自定义日志格式配置信息。
    pub async fn describe_domain_custom_log_config(
        &self,
        request: DescribeDomainCustomLogConfigRequest,
    ) -> Result<DescribeDomainCustomLogConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainCustomLogConfig",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeCustomLogConfig根据configId查询自定义日志配置详细信息。
    pub async fn describe_custom_log_config(
        &self,
        request: DescribeCustomLogConfigRequest,
    ) -> Result<DescribeCustomLogConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCustomLogConfig",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeCdnDomainLogs查询域名的离线日志下载地址。
    pub async fn describe_cdn_domain_logs(
        &self,
        request: DescribeCdnDomainLogsRequest,
    ) -> Result<DescribeCdnDomainLogsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCdnDomainLogs",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ListDomainsByLogConfigId按日志配置ID查询域名列表。
    pub async fn list_domains_by_log_config_id(
        &self,
        request: ListDomainsByLogConfigIdRequest,
    ) -> Result<ListDomainsByLogConfigIdResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListDomainsByLogConfigId",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ListUserCustomLogConfig查询用户下所有自定义日志配置信息。
    pub async fn list_user_custom_log_config(
        &self,
        request: ListUserCustomLogConfigRequest,
    ) -> Result<ListUserCustomLogConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListUserCustomLogConfig",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用OpenCdnService开通CDN服务。开通服务后才能进行域名操作。
    pub async fn open_cdn_service(
        &self,
        request: OpenCdnServiceRequest,
    ) -> Result<OpenCdnServiceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "OpenCdnService",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeCdnRegionAndIsp查询地域和运营商列表。
    pub async fn describe_cdn_region_and_isp(
        &self,
        request: DescribeCdnRegionAndIspRequest,
    ) -> Result<DescribeCdnRegionAndIspResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCdnRegionAndIsp",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeCdnOrderCommodityCode查询客户UID对应的商品Code。
    pub async fn describe_cdn_order_commodity_code(
        &self,
        request: DescribeCdnOrderCommodityCodeRequest,
    ) -> Result<DescribeCdnOrderCommodityCodeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCdnOrderCommodityCode",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeCdnService查询CDN服务状态。包括：当前计费类型、服务开通时间、下次生效的计费类型、当前业务状态等。
    pub async fn describe_cdn_service(
        &self,
        request: DescribeCdnServiceRequest,
    ) -> Result<DescribeCdnServiceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCdnService",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeCdnDomainByCertificate根据证书信息获取加速域名。
    pub async fn describe_cdn_domain_by_certificate(
        &self,
        request: DescribeCdnDomainByCertificateRequest,
    ) -> Result<DescribeCdnDomainByCertificateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCdnDomainByCertificate",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDomainCertificateInfo查询指定加速域名证书信息。
    pub async fn describe_domain_certificate_info(
        &self,
        request: DescribeDomainCertificateInfoRequest,
    ) -> Result<DescribeDomainCertificateInfoResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainCertificateInfo",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeCdnCertificateDetail查询CDN证书详细信息。
    pub async fn describe_cdn_certificate_detail(
        &self,
        request: DescribeCdnCertificateDetailRequest,
    ) -> Result<DescribeCdnCertificateDetailResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCdnCertificateDetail",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeCdnCertificateList按照域名查询证书列表信息。
    pub async fn describe_cdn_certificate_list(
        &self,
        request: DescribeCdnCertificateListRequest,
    ) -> Result<DescribeCdnCertificateListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCdnCertificateList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeCdnSSLCertificateList按照域名查询证书列表信息。
    pub async fn describe_cdn_ssl_certificate_list(
        &self,
        request: DescribeCdnSSLCertificateListRequest,
    ) -> Result<DescribeCdnSSLCertificateListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCdnSSLCertificateList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeCdnHttpsDomainList查询用户所有证书信息。
    pub async fn describe_cdn_https_domain_list(
        &self,
        request: DescribeCdnHttpsDomainListRequest,
    ) -> Result<DescribeCdnHttpsDomainListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCdnHttpsDomainList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeCertificateInfoByID按照证书ID查询指定证书信息。
    pub async fn describe_certificate_info_by_id(
        &self,
        request: DescribeCertificateInfoByIDRequest,
    ) -> Result<DescribeCertificateInfoByIDResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCertificateInfoByID",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeCdnCertificateDetailById根据证书ID获取证书详细信息。
    pub async fn describe_cdn_certificate_detail_by_id(
        &self,
        request: DescribeCdnCertificateDetailByIdRequest,
    ) -> Result<DescribeCdnCertificateDetailByIdResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCdnCertificateDetailById",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeUserCertificateExpireCount查询用户证书过期的域名数。
    pub async fn describe_user_certificate_expire_count(
        &self,
        request: DescribeUserCertificateExpireCountRequest,
    ) -> Result<DescribeUserCertificateExpireCountResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeUserCertificateExpireCount",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateCdnCertificateSigningRequest创建 CSR（证书签名请求）文件。
    pub async fn create_cdn_certificate_signing_request(
        &self,
        request: CreateCdnCertificateSigningRequestRequest,
    ) -> Result<CreateCdnCertificateSigningRequestResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateCdnCertificateSigningRequest",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用SetCdnDomainCSRCertificate设置指定域名下的HTTPS证书。
    pub async fn set_cdn_domain_csr_certificate(
        &self,
        request: SetCdnDomainCSRCertificateRequest,
    ) -> Result<SetCdnDomainCSRCertificateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SetCdnDomainCSRCertificate",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用SetCdnDomainSMCertificate设置某域名下国密证书功能是否启用。
    pub async fn set_cdn_domain_sm_certificate(
        &self,
        request: SetCdnDomainSMCertificateRequest,
    ) -> Result<SetCdnDomainSMCertificateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SetCdnDomainSMCertificate",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeCdnSMCertificateList查询指定加速域名下国密证书列表信息。
    pub async fn describe_cdn_sm_certificate_list(
        &self,
        request: DescribeCdnSMCertificateListRequest,
    ) -> Result<DescribeCdnSMCertificateListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCdnSMCertificateList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeCdnSMCertificateDetail查询国密证书的详细信息。
    pub async fn describe_cdn_sm_certificate_detail(
        &self,
        request: DescribeCdnSMCertificateDetailRequest,
    ) -> Result<DescribeCdnSMCertificateDetailResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCdnSMCertificateDetail",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用SetCdnDomainSSLCertificate用于设置某域名下证书功能是否启用及更新证书信息。
    pub async fn set_cdn_domain_ssl_certificate(
        &self,
        request: SetCdnDomainSSLCertificateRequest,
    ) -> Result<SetCdnDomainSSLCertificateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SetCdnDomainSSLCertificate",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用TagResources添加资源标签。
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

    /// 调用DescribeTagResources查询资源对应的标签。
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

    /// 调用DescribeUserTags查询用户标签。
    pub async fn describe_user_tags(
        &self,
        request: DescribeUserTagsRequest,
    ) -> Result<DescribeUserTagsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeUserTags",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用UntagResources删除资源标签。
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

    /// 调用ListTagResources查询资源标签列表。
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

    /// 调用CreateCdnSubTask定制运营报表，定制后可查看定制报表数据。
    pub async fn create_cdn_sub_task(
        &self,
        request: CreateCdnSubTaskRequest,
    ) -> Result<CreateCdnSubTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateCdnSubTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeCdnSubList查询已定制的报表任务。
    pub async fn describe_cdn_sub_list(
        &self,
        request: DescribeCdnSubListRequest,
    ) -> Result<DescribeCdnSubListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCdnSubList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用UpdateCdnSubTask更新已定制的运营报表。
    pub async fn update_cdn_sub_task(
        &self,
        request: UpdateCdnSubTaskRequest,
    ) -> Result<UpdateCdnSubTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateCdnSubTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteCdnSubTask删除所有已定制的运营报表。
    pub async fn delete_cdn_sub_task(
        &self,
        request: DeleteCdnSubTaskRequest,
    ) -> Result<DeleteCdnSubTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteCdnSubTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeCdnReport查询已定制的报表数据。
    pub async fn describe_cdn_report(
        &self,
        request: DescribeCdnReportRequest,
    ) -> Result<DescribeCdnReportResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCdnReport",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询所有的定制报表列表。
    pub async fn describe_cdn_report_list(
        &self,
        request: DescribeCdnReportListRequest,
    ) -> Result<DescribeCdnReportListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCdnReportList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateCdnDeliverTask创建报表订阅任务，订阅成功后系统会周期性地以邮件方式向您发送报表统计数据。
    pub async fn create_cdn_deliver_task(
        &self,
        request: CreateCdnDeliverTaskRequest,
    ) -> Result<CreateCdnDeliverTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateCdnDeliverTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeCdnDeliverList查询报表订阅任务列表，系统将返回该用户所有的订阅任务列表。
    pub async fn describe_cdn_deliver_list(
        &self,
        request: DescribeCdnDeliverListRequest,
    ) -> Result<DescribeCdnDeliverListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCdnDeliverList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用UpdateCdnDeliverTask根据订阅任务ID更新已订阅的报表任务。
    pub async fn update_cdn_deliver_task(
        &self,
        request: UpdateCdnDeliverTaskRequest,
    ) -> Result<UpdateCdnDeliverTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateCdnDeliverTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteCdnDeliverTask根据任务ID删除已订阅的报表任务。
    pub async fn delete_cdn_deliver_task(
        &self,
        request: DeleteCdnDeliverTaskRequest,
    ) -> Result<DeleteCdnDeliverTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteCdnDeliverTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeIpInfo验证指定的IP是否为阿里云CDN节点的IP地址。
    pub async fn describe_ip_info(
        &self,
        request: DescribeIpInfoRequest,
    ) -> Result<DescribeIpInfoResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeIpInfo",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeStagingIp查询模拟环境的节点IP地址。
    pub async fn describe_staging_ip(
        &self,
        request: DescribeStagingIpRequest,
    ) -> Result<DescribeStagingIpResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeStagingIp",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeL2VipsByDomain查询指定域名的L2节点的IP地址。
    pub async fn describe_l2_vips_by_domain(
        &self,
        request: DescribeL2VipsByDomainRequest,
    ) -> Result<DescribeL2VipsByDomainResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeL2VipsByDomain",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeUserVipsByDomain查询域名的IP列表。
    pub async fn describe_user_vips_by_domain(
        &self,
        request: DescribeUserVipsByDomainRequest,
    ) -> Result<DescribeUserVipsByDomainResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeUserVipsByDomain",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeIpStatus查询节点IP可用状态，进而根据节点IP可用状态来了解节点加速服务的可用状态。
    pub async fn describe_ip_status(
        &self,
        request: DescribeIpStatusRequest,
    ) -> Result<DescribeIpStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeIpStatus",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用AddFCTrigger添加指定的函数计算触发器。
    pub async fn add_fc_trigger(
        &self,
        request: AddFCTriggerRequest,
    ) -> Result<AddFCTriggerResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddFCTrigger",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用UpdateFCTrigger更新指定的函数计算触发器。
    pub async fn update_fc_trigger(
        &self,
        request: UpdateFCTriggerRequest,
    ) -> Result<UpdateFCTriggerResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateFCTrigger",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeFCTrigger查询指定的函数计算触发器。
    pub async fn describe_fc_trigger(
        &self,
        request: DescribeFCTriggerRequest,
    ) -> Result<DescribeFCTriggerResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeFCTrigger",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteFCTrigger删除指定的函数计算触发器。
    pub async fn delete_fc_trigger(
        &self,
        request: DeleteFCTriggerRequest,
    ) -> Result<DeleteFCTriggerResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteFCTrigger",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ListFCTrigger查询指定事件的函数计算触发器列表。
    pub async fn list_fc_trigger(
        &self,
        request: ListFCTriggerRequest,
    ) -> Result<ListFCTriggerResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListFCTrigger",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDomainCcActivityLog查询频次控制规则拦截日志。
    pub async fn describe_domain_cc_activity_log(
        &self,
        request: DescribeDomainCcActivityLogRequest,
    ) -> Result<DescribeDomainCcActivityLogResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainCcActivityLog",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDomainPvData按小时粒度获取加速域名的PV页面访问统计，支持获取最近90天的数据。
    pub async fn describe_domain_pv_data(
        &self,
        request: DescribeDomainPvDataRequest,
    ) -> Result<DescribeDomainPvDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainPvData",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDomainUvData按小时粒度获取加速域名的UV页面独立访问统计，支持获取最近90天的数据。
    pub async fn describe_domain_uv_data(
        &self,
        request: DescribeDomainUvDataRequest,
    ) -> Result<DescribeDomainUvDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainUvData",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取加速域名某个指定时间段内的热门URL列表（TOP100）。
    pub async fn describe_domain_top_url_visit(
        &self,
        request: DescribeDomainTopUrlVisitRequest,
    ) -> Result<DescribeDomainTopUrlVisitResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainTopUrlVisit",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDomainTopClientIpVisit查询加速域名在指定时间范围内按照访问次数或流量排序的客户端IP排行，支持获取最近90天的数据。
    pub async fn describe_domain_top_client_ip_visit(
        &self,
        request: DescribeDomainTopClientIpVisitRequest,
    ) -> Result<DescribeDomainTopClientIpVisitResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainTopClientIpVisit",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDomainTopReferVisit获取加速域名某天的热门页面引用次数排名，支持获取最近90天的数据。
    pub async fn describe_domain_top_refer_visit(
        &self,
        request: DescribeDomainTopReferVisitRequest,
    ) -> Result<DescribeDomainTopReferVisitResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainTopReferVisit",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDomainSrcTopUrlVisit获取加速域名的回源热门URL。
    pub async fn describe_domain_src_top_url_visit(
        &self,
        request: DescribeDomainSrcTopUrlVisitRequest,
    ) -> Result<DescribeDomainSrcTopUrlVisitResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainSrcTopUrlVisit",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeTopDomainsByFlow获取用户按流量排名的域名，支持获取最近30天的数据。
    pub async fn describe_top_domains_by_flow(
        &self,
        request: DescribeTopDomainsByFlowRequest,
    ) -> Result<DescribeTopDomainsByFlowResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeTopDomainsByFlow",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDomainRegionData获取加速域名天粒度的用户区域分布数据统计，支持获取最近90天的数据。
    pub async fn describe_domain_region_data(
        &self,
        request: DescribeDomainRegionDataRequest,
    ) -> Result<DescribeDomainRegionDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainRegionData",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDomainISPData查询加速域名天粒度的用户运营商分布数据统计，支持获取最近90天的数据。
    pub async fn describe_domain_isp_data(
        &self,
        request: DescribeDomainISPDataRequest,
    ) -> Result<DescribeDomainISPDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainISPData",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调DescribeDomainAverageResponseTime获取加速域名的平均响应时间，支持获取最近90天的数据
    pub async fn describe_domain_average_response_time(
        &self,
        request: DescribeDomainAverageResponseTimeRequest,
    ) -> Result<DescribeDomainAverageResponseTimeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainAverageResponseTime",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDomainRealTimeDetailData批量获取域名实时监控详细数据。
    pub async fn describe_domain_real_time_detail_data(
        &self,
        request: DescribeDomainRealTimeDetailDataRequest,
    ) -> Result<DescribeDomainRealTimeDetailDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainRealTimeDetailData",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDomainMultiUsageData批量查询加速域名的流量和请求数类五分钟粒度的数据。
    pub async fn describe_domain_multi_usage_data(
        &self,
        request: DescribeDomainMultiUsageDataRequest,
    ) -> Result<DescribeDomainMultiUsageDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDomainMultiUsageData",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeUserConfigs查询安全功能相关的配置。
    pub async fn describe_user_configs(
        &self,
        request: DescribeUserConfigsRequest,
    ) -> Result<DescribeUserConfigsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeUserConfigs",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用SetReqHeaderConfig设置回源自定义头。
    pub async fn set_req_header_config(
        &self,
        request: SetReqHeaderConfigRequest,
    ) -> Result<SetReqHeaderConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SetReqHeaderConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 变更CDN服务的计费类型。
    pub async fn modify_cdn_service(
        &self,
        request: ModifyCdnServiceRequest,
    ) -> Result<ModifyCdnServiceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyCdnService",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeUserCdnStatus获取用户状态信息。
    pub async fn describe_user_cdn_status(
        &self,
        request: DescribeUserCdnStatusRequest,
    ) -> Result<DescribeUserCdnStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeUserCdnStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeCdnTypes获取域名类型列表。
    pub async fn describe_cdn_types(
        &self,
        request: DescribeCdnTypesRequest,
    ) -> Result<DescribeCdnTypesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCdnTypes",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyCdnDomainOwner将其他阿里云账号下的域名迁移至本账号。
    pub async fn modify_cdn_domain_owner(
        &self,
        request: ModifyCdnDomainOwnerRequest,
    ) -> Result<ModifyCdnDomainOwnerResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyCdnDomainOwner",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeCdnConditionIPBInfo查看高级条件所需要的运营商、地区和国家信息。
    pub async fn describe_cdn_condition_ipb_info(
        &self,
        request: DescribeCdnConditionIPBInfoRequest,
    ) -> Result<DescribeCdnConditionIPBInfoResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCdnConditionIPBInfo",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeCdnSecFuncInfo查询CDN应用安全具体功能信息。
    pub async fn describe_cdn_sec_func_info(
        &self,
        request: DescribeCdnSecFuncInfoRequest,
    ) -> Result<DescribeCdnSecFuncInfoResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCdnSecFuncInfo",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CheckCdnDomainExist检测域名是否存在。
    pub async fn check_cdn_domain_exist(
        &self,
        request: CheckCdnDomainExistRequest,
    ) -> Result<CheckCdnDomainExistResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CheckCdnDomainExist",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CheckCdnDomainICP查询域名是否备案。
    pub async fn check_cdn_domain_icp(
        &self,
        request: CheckCdnDomainICPRequest,
    ) -> Result<CheckCdnDomainICPResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CheckCdnDomainICP",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用BatchDescribeCdnIpInfo批量查询IP是否为阿里云CDN服务IP。
    pub async fn batch_describe_cdn_ip_info(
        &self,
        request: BatchDescribeCdnIpInfoRequest,
    ) -> Result<BatchDescribeCdnIpInfoResponse, SdkError> {
        let api_request = ApiRequest {
            action: "BatchDescribeCdnIpInfo",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeCdnFullDomainsBlockIPHistory获取封禁历史记录。
    pub async fn describe_cdn_full_domains_block_ip_history(
        &self,
        request: DescribeCdnFullDomainsBlockIPHistoryRequest,
    ) -> Result<DescribeCdnFullDomainsBlockIPHistoryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCdnFullDomainsBlockIPHistory",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置封禁的IP或解禁IP，该操作对您账号下所有的CDN域名同时生效。
    pub async fn set_cdn_full_domains_block_ip(
        &self,
        request: SetCdnFullDomainsBlockIPRequest,
    ) -> Result<SetCdnFullDomainsBlockIPResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SetCdnFullDomainsBlockIP",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeCdnFullDomainsBlockIPConfig查询全量封禁的相关配置。
    pub async fn describe_cdn_full_domains_block_ip_config(
        &self,
        request: DescribeCdnFullDomainsBlockIPConfigRequest,
    ) -> Result<DescribeCdnFullDomainsBlockIPConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCdnFullDomainsBlockIPConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

}