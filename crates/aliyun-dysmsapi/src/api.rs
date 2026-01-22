//! API 调用 - 自动生成，请勿手动修改

use aliyun_sdk_core::{Client, ClientConfig, ApiRequest, HttpMethod, SdkError};
use super::types::*;

/// Dysmsapi API 版本
pub const API_VERSION: &str = "2018-05-01";

/// Dysmsapi 客户端
#[derive(Debug, Clone)]
pub struct DysmsapiClient {
    client: Client,
}

impl DysmsapiClient {
    /// 创建新客户端
    pub fn new(config: ClientConfig) -> Result<Self, SdkError> {
        Ok(Self {
            client: Client::new(config)?,
        })
    }

    /// 批量发送短信到中国境外。
    pub async fn batch_send_message_to_globe(
        &self,
        request: BatchSendMessageToGlobeRequest,
    ) -> Result<BatchSendMessageToGlobeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "BatchSendMessageToGlobe",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 使用短信模板发送短信，只支持发往中国内地。
    pub async fn send_message_with_template(
        &self,
        request: SendMessageWithTemplateRequest,
    ) -> Result<SendMessageWithTemplateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SendMessageWithTemplate",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 发送短信到中国香港、中国澳门、中国台湾以及中国境外地区。
    pub async fn send_message_to_globe(
        &self,
        request: SendMessageToGlobeRequest,
    ) -> Result<SendMessageToGlobeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SendMessageToGlobe",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 使用短信MessageId查询短信发送记录。
    pub async fn query_message(
        &self,
        request: QueryMessageRequest,
    ) -> Result<QueryMessageResponse, SdkError> {
        let api_request = ApiRequest {
            action: "QueryMessage",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 将短信转化率统计数据反馈给阿里云国际短信平台。
    pub async fn conversion_data(
        &self,
        request: ConversionDataRequest,
    ) -> Result<ConversionDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ConversionData",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 将每一条消息ID(MessageId) 对应短信的接收情况反馈给阿里云国际短信平台。
    pub async fn sms_conversion(
        &self,
        request: SmsConversionRequest,
    ) -> Result<SmsConversionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SmsConversion",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

}