//! 类型定义 - 自动生成，请勿手动修改

use serde::{Deserialize, Serialize};

/// 号码明细。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendMessageWithTemplateResponseNumberDetail {
    /// 号码所属的运营商网络。
    #[serde(rename = "Carrier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,
    /// 号码所属地区。
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// 号码所属国家。
    #[serde(rename = "Country")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

impl SendMessageWithTemplateResponseNumberDetail {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.carrier {
            params.push(("Carrier".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region {
            params.push(("Region".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.country {
            params.push(("Country".to_string(), v.to_string()));
        }
        params
    }
}

/// 号码明细。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendMessageToGlobeResponseNumberDetail {
    /// 号码所属的运营商网络。
    #[serde(rename = "Carrier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,
    /// 号码所属地区。
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// 号码所属国家。
    #[serde(rename = "Country")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

impl SendMessageToGlobeResponseNumberDetail {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.carrier {
            params.push(("Carrier".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region {
            params.push(("Region".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.country {
            params.push(("Country".to_string(), v.to_string()));
        }
        params
    }
}

/// 号码明细。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryMessageResponseNumberDetail {
    /// 号码所属的运营商网络。
    #[serde(rename = "Carrier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,
    /// 号码所属地区。
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// 号码所属国家。
    #[serde(rename = "Country")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

impl QueryMessageResponseNumberDetail {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.carrier {
            params.push(("Carrier".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region {
            params.push(("Region".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.country {
            params.push(("Country".to_string(), v.to_string()));
        }
        params
    }
}

/// BatchSendMessageToGlobe 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchSendMessageToGlobeRequest {
    /// 接收方号码。号码格式为：国际区号+号码。
    #[serde(rename = "To")]
    pub to: String,
    /// 发送方号码。支持Sender ID的发送，只允许数字、字母，含有字母标识最长11位，纯数字标识支持15位。
    #[serde(rename = "From")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// 短信内容。
    #[serde(rename = "Message")]
    pub message: String,
    /// 短信类型。取值：
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 任务ID。长度不超过255个字符。可以在短信回执消息体的TaskId字段获取。
    #[serde(rename = "TaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// 短信有效时长，单位：秒。
    #[serde(rename = "ValidityPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_period: Option<i64>,
    /// 短信通道id
    #[serde(rename = "ChannelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
}

impl BatchSendMessageToGlobeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("To".to_string(), self.to.to_string()));
        if let Some(ref v) = self.from {
            params.push(("From".to_string(), v.to_string()));
        }
        params.push(("Message".to_string(), self.message.to_string()));
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_id {
            params.push(("TaskId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.validity_period {
            params.push(("ValidityPeriod".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.channel_id {
            params.push(("ChannelId".to_string(), v.to_string()));
        }
        params
    }
}

/// 接口返回参数。
#[derive(Debug, Clone, Deserialize)]
pub struct BatchSendMessageToGlobeResponse {
    /// 状态码。返回OK代表请求成功，其他错误码，请参见[错误码列表](https://www.alibabacloud.com/help/zh/short-message-service/latest...
    #[serde(rename = "ResponseCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_code: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 发送失败的号码列表。
    #[serde(rename = "FailedList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_list: Option<String>,
    /// 状态码描述。
    #[serde(rename = "ResponseDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_description: Option<String>,
    /// 发送方标识，返回传入的Sender ID。
    #[serde(rename = "From")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// 发送成功的消息ID。
    #[serde(rename = "MessageIdList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id_list: Option<String>,
    /// 成功发送条数。
    #[serde(rename = "SuccessCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_count: Option<String>,
}

/// SendMessageWithTemplate 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SendMessageWithTemplateRequest {
    /// 接收短信号码。号码格式为：国际区号+号码。例如：861503871\*\*\*\*。
    #[serde(rename = "To")]
    pub to: String,
    /// 发送方标识，请传入短信签名名称。您可以登录[短信服务控制台](https://sms-intl.console.aliyun.com/overview)，选择**发往中国大陆** > **短信签...
    #[serde(rename = "From")]
    pub from: String,
    /// 短信模板编码。您可以登录[短信服务控制台](https://sms-intl.console.aliyun.com/overview)，选择**发往中国大陆** > **短信内容**，在短信内容...
    #[serde(rename = "TemplateCode")]
    pub template_code: String,
    /// 短信模板变量对应的实际值。如果模板中存在变量，该参数为必填项。
    #[serde(rename = "TemplateParam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_param: Option<String>,
    /// 上行短信扩展码。
    #[serde(rename = "SmsUpExtendCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_up_extend_code: Option<String>,
    /// 短信有效时长
    #[serde(rename = "ValidityPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_period: Option<i64>,
    /// 通道ID
    #[serde(rename = "ChannelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
}

impl SendMessageWithTemplateRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("To".to_string(), self.to.to_string()));
        params.push(("From".to_string(), self.from.to_string()));
        params.push(("TemplateCode".to_string(), self.template_code.to_string()));
        if let Some(ref v) = self.template_param {
            params.push(("TemplateParam".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sms_up_extend_code {
            params.push(("SmsUpExtendCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.validity_period {
            params.push(("ValidityPeriod".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.channel_id {
            params.push(("ChannelId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SendMessageWithTemplateResponse {
    /// 短信提交状态。
    #[serde(rename = "ResponseCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_code: Option<String>,
    /// 号码明细。
    #[serde(rename = "NumberDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_detail: Option<SendMessageWithTemplateResponseNumberDetail>,
    /// 短信提交状态的详细描述。
    #[serde(rename = "ResponseDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_description: Option<String>,
    /// 短信计费条数。
    #[serde(rename = "Segments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments: Option<String>,
    /// 接收短信号码。号码格式为：国际区号+号码。例如：861503871\*\*\*\*。
    #[serde(rename = "To")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    /// 消息ID。
    #[serde(rename = "MessageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// SendMessageToGlobe 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SendMessageToGlobeRequest {
    /// 接收方号码。号码格式为：`国际区号+号码`。例如：8521245567\*\*\*\*。
    #[serde(rename = "To")]
    pub to: String,
    /// 发送方号码。支持Sender ID的发送，只允许数字、字母，含有字母标识最长11位，纯数字标识支持15位。
    #[serde(rename = "From")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// 短信的内容。
    #[serde(rename = "Message")]
    pub message: String,
    /// 短信类型。取值：
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 任务ID。长度不超过255个字符。可以在短信回执消息体的TaskId字段获取。
    #[serde(rename = "TaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// 短信有效时长，单位：秒。
    #[serde(rename = "ValidityPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_period: Option<i64>,
    /// 通道ID。
    #[serde(rename = "ChannelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
}

impl SendMessageToGlobeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("To".to_string(), self.to.to_string()));
        if let Some(ref v) = self.from {
            params.push(("From".to_string(), v.to_string()));
        }
        params.push(("Message".to_string(), self.message.to_string()));
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_id {
            params.push(("TaskId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.validity_period {
            params.push(("ValidityPeriod".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.channel_id {
            params.push(("ChannelId".to_string(), v.to_string()));
        }
        params
    }
}

/// 接口返回参数。
#[derive(Debug, Clone, Deserialize)]
pub struct SendMessageToGlobeResponse {
    /// 短信提交状态码。
    #[serde(rename = "ResponseCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_code: Option<String>,
    /// 号码明细。
    #[serde(rename = "NumberDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_detail: Option<SendMessageToGlobeResponseNumberDetail>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 短信计费条数。
    #[serde(rename = "Segments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments: Option<String>,
    /// 短信提交状态码描述。
    #[serde(rename = "ResponseDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_description: Option<String>,
    /// 发送方号码，返回传入的Sender ID。
    #[serde(rename = "From")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// 接收方号码。
    #[serde(rename = "To")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    /// 消息ID。
    #[serde(rename = "MessageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
}

/// QueryMessage 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct QueryMessageRequest {
    /// 消息ID。
    #[serde(rename = "MessageId")]
    pub message_id: String,
}

impl QueryMessageRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("MessageId".to_string(), self.message_id.to_string()));
        params
    }
}

/// 接口返回参数。
#[derive(Debug, Clone, Deserialize)]
pub struct QueryMessageResponse {
    /// 短信的发送状态。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 短信发送状态码的描述。
    #[serde(rename = "ErrorDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
    /// 短信提交状态。
    #[serde(rename = "ResponseCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_code: Option<String>,
    /// 发送短信收到运营商回执的时间。
    #[serde(rename = "ReceiveDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receive_date: Option<String>,
    /// 号码明细。
    #[serde(rename = "NumberDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_detail: Option<QueryMessageResponseNumberDetail>,
    /// 短信内容。
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 短信提交状态的详细描述。
    #[serde(rename = "ResponseDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_description: Option<String>,
    /// 短信发送状态码。
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// 短信转发给运营商的时间。
    #[serde(rename = "SendDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_date: Option<String>,
    /// 接收方号码。
    #[serde(rename = "To")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    /// 消息ID。
    #[serde(rename = "MessageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ConversionData 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ConversionDataRequest {
}

impl ConversionDataRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 接口返回参数。
#[derive(Debug, Clone, Deserialize)]
pub struct ConversionDataResponse {
    /// 状态码。返回OK代表请求成功，其他错误码，请参见[错误码列表](https://www.alibabacloud.com/help/zh/doc-detail/180674.html)。
    #[serde(rename = "ResponseCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_code: Option<String>,
    /// 状态码描述。
    #[serde(rename = "ResponseDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_description: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// SmsConversion 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SmsConversionRequest {
    /// 消息ID。
    #[serde(rename = "MessageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// 如果您的用户回复了您发送的消息，则设置为 true。否则，设置为 false。
    #[serde(rename = "Delivered")]
    pub delivered: bool,
    /// 触达发送目标的时间戳。必须是Unix时间戳，毫秒级别长整型。
    #[serde(rename = "ConversionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversion_time: Option<i64>,
    /// 接收方号码。号码格式为：国际区号+号码。
    #[serde(rename = "To")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

impl SmsConversionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.message_id {
            params.push(("MessageId".to_string(), v.to_string()));
        }
        params.push(("Delivered".to_string(), self.delivered.to_string()));
        if let Some(ref v) = self.conversion_time {
            params.push(("ConversionTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.to {
            params.push(("To".to_string(), v.to_string()));
        }
        params
    }
}

/// 接口返回参数。
#[derive(Debug, Clone, Deserialize)]
pub struct SmsConversionResponse {
    /// 状态码。返回OK代表请求成功，其他错误码，请参见[错误码列表](https://www.alibabacloud.com/help/zh/doc-detail/180674.html)。
    #[serde(rename = "ResponseCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_code: Option<String>,
    /// 状态码描述。
    #[serde(rename = "ResponseDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_description: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
