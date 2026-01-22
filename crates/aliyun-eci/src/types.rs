//! 类型定义 - 自动生成，请勿手动修改

#![allow(unused_mut)]

use serde::{Deserialize, Serialize};

/// 地域信息集合。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRegionsResponseRegionsItem {
    /// 可用区。
    #[serde(rename = "Zones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zones: Option<Vec<String>>,
    /// 推荐的可用区列表。
    #[serde(rename = "RecommendZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommend_zones: Option<Vec<String>>,
    /// 地域对应的接入地址（Endpoint）。
    #[serde(rename = "RegionEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_endpoint: Option<String>,
    /// 地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 不可用的可用区列表。
    #[serde(rename = "UnavailableZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unavailable_zones: Option<Vec<String>>,
}

impl DescribeRegionsResponseRegionsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.zones {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Zones.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.recommend_zones {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("RecommendZones.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.region_endpoint {
            params.push(("RegionEndpoint".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.unavailable_zones {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("UnavailableZones.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 标签列表。最多可以绑定20个。更多信息，请参见[使用标签管理实例](~~146608~~)。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateContainerGroupRequestTagItem {
    /// 标签键。如果传入该值，则不允许为空字符串，且不允许重复。最多支持64个字符，不能以`aliyun`和`acs:`开头，不能包含`http://`或者`https://`。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。如果传入该值，可以为空字符串。最多支持128个字符，不能以`acs:`开头，不能包含`http://`或者`https://`。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreateContainerGroupRequestTagItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key {
            params.push(("Key".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        params
    }
}

/// 镜像仓库信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateContainerGroupRequestImageRegistryCredentialItem {
    /// 镜像仓库密码。
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// 镜像仓库注册地址。
    #[serde(rename = "Server")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// 镜像仓库用户名。
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

impl CreateContainerGroupRequestImageRegistryCredentialItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.password {
            params.push(("Password".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.server {
            params.push(("Server".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_name {
            params.push(("UserName".to_string(), v.to_string()));
        }
        params
    }
}

/// 容器的环境变量值。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateContainerGroupRequestContainerItemEnvironmentVarItem {
    /// 环境变量名。长度为1~128位。格式要求：`[0-9a-zA-Z]`，以及下划线，不能以数字开头。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 环境变量值。长度为0~256位。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 环境变量值引用。目前只支持配置为status.podIP。
    #[serde(rename = "FieldRef.FieldPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_ref_field_path: Option<String>,
}

impl CreateContainerGroupRequestContainerItemEnvironmentVarItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key {
            params.push(("Key".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.field_ref_field_path {
            params.push(("FieldRef.FieldPath".to_string(), v.to_string()));
        }
        params
    }
}

/// 数据卷信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateContainerGroupRequestContainerItemVolumeMountItem {
    /// 数据卷的挂载传播设置。挂载传播允许将Container挂载的卷共享到同一Pod中的其他Container，甚至可以共享到同一节点上的其他Pod。取值范围：
    #[serde(rename = "MountPropagation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_propagation: Option<String>,
    /// 容器挂载数据卷的目录。
    #[serde(rename = "MountPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_path: Option<String>,
    /// 数据卷是否只读。默认为false。
    #[serde(rename = "ReadOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// 数据卷子路径。
    #[serde(rename = "SubPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_path: Option<String>,
    /// 数据卷名称。同Volume中的Name。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl CreateContainerGroupRequestContainerItemVolumeMountItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.mount_propagation {
            params.push(("MountPropagation".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.mount_path {
            params.push(("MountPath".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.read_only {
            params.push(("ReadOnly".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sub_path {
            params.push(("SubPath".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        params
    }
}

/// 端口号。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateContainerGroupRequestContainerItemPortItem {
    /// 协议类型。取值范围：
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// 端口号。取值范围：1~65535。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}

impl CreateContainerGroupRequestContainerItemPortItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.protocol {
            params.push(("Protocol".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        params
    }
}

/// 生成的HTTP请求头参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateContainerGroupRequestContainerItemLifecyclePreStopHandlerHttpGetHttpHeaderItem {
    /// 使用HTTP请求方式设置preStop回调函数时，HTTP Get请求头中自定义设置的字段值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 使用HTTP请求方式设置preStop回调函数时，HTTP Get请求头中自定义设置的字段名。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl CreateContainerGroupRequestContainerItemLifecyclePreStopHandlerHttpGetHttpHeaderItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        params
    }
}

/// 生成的HTTP请求头参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateContainerGroupRequestContainerItemLifecyclePostStartHandlerHttpGetHttpHeaderItem {
    /// 使用HTTP请求方式设置postStart回调函数时，HTTP Get请求头中自定义设置的字段值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 使用HTTP请求方式设置postStart回调函数时，HTTP Get请求头中自定义设置的字段名。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl CreateContainerGroupRequestContainerItemLifecyclePostStartHandlerHttpGetHttpHeaderItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        params
    }
}

/// 容器信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateContainerGroupRequestContainerItem {
    /// 检查超时的时间，默认为1秒，最小为1秒。
    #[serde(rename = "ReadinessProbe.TimeoutSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readiness_probe_timeout_seconds: Option<i32>,
    /// 从上次检查失败后重新认定检查成功的检查次数阈值（必须是连续成功），默认为1。当前必须为1。
    #[serde(rename = "ReadinessProbe.SuccessThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readiness_probe_success_threshold: Option<i32>,
    /// 为容器内的进程授予某些特定的权限。目前仅支持配置为NET\_ADMIN和NET\_RAW。
    #[serde(rename = "SecurityContext.Capability.Add")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_context_capability_add: Option<Vec<String>>,
    /// 使用TCP Socket方式进行健康检查时，TCP Socket检测的端口。
    #[serde(rename = "ReadinessProbe.TcpSocket.Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readiness_probe_tcp_socket_port: Option<i32>,
    /// 使用HTTP请求方式进行健康检查时，HTTP Get请求对应的协议类型，取值范围：
    #[serde(rename = "ReadinessProbe.HttpGet.Scheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readiness_probe_http_get_scheme: Option<String>,
    /// 检查执行的周期，默认为10秒，最小为1秒。
    #[serde(rename = "LivenessProbe.PeriodSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liveness_probe_period_seconds: Option<i32>,
    /// 容器运行的根文件系统是否为只读，目前仅支持配置为true。
    #[serde(rename = "SecurityContext.ReadOnlyRootFilesystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_context_read_only_root_filesystem: Option<bool>,
    /// 容器的环境变量值。
    #[serde(rename = "EnvironmentVar")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_var: Option<Vec<CreateContainerGroupRequestContainerItemEnvironmentVarItem>>,
    /// 使用TCP Socket方式进行健康检查时，TCP Socket检测的端口。
    #[serde(rename = "LivenessProbe.TcpSocket.Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liveness_probe_tcp_socket_port: Option<i32>,
    /// 是否开启交互。默认为false。
    #[serde(rename = "Tty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,
    /// 容器工作目录。
    #[serde(rename = "WorkingDir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,
    /// 容器启动命令对应的参数。最多10个。
    #[serde(rename = "Arg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arg: Option<Vec<String>>,
    /// 此容器是否应在容器运行时为标准输入分配缓冲区。如果未设置，则容器中标准输入的读取值将导致EOF。默认为false。
    #[serde(rename = "Stdin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdin: Option<bool>,
    /// 检查开始执行的时间，以容器启动完成为起点计算。
    #[serde(rename = "LivenessProbe.InitialDelaySeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liveness_probe_initial_delay_seconds: Option<i32>,
    /// 数据卷信息。
    #[serde(rename = "VolumeMount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_mount: Option<Vec<CreateContainerGroupRequestContainerItemVolumeMountItem>>,
    /// 镜像拉取策略。取值范围：
    #[serde(rename = "ImagePullPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_pull_policy: Option<String>,
    /// 当标准输入为true时，标准输入流将在多个附加会话中是否保持开启状态。
    #[serde(rename = "StdinOnce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdin_once: Option<bool>,
    /// 使用TCPSocket方式设置preStop回调函数时，TCP Socket检测的端口。
    #[serde(rename = "LifecyclePreStopHandlerTcpSocketPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_pre_stop_handler_tcp_socket_port: Option<i32>,
    /// 使用HTTP请求方式设置postStart回调函数时，HTTP Get请求对应的协议类型，取值范围：
    #[serde(rename = "LifecyclePostStartHandlerHttpGetScheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_post_start_handler_http_get_scheme: Option<String>,
    /// 检查执行的周期，默认为10秒，最小为1秒。
    #[serde(rename = "ReadinessProbe.PeriodSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readiness_probe_period_seconds: Option<i32>,
    /// 从上次检查失败后重新认定检查成功的检查次数阈值（必须是连续成功），默认为1。当前必须为1。
    #[serde(rename = "LivenessProbe.SuccessThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liveness_probe_success_threshold: Option<i32>,
    /// 容器启动命令。最多20个。每个命令最多包含256个字符。
    #[serde(rename = "Command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    /// 使用HTTP请求方式设置postStart回调函数时，接收HTTP Get请求的主机地址。
    #[serde(rename = "LifecyclePostStartHandlerHttpGetHost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_post_start_handler_http_get_host: Option<String>,
    /// 消息通知策略，默认为空，目前仅支持轻量消息队列的消息通知。
    #[serde(rename = "TerminationMessagePolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_message_policy: Option<String>,
    /// 使用HTTP请求方式进行健康检查时，HTTP Get请求检测的路径。
    #[serde(rename = "ReadinessProbe.HttpGet.Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readiness_probe_http_get_path: Option<String>,
    /// 使用命令行方式进行健康检查时，在容器内执行的命令。
    #[serde(rename = "LivenessProbe.Exec.Command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liveness_probe_exec_command: Option<Vec<String>>,
    /// 使用TCPSocket方式设置postStart回调函数时，TCP Socket检测的端口。
    #[serde(rename = "LifecyclePostStartHandlerTcpSocketPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_post_start_handler_tcp_socket_port: Option<i32>,
    /// 使用HTTP请求方式设置postStart回调函数时，HTTP Get请求检测的路径。
    #[serde(rename = "LifecyclePostStartHandlerHttpGetPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_post_start_handler_http_get_path: Option<String>,
    /// 使用命令行方式设置postStart回调函数时，在容器内执行的命令。
    #[serde(rename = "LifecyclePostStartHandlerExec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_post_start_handler_exec: Option<Vec<String>>,
    /// 使用HTTP请求方式设置preStop回调函数时，HTTP Get请求检测的路径。
    #[serde(rename = "LifecyclePreStopHandlerHttpGetPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_pre_stop_handler_http_get_path: Option<String>,
    /// 端口号。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<Vec<CreateContainerGroupRequestContainerItemPortItem>>,
    /// 容器的报错地址。
    #[serde(rename = "TerminationMessagePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_message_path: Option<String>,
    /// 使用HTTP请求方式设置preStop回调函数时，HTTP Get请求对应的协议类型，取值范围：
    #[serde(rename = "LifecyclePreStopHandlerHttpGetScheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_pre_stop_handler_http_get_scheme: Option<String>,
    /// 使用HTTP请求方式进行健康检查时，HTTP Get请求对应的协议类型，取值范围：
    #[serde(rename = "LivenessProbe.HttpGet.Scheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liveness_probe_http_get_scheme: Option<String>,
    /// 使用HTTP请求方式进行健康检查时，HTTP Get请求检测的端口号。
    #[serde(rename = "ReadinessProbe.HttpGet.Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readiness_probe_http_get_port: Option<i32>,
    /// 使用TCP Socket方式设置postStart回调函数时，TCP Socket检测的主机地址。
    #[serde(rename = "LifecyclePostStartHandlerTcpSocketHost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_post_start_handler_tcp_socket_host: Option<String>,
    /// 指定容器使用的GPU个数。
    #[serde(rename = "Gpu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu: Option<i32>,
    /// 检查开始执行的时间，以容器启动完成为起点计算。
    #[serde(rename = "ReadinessProbe.InitialDelaySeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readiness_probe_initial_delay_seconds: Option<i32>,
    /// 使用命令行方式设置preStop回调函数时，在容器内执行的命令。
    #[serde(rename = "LifecyclePreStopHandlerExec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_pre_stop_handler_exec: Option<Vec<String>>,
    /// 容器的内存大小。单位：GiB。
    #[serde(rename = "Memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<f32>,
    /// 容器名称。
    #[serde(rename = "Name")]
    pub name: String,
    /// 使用HTTP请求方式设置preStop回调函数时，接收HTTP Get请求的主机地址。
    #[serde(rename = "LifecyclePreStopHandlerHttpGetHost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_pre_stop_handler_http_get_host: Option<String>,
    /// 使用TCP Socket方式设置preStop回调函数时，TCP Socket检测的主机地址。
    #[serde(rename = "LifecyclePreStopHandlerTcpSocketHost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_pre_stop_handler_tcp_socket_host: Option<String>,
    /// 容器镜像。
    #[serde(rename = "Image")]
    pub image: String,
    /// 使用HTTP请求方式设置preStop回调函数时，HTTP Get请求检测的端口号。
    #[serde(rename = "LifecyclePreStopHandlerHttpGetPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_pre_stop_handler_http_get_port: Option<i32>,
    /// 从上次检查成功后认定检查失败的检查次数阈值（必须是连续失败），默认为3。
    #[serde(rename = "LivenessProbe.FailureThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liveness_probe_failure_threshold: Option<i32>,
    /// 使用命令行方式进行健康检查时，在容器内执行的命令。
    #[serde(rename = "ReadinessProbe.Exec.Command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readiness_probe_exec_command: Option<Vec<String>>,
    /// 生成的HTTP请求头参数。
    #[serde(rename = "LifecyclePreStopHandlerHttpGetHttpHeader")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_pre_stop_handler_http_get_http_header: Option<Vec<CreateContainerGroupRequestContainerItemLifecyclePreStopHandlerHttpGetHttpHeaderItem>>,
    /// 从上次检查成功后认定检查失败的检查次数阈值（必须是连续失败），默认为3。
    #[serde(rename = "ReadinessProbe.FailureThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readiness_probe_failure_threshold: Option<i32>,
    /// 容器的vCPU大小。单位：核。
    #[serde(rename = "Cpu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<f32>,
    /// 使用HTTP请求方式进行健康检查时，HTTP Get请求检测的端口号。
    #[serde(rename = "LivenessProbe.HttpGet.Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liveness_probe_http_get_port: Option<i32>,
    /// 使用HTTP请求方式进行健康检查时，HTTP Get请求检测的路径。
    #[serde(rename = "LivenessProbe.HttpGet.Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liveness_probe_http_get_path: Option<String>,
    /// 检查超时的时间，默认为1秒，最小为1秒。
    #[serde(rename = "LivenessProbe.TimeoutSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liveness_probe_timeout_seconds: Option<i32>,
    /// 设置运行容器的用户ID。
    #[serde(rename = "SecurityContext.RunAsUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_context_run_as_user: Option<i64>,
    /// 使用HTTP请求方式设置postStart回调函数时，HTTP Get请求检查的端口号。
    #[serde(rename = "LifecyclePostStartHandlerHttpGetPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_post_start_handler_http_get_port: Option<i32>,
    /// 生成的HTTP请求头参数。
    #[serde(rename = "LifecyclePostStartHandlerHttpGetHttpHeader")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_post_start_handler_http_get_http_header: Option<Vec<CreateContainerGroupRequestContainerItemLifecyclePostStartHandlerHttpGetHttpHeaderItem>>,
    /// 查询ECI实例详情时，是否隐藏环境变量信息。取值范围：
    #[serde(rename = "EnvironmentVarHide")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_var_hide: Option<bool>,
    /// 设置运行容器的用户组。
    #[serde(rename = "SecurityContextRunAsGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_context_run_as_group: Option<i64>,
    /// 是否以非root模式运行容器。取值范围：
    #[serde(rename = "SecurityContextRunAsNonRoot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_context_run_as_non_root: Option<bool>,
    /// 容器是否启用特权，即是否以特权模式运行容器。取值范围：
    #[serde(rename = "SecurityContextPrivileged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_context_privileged: Option<bool>,
}

impl CreateContainerGroupRequestContainerItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.readiness_probe_timeout_seconds {
            params.push(("ReadinessProbe.TimeoutSeconds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.readiness_probe_success_threshold {
            params.push(("ReadinessProbe.SuccessThreshold".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.security_context_capability_add {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("SecurityContext.Capability.Add.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.readiness_probe_tcp_socket_port {
            params.push(("ReadinessProbe.TcpSocket.Port".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.readiness_probe_http_get_scheme {
            params.push(("ReadinessProbe.HttpGet.Scheme".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.liveness_probe_period_seconds {
            params.push(("LivenessProbe.PeriodSeconds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.security_context_read_only_root_filesystem {
            params.push(("SecurityContext.ReadOnlyRootFilesystem".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.environment_var {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("EnvironmentVar.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.liveness_probe_tcp_socket_port {
            params.push(("LivenessProbe.TcpSocket.Port".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tty {
            params.push(("Tty".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.working_dir {
            params.push(("WorkingDir".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.arg {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Arg.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.stdin {
            params.push(("Stdin".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.liveness_probe_initial_delay_seconds {
            params.push(("LivenessProbe.InitialDelaySeconds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.volume_mount {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("VolumeMount.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.image_pull_policy {
            params.push(("ImagePullPolicy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.stdin_once {
            params.push(("StdinOnce".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lifecycle_pre_stop_handler_tcp_socket_port {
            params.push(("LifecyclePreStopHandlerTcpSocketPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lifecycle_post_start_handler_http_get_scheme {
            params.push(("LifecyclePostStartHandlerHttpGetScheme".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.readiness_probe_period_seconds {
            params.push(("ReadinessProbe.PeriodSeconds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.liveness_probe_success_threshold {
            params.push(("LivenessProbe.SuccessThreshold".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.command {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Command.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.lifecycle_post_start_handler_http_get_host {
            params.push(("LifecyclePostStartHandlerHttpGetHost".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.termination_message_policy {
            params.push(("TerminationMessagePolicy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.readiness_probe_http_get_path {
            params.push(("ReadinessProbe.HttpGet.Path".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.liveness_probe_exec_command {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("LivenessProbe.Exec.Command.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.lifecycle_post_start_handler_tcp_socket_port {
            params.push(("LifecyclePostStartHandlerTcpSocketPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lifecycle_post_start_handler_http_get_path {
            params.push(("LifecyclePostStartHandlerHttpGetPath".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lifecycle_post_start_handler_exec {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("LifecyclePostStartHandlerExec.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.lifecycle_pre_stop_handler_http_get_path {
            params.push(("LifecyclePreStopHandlerHttpGetPath".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Port.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.termination_message_path {
            params.push(("TerminationMessagePath".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lifecycle_pre_stop_handler_http_get_scheme {
            params.push(("LifecyclePreStopHandlerHttpGetScheme".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.liveness_probe_http_get_scheme {
            params.push(("LivenessProbe.HttpGet.Scheme".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.readiness_probe_http_get_port {
            params.push(("ReadinessProbe.HttpGet.Port".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lifecycle_post_start_handler_tcp_socket_host {
            params.push(("LifecyclePostStartHandlerTcpSocketHost".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.gpu {
            params.push(("Gpu".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.readiness_probe_initial_delay_seconds {
            params.push(("ReadinessProbe.InitialDelaySeconds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lifecycle_pre_stop_handler_exec {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("LifecyclePreStopHandlerExec.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.memory {
            params.push(("Memory".to_string(), v.to_string()));
        }
        params.push(("Name".to_string(), self.name.to_string()));
        if let Some(ref v) = self.lifecycle_pre_stop_handler_http_get_host {
            params.push(("LifecyclePreStopHandlerHttpGetHost".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lifecycle_pre_stop_handler_tcp_socket_host {
            params.push(("LifecyclePreStopHandlerTcpSocketHost".to_string(), v.to_string()));
        }
        params.push(("Image".to_string(), self.image.to_string()));
        if let Some(ref v) = self.lifecycle_pre_stop_handler_http_get_port {
            params.push(("LifecyclePreStopHandlerHttpGetPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.liveness_probe_failure_threshold {
            params.push(("LivenessProbe.FailureThreshold".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.readiness_probe_exec_command {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("ReadinessProbe.Exec.Command.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.lifecycle_pre_stop_handler_http_get_http_header {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("LifecyclePreStopHandlerHttpGetHttpHeader.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.readiness_probe_failure_threshold {
            params.push(("ReadinessProbe.FailureThreshold".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cpu {
            params.push(("Cpu".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.liveness_probe_http_get_port {
            params.push(("LivenessProbe.HttpGet.Port".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.liveness_probe_http_get_path {
            params.push(("LivenessProbe.HttpGet.Path".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.liveness_probe_timeout_seconds {
            params.push(("LivenessProbe.TimeoutSeconds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.security_context_run_as_user {
            params.push(("SecurityContext.RunAsUser".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lifecycle_post_start_handler_http_get_port {
            params.push(("LifecyclePostStartHandlerHttpGetPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lifecycle_post_start_handler_http_get_http_header {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("LifecyclePostStartHandlerHttpGetHttpHeader.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.environment_var_hide {
            params.push(("EnvironmentVarHide".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.security_context_run_as_group {
            params.push(("SecurityContextRunAsGroup".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.security_context_run_as_non_root {
            params.push(("SecurityContextRunAsNonRoot".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.security_context_privileged {
            params.push(("SecurityContextPrivileged".to_string(), v.to_string()));
        }
        params
    }
}

/// ConfigFile类型的数据卷信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateContainerGroupRequestVolumeItemConfigFileVolumeConfigFileToPathItem {
    /// 相对于挂载目录，配置文件所在的相对文件路径。
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// 配置文件的权限，如果没有设置，则采用ConfigFileVolume.DefaultMode的值。采用四位八进制数表示，例如0644表示权限为rw-r–r--，即用户权限为rw-，用户所在组权限...
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<i32>,
    /// 配置文件内容，需要将内容进行Base64编码。
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

impl CreateContainerGroupRequestVolumeItemConfigFileVolumeConfigFileToPathItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.path {
            params.push(("Path".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.mode {
            params.push(("Mode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.content {
            params.push(("Content".to_string(), v.to_string()));
        }
        params
    }
}

/// 数据卷信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateContainerGroupRequestVolumeItem {
    /// 数据卷类型。取值范围：
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// DiskVolume的大小。单位为GiB。
    #[serde(rename = "DiskVolume.DiskSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_volume_disk_size: Option<i32>,
    /// NFS数据卷路径。
    #[serde(rename = "NFSVolume.Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nfs_volume_path: Option<String>,
    /// DiskVolume的系统类型。
    #[serde(rename = "DiskVolume.FsType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_volume_fs_type: Option<String>,
    /// 挂载的文件系统类型，默认取决于FlexVolume的script。
    #[serde(rename = "FlexVolume.FsType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flex_volume_fs_type: Option<String>,
    /// HostPath Volume的类型。取值范围：
    #[serde(rename = "HostPathVolume.Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_path_volume_type: Option<String>,
    /// ConfigFileVolume默认的权限。
    #[serde(rename = "ConfigFileVolume.DefaultMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_file_volume_default_mode: Option<i32>,
    /// NFS数据卷是否只读。默认为false。
    #[serde(rename = "NFSVolume.ReadOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nfs_volume_read_only: Option<bool>,
    /// ConfigFile类型的数据卷信息。
    #[serde(rename = "ConfigFileVolume.ConfigFileToPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_file_volume_config_file_to_path: Option<Vec<CreateContainerGroupRequestVolumeItemConfigFileVolumeConfigFileToPathItem>>,
    /// HostPath Volume在主机上的目录路径。
    #[serde(rename = "HostPathVolume.Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_path_volume_path: Option<String>,
    /// FlexVolume对象选项列表。为KV形式，采用JSON传递。
    #[serde(rename = "FlexVolume.Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flex_volume_options: Option<String>,
    /// 使用FlexVolume插件挂载数据卷时的驱动类型。取值范围如下：
    #[serde(rename = "FlexVolume.Driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flex_volume_driver: Option<String>,
    /// NFS服务器地址。
    #[serde(rename = "NFSVolume.Server")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nfs_volume_server: Option<String>,
    /// DiskVolume的ID。
    #[serde(rename = "DiskVolume.DiskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_volume_disk_id: Option<String>,
    /// 数据卷名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// EmptyDirVolume的存储媒介，默认为空，使用node文件系统；支持 memory，表示使用内存。
    #[serde(rename = "EmptyDirVolume.Medium")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_dir_volume_medium: Option<String>,
    /// EmptyDirVolume的大小。单位为GiB。
    #[serde(rename = "EmptyDirVolume.SizeLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_dir_volume_size_limit: Option<String>,
}

impl CreateContainerGroupRequestVolumeItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.disk_volume_disk_size {
            params.push(("DiskVolume.DiskSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.nfs_volume_path {
            params.push(("NFSVolume.Path".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.disk_volume_fs_type {
            params.push(("DiskVolume.FsType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.flex_volume_fs_type {
            params.push(("FlexVolume.FsType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.host_path_volume_type {
            params.push(("HostPathVolume.Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.config_file_volume_default_mode {
            params.push(("ConfigFileVolume.DefaultMode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.nfs_volume_read_only {
            params.push(("NFSVolume.ReadOnly".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.config_file_volume_config_file_to_path {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("ConfigFileVolume.ConfigFileToPath.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.host_path_volume_path {
            params.push(("HostPathVolume.Path".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.flex_volume_options {
            params.push(("FlexVolume.Options".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.flex_volume_driver {
            params.push(("FlexVolume.Driver".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.nfs_volume_server {
            params.push(("NFSVolume.Server".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.disk_volume_disk_id {
            params.push(("DiskVolume.DiskId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.empty_dir_volume_medium {
            params.push(("EmptyDirVolume.Medium".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.empty_dir_volume_size_limit {
            params.push(("EmptyDirVolume.SizeLimit".to_string(), v.to_string()));
        }
        params
    }
}

/// 挂载数据卷信息列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateContainerGroupRequestInitContainerItemVolumeMountItem {
    /// 数据卷的挂载传播设置。挂载传播允许将Container挂载的卷共享到同一Pod中的其他Container，甚至可以共享到同一节点上的其他Pod。取值范围：
    #[serde(rename = "MountPropagation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_propagation: Option<String>,
    /// 挂载目录，容器的挂载目录下的内容被volume的内容直接覆盖，所以要慎用。
    #[serde(rename = "MountPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_path: Option<String>,
    /// 挂载路径是否只读。默认为false。
    #[serde(rename = "ReadOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// 数据卷下的子目录，方便Pod将同一个Volume下不同目录挂载到容器不同目录。
    #[serde(rename = "SubPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_path: Option<String>,
    /// 挂载数据卷的名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl CreateContainerGroupRequestInitContainerItemVolumeMountItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.mount_propagation {
            params.push(("MountPropagation".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.mount_path {
            params.push(("MountPath".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.read_only {
            params.push(("ReadOnly".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sub_path {
            params.push(("SubPath".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        params
    }
}

/// init容器端口号。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateContainerGroupRequestInitContainerItemPortItem {
    /// 协议类型。取值范围：
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// 端口号。取值范围：1~65535。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}

impl CreateContainerGroupRequestInitContainerItemPortItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.protocol {
            params.push(("Protocol".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        params
    }
}

/// 容器的环境变量信息列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateContainerGroupRequestInitContainerItemEnvironmentVarItem {
    /// 环境变量名。长度为1~128位。格式要求：`[0-9a-zA-Z]`，以及下划线，不能以数字开头。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 环境变量值。长度为0~256位。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 环境变量值引用。目前只支持配置为status.podIP。
    #[serde(rename = "FieldRef.FieldPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_ref_field_path: Option<String>,
}

impl CreateContainerGroupRequestInitContainerItemEnvironmentVarItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key {
            params.push(("Key".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.field_ref_field_path {
            params.push(("FieldRef.FieldPath".to_string(), v.to_string()));
        }
        params
    }
}

/// Init容器列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateContainerGroupRequestInitContainerItem {
    /// 为容器内的进程授予某些特定的权限。目前仅支持配置为NET\_ADMIN和NET\_RAW。
    #[serde(rename = "SecurityContext.Capability.Add")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_context_capability_add: Option<Vec<String>>,
    /// 容器使用的镜像。
    #[serde(rename = "Image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// 挂载数据卷信息列表。
    #[serde(rename = "VolumeMount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_mount: Option<Vec<CreateContainerGroupRequestInitContainerItemVolumeMountItem>>,
    /// init容器端口号。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<Vec<CreateContainerGroupRequestInitContainerItemPortItem>>,
    /// 容器运行的根文件系统是否为只读。目前仅支持配置为true。
    #[serde(rename = "SecurityContext.ReadOnlyRootFilesystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_context_read_only_root_filesystem: Option<bool>,
    /// 容器退出的消息来源。当容器退出时，从指定的终止消息文件中检索终止消息。
    #[serde(rename = "TerminationMessagePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_message_path: Option<String>,
    /// 容器的环境变量信息列表。
    #[serde(rename = "EnvironmentVar")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_var: Option<Vec<CreateContainerGroupRequestInitContainerItemEnvironmentVarItem>>,
    /// 镜像拉取策略。取值范围：
    #[serde(rename = "ImagePullPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_pull_policy: Option<String>,
    /// 工作目录。
    #[serde(rename = "WorkingDir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,
    /// 容器的vCPU大小。单位：核。
    #[serde(rename = "Cpu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<f32>,
    /// 容器启动参数。
    #[serde(rename = "Arg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arg: Option<Vec<String>>,
    /// 容器启动指令。
    #[serde(rename = "Command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    /// 指定容器使用的GPU个数。
    #[serde(rename = "Gpu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu: Option<i32>,
    /// 设置运行容器的用户ID。
    #[serde(rename = "SecurityContext.RunAsUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_context_run_as_user: Option<i64>,
    /// 容器的内存大小。单位：GiB。
    #[serde(rename = "Memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<f32>,
    /// 挂载信息，默认为空。
    #[serde(rename = "TerminationMessagePolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_message_policy: Option<String>,
    /// 容器名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl CreateContainerGroupRequestInitContainerItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.security_context_capability_add {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("SecurityContext.Capability.Add.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.image {
            params.push(("Image".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.volume_mount {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("VolumeMount.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.port {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Port.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.security_context_read_only_root_filesystem {
            params.push(("SecurityContext.ReadOnlyRootFilesystem".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.termination_message_path {
            params.push(("TerminationMessagePath".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.environment_var {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("EnvironmentVar.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.image_pull_policy {
            params.push(("ImagePullPolicy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.working_dir {
            params.push(("WorkingDir".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cpu {
            params.push(("Cpu".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.arg {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Arg.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.command {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Command.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.gpu {
            params.push(("Gpu".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.security_context_run_as_user {
            params.push(("SecurityContext.RunAsUser".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.memory {
            params.push(("Memory".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.termination_message_policy {
            params.push(("TerminationMessagePolicy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        params
    }
}

/// 对象的选项列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateContainerGroupRequestDnsConfigOptionItem {
    /// 对象的Value。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 对象的Name。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl CreateContainerGroupRequestDnsConfigOptionItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        params
    }
}

/// 添加一个ECI的别名。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateContainerGroupRequestHostAliaseItem {
    /// 添加IP。
    #[serde(rename = "Ip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// 添加主机名。
    #[serde(rename = "Hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<Vec<String>>,
}

impl CreateContainerGroupRequestHostAliaseItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.ip {
            params.push(("Ip".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.hostname {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Hostname.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 通过安全上下文修改安全sysctl参数。更多信息，请参见[配置Security Context](~~462313~~)。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateContainerGroupRequestSecurityContextSysctlItem {
    /// 通过安全上下文修改sysctl参数时，安全sysctl参数的取值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 通过安全上下文修改sysctl参数时，安全sysctl参数的名称。取值范围：
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl CreateContainerGroupRequestSecurityContextSysctlItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        params
    }
}

/// 通过安全上下文修改非安全sysctl参数。更多信息，请参见[配置Security Context](~~462313~~)。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateContainerGroupRequestHostSecurityContextSysctlItem {
    /// 通过安全上下文修改sysctl参数时，非安全sysctl参数的取值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 通过安全上下文修改sysctl参数时，非安全sysctl参数的名称。取值范围：
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl CreateContainerGroupRequestHostSecurityContextSysctlItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        params
    }
}

/// ACR企业版实例信息列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateContainerGroupRequestAcrRegistryInfoItem {
    /// ACR企业版实例的域名。默认为相应实例的所有域名。支持指定个别域名，多个以半角逗号分隔。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<Vec<String>>,
    /// ACR企业版实例的名称。
    #[serde(rename = "InstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    /// ACR企业版实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// ACR企业版实例所属地域。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// ECI实例等资源归属账号下的RAM角色的ARN。
    #[serde(rename = "ArnService")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn_service: Option<String>,
    /// ACR实例归属账号下的RAM角色的ARN。
    #[serde(rename = "ArnUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn_user: Option<String>,
}

impl CreateContainerGroupRequestAcrRegistryInfoItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Domain.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.instance_name {
            params.push(("InstanceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.arn_service {
            params.push(("ArnService".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.arn_user {
            params.push(("ArnUser".to_string(), v.to_string()));
        }
        params
    }
}

/// 实例绑定的标签列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateContainerGroupRequestTagItem {
    /// 标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl UpdateContainerGroupRequestTagItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key {
            params.push(("Key".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        params
    }
}

/// ConfigFile数据卷对应的配置文件信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateContainerGroupRequestVolumeItemConfigFileVolumeConfigFileToPathItem {
    /// 配置文件的相对文件路径。
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// 配置文件的内容。采用Base64编码。
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

impl UpdateContainerGroupRequestVolumeItemConfigFileVolumeConfigFileToPathItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.path {
            params.push(("Path".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.content {
            params.push(("Content".to_string(), v.to_string()));
        }
        params
    }
}

/// 数据卷列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateContainerGroupRequestVolumeItem {
    /// 数据卷名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 数据卷类型。取值范围：
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// NFS数据卷要挂载的路径。
    #[serde(rename = "NFSVolume.Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nfs_volume_path: Option<String>,
    /// NFS数据卷的挂载点地址。
    #[serde(rename = "NFSVolume.Server")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nfs_volume_server: Option<String>,
    /// NFS数据卷的读取权限。取值范围：
    #[serde(rename = "NFSVolume.ReadOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nfs_volume_read_only: Option<bool>,
    /// ConfigFile数据卷对应的配置文件信息。
    #[serde(rename = "ConfigFileVolume.ConfigFileToPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_file_volume_config_file_to_path: Option<Vec<UpdateContainerGroupRequestVolumeItemConfigFileVolumeConfigFileToPathItem>>,
    /// EmptyDir数据卷的存储媒介，默认为空，使用node文件系统；支持配置为Memory，表示使用内存。
    #[serde(rename = "EmptyDirVolume.Medium")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_dir_volume_medium: Option<String>,
    /// EmptyDir数据卷的大小。取值请带上单位，建议使用Gi或Mi。
    #[serde(rename = "EmptyDirVolume.SizeLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_dir_volume_size_limit: Option<String>,
    /// 使用FlexVolume插件挂载云盘时，云盘的文件系统类型。支持的类型包括ext4、ext3、xfs、vfat。默认为ext4。
    #[serde(rename = "FlexVolume.FsType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flex_volume_fs_type: Option<String>,
    /// 使用FlexVolume插件挂载数据卷时的驱动类型。取值范围如下：
    #[serde(rename = "FlexVolume.Driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flex_volume_driver: Option<String>,
    /// FlexVolume对象选项列表。为KV形式，采用JSON传递。例如通过FlexVolume挂载云盘时，Options表示云盘的配置参数。更多信息，请参见[数据卷概述](~~90672~~)。
    #[serde(rename = "FlexVolume.Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flex_volume_options: Option<String>,
    /// HostPath数据卷在宿主机上的路径。
    #[serde(rename = "HostPathVolume.Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_path_volume_path: Option<String>,
    /// HostPath数据卷的类型。取值范围：
    #[serde(rename = "HostPathVolume.Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_path_volume_type: Option<String>,
}

impl UpdateContainerGroupRequestVolumeItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.nfs_volume_path {
            params.push(("NFSVolume.Path".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.nfs_volume_server {
            params.push(("NFSVolume.Server".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.nfs_volume_read_only {
            params.push(("NFSVolume.ReadOnly".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.config_file_volume_config_file_to_path {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("ConfigFileVolume.ConfigFileToPath.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.empty_dir_volume_medium {
            params.push(("EmptyDirVolume.Medium".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.empty_dir_volume_size_limit {
            params.push(("EmptyDirVolume.SizeLimit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.flex_volume_fs_type {
            params.push(("FlexVolume.FsType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.flex_volume_driver {
            params.push(("FlexVolume.Driver".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.flex_volume_options {
            params.push(("FlexVolume.Options".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.host_path_volume_path {
            params.push(("HostPathVolume.Path".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.host_path_volume_type {
            params.push(("HostPathVolume.Type".to_string(), v.to_string()));
        }
        params
    }
}

/// DNS配置信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateContainerGroupRequestDnsConfigOptionItem {
    /// DNS配置的选项变量值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// DNS配置的选项变量名。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UpdateContainerGroupRequestDnsConfigOptionItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        params
    }
}

/// 容器的环境变量信息列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateContainerGroupRequestContainerItemEnvironmentVarItem {
    /// 容器的环境变量名。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 容器的环境变量值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 使用Pod字段作为环境变量。目前仅支持status.podIP。
    #[serde(rename = "FieldRef.FieldPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_ref_field_path: Option<String>,
}

impl UpdateContainerGroupRequestContainerItemEnvironmentVarItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key {
            params.push(("Key".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.field_ref_field_path {
            params.push(("FieldRef.FieldPath".to_string(), v.to_string()));
        }
        params
    }
}

/// 容器的数据卷信息列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateContainerGroupRequestContainerItemVolumeMountItem {
    /// 数据卷的挂载传播设置。挂载传播允许将Container挂载的卷共享到同一Pod中的其他Container，甚至可以共享到同一节点上的其他Pod。取值范围：
    #[serde(rename = "MountPropagation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_propagation: Option<String>,
    /// 容器挂载的目录。容器挂载目录下的内容会被volume的内容直接覆盖，请谨慎使用。
    #[serde(rename = "MountPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_path: Option<String>,
    /// 是否只读。默认值：false
    #[serde(rename = "ReadOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// 数据卷的子目录，方便Pod将同一个Volume下不同目录挂载到容器不同目录。
    #[serde(rename = "SubPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_path: Option<String>,
    /// 容器挂载的数据卷名称。从ECI实例（容器组）挂载的数据卷中选择，即取值范围为配置的Volume.N.Name参数。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UpdateContainerGroupRequestContainerItemVolumeMountItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.mount_propagation {
            params.push(("MountPropagation".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.mount_path {
            params.push(("MountPath".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.read_only {
            params.push(("ReadOnly".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sub_path {
            params.push(("SubPath".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        params
    }
}

/// 容器端口号。取值范围：1~65535
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateContainerGroupRequestContainerItemPortItem {
    /// TCP/UDP。
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// 端口号。取值范围：1~65535
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}

impl UpdateContainerGroupRequestContainerItemPortItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.protocol {
            params.push(("Protocol".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        params
    }
}

/// 生成的http请求头中，有效的http请求头信息集合。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateContainerGroupRequestContainerItemLifecyclePostStartHandlerHttpGetHttpHeadersItem {
    /// 使用HTTP请求方式设置postStart回调函数时，HTTP Get请求的请求参数值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 使用HTTP请求方式设置postStart回调函数时，HTTP Get请求的请求参数。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UpdateContainerGroupRequestContainerItemLifecyclePostStartHandlerHttpGetHttpHeadersItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        params
    }
}

/// 生成的http请求头信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateContainerGroupRequestContainerItemLifecyclePreStopHandlerHttpGetHttpHeaderItem {
    /// 使用HTTP请求方式设置preStop回调函数时，HTTP Get请求的请求参数值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 使用HTTP请求方式设置preStop回调函数时，HTTP Get请求的请求参数。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UpdateContainerGroupRequestContainerItemLifecyclePreStopHandlerHttpGetHttpHeaderItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        params
    }
}

/// 指定新的容器组配置信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateContainerGroupRequestContainerItem {
    /// 检查超时的时间。默认为1秒，最小为1秒。
    #[serde(rename = "ReadinessProbe.TimeoutSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readiness_probe_timeout_seconds: Option<i32>,
    /// 从上次检查失败后重新认定检查成功的检查次数阈值（必须是连续成功），默认为1。
    #[serde(rename = "ReadinessProbe.SuccessThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readiness_probe_success_threshold: Option<i32>,
    /// 为容器内的进程授予某些特定的权限。目前仅支持配置为NET\_ADMIN和NET\_RAW。
    #[serde(rename = "SecurityContext.Capability.Add")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_context_capability_add: Option<Vec<String>>,
    /// TcpSocket检测的端口。
    #[serde(rename = "ReadinessProbe.TcpSocket.Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readiness_probe_tcp_socket_port: Option<i32>,
    /// 使用HTTP请求方式进行健康检查时，HTTP Get请求对应的协议类型，取值范围：
    #[serde(rename = "ReadinessProbe.HttpGet.Scheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readiness_probe_http_get_scheme: Option<String>,
    /// 检查执行的周期。默认为10秒，最小为1秒。
    #[serde(rename = "LivenessProbe.PeriodSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liveness_probe_period_seconds: Option<i32>,
    /// 是否只读根文件系统。取值目前仅支持：true。
    #[serde(rename = "SecurityContext.ReadOnlyRootFilesystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_context_read_only_root_filesystem: Option<bool>,
    /// 容器的环境变量信息列表。
    #[serde(rename = "EnvironmentVar")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_var: Option<Vec<UpdateContainerGroupRequestContainerItemEnvironmentVarItem>>,
    /// TcpSocket检测的端口。
    #[serde(rename = "LivenessProbe.TcpSocket.Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liveness_probe_tcp_socket_port: Option<i32>,
    /// 是否开启交互。默认为false，如果Command为/bin/bash类型，需要设置为true。
    #[serde(rename = "Tty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,
    /// 容器工作目录。
    #[serde(rename = "WorkingDir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,
    /// 容器启动参数。最多10个。
    #[serde(rename = "Arg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arg: Option<Vec<String>>,
    /// 此容器是否应在容器运行时为标准输入分配缓冲区。如果未设置，则容器中标准输入的读取值将导致EOF。默认为false。
    #[serde(rename = "Stdin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdin: Option<bool>,
    /// 检查开始执行的时间。以容器启动完成为起点开始计算。
    #[serde(rename = "LivenessProbe.InitialDelaySeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liveness_probe_initial_delay_seconds: Option<i32>,
    /// 容器的数据卷信息列表。
    #[serde(rename = "VolumeMount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_mount: Option<Vec<UpdateContainerGroupRequestContainerItemVolumeMountItem>>,
    /// 镜像拉取策略。取值范围：
    #[serde(rename = "ImagePullPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_pull_policy: Option<String>,
    /// 当标准输入为true时，标准输入流将在多个附加会话中保持开启状态。如果StdinOnce设为true，标准输入在容器开启时被打开，在首个客户端附加到标准输入之前都为空，然后会一直保持开启状态，接...
    #[serde(rename = "StdinOnce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdin_once: Option<bool>,
    /// 使用TCPSocket方式设置preStop回调函数时，TCP Socket检测的端口。
    #[serde(rename = "LifecyclePreStopHandlerTcpSocketPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_pre_stop_handler_tcp_socket_port: Option<i32>,
    /// 使用HTTP请求方式设置postStart回调函数时，HTTP Get请求检测的路径。
    #[serde(rename = "LifecyclePostStartHandlerHttpGetScheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_post_start_handler_http_get_scheme: Option<String>,
    /// 检查执行的周期，默认为10秒，最小为1秒。
    #[serde(rename = "ReadinessProbe.PeriodSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readiness_probe_period_seconds: Option<i32>,
    /// 从上次检查失败后重新认定检查成功的检查次数阈值（必须是连续成功）。默认为1次，当前必须为1次。
    #[serde(rename = "LivenessProbe.SuccessThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liveness_probe_success_threshold: Option<i32>,
    /// 容器启动命令。最多20个，单个命令支持256个字符。
    #[serde(rename = "Command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    /// 使用HTTP请求方式设置postStart回调函数时，接收HTTP Get请求的主机地址。
    #[serde(rename = "LifecyclePostStartHandlerHttpGetHost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_post_start_handler_http_get_host: Option<String>,
    /// HttpGet检测的路径。
    #[serde(rename = "ReadinessProbe.HttpGet.Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readiness_probe_http_get_path: Option<String>,
    /// 容器内检测命令。
    #[serde(rename = "LivenessProbe.Exec.Command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liveness_probe_exec_command: Option<Vec<String>>,
    /// 使用TCPSocket方式设置postStart回调函数时，TCP Socket检测的端口。
    #[serde(rename = "LifecyclePostStartHandlerTcpSocketPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_post_start_handler_tcp_socket_port: Option<i32>,
    /// 使用HTTP请求方式设置postStart回调函数时，HTTP Get请求检测的路径。
    #[serde(rename = "LifecyclePostStartHandlerHttpGetPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_post_start_handler_http_get_path: Option<String>,
    /// 使用命令行方式设置postStart回调函数时，在容器内执行的命令。
    #[serde(rename = "LifecyclePostStartHandlerExec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_post_start_handler_exec: Option<Vec<String>>,
    /// 使用HTTP请求方式设置preStop回调函数时，HTTP Get请求检测的路径。
    #[serde(rename = "LifecyclePreStopHandlerHttpGetPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_pre_stop_handler_http_get_path: Option<String>,
    /// 容器端口号。取值范围：1~65535
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<Vec<UpdateContainerGroupRequestContainerItemPortItem>>,
    /// 使用HTTP请求方式设置preStop回调函数时，HTTP Get请求对应的协议类型，取值范围：
    #[serde(rename = "LifecyclePreStopHandlerHttpGetScheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_pre_stop_handler_http_get_scheme: Option<String>,
    /// 使用HTTP请求方式进行健康检查时，HTTP Get请求对应的协议类型，取值范围：
    #[serde(rename = "LivenessProbe.HttpGet.Scheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liveness_probe_http_get_scheme: Option<String>,
    /// 生成的http请求头中，有效的http请求头信息集合。
    #[serde(rename = "LifecyclePostStartHandlerHttpGetHttpHeaders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_post_start_handler_http_get_http_headers: Option<Vec<UpdateContainerGroupRequestContainerItemLifecyclePostStartHandlerHttpGetHttpHeadersItem>>,
    /// HttpGet检测的端口号。
    #[serde(rename = "ReadinessProbe.HttpGet.Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readiness_probe_http_get_port: Option<i32>,
    /// 使用TCP Socket方式设置postStart回调函数时，TCP Socket检测的主机地址。
    #[serde(rename = "LifecyclePostStartHandlerTcpSocketHost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_post_start_handler_tcp_socket_host: Option<String>,
    /// 指定容器使用的GPU个数。
    #[serde(rename = "Gpu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu: Option<i32>,
    /// 检查开始执行的时间，以容器启动完成为起点计算。
    #[serde(rename = "ReadinessProbe.InitialDelaySeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readiness_probe_initial_delay_seconds: Option<i32>,
    /// 使用命令行方式设置preStop回调函数时，在容器内执行的命令。
    #[serde(rename = "LifecyclePreStopHandlerExec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_pre_stop_handler_exec: Option<Vec<String>>,
    /// 容器内存大小。
    #[serde(rename = "Memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<f32>,
    /// 容器名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 使用HTTP请求方式设置preStop回调函数时，接收HTTP Get请求的主机地址。
    #[serde(rename = "LifecyclePreStopHandlerHttpGetHost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_pre_stop_handler_http_get_host: Option<String>,
    /// 使用TCP Socket方式设置preStop回调函数时，TCP Socket检测的主机地址。
    #[serde(rename = "LifecyclePreStopHandlerTcpSocketHost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_pre_stop_handler_tcp_socket_host: Option<String>,
    /// 容器镜像。
    #[serde(rename = "Image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// 使用HTTP请求方式设置preStop回调函数时，HTTP Get请求检测的端口号。
    #[serde(rename = "LifecyclePreStopHandlerHttpGetPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_pre_stop_handler_http_get_port: Option<i32>,
    /// 从上次检查成功后认定检查失败的检查次数阈值（必须是连续失败）。默认为3次。
    #[serde(rename = "LivenessProbe.FailureThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liveness_probe_failure_threshold: Option<i32>,
    /// 容器内检测的命令。
    #[serde(rename = "ReadinessProbe.Exec.Command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readiness_probe_exec_command: Option<Vec<String>>,
    /// 生成的http请求头信息。
    #[serde(rename = "LifecyclePreStopHandlerHttpGetHttpHeader")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_pre_stop_handler_http_get_http_header: Option<Vec<UpdateContainerGroupRequestContainerItemLifecyclePreStopHandlerHttpGetHttpHeaderItem>>,
    /// 从上次检查成功后认定检查失败的检查次数阈值（必须是连续失败）。默认为3次。
    #[serde(rename = "ReadinessProbe.FailureThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readiness_probe_failure_threshold: Option<i32>,
    /// 容器vCPU大小。
    #[serde(rename = "Cpu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<f32>,
    /// HttpGet检测的端口号。
    #[serde(rename = "LivenessProbe.HttpGet.Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liveness_probe_http_get_port: Option<i32>,
    /// HttpGet检测的路径。
    #[serde(rename = "LivenessProbe.HttpGet.Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liveness_probe_http_get_path: Option<String>,
    /// 检查超时的时间。默认为1秒，最小为1秒。
    #[serde(rename = "LivenessProbe.TimeoutSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liveness_probe_timeout_seconds: Option<i32>,
    /// 用于运行容器进程入口点的UID。
    #[serde(rename = "SecurityContext.RunAsUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_context_run_as_user: Option<i64>,
    /// 使用HTTP请求方式设置postStart回调函数时，HTTP Get请求检查的端口号。
    #[serde(rename = "LifecyclePostStartHandlerHttpGetPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_post_start_handler_http_get_port: Option<i32>,
}

impl UpdateContainerGroupRequestContainerItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.readiness_probe_timeout_seconds {
            params.push(("ReadinessProbe.TimeoutSeconds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.readiness_probe_success_threshold {
            params.push(("ReadinessProbe.SuccessThreshold".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.security_context_capability_add {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("SecurityContext.Capability.Add.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.readiness_probe_tcp_socket_port {
            params.push(("ReadinessProbe.TcpSocket.Port".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.readiness_probe_http_get_scheme {
            params.push(("ReadinessProbe.HttpGet.Scheme".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.liveness_probe_period_seconds {
            params.push(("LivenessProbe.PeriodSeconds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.security_context_read_only_root_filesystem {
            params.push(("SecurityContext.ReadOnlyRootFilesystem".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.environment_var {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("EnvironmentVar.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.liveness_probe_tcp_socket_port {
            params.push(("LivenessProbe.TcpSocket.Port".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tty {
            params.push(("Tty".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.working_dir {
            params.push(("WorkingDir".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.arg {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Arg.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.stdin {
            params.push(("Stdin".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.liveness_probe_initial_delay_seconds {
            params.push(("LivenessProbe.InitialDelaySeconds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.volume_mount {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("VolumeMount.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.image_pull_policy {
            params.push(("ImagePullPolicy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.stdin_once {
            params.push(("StdinOnce".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lifecycle_pre_stop_handler_tcp_socket_port {
            params.push(("LifecyclePreStopHandlerTcpSocketPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lifecycle_post_start_handler_http_get_scheme {
            params.push(("LifecyclePostStartHandlerHttpGetScheme".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.readiness_probe_period_seconds {
            params.push(("ReadinessProbe.PeriodSeconds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.liveness_probe_success_threshold {
            params.push(("LivenessProbe.SuccessThreshold".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.command {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Command.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.lifecycle_post_start_handler_http_get_host {
            params.push(("LifecyclePostStartHandlerHttpGetHost".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.readiness_probe_http_get_path {
            params.push(("ReadinessProbe.HttpGet.Path".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.liveness_probe_exec_command {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("LivenessProbe.Exec.Command.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.lifecycle_post_start_handler_tcp_socket_port {
            params.push(("LifecyclePostStartHandlerTcpSocketPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lifecycle_post_start_handler_http_get_path {
            params.push(("LifecyclePostStartHandlerHttpGetPath".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lifecycle_post_start_handler_exec {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("LifecyclePostStartHandlerExec.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.lifecycle_pre_stop_handler_http_get_path {
            params.push(("LifecyclePreStopHandlerHttpGetPath".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Port.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.lifecycle_pre_stop_handler_http_get_scheme {
            params.push(("LifecyclePreStopHandlerHttpGetScheme".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.liveness_probe_http_get_scheme {
            params.push(("LivenessProbe.HttpGet.Scheme".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lifecycle_post_start_handler_http_get_http_headers {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("LifecyclePostStartHandlerHttpGetHttpHeaders.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.readiness_probe_http_get_port {
            params.push(("ReadinessProbe.HttpGet.Port".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lifecycle_post_start_handler_tcp_socket_host {
            params.push(("LifecyclePostStartHandlerTcpSocketHost".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.gpu {
            params.push(("Gpu".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.readiness_probe_initial_delay_seconds {
            params.push(("ReadinessProbe.InitialDelaySeconds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lifecycle_pre_stop_handler_exec {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("LifecyclePreStopHandlerExec.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.memory {
            params.push(("Memory".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lifecycle_pre_stop_handler_http_get_host {
            params.push(("LifecyclePreStopHandlerHttpGetHost".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lifecycle_pre_stop_handler_tcp_socket_host {
            params.push(("LifecyclePreStopHandlerTcpSocketHost".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.image {
            params.push(("Image".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lifecycle_pre_stop_handler_http_get_port {
            params.push(("LifecyclePreStopHandlerHttpGetPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.liveness_probe_failure_threshold {
            params.push(("LivenessProbe.FailureThreshold".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.readiness_probe_exec_command {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("ReadinessProbe.Exec.Command.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.lifecycle_pre_stop_handler_http_get_http_header {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("LifecyclePreStopHandlerHttpGetHttpHeader.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.readiness_probe_failure_threshold {
            params.push(("ReadinessProbe.FailureThreshold".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cpu {
            params.push(("Cpu".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.liveness_probe_http_get_port {
            params.push(("LivenessProbe.HttpGet.Port".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.liveness_probe_http_get_path {
            params.push(("LivenessProbe.HttpGet.Path".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.liveness_probe_timeout_seconds {
            params.push(("LivenessProbe.TimeoutSeconds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.security_context_run_as_user {
            params.push(("SecurityContext.RunAsUser".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lifecycle_post_start_handler_http_get_port {
            params.push(("LifecyclePostStartHandlerHttpGetPort".to_string(), v.to_string()));
        }
        params
    }
}

/// 挂载数据卷信息列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateContainerGroupRequestInitContainerItemVolumeMountItem {
    /// 数据卷的挂载传播设置。挂载传播允许将Container挂载的卷共享到同一Pod中的其他Container，甚至可以共享到同一节点上的其他Pod。取值范围：
    #[serde(rename = "MountPropagation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_propagation: Option<String>,
    /// Init容器挂载目录。容器挂载目录下的内容将被volume的内容直接覆盖，请谨慎使用。
    #[serde(rename = "MountPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_path: Option<String>,
    /// 是否只读。默认值：false
    #[serde(rename = "ReadOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// 数据卷的子目录，方便Pod将同一个Volume下不同目录挂载到容器不同目录。
    #[serde(rename = "SubPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_path: Option<String>,
    /// Init容器挂载的数据卷名称。从ECI实例（容器组）挂载的数据卷中选择，即取值范围为配置的Volume.N.Name参数。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UpdateContainerGroupRequestInitContainerItemVolumeMountItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.mount_propagation {
            params.push(("MountPropagation".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.mount_path {
            params.push(("MountPath".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.read_only {
            params.push(("ReadOnly".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sub_path {
            params.push(("SubPath".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        params
    }
}

/// 端口号。取值范围：1~65535。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateContainerGroupRequestInitContainerItemPortItem {
    /// TCP/UDP。
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// Init容器端口号 。取值范围：1~65535。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}

impl UpdateContainerGroupRequestInitContainerItemPortItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.protocol {
            params.push(("Protocol".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        params
    }
}

/// 容器的环境变量信息列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateContainerGroupRequestInitContainerItemEnvironmentVarItem {
    /// Init容器的环境变量名。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Init容器的环境变量值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 环境变量值引用。目前只支持配置为status.podIP。
    #[serde(rename = "FieldRef.FieldPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_ref_field_path: Option<String>,
}

impl UpdateContainerGroupRequestInitContainerItemEnvironmentVarItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key {
            params.push(("Key".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.field_ref_field_path {
            params.push(("FieldRef.FieldPath".to_string(), v.to_string()));
        }
        params
    }
}

/// 指定新的Init容器信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateContainerGroupRequestInitContainerItem {
    /// 为容器内的进程授予某些特定的权限。目前仅支持配置为NET\_ADMIN和NET\_RAW。
    #[serde(rename = "SecurityContext.Capability.Add")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_context_capability_add: Option<Vec<String>>,
    /// Init容器镜像。
    #[serde(rename = "Image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// 挂载数据卷信息列表。
    #[serde(rename = "VolumeMount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_mount: Option<Vec<UpdateContainerGroupRequestInitContainerItemVolumeMountItem>>,
    /// 端口号。取值范围：1~65535。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<Vec<UpdateContainerGroupRequestInitContainerItemPortItem>>,
    /// 容器运行的根文件系统是否为只读。目前仅支持配置为true。
    #[serde(rename = "SecurityContext.ReadOnlyRootFilesystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_context_read_only_root_filesystem: Option<bool>,
    /// 容器的环境变量信息列表。
    #[serde(rename = "EnvironmentVar")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_var: Option<Vec<UpdateContainerGroupRequestInitContainerItemEnvironmentVarItem>>,
    /// 镜像拉取策略。取值范围：
    #[serde(rename = "ImagePullPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_pull_policy: Option<String>,
    /// 当标准输入为true时，标准输入流将在多个附加会话中保持开启状态。如果StdinOnce设为true，标准输入在容器开启时被打开，在首个客户端附加到标准输入之前都为空，然后会一直保持开启状态，接...
    #[serde(rename = "StdinOnce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdin_once: Option<bool>,
    /// Init容器vCPU大小。
    #[serde(rename = "Cpu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<f32>,
    /// 是否开启交互。默认为false，如果Command为/bin/bash类型时，需要设置为true。
    #[serde(rename = "Tty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,
    /// Init容器工作目录。
    #[serde(rename = "WorkingDir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,
    /// Init容器指令。
    #[serde(rename = "Command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    /// Init容器启动参数。
    #[serde(rename = "Arg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arg: Option<Vec<String>>,
    /// 设置运行容器的用户ID。
    #[serde(rename = "SecurityContext.RunAsUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_context_run_as_user: Option<i64>,
    /// 指定Init容器使用的GPU个数。
    #[serde(rename = "Gpu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu: Option<i32>,
    /// Init容器内存大小。
    #[serde(rename = "Memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<f32>,
    /// 此容器是否应在容器运行时为标准输入分配缓冲区。如果未设置，则容器中标准输入的读取值将导致EOF。默认为false。
    #[serde(rename = "Stdin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdin: Option<bool>,
    /// Init容器名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UpdateContainerGroupRequestInitContainerItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.security_context_capability_add {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("SecurityContext.Capability.Add.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.image {
            params.push(("Image".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.volume_mount {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("VolumeMount.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.port {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Port.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.security_context_read_only_root_filesystem {
            params.push(("SecurityContext.ReadOnlyRootFilesystem".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.environment_var {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("EnvironmentVar.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.image_pull_policy {
            params.push(("ImagePullPolicy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.stdin_once {
            params.push(("StdinOnce".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cpu {
            params.push(("Cpu".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tty {
            params.push(("Tty".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.working_dir {
            params.push(("WorkingDir".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.command {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Command.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.arg {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Arg.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.security_context_run_as_user {
            params.push(("SecurityContext.RunAsUser".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.gpu {
            params.push(("Gpu".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.memory {
            params.push(("Memory".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.stdin {
            params.push(("Stdin".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        params
    }
}

/// 镜像仓库凭证信息列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateContainerGroupRequestImageRegistryCredentialItem {
    /// 镜像仓库密码。
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// 不带`http://`或`https://`前缀的镜像仓库地址。
    #[serde(rename = "Server")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// 镜像仓库用户名。
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

impl UpdateContainerGroupRequestImageRegistryCredentialItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.password {
            params.push(("Password".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.server {
            params.push(("Server".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_name {
            params.push(("UserName".to_string(), v.to_string()));
        }
        params
    }
}

/// ACR企业版实例信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateContainerGroupRequestAcrRegistryInfoItem {
    /// ACR企业版实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// ACR企业版实例的名称。
    #[serde(rename = "InstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    /// ACR企业版实例所属地域。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// ACR企业版实例的域名。默认为相应实例的所有域名。支持指定个别域名，多个以半角逗号分隔。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<Vec<String>>,
}

impl UpdateContainerGroupRequestAcrRegistryInfoItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_name {
            params.push(("InstanceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Domain.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 实例标签信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupsRequestTagItem {
    /// 实例标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 实例标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DescribeContainerGroupsRequestTagItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key {
            params.push(("Key".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        params
    }
}

/// 自定义实例内一个容器的Hostname映射。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupsResponseContainerGroupsItemHostAliasesItem {
    /// 主机信息。
    #[serde(rename = "Hostnames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostnames: Option<Vec<String>>,
    /// IP地址。
    #[serde(rename = "Ip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

impl DescribeContainerGroupsResponseContainerGroupsItemHostAliasesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.hostnames {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Hostnames.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.ip {
            params.push(("Ip".to_string(), v.to_string()));
        }
        params
    }
}

/// 实例标签信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupsResponseContainerGroupsItemTagsItem {
    /// 标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DescribeContainerGroupsResponseContainerGroupsItemTagsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key {
            params.push(("Key".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        params
    }
}

/// 事件信息，上限50。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupsResponseContainerGroupsItemEventsItem {
    /// 事件类型，可能值：
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 事件结束时间。
    #[serde(rename = "LastTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_timestamp: Option<String>,
    /// 事件消息。
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 事件的归属对象名。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 事件名。
    #[serde(rename = "Reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// 事件数量。
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// 事件起始时间。
    #[serde(rename = "FirstTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_timestamp: Option<String>,
}

impl DescribeContainerGroupsResponseContainerGroupsItemEventsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.last_timestamp {
            params.push(("LastTimestamp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message {
            params.push(("Message".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.reason {
            params.push(("Reason".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.count {
            params.push(("Count".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.first_timestamp {
            params.push(("FirstTimestamp".to_string(), v.to_string()));
        }
        params
    }
}

/// TcpSocket。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupsResponseContainerGroupsItemContainersItemLivenessProbeTcpSocket {
    /// 主机名。
    #[serde(rename = "Host")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// 端口号。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}

impl DescribeContainerGroupsResponseContainerGroupsItemContainersItemLivenessProbeTcpSocket {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.host {
            params.push(("Host".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        params
    }
}

/// HttpGet。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupsResponseContainerGroupsItemContainersItemLivenessProbeHttpGet {
    /// HTTP或HTTPS。
    #[serde(rename = "Scheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
    /// HttpGet检测的路径。
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// HttpGet检测的端口。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}

impl DescribeContainerGroupsResponseContainerGroupsItemContainersItemLivenessProbeHttpGet {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.scheme {
            params.push(("Scheme".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.path {
            params.push(("Path".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        params
    }
}

/// 存活探针。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupsResponseContainerGroupsItemContainersItemLivenessProbe {
    /// 从上次检查失败后重新认定检查成功的检查次数阈值（必须是连续成功），默认为1。当前必须为1。
    #[serde(rename = "SuccessThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_threshold: Option<i32>,
    /// 检查开始执行的时间，以容器启动完成为起点计算。
    #[serde(rename = "InitialDelaySeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_delay_seconds: Option<i32>,
    /// 从上次检查成功后认定检查失败的检查次数阈值（必须是连续失败），默认为3。
    #[serde(rename = "FailureThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_threshold: Option<i32>,
    /// 检查超时的时间，默认为1秒，最小为1秒。
    #[serde(rename = "TimeoutSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i32>,
    /// TcpSocket。
    #[serde(rename = "TcpSocket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_socket: Option<DescribeContainerGroupsResponseContainerGroupsItemContainersItemLivenessProbeTcpSocket>,
    /// 执行命令。
    #[serde(rename = "Execs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execs: Option<Vec<String>>,
    /// HttpGet。
    #[serde(rename = "HttpGet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_get: Option<DescribeContainerGroupsResponseContainerGroupsItemContainersItemLivenessProbeHttpGet>,
    /// 检查执行的周期，默认为10秒，最小为1秒。
    #[serde(rename = "PeriodSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_seconds: Option<i32>,
}

impl DescribeContainerGroupsResponseContainerGroupsItemContainersItemLivenessProbe {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.success_threshold {
            params.push(("SuccessThreshold".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.initial_delay_seconds {
            params.push(("InitialDelaySeconds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.failure_threshold {
            params.push(("FailureThreshold".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.timeout_seconds {
            params.push(("TimeoutSeconds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tcp_socket {
            for (k, v2) in v.to_query_params() {
                params.push((format!("TcpSocket.{}", k), v2));
            }
        }
        if let Some(ref v) = self.execs {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Execs.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.http_get {
            for (k, v2) in v.to_query_params() {
                params.push((format!("HttpGet.{}", k), v2));
            }
        }
        if let Some(ref v) = self.period_seconds {
            params.push(("PeriodSeconds".to_string(), v.to_string()));
        }
        params
    }
}

/// 挂载的数据卷信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupsResponseContainerGroupsItemContainersItemVolumeMountsItem {
    /// 数据卷的挂载传播设置。挂载传播允许将Container挂载的卷共享到同一Pod中的其他Container，甚至可以共享到同一节点上的其他Pod。可能值：
    #[serde(rename = "MountPropagation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_propagation: Option<String>,
    /// 挂载目录，容器的挂载目录下的内容被Volume的内容直接覆盖。
    #[serde(rename = "MountPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_path: Option<String>,
    /// 是否只读。
    #[serde(rename = "ReadOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// 数据卷名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 数据卷下的子目录。方便实例将同一个数据卷下的不同目录挂载到容器的不同目录。
    #[serde(rename = "SubPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_path: Option<String>,
}

impl DescribeContainerGroupsResponseContainerGroupsItemContainersItemVolumeMountsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.mount_propagation {
            params.push(("MountPropagation".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.mount_path {
            params.push(("MountPath".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.read_only {
            params.push(("ReadOnly".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sub_path {
            params.push(("SubPath".to_string(), v.to_string()));
        }
        params
    }
}

/// 暴露端口和协议。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupsResponseContainerGroupsItemContainersItemPortsItem {
    /// 协议类型。
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// 端口号。可能值：1~65535。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}

impl DescribeContainerGroupsResponseContainerGroupsItemContainersItemPortsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.protocol {
            params.push(("Protocol".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        params
    }
}

/// 上一次状态。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupsResponseContainerGroupsItemContainersItemPreviousState {
    /// 容器运行开始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 容器运行结束时间。
    #[serde(rename = "FinishTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_time: Option<String>,
    /// 状态详情。
    #[serde(rename = "DetailStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_status: Option<String>,
    /// 容器状态。可能值：
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// 容器状态信息。
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 容器状态信号。
    #[serde(rename = "Signal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal: Option<i32>,
    /// 容器运行退出码。
    #[serde(rename = "ExitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    /// 容器状态Reason。
    #[serde(rename = "Reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl DescribeContainerGroupsResponseContainerGroupsItemContainersItemPreviousState {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.finish_time {
            params.push(("FinishTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.detail_status {
            params.push(("DetailStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.state {
            params.push(("State".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message {
            params.push(("Message".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.signal {
            params.push(("Signal".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.exit_code {
            params.push(("ExitCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.reason {
            params.push(("Reason".to_string(), v.to_string()));
        }
        params
    }
}

/// 容器当前状态。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupsResponseContainerGroupsItemContainersItemCurrentState {
    /// 容器运行开始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 容器运行结束时间。
    #[serde(rename = "FinishTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_time: Option<String>,
    /// 容器状态详情。
    #[serde(rename = "DetailStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_status: Option<String>,
    /// 容器状态，可能值：
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// 容器状态信息。
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 容器状态信号。
    #[serde(rename = "Signal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal: Option<i32>,
    /// 容器运行退出码。
    #[serde(rename = "ExitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    /// 容器状态Reason。
    #[serde(rename = "Reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl DescribeContainerGroupsResponseContainerGroupsItemContainersItemCurrentState {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.finish_time {
            params.push(("FinishTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.detail_status {
            params.push(("DetailStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.state {
            params.push(("State".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message {
            params.push(("Message".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.signal {
            params.push(("Signal".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.exit_code {
            params.push(("ExitCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.reason {
            params.push(("Reason".to_string(), v.to_string()));
        }
        params
    }
}

/// 容器内的进程所具备的特定权限。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupsResponseContainerGroupsItemContainersItemSecurityContextCapability {
    /// 容器内的进程所具备的特定权限。
    #[serde(rename = "Adds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adds: Option<Vec<String>>,
}

impl DescribeContainerGroupsResponseContainerGroupsItemContainersItemSecurityContextCapability {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.adds {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Adds.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 实例运行的安全上下文。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupsResponseContainerGroupsItemContainersItemSecurityContext {
    /// 是否只读根文件系统，目前仅支持True。
    #[serde(rename = "ReadOnlyRootFilesystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only_root_filesystem: Option<bool>,
    /// 运行容器的用户ID。
    #[serde(rename = "RunAsUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_as_user: Option<i64>,
    /// 容器内的进程所具备的特定权限。
    #[serde(rename = "Capability")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability: Option<DescribeContainerGroupsResponseContainerGroupsItemContainersItemSecurityContextCapability>,
}

impl DescribeContainerGroupsResponseContainerGroupsItemContainersItemSecurityContext {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.read_only_root_filesystem {
            params.push(("ReadOnlyRootFilesystem".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.run_as_user {
            params.push(("RunAsUser".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.capability {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Capability.{}", k), v2));
            }
        }
        params
    }
}

/// 指定字段。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupsResponseContainerGroupsItemContainersItemEnvironmentVarsItemValueFromFieldRef {
    /// 字段的路径。
    #[serde(rename = "FieldPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_path: Option<String>,
}

impl DescribeContainerGroupsResponseContainerGroupsItemContainersItemEnvironmentVarsItemValueFromFieldRef {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.field_path {
            params.push(("FieldPath".to_string(), v.to_string()));
        }
        params
    }
}

/// 环境变量值的来源。 如果值不为空，则无法使用。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupsResponseContainerGroupsItemContainersItemEnvironmentVarsItemValueFrom {
    /// 指定字段。
    #[serde(rename = "FieldRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_ref: Option<DescribeContainerGroupsResponseContainerGroupsItemContainersItemEnvironmentVarsItemValueFromFieldRef>,
}

impl DescribeContainerGroupsResponseContainerGroupsItemContainersItemEnvironmentVarsItemValueFrom {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.field_ref {
            for (k, v2) in v.to_query_params() {
                params.push((format!("FieldRef.{}", k), v2));
            }
        }
        params
    }
}

/// 环境变量。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupsResponseContainerGroupsItemContainersItemEnvironmentVarsItem {
    /// 环境变量名。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 环境变量值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 环境变量值的来源。 如果值不为空，则无法使用。
    #[serde(rename = "ValueFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_from: Option<DescribeContainerGroupsResponseContainerGroupsItemContainersItemEnvironmentVarsItemValueFrom>,
}

impl DescribeContainerGroupsResponseContainerGroupsItemContainersItemEnvironmentVarsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key {
            params.push(("Key".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value_from {
            for (k, v2) in v.to_query_params() {
                params.push((format!("ValueFrom.{}", k), v2));
            }
        }
        params
    }
}

/// 使用TCP Socket方式进行健康检查。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupsResponseContainerGroupsItemContainersItemReadinessProbeTcpSocket {
    /// Host。
    #[serde(rename = "Host")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// 端口号。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}

impl DescribeContainerGroupsResponseContainerGroupsItemContainersItemReadinessProbeTcpSocket {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.host {
            params.push(("Host".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        params
    }
}

/// 使用HTTP请求方式进行健康检查。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupsResponseContainerGroupsItemContainersItemReadinessProbeHttpGet {
    /// HTTP／HTTPS。
    #[serde(rename = "Scheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
    /// HttpGet检测的路径。
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// HttpGet检测的端口号。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}

impl DescribeContainerGroupsResponseContainerGroupsItemContainersItemReadinessProbeHttpGet {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.scheme {
            params.push(("Scheme".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.path {
            params.push(("Path".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        params
    }
}

/// 应用业务探针（即就绪探针），用于检查容器是否已经就绪，可以为请求提供服务。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupsResponseContainerGroupsItemContainersItemReadinessProbe {
    /// 从上次检查失败后重新认定检查成功的检查次数阈值（必须是连续成功），默认为1。当前必须为1。
    #[serde(rename = "SuccessThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_threshold: Option<i32>,
    /// 检查开始执行的时间，以容器启动完成为起点计算。
    #[serde(rename = "InitialDelaySeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_delay_seconds: Option<i32>,
    /// 从上次检查成功后认定检查失败的检查次数阈值（必须是连续失败），默认为3。
    #[serde(rename = "FailureThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_threshold: Option<i32>,
    /// 检查超时的时间，默认为1秒，最小为1秒。
    #[serde(rename = "TimeoutSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i32>,
    /// 使用TCP Socket方式进行健康检查。
    #[serde(rename = "TcpSocket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_socket: Option<DescribeContainerGroupsResponseContainerGroupsItemContainersItemReadinessProbeTcpSocket>,
    /// 使用命令行方式进行健康检查时，在容器内执行的命令。
    #[serde(rename = "Execs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execs: Option<Vec<String>>,
    /// 使用HTTP请求方式进行健康检查。
    #[serde(rename = "HttpGet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_get: Option<DescribeContainerGroupsResponseContainerGroupsItemContainersItemReadinessProbeHttpGet>,
    /// 检查执行的周期，默认为10秒，最小为1秒。
    #[serde(rename = "PeriodSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_seconds: Option<i32>,
}

impl DescribeContainerGroupsResponseContainerGroupsItemContainersItemReadinessProbe {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.success_threshold {
            params.push(("SuccessThreshold".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.initial_delay_seconds {
            params.push(("InitialDelaySeconds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.failure_threshold {
            params.push(("FailureThreshold".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.timeout_seconds {
            params.push(("TimeoutSeconds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tcp_socket {
            for (k, v2) in v.to_query_params() {
                params.push((format!("TcpSocket.{}", k), v2));
            }
        }
        if let Some(ref v) = self.execs {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Execs.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.http_get {
            for (k, v2) in v.to_query_params() {
                params.push((format!("HttpGet.{}", k), v2));
            }
        }
        if let Some(ref v) = self.period_seconds {
            params.push(("PeriodSeconds".to_string(), v.to_string()));
        }
        params
    }
}

/// 实例包含的容器列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupsResponseContainerGroupsItemContainersItem {
    /// 存活探针。
    #[serde(rename = "LivenessProbe")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liveness_probe: Option<DescribeContainerGroupsResponseContainerGroupsItemContainersItemLivenessProbe>,
    /// 容器启动命令。
    #[serde(rename = "Commands")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commands: Option<Vec<String>>,
    /// 挂载的数据卷信息。
    #[serde(rename = "VolumeMounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_mounts: Option<Vec<DescribeContainerGroupsResponseContainerGroupsItemContainersItemVolumeMountsItem>>,
    /// 容器启动参数。
    #[serde(rename = "Args")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    /// 容器镜像。
    #[serde(rename = "Image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// 暴露端口和协议。
    #[serde(rename = "Ports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<DescribeContainerGroupsResponseContainerGroupsItemContainersItemPortsItem>>,
    /// 重启次数。
    #[serde(rename = "RestartCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_count: Option<i32>,
    /// 镜像拉取策略。可能值：
    #[serde(rename = "ImagePullPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_pull_policy: Option<String>,
    /// 当标准输入为true时，标准输入流将在多个附加会话中保持开启状态。如果StdinOnce设为true，标准输入在容器开启时被打开，在首个客户端附加到标准输入之前都为空，然后会一直保持开启状态，接...
    #[serde(rename = "StdinOnce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdin_once: Option<bool>,
    /// 容器的vCPU大小。
    #[serde(rename = "Cpu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<f32>,
    /// 上一次状态。
    #[serde(rename = "PreviousState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_state: Option<DescribeContainerGroupsResponseContainerGroupsItemContainersItemPreviousState>,
    /// 是否开启交互。默认为false。例如：当Command配置为`/bin/bash`时，需要配置为true。
    #[serde(rename = "Tty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,
    /// 工作目录。
    #[serde(rename = "WorkingDir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,
    /// 容器当前状态。
    #[serde(rename = "CurrentState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_state: Option<DescribeContainerGroupsResponseContainerGroupsItemContainersItemCurrentState>,
    /// 指定容器是否已通过其就绪探针。
    #[serde(rename = "Ready")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready: Option<bool>,
    /// GPU个数。
    #[serde(rename = "Gpu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu: Option<i32>,
    /// 实例运行的安全上下文。
    #[serde(rename = "SecurityContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_context: Option<DescribeContainerGroupsResponseContainerGroupsItemContainersItemSecurityContext>,
    /// 容器内存大小。单位为GiB。
    #[serde(rename = "Memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<f32>,
    /// 此容器是否应在容器运行时为标准输入分配缓冲区。如果未设置，则容器中标准输入的读取值将导致EOF。默认为false。
    #[serde(rename = "Stdin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdin: Option<bool>,
    /// 容器名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 环境变量。
    #[serde(rename = "EnvironmentVars")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_vars: Option<Vec<DescribeContainerGroupsResponseContainerGroupsItemContainersItemEnvironmentVarsItem>>,
    /// 应用业务探针（即就绪探针），用于检查容器是否已经就绪，可以为请求提供服务。
    #[serde(rename = "ReadinessProbe")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readiness_probe: Option<DescribeContainerGroupsResponseContainerGroupsItemContainersItemReadinessProbe>,
}

impl DescribeContainerGroupsResponseContainerGroupsItemContainersItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.liveness_probe {
            for (k, v2) in v.to_query_params() {
                params.push((format!("LivenessProbe.{}", k), v2));
            }
        }
        if let Some(ref v) = self.commands {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Commands.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.volume_mounts {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("VolumeMounts.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.args {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Args.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.image {
            params.push(("Image".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ports {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Ports.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.restart_count {
            params.push(("RestartCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.image_pull_policy {
            params.push(("ImagePullPolicy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.stdin_once {
            params.push(("StdinOnce".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cpu {
            params.push(("Cpu".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.previous_state {
            for (k, v2) in v.to_query_params() {
                params.push((format!("PreviousState.{}", k), v2));
            }
        }
        if let Some(ref v) = self.tty {
            params.push(("Tty".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.working_dir {
            params.push(("WorkingDir".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.current_state {
            for (k, v2) in v.to_query_params() {
                params.push((format!("CurrentState.{}", k), v2));
            }
        }
        if let Some(ref v) = self.ready {
            params.push(("Ready".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.gpu {
            params.push(("Gpu".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.security_context {
            for (k, v2) in v.to_query_params() {
                params.push((format!("SecurityContext.{}", k), v2));
            }
        }
        if let Some(ref v) = self.memory {
            params.push(("Memory".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.stdin {
            params.push(("Stdin".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.environment_vars {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("EnvironmentVars.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.readiness_probe {
            for (k, v2) in v.to_query_params() {
                params.push((format!("ReadinessProbe.{}", k), v2));
            }
        }
        params
    }
}

/// Init容器挂载的数据卷信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupsResponseContainerGroupsItemInitContainersItemVolumeMountsItem {
    /// 数据卷的挂载传播设置。挂载传播允许将Container挂载的卷共享到同一Pod中的其他Container，甚至可以共享到同一节点上的其他Pod。可能值：
    #[serde(rename = "MountPropagation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_propagation: Option<String>,
    /// 挂载目录，容器的挂载目录下的内容被Volume的内容直接覆盖。
    #[serde(rename = "MountPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_path: Option<String>,
    /// 是否只读。
    #[serde(rename = "ReadOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// 数据卷名称，同 Volume 中的 Name。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl DescribeContainerGroupsResponseContainerGroupsItemInitContainersItemVolumeMountsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.mount_propagation {
            params.push(("MountPropagation".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.mount_path {
            params.push(("MountPath".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.read_only {
            params.push(("ReadOnly".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        params
    }
}

/// 暴露端口和协议。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupsResponseContainerGroupsItemInitContainersItemPortsItem {
    /// 协议类型。
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// 端口号。可能值：1~65535。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}

impl DescribeContainerGroupsResponseContainerGroupsItemInitContainersItemPortsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.protocol {
            params.push(("Protocol".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        params
    }
}

/// 上一次状态。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupsResponseContainerGroupsItemInitContainersItemPreviousState {
    /// 容器运行开始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 容器运行结束时间。
    #[serde(rename = "FinishTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_time: Option<String>,
    /// 状态详情。
    #[serde(rename = "DetailStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_status: Option<String>,
    /// 容器状态，枚举值：Waiting，Running，Terminated。
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// 容器状态信息。
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 容器状态信号。
    #[serde(rename = "Signal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal: Option<i32>,
    /// 容器运行退出码。
    #[serde(rename = "ExitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    /// 容器状态Reason。
    #[serde(rename = "Reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl DescribeContainerGroupsResponseContainerGroupsItemInitContainersItemPreviousState {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.finish_time {
            params.push(("FinishTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.detail_status {
            params.push(("DetailStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.state {
            params.push(("State".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message {
            params.push(("Message".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.signal {
            params.push(("Signal".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.exit_code {
            params.push(("ExitCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.reason {
            params.push(("Reason".to_string(), v.to_string()));
        }
        params
    }
}

/// 当前状态。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupsResponseContainerGroupsItemInitContainersItemCurrentState {
    /// 容器运行开始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 容器运行结束时间。
    #[serde(rename = "FinishTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_time: Option<String>,
    /// 状态详情。
    #[serde(rename = "DetailStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_status: Option<String>,
    /// 容器状态，可能值：
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// 事件消息。
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 容器状态信号。
    #[serde(rename = "Signal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal: Option<i32>,
    /// 容器退出码。
    #[serde(rename = "ExitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    /// 容器状态Reason。
    #[serde(rename = "Reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl DescribeContainerGroupsResponseContainerGroupsItemInitContainersItemCurrentState {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.finish_time {
            params.push(("FinishTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.detail_status {
            params.push(("DetailStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.state {
            params.push(("State".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message {
            params.push(("Message".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.signal {
            params.push(("Signal".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.exit_code {
            params.push(("ExitCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.reason {
            params.push(("Reason".to_string(), v.to_string()));
        }
        params
    }
}

/// 容器内的进程所具备的特定权限。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupsResponseContainerGroupsItemInitContainersItemSecurityContextCapability {
    /// 容器内的进程所具备的特定权限。
    #[serde(rename = "Adds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adds: Option<Vec<String>>,
}

impl DescribeContainerGroupsResponseContainerGroupsItemInitContainersItemSecurityContextCapability {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.adds {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Adds.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 容器运行的安全上下文。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupsResponseContainerGroupsItemInitContainersItemSecurityContext {
    /// 是否只读根文件系统，目前仅支持True。
    #[serde(rename = "ReadOnlyRootFilesystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only_root_filesystem: Option<bool>,
    /// 用于运行容器进程入口点的UID。
    #[serde(rename = "RunAsUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_as_user: Option<i64>,
    /// 容器内的进程所具备的特定权限。
    #[serde(rename = "Capability")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability: Option<DescribeContainerGroupsResponseContainerGroupsItemInitContainersItemSecurityContextCapability>,
}

impl DescribeContainerGroupsResponseContainerGroupsItemInitContainersItemSecurityContext {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.read_only_root_filesystem {
            params.push(("ReadOnlyRootFilesystem".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.run_as_user {
            params.push(("RunAsUser".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.capability {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Capability.{}", k), v2));
            }
        }
        params
    }
}

/// 指定的字段。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupsResponseContainerGroupsItemInitContainersItemEnvironmentVarsItemValueFromFieldRef {
    /// 指定的版本中选择字段的路径。目前只支持 `status.podIP`。
    #[serde(rename = "FieldPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_path: Option<String>,
}

impl DescribeContainerGroupsResponseContainerGroupsItemInitContainersItemEnvironmentVarsItemValueFromFieldRef {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.field_path {
            params.push(("FieldPath".to_string(), v.to_string()));
        }
        params
    }
}

/// 环境变量值的来源。 如果值不为空，则无法使用。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupsResponseContainerGroupsItemInitContainersItemEnvironmentVarsItemValueFrom {
    /// 指定的字段。
    #[serde(rename = "FieldRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_ref: Option<DescribeContainerGroupsResponseContainerGroupsItemInitContainersItemEnvironmentVarsItemValueFromFieldRef>,
}

impl DescribeContainerGroupsResponseContainerGroupsItemInitContainersItemEnvironmentVarsItemValueFrom {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.field_ref {
            for (k, v2) in v.to_query_params() {
                params.push((format!("FieldRef.{}", k), v2));
            }
        }
        params
    }
}

/// 环境变量。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupsResponseContainerGroupsItemInitContainersItemEnvironmentVarsItem {
    /// 环境变量名。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 环境变量值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 环境变量值的来源。 如果值不为空，则无法使用。
    #[serde(rename = "ValueFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_from: Option<DescribeContainerGroupsResponseContainerGroupsItemInitContainersItemEnvironmentVarsItemValueFrom>,
}

impl DescribeContainerGroupsResponseContainerGroupsItemInitContainersItemEnvironmentVarsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key {
            params.push(("Key".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value_from {
            for (k, v2) in v.to_query_params() {
                params.push((format!("ValueFrom.{}", k), v2));
            }
        }
        params
    }
}

/// Init容器列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupsResponseContainerGroupsItemInitContainersItem {
    /// Init容器挂载的数据卷信息。
    #[serde(rename = "VolumeMounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_mounts: Option<Vec<DescribeContainerGroupsResponseContainerGroupsItemInitContainersItemVolumeMountsItem>>,
    /// 启动参数。
    #[serde(rename = "Args")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    /// 镜像。
    #[serde(rename = "Image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// 暴露端口和协议。
    #[serde(rename = "Ports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<DescribeContainerGroupsResponseContainerGroupsItemInitContainersItemPortsItem>>,
    /// 重启次数。
    #[serde(rename = "RestartCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_count: Option<i32>,
    /// 镜像拉取策略。可能值：
    #[serde(rename = "ImagePullPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_pull_policy: Option<String>,
    /// 上一次状态。
    #[serde(rename = "PreviousState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_state: Option<DescribeContainerGroupsResponseContainerGroupsItemInitContainersItemPreviousState>,
    /// 工作目录。
    #[serde(rename = "WorkingDir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,
    /// vCPU大小。
    #[serde(rename = "Cpu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<f32>,
    /// 当前状态。
    #[serde(rename = "CurrentState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_state: Option<DescribeContainerGroupsResponseContainerGroupsItemInitContainersItemCurrentState>,
    /// 启动命令。
    #[serde(rename = "Command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    /// 指定容器是否已通过其就绪探针。
    #[serde(rename = "Ready")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready: Option<bool>,
    /// GPU个数。
    #[serde(rename = "Gpu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu: Option<i32>,
    /// 容器运行的安全上下文。
    #[serde(rename = "SecurityContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_context: Option<DescribeContainerGroupsResponseContainerGroupsItemInitContainersItemSecurityContext>,
    /// Init容器内存大小。单位为GiB。
    #[serde(rename = "Memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<f32>,
    /// 容器名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 环境变量。
    #[serde(rename = "EnvironmentVars")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_vars: Option<Vec<DescribeContainerGroupsResponseContainerGroupsItemInitContainersItemEnvironmentVarsItem>>,
}

impl DescribeContainerGroupsResponseContainerGroupsItemInitContainersItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.volume_mounts {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("VolumeMounts.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.args {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Args.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.image {
            params.push(("Image".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ports {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Ports.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.restart_count {
            params.push(("RestartCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.image_pull_policy {
            params.push(("ImagePullPolicy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.previous_state {
            for (k, v2) in v.to_query_params() {
                params.push((format!("PreviousState.{}", k), v2));
            }
        }
        if let Some(ref v) = self.working_dir {
            params.push(("WorkingDir".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cpu {
            params.push(("Cpu".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.current_state {
            for (k, v2) in v.to_query_params() {
                params.push((format!("CurrentState.{}", k), v2));
            }
        }
        if let Some(ref v) = self.command {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Command.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.ready {
            params.push(("Ready".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.gpu {
            params.push(("Gpu".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.security_context {
            for (k, v2) in v.to_query_params() {
                params.push((format!("SecurityContext.{}", k), v2));
            }
        }
        if let Some(ref v) = self.memory {
            params.push(("Memory".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.environment_vars {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("EnvironmentVars.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 对象选项列表，每个对象由Name和Value（可选）构成。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupsResponseContainerGroupsItemDnsConfigOptionsItem {
    /// 对象变量值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 对象变量名。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl DescribeContainerGroupsResponseContainerGroupsItemDnsConfigOptionsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        params
    }
}

/// DNS配置信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupsResponseContainerGroupsItemDnsConfig {
    /// DNS搜索域列表。
    #[serde(rename = "Searches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searches: Option<Vec<String>>,
    /// 对象选项列表，每个对象由Name和Value（可选）构成。
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<DescribeContainerGroupsResponseContainerGroupsItemDnsConfigOptionsItem>>,
    /// DNS服务器IP地址列表。
    #[serde(rename = "NameServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_servers: Option<Vec<String>>,
}

impl DescribeContainerGroupsResponseContainerGroupsItemDnsConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.searches {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Searches.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.options {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Options.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.name_servers {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("NameServers.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 配置文件路径列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupsResponseContainerGroupsItemVolumesItemConfigFileVolumeConfigFileToPathsItem {
    /// 相对文件路径。
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// 配置文件内容 (32KB)。
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

impl DescribeContainerGroupsResponseContainerGroupsItemVolumesItemConfigFileVolumeConfigFileToPathsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.path {
            params.push(("Path".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.content {
            params.push(("Content".to_string(), v.to_string()));
        }
        params
    }
}

/// 数据卷信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupsResponseContainerGroupsItemVolumesItem {
    /// 数据卷类型，可能值：
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// DiskVolume的ID。
    #[serde(rename = "DiskVolumeDiskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_volume_disk_id: Option<String>,
    /// 是否只读。
    #[serde(rename = "NFSVolumeReadOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nfs_volume_read_only: Option<bool>,
    /// 配置文件路径列表。
    #[serde(rename = "ConfigFileVolumeConfigFileToPaths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_file_volume_config_file_to_paths: Option<Vec<DescribeContainerGroupsResponseContainerGroupsItemVolumesItemConfigFileVolumeConfigFileToPathsItem>>,
    /// 挂载的文件系统类型，默认取决于FlexVolume的script。
    #[serde(rename = "FlexVolumeFsType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flex_volume_fs_type: Option<String>,
    /// 用于FlexVolume的驱动程序名称。
    #[serde(rename = "FlexVolumeDriver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flex_volume_driver: Option<String>,
    /// 数据卷类型。
    #[serde(rename = "DiskVolumeFsType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_volume_fs_type: Option<String>,
    /// FlexVolume对象选项列表。
    #[serde(rename = "FlexVolumeOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flex_volume_options: Option<String>,
    /// NFS服务器地址。
    #[serde(rename = "NFSVolumeServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nfs_volume_server: Option<String>,
    /// NFS数据卷路径。
    #[serde(rename = "NFSVolumePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nfs_volume_path: Option<String>,
    /// 数据卷名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// EmptyDir数据卷的存储媒介。默认为空，表示使用node文件系统。支持配置为以下值：
    #[serde(rename = "EmptyDirVolumeMedium")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_dir_volume_medium: Option<String>,
    /// EmptyDir数据卷的大小。
    #[serde(rename = "EmptyDirVolumeSizeLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_dir_volume_size_limit: Option<String>,
}

impl DescribeContainerGroupsResponseContainerGroupsItemVolumesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.disk_volume_disk_id {
            params.push(("DiskVolumeDiskId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.nfs_volume_read_only {
            params.push(("NFSVolumeReadOnly".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.config_file_volume_config_file_to_paths {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("ConfigFileVolumeConfigFileToPaths.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.flex_volume_fs_type {
            params.push(("FlexVolumeFsType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.flex_volume_driver {
            params.push(("FlexVolumeDriver".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.disk_volume_fs_type {
            params.push(("DiskVolumeFsType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.flex_volume_options {
            params.push(("FlexVolumeOptions".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.nfs_volume_server {
            params.push(("NFSVolumeServer".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.nfs_volume_path {
            params.push(("NFSVolumePath".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.empty_dir_volume_medium {
            params.push(("EmptyDirVolumeMedium".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.empty_dir_volume_size_limit {
            params.push(("EmptyDirVolumeSizeLimit".to_string(), v.to_string()));
        }
        params
    }
}

/// sysctl参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupsResponseContainerGroupsItemEciSecurityContextSysctlsItem {
    /// sysctl参数值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// sysctl参数名。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl DescribeContainerGroupsResponseContainerGroupsItemEciSecurityContextSysctlsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        params
    }
}

/// 容器组运行的安全上下文。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupsResponseContainerGroupsItemEciSecurityContext {
    /// sysctl参数。
    #[serde(rename = "Sysctls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sysctls: Option<Vec<DescribeContainerGroupsResponseContainerGroupsItemEciSecurityContextSysctlsItem>>,
}

impl DescribeContainerGroupsResponseContainerGroupsItemEciSecurityContext {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.sysctls {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Sysctls.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 实例信息列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupsResponseContainerGroupsItem {
    /// 实例状态。可能值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 接到请求后的系统创建时间。UTC时间，RFC3339标准。
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// 实例所属的VPC ID。
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 公网IP。
    #[serde(rename = "InternetIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internet_ip: Option<String>,
    /// 该参数未开放使用。
    #[serde(rename = "TenantSecurityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_security_group_id: Option<String>,
    /// 安全组ID。
    #[serde(rename = "SecurityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_id: Option<String>,
    /// 自定义实例内一个容器的Hostname映射。
    #[serde(rename = "HostAliases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_aliases: Option<Vec<DescribeContainerGroupsResponseContainerGroupsItemHostAliasesItem>>,
    /// 实例标签信息。
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<DescribeContainerGroupsResponseContainerGroupsItemTagsItem>>,
    /// 事件信息，上限50。
    #[serde(rename = "Events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<DescribeContainerGroupsResponseContainerGroupsItemEventsItem>>,
    /// 容器全部成功退出后的时间。UTC时间，RFC3339标准。
    #[serde(rename = "SucceededTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub succeeded_time: Option<String>,
    /// 实例的抢占策略。取值范围：
    #[serde(rename = "SpotStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_strategy: Option<String>,
    /// 抢占式实例的每小时最高价格。
    #[serde(rename = "SpotPriceLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_price_limit: Option<f64>,
    /// 临时存储空间大小。单位为：GiB。
    #[serde(rename = "EphemeralStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral_storage: Option<i32>,
    /// 该参数未开放使用。
    #[serde(rename = "TenantEniInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_eni_instance_id: Option<String>,
    /// 折扣。
    #[serde(rename = "Discount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<i32>,
    /// 容器组的重启策略。取值范围：
    #[serde(rename = "RestartPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_policy: Option<String>,
    /// 实例级别内存大小。单位为GiB。
    #[serde(rename = "Memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<f32>,
    /// 该参数未开放使用。
    #[serde(rename = "TenantVSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_v_switch_id: Option<String>,
    /// 实例包含的容器列表。
    #[serde(rename = "Containers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<DescribeContainerGroupsResponseContainerGroupsItemContainersItem>>,
    /// 弹性网卡ID。
    #[serde(rename = "EniInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eni_instance_id: Option<String>,
    /// Init容器列表。
    #[serde(rename = "InitContainers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_containers: Option<Vec<DescribeContainerGroupsResponseContainerGroupsItemInitContainersItem>>,
    /// 实例ID。
    #[serde(rename = "ContainerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_group_id: Option<String>,
    /// 该参数未开放使用。
    #[serde(rename = "TenantEniIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_eni_ip: Option<String>,
    /// 指定的ECS实例规格。
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// 内网IP。
    #[serde(rename = "IntranetIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intranet_ip: Option<String>,
    /// IPv6地址。
    #[serde(rename = "Ipv6Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_address: Option<String>,
    /// 实例所属的地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// DNS配置信息。
    #[serde(rename = "DnsConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_config: Option<DescribeContainerGroupsResponseContainerGroupsItemDnsConfig>,
    /// 数据卷信息。
    #[serde(rename = "Volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<DescribeContainerGroupsResponseContainerGroupsItemVolumesItem>>,
    /// 实例RAM角色名称，ECI与ECS共用实例RAM角色，具体操作，请参见[通过API使用实例RAM角色](~~61178~~)。
    #[serde(rename = "RamRoleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ram_role_name: Option<String>,
    /// 虚拟交换机ID。
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 实例级别vCPU大小。
    #[serde(rename = "Cpu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<f32>,
    /// 由于账户欠费导致实例运行失败的时间。UTC时间，RFC3339标准。
    #[serde(rename = "ExpiredTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_time: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 实例所属的可用区。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 实例名称。
    #[serde(rename = "ContainerGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_group_name: Option<String>,
    /// 容器组运行的安全上下文。
    #[serde(rename = "EciSecurityContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eci_security_context: Option<DescribeContainerGroupsResponseContainerGroupsItemEciSecurityContext>,
    /// 实例运行失败的时间。UTC时间，RFC 3339标准。
    #[serde(rename = "FailedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_time: Option<String>,
    /// 算力类别。可能值：
    #[serde(rename = "ComputeCategory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_category: Option<String>,
    /// DNS策略。可能值
    #[serde(rename = "DnsPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_policy: Option<String>,
}

impl DescribeContainerGroupsResponseContainerGroupsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.creation_time {
            params.push(("CreationTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_id {
            params.push(("VpcId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.internet_ip {
            params.push(("InternetIp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tenant_security_group_id {
            params.push(("TenantSecurityGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.security_group_id {
            params.push(("SecurityGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.host_aliases {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("HostAliases.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.tags {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tags.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.events {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Events.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.succeeded_time {
            params.push(("SucceededTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.spot_strategy {
            params.push(("SpotStrategy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.spot_price_limit {
            params.push(("SpotPriceLimit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ephemeral_storage {
            params.push(("EphemeralStorage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tenant_eni_instance_id {
            params.push(("TenantEniInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.discount {
            params.push(("Discount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.restart_policy {
            params.push(("RestartPolicy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.memory {
            params.push(("Memory".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tenant_v_switch_id {
            params.push(("TenantVSwitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.containers {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Containers.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.eni_instance_id {
            params.push(("EniInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.init_containers {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("InitContainers.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.container_group_id {
            params.push(("ContainerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tenant_eni_ip {
            params.push(("TenantEniIp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_type {
            params.push(("InstanceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.intranet_ip {
            params.push(("IntranetIp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ipv6_address {
            params.push(("Ipv6Address".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dns_config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("DnsConfig.{}", k), v2));
            }
        }
        if let Some(ref v) = self.volumes {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Volumes.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.ram_role_name {
            params.push(("RamRoleName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("VSwitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cpu {
            params.push(("Cpu".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.expired_time {
            params.push(("ExpiredTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.container_group_name {
            params.push(("ContainerGroupName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.eci_security_context {
            for (k, v2) in v.to_query_params() {
                params.push((format!("EciSecurityContext.{}", k), v2));
            }
        }
        if let Some(ref v) = self.failed_time {
            params.push(("FailedTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.compute_category {
            params.push(("ComputeCategory".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dns_policy {
            params.push(("DnsPolicy".to_string(), v.to_string()));
        }
        params
    }
}

/// ECI实例绑定的标签。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupStatusRequestTagItem {
    /// ECI实例的标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// ECI实例的标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

impl DescribeContainerGroupStatusRequestTagItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key {
            params.push(("Key".to_string(), v.to_string()));
        }
        params
    }
}

/// Pod conditions信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupStatusResponseDataItemPodStatusConditionsItem {
    /// condition类型。可能值：
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// condition状态。
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 事件原因。
    #[serde(rename = "Reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// 事件消息。
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 最后一次状态变化时间。
    #[serde(rename = "lastTransitionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<String>,
}

impl DescribeContainerGroupStatusResponseDataItemPodStatusConditionsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.reason {
            params.push(("Reason".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message {
            params.push(("Message".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.last_transition_time {
            params.push(("lastTransitionTime".to_string(), v.to_string()));
        }
        params
    }
}

/// 容器正在等待创建，还未开始运行。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupStatusResponseDataItemPodStatusContainerStatusesItemStateWaiting {
    /// 事件原因。
    #[serde(rename = "Reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// 事件消息。
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl DescribeContainerGroupStatusResponseDataItemPodStatusContainerStatusesItemStateWaiting {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.reason {
            params.push(("Reason".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message {
            params.push(("Message".to_string(), v.to_string()));
        }
        params
    }
}

/// 容器已经成功创建，并且正在运行。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupStatusResponseDataItemPodStatusContainerStatusesItemStateRunning {
    /// 开始时间。
    #[serde(rename = "StartedAtstartedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_atstarted_at: Option<String>,
}

impl DescribeContainerGroupStatusResponseDataItemPodStatusContainerStatusesItemStateRunning {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.started_atstarted_at {
            params.push(("StartedAtstartedAt".to_string(), v.to_string()));
        }
        params
    }
}

/// 容器运行终止并退出，包括运行成功终止和运行失败终止。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupStatusResponseDataItemPodStatusContainerStatusesItemStateTerminated {
    /// 容器ID。
    #[serde(rename = "ContainerID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,
    /// 退出码。
    #[serde(rename = "ExitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    /// 结束时间。
    #[serde(rename = "FinishedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<String>,
    /// 开始时间。
    #[serde(rename = "StartedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    /// 信号码。
    #[serde(rename = "Signal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal: Option<i32>,
    /// 事件原因。
    #[serde(rename = "Reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// 事件消息。
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl DescribeContainerGroupStatusResponseDataItemPodStatusContainerStatusesItemStateTerminated {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.container_id {
            params.push(("ContainerID".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.exit_code {
            params.push(("ExitCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.finished_at {
            params.push(("FinishedAt".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.started_at {
            params.push(("StartedAt".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.signal {
            params.push(("Signal".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.reason {
            params.push(("Reason".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message {
            params.push(("Message".to_string(), v.to_string()));
        }
        params
    }
}

/// 容器状态。包括：
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupStatusResponseDataItemPodStatusContainerStatusesItemState {
    /// 容器正在等待创建，还未开始运行。
    #[serde(rename = "Waiting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waiting: Option<DescribeContainerGroupStatusResponseDataItemPodStatusContainerStatusesItemStateWaiting>,
    /// 容器已经成功创建，并且正在运行。
    #[serde(rename = "Running")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running: Option<DescribeContainerGroupStatusResponseDataItemPodStatusContainerStatusesItemStateRunning>,
    /// 容器运行终止并退出，包括运行成功终止和运行失败终止。
    #[serde(rename = "Terminated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminated: Option<DescribeContainerGroupStatusResponseDataItemPodStatusContainerStatusesItemStateTerminated>,
}

impl DescribeContainerGroupStatusResponseDataItemPodStatusContainerStatusesItemState {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.waiting {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Waiting.{}", k), v2));
            }
        }
        if let Some(ref v) = self.running {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Running.{}", k), v2));
            }
        }
        if let Some(ref v) = self.terminated {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Terminated.{}", k), v2));
            }
        }
        params
    }
}

/// 容器正在等待创建，还未开始运行。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupStatusResponseDataItemPodStatusContainerStatusesItemLastStateWaiting {
    /// 事件原因。
    #[serde(rename = "Reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// 事件消息。
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl DescribeContainerGroupStatusResponseDataItemPodStatusContainerStatusesItemLastStateWaiting {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.reason {
            params.push(("Reason".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message {
            params.push(("Message".to_string(), v.to_string()));
        }
        params
    }
}

/// 容器已经成功创建，并且正在运行。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupStatusResponseDataItemPodStatusContainerStatusesItemLastStateRunning {
    /// 开始时间。
    #[serde(rename = "StartedAtstartedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_atstarted_at: Option<String>,
}

impl DescribeContainerGroupStatusResponseDataItemPodStatusContainerStatusesItemLastStateRunning {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.started_atstarted_at {
            params.push(("StartedAtstartedAt".to_string(), v.to_string()));
        }
        params
    }
}

/// 容器运行终止并退出，包括运行成功终止和运行失败终止。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupStatusResponseDataItemPodStatusContainerStatusesItemLastStateTerminated {
    /// 容器ID。
    #[serde(rename = "ContainerID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,
    /// 退出码。
    #[serde(rename = "ExitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    /// 结束时间。
    #[serde(rename = "FinishedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<String>,
    /// 开始时间。
    #[serde(rename = "StartedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    /// 信号码。
    #[serde(rename = "Signal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal: Option<i32>,
    /// 事件原因。
    #[serde(rename = "Reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// 事件消息。
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl DescribeContainerGroupStatusResponseDataItemPodStatusContainerStatusesItemLastStateTerminated {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.container_id {
            params.push(("ContainerID".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.exit_code {
            params.push(("ExitCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.finished_at {
            params.push(("FinishedAt".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.started_at {
            params.push(("StartedAt".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.signal {
            params.push(("Signal".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.reason {
            params.push(("Reason".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message {
            params.push(("Message".to_string(), v.to_string()));
        }
        params
    }
}

/// 容器上一次状态。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupStatusResponseDataItemPodStatusContainerStatusesItemLastState {
    /// 容器正在等待创建，还未开始运行。
    #[serde(rename = "Waiting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waiting: Option<DescribeContainerGroupStatusResponseDataItemPodStatusContainerStatusesItemLastStateWaiting>,
    /// 容器已经成功创建，并且正在运行。
    #[serde(rename = "Running")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running: Option<DescribeContainerGroupStatusResponseDataItemPodStatusContainerStatusesItemLastStateRunning>,
    /// 容器运行终止并退出，包括运行成功终止和运行失败终止。
    #[serde(rename = "Terminated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminated: Option<DescribeContainerGroupStatusResponseDataItemPodStatusContainerStatusesItemLastStateTerminated>,
}

impl DescribeContainerGroupStatusResponseDataItemPodStatusContainerStatusesItemLastState {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.waiting {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Waiting.{}", k), v2));
            }
        }
        if let Some(ref v) = self.running {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Running.{}", k), v2));
            }
        }
        if let Some(ref v) = self.terminated {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Terminated.{}", k), v2));
            }
        }
        params
    }
}

/// 容器状态信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupStatusResponseDataItemPodStatusContainerStatusesItem {
    /// 容器镜像。
    #[serde(rename = "Image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// 镜像ID。
    #[serde(rename = "ImageID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    /// 容器名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 容器是否Ready。
    #[serde(rename = "Ready")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready: Option<bool>,
    /// 重启次数。
    #[serde(rename = "RestartCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_count: Option<i32>,
    /// 容器是否启动。
    #[serde(rename = "Started")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started: Option<bool>,
    /// 容器状态。包括：
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<DescribeContainerGroupStatusResponseDataItemPodStatusContainerStatusesItemState>,
    /// 容器上一次状态。
    #[serde(rename = "LastState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_state: Option<DescribeContainerGroupStatusResponseDataItemPodStatusContainerStatusesItemLastState>,
}

impl DescribeContainerGroupStatusResponseDataItemPodStatusContainerStatusesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.image {
            params.push(("Image".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.image_id {
            params.push(("ImageID".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ready {
            params.push(("Ready".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.restart_count {
            params.push(("RestartCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.started {
            params.push(("Started".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.state {
            for (k, v2) in v.to_query_params() {
                params.push((format!("State.{}", k), v2));
            }
        }
        if let Some(ref v) = self.last_state {
            for (k, v2) in v.to_query_params() {
                params.push((format!("LastState.{}", k), v2));
            }
        }
        params
    }
}

/// Pod IP地址集合。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupStatusResponseDataItemPodStatusPodIpsItem {
    /// Pod IP地址。
    #[serde(rename = "Ip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

impl DescribeContainerGroupStatusResponseDataItemPodStatusPodIpsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.ip {
            params.push(("Ip".to_string(), v.to_string()));
        }
        params
    }
}

/// ECI实例的状态信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupStatusResponseDataItemPodStatus {
    /// Pod conditions信息。
    #[serde(rename = "Conditions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<DescribeContainerGroupStatusResponseDataItemPodStatusConditionsItem>>,
    /// 容器状态信息。
    #[serde(rename = "ContainerStatuses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_statuses: Option<Vec<DescribeContainerGroupStatusResponseDataItemPodStatusContainerStatusesItem>>,
    /// 主机IP。
    #[serde(rename = "HostIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_ip: Option<String>,
    /// Pod生命周期阶段。
    #[serde(rename = "Phase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    /// Pod IP地址。
    #[serde(rename = "PodIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_ip: Option<String>,
    /// Pod IP地址集合。
    #[serde(rename = "PodIps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_ips: Option<Vec<DescribeContainerGroupStatusResponseDataItemPodStatusPodIpsItem>>,
    /// Pod Qos。
    #[serde(rename = "QosClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qos_class: Option<String>,
    /// 容器开始运行时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}

impl DescribeContainerGroupStatusResponseDataItemPodStatus {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.conditions {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Conditions.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.container_statuses {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("ContainerStatuses.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.host_ip {
            params.push(("HostIp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.phase {
            params.push(("Phase".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.pod_ip {
            params.push(("PodIp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.pod_ips {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("PodIps.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.qos_class {
            params.push(("QosClass".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        params
    }
}

/// ECI实例状态信息集合。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupStatusResponseDataItem {
    /// ECI实例ID。
    #[serde(rename = "ContainerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_group_id: Option<String>,
    /// ECI实例名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// ECI实例所在命名空间。
    #[serde(rename = "Namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// ECI实例状态。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// ECI实例UUID，对应K8s的POD UID。
    #[serde(rename = "uuid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    /// ECI实例的注解。
    #[serde(rename = "Annotations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<String>,
    /// ECI实例的状态信息。
    #[serde(rename = "PodStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_status: Option<DescribeContainerGroupStatusResponseDataItemPodStatus>,
}

impl DescribeContainerGroupStatusResponseDataItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.container_group_id {
            params.push(("ContainerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.namespace {
            params.push(("Namespace".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.uuid {
            params.push(("uuid".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.annotations {
            params.push(("Annotations".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.pod_status {
            for (k, v2) in v.to_query_params() {
                params.push((format!("PodStatus.{}", k), v2));
            }
        }
        params
    }
}

/// ECI实例绑定的标签。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupEventsRequestTagItem {
    /// ECI实例的标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// ECI实例的标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

impl DescribeContainerGroupEventsRequestTagItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key {
            params.push(("Key".to_string(), v.to_string()));
        }
        params
    }
}

/// 来源。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupEventsResponseDataItemEventsItemSource {
    /// 组件。
    #[serde(rename = "Component")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    /// 主机类型。
    #[serde(rename = "Host")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
}

impl DescribeContainerGroupEventsResponseDataItemEventsItemSource {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.component {
            params.push(("Component".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.host {
            params.push(("Host".to_string(), v.to_string()));
        }
        params
    }
}

/// 事件关联的资源对象。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupEventsResponseDataItemEventsItemInvolvedObject {
    /// 资源类型。
    #[serde(rename = "Kind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// 资源名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 资源所在命名空间。
    #[serde(rename = "Namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// 资源ID。
    #[serde(rename = "Uid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    /// K8s版本。
    #[serde(rename = "ApiVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
}

impl DescribeContainerGroupEventsResponseDataItemEventsItemInvolvedObject {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.kind {
            params.push(("Kind".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.namespace {
            params.push(("Namespace".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.uid {
            params.push(("Uid".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.api_version {
            params.push(("ApiVersion".to_string(), v.to_string()));
        }
        params
    }
}

/// 事件元数据。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupEventsResponseDataItemEventsItemMetadata {
    /// 事件名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 命名空间。
    #[serde(rename = "Namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

impl DescribeContainerGroupEventsResponseDataItemEventsItemMetadata {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.namespace {
            params.push(("Namespace".to_string(), v.to_string()));
        }
        params
    }
}

/// 事件列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupEventsResponseDataItemEventsItem {
    /// 事件类型。可能值：
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 事件数量。
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// 事件第一次出现的时间。
    #[serde(rename = "FirstTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_timestamp: Option<String>,
    /// 事件最后一次出现的时间。
    #[serde(rename = "LastTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_timestamp: Option<String>,
    /// 事件信息。
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 事件原因。
    #[serde(rename = "Reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// 上报事件的组件。
    #[serde(rename = "ReportingComponent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting_component: Option<String>,
    /// 上报事件的实例。
    #[serde(rename = "ReportingInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting_instance: Option<String>,
    /// 来源。
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<DescribeContainerGroupEventsResponseDataItemEventsItemSource>,
    /// 事件关联的资源对象。
    #[serde(rename = "involvedObject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub involved_object: Option<DescribeContainerGroupEventsResponseDataItemEventsItemInvolvedObject>,
    /// 事件元数据。
    #[serde(rename = "Metadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<DescribeContainerGroupEventsResponseDataItemEventsItemMetadata>,
}

impl DescribeContainerGroupEventsResponseDataItemEventsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.count {
            params.push(("Count".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.first_timestamp {
            params.push(("FirstTimestamp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.last_timestamp {
            params.push(("LastTimestamp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message {
            params.push(("Message".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.reason {
            params.push(("Reason".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.reporting_component {
            params.push(("ReportingComponent".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.reporting_instance {
            params.push(("ReportingInstance".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Source.{}", k), v2));
            }
        }
        if let Some(ref v) = self.involved_object {
            for (k, v2) in v.to_query_params() {
                params.push((format!("involvedObject.{}", k), v2));
            }
        }
        if let Some(ref v) = self.metadata {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Metadata.{}", k), v2));
            }
        }
        params
    }
}

/// 事件信息列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupEventsResponseDataItem {
    /// ECI实例ID。
    #[serde(rename = "ContainerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_group_id: Option<String>,
    /// ECI实例名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// ECI实例所在命名空间。
    #[serde(rename = "Namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// ECI实例的UUID。
    #[serde(rename = "uuid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    /// ECI实例的注解。
    #[serde(rename = "Annotations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<String>,
    /// 事件列表。
    #[serde(rename = "Events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<DescribeContainerGroupEventsResponseDataItemEventsItem>>,
}

impl DescribeContainerGroupEventsResponseDataItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.container_group_id {
            params.push(("ContainerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.namespace {
            params.push(("Namespace".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.uuid {
            params.push(("uuid".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.annotations {
            params.push(("Annotations".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.events {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Events.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 容器镜像信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommitContainerRequestImage {
    /// ACR镜像仓库地址。
    #[serde(rename = "Repository")]
    pub repository: String,
    /// 镜像标签。默认为空，代表不修改标签。
    #[serde(rename = "Tag")]
    pub tag: String,
    /// 镜像的说明信息。
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 镜像的authorization。
    #[serde(rename = "Author")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
}

impl CommitContainerRequestImage {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Repository".to_string(), self.repository.to_string()));
        params.push(("Tag".to_string(), self.tag.to_string()));
        if let Some(ref v) = self.message {
            params.push(("Message".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.author {
            params.push(("Author".to_string(), v.to_string()));
        }
        params
    }
}

/// ACR企业版实例的访问凭证配置信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommitContainerRequestAcrRegistryInfo {
    /// ACR企业版实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// ACR企业版实例所属的地域。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 跨账号授权场景下，使用者（被授权者）的RAM角色ARN。
    #[serde(rename = "ArnService")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn_service: Option<String>,
    /// 跨账号授权场景下，授权者的RAM角色ARN。
    #[serde(rename = "ArnUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn_user: Option<String>,
}

impl CommitContainerRequestAcrRegistryInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.arn_service {
            params.push(("ArnService".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.arn_user {
            params.push(("ArnUser".to_string(), v.to_string()));
        }
        params
    }
}

/// 授权需要的ARN信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommitContainerRequestArn {
    /// 授权角色的ARN。
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// 授权类型。可配置为service，表示使用RAM角色进行授权。
    #[serde(rename = "RoleType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_type: Option<String>,
}

impl CommitContainerRequestArn {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.role_arn {
            params.push(("RoleArn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.role_type {
            params.push(("RoleType".to_string(), v.to_string()));
        }
        params
    }
}

/// 任务阶段信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCommitContainerTaskResponseCommitTasksItemCommitPhaseInfosItem {
    /// 阶段名称。可能值：
    #[serde(rename = "Phase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    /// 阶段状态。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 阶段信息。
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 阶段时间。
    #[serde(rename = "RecordTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_time: Option<String>,
}

impl DescribeCommitContainerTaskResponseCommitTasksItemCommitPhaseInfosItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.phase {
            params.push(("Phase".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message {
            params.push(("Message".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.record_time {
            params.push(("RecordTime".to_string(), v.to_string()));
        }
        params
    }
}

/// 任务详细信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCommitContainerTaskResponseCommitTasksItem {
    /// 任务ID。
    #[serde(rename = "TaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// 任务状态。可能值：
    #[serde(rename = "TaskStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_status: Option<String>,
    /// 任务进度（百分比）。
    #[serde(rename = "TaskProgress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_progress: Option<String>,
    /// 状态消息。
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// 容器名称。
    #[serde(rename = "ContainerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    /// 任务阶段信息。
    #[serde(rename = "CommitPhaseInfos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_phase_infos: Option<Vec<DescribeCommitContainerTaskResponseCommitTasksItemCommitPhaseInfosItem>>,
    /// 任务开始时间。
    #[serde(rename = "TaskCreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_creation_time: Option<String>,
    /// 任务完成时间。
    #[serde(rename = "TaskFinishedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_finished_time: Option<String>,
}

impl DescribeCommitContainerTaskResponseCommitTasksItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.task_id {
            params.push(("TaskId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_status {
            params.push(("TaskStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_progress {
            params.push(("TaskProgress".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status_message {
            params.push(("StatusMessage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.container_name {
            params.push(("ContainerName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.commit_phase_infos {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("CommitPhaseInfos.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.task_creation_time {
            params.push(("TaskCreationTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_finished_time {
            params.push(("TaskFinishedTime".to_string(), v.to_string()));
        }
        params
    }
}

/// 镜像仓库信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateImageCacheRequestImageRegistryCredentialItem {
    /// 镜像仓库密码。
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// 不带 `http://` 或  `https://` 前缀的镜像仓库地址。
    #[serde(rename = "Server")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// 镜像仓库用户名。
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

impl CreateImageCacheRequestImageRegistryCredentialItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.password {
            params.push(("Password".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.server {
            params.push(("Server".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_name {
            params.push(("UserName".to_string(), v.to_string()));
        }
        params
    }
}

/// 镜像缓存标签信息，最多20个。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateImageCacheRequestTagItem {
    /// 镜像缓存标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 镜像缓存标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreateImageCacheRequestTagItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key {
            params.push(("Key".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        params
    }
}

/// ACR实例信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateImageCacheRequestAcrRegistryInfoItem {
    /// ACR企业版实例的域名。默认为相应实例的所有域名。支持指定个别域名，多个以半角逗号分隔。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<Vec<String>>,
    /// ACR企业版实例的名称。
    #[serde(rename = "InstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    /// ACR企业版实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// ACR企业版实例所属地域。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// ECI实例等资源归属账号下的RAM角色的ARN。
    #[serde(rename = "ArnService")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn_service: Option<String>,
    /// ACR实例归属账号下的RAM角色的ARN。
    #[serde(rename = "ArnUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn_user: Option<String>,
}

impl CreateImageCacheRequestAcrRegistryInfoItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Domain.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.instance_name {
            params.push(("InstanceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.arn_service {
            params.push(("ArnService".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.arn_user {
            params.push(("ArnUser".to_string(), v.to_string()));
        }
        params
    }
}

/// 镜像仓库信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateImageCacheRequestImageRegistryCredentialItem {
    /// 镜像仓库密码。
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// 不带 `http://` 或  `https://` 前缀的镜像仓库地址。
    #[serde(rename = "Server")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// 镜像仓库用户名。
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

impl UpdateImageCacheRequestImageRegistryCredentialItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.password {
            params.push(("Password".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.server {
            params.push(("Server".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_name {
            params.push(("UserName".to_string(), v.to_string()));
        }
        params
    }
}

/// 镜像缓存绑定的标签信息，最多20个。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateImageCacheRequestTagItem {
    /// 镜像缓存标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 镜像缓存标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl UpdateImageCacheRequestTagItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key {
            params.push(("Key".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        params
    }
}

/// ACR企业版实例信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateImageCacheRequestAcrRegistryInfoItem {
    /// ACR企业版实例的域名。默认为相应实例的所有域名。支持指定个别域名，多个以半角逗号分隔。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<Vec<String>>,
    /// ACR企业版实例的名称。
    #[serde(rename = "InstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    /// ACR企业版实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// ACR企业版实例所属地域。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
}

impl UpdateImageCacheRequestAcrRegistryInfoItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Domain.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.instance_name {
            params.push(("InstanceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params
    }
}

/// 镜像缓存绑定的标签信息，最多20个。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeImageCachesRequestTagItem {
    /// 镜像缓存标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 镜像缓存标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DescribeImageCachesRequestTagItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key {
            params.push(("Key".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        params
    }
}

/// 镜像缓存的标签列表信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeImageCachesResponseImageCachesItemTagsItem {
    /// 标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DescribeImageCachesResponseImageCachesItemTagsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key {
            params.push(("Key".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        params
    }
}

/// 镜像缓存拉取镜像事件信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeImageCachesResponseImageCachesItemEventsItem {
    /// 事件类型。
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 事件结束时间。
    #[serde(rename = "LastTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_timestamp: Option<String>,
    /// 事件消息。
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 事件名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 事件数。
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// 事件开始时间。
    #[serde(rename = "FirstTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_timestamp: Option<String>,
    /// 事件原因。
    #[serde(rename = "Reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl DescribeImageCachesResponseImageCachesItemEventsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.last_timestamp {
            params.push(("LastTimestamp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message {
            params.push(("Message".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.count {
            params.push(("Count".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.first_timestamp {
            params.push(("FirstTimestamp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.reason {
            params.push(("Reason".to_string(), v.to_string()));
        }
        params
    }
}

/// 镜像缓存信息列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeImageCachesResponseImageCachesItem {
    /// 镜像缓存包含的镜像列表。
    #[serde(rename = "Images")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,
    /// 创建时间。
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// 镜像缓存状态。可能值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 镜像缓存对应快照的创建进度。
    #[serde(rename = "Progress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<String>,
    /// 到期时间。
    #[serde(rename = "ExpireDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date_time: Option<String>,
    /// 最新一次匹配到镜像缓存的时间。
    #[serde(rename = "LastMatchedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_matched_time: Option<String>,
    /// 容器组ID。
    #[serde(rename = "ContainerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_group_id: Option<String>,
    /// 镜像缓存的标签列表信息。
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<DescribeImageCachesResponseImageCachesItemTagsItem>>,
    /// 镜像缓存拉取镜像事件信息。
    #[serde(rename = "Events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<DescribeImageCachesResponseImageCachesItemEventsItem>>,
    /// 镜像缓存ID。
    #[serde(rename = "ImageCacheId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_cache_id: Option<String>,
    /// 所属地域。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 镜像缓存对应的快照ID。
    #[serde(rename = "SnapshotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    /// 本地快照ID。开启极速镜像缓存功能后，将临时创建一份本地快照。
    #[serde(rename = "FlashSnapshotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flash_snapshot_id: Option<String>,
    /// 所属资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 镜像缓存大小。单位：GiB。
    #[serde(rename = "ImageCacheSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_cache_size: Option<i32>,
    /// 镜像缓存名称。
    #[serde(rename = "ImageCacheName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_cache_name: Option<String>,
    /// 镜像缓存的淘汰策略。默认为空，表示一直保留。
    #[serde(rename = "EliminationStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elimination_strategy: Option<String>,
}

impl DescribeImageCachesResponseImageCachesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.images {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Images.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.creation_time {
            params.push(("CreationTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.progress {
            params.push(("Progress".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.expire_date_time {
            params.push(("ExpireDateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.last_matched_time {
            params.push(("LastMatchedTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.container_group_id {
            params.push(("ContainerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tags.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.events {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Events.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.image_cache_id {
            params.push(("ImageCacheId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.snapshot_id {
            params.push(("SnapshotId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.flash_snapshot_id {
            params.push(("FlashSnapshotId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.image_cache_size {
            params.push(("ImageCacheSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.image_cache_name {
            params.push(("ImageCacheName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.elimination_strategy {
            params.push(("EliminationStrategy".to_string(), v.to_string()));
        }
        params
    }
}

/// 数据源。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateDataCacheRequestDataSource {
    /// 数据源类型。取值范围：
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 数据源的配置参数。
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
}

impl CreateDataCacheRequestDataSource {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        // 跳过: Options (serde_json::Value)
        params
    }
}

/// 数据缓存绑定的标签列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateDataCacheRequestTagItem {
    /// 标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreateDataCacheRequestTagItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key {
            params.push(("Key".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        params
    }
}

/// 创建一个弹性公网IP（EIP）并绑定。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateDataCacheRequestEipCreateParam {
    /// EIP带宽。单位为Mbps。默认为5 Mbps。
    #[serde(rename = "Bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<i32>,
    /// 绑定已有的共享带宽包。
    #[serde(rename = "CommonBandwidthPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_bandwidth_package: Option<String>,
    /// EIP的计量方式。取值范围：
    #[serde(rename = "InternetChargeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internet_charge_type: Option<String>,
    /// IP地址池ID。
    #[serde(rename = "PublicIpAddressPoolId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ip_address_pool_id: Option<String>,
    /// EIP的线路类型。取值范围：
    #[serde(rename = "ISP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp: Option<String>,
}

impl CreateDataCacheRequestEipCreateParam {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.bandwidth {
            params.push(("Bandwidth".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.common_bandwidth_package {
            params.push(("CommonBandwidthPackage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.internet_charge_type {
            params.push(("InternetChargeType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.public_ip_address_pool_id {
            params.push(("PublicIpAddressPoolId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.isp {
            params.push(("ISP".to_string(), v.to_string()));
        }
        params
    }
}

/// 标签列表
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDataCachesRequestTagItem {
    /// 标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DescribeDataCachesRequestTagItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key {
            params.push(("Key".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        params
    }
}

/// 标签列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDataCachesResponseDataCachesItemTagsItem {
    /// 标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DescribeDataCachesResponseDataCachesItemTagsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key {
            params.push(("Key".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        params
    }
}

/// 事件列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDataCachesResponseDataCachesItemEventsItem {
    /// 事件类型。可能值：
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 事件结束时间。
    #[serde(rename = "LastTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_timestamp: Option<String>,
    /// 事件信息。
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 事件名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 事件数。
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// 事件开始时间。
    #[serde(rename = "FirstTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_timestamp: Option<String>,
    /// 事件原因。
    #[serde(rename = "Reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl DescribeDataCachesResponseDataCachesItemEventsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.last_timestamp {
            params.push(("LastTimestamp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message {
            params.push(("Message".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.count {
            params.push(("Count".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.first_timestamp {
            params.push(("FirstTimestamp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.reason {
            params.push(("Reason".to_string(), v.to_string()));
        }
        params
    }
}

/// 数据源信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDataCachesResponseDataCachesItemDataSource {
    /// 数据源类型。可能值：
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 数据源的配置参数。
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<String>,
}

impl DescribeDataCachesResponseDataCachesItemDataSource {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.options {
            params.push(("Options".to_string(), v.to_string()));
        }
        params
    }
}

/// 数据缓存信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDataCachesResponseDataCachesItem {
    /// 数据缓存ID。
    #[serde(rename = "DataCacheId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_cache_id: Option<String>,
    /// 创建时间。
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// 数据缓存状态。可能值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 创建进度。
    #[serde(rename = "Progress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<String>,
    /// 最新一次匹配数据缓存的时间。
    #[serde(rename = "LastMatchedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_matched_time: Option<String>,
    /// 到期时间。
    #[serde(rename = "ExpireDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date_time: Option<String>,
    /// 创建数据缓存过程中，生成的ECI实例ID。
    #[serde(rename = "ContainerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_group_id: Option<String>,
    /// 标签列表。
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<DescribeDataCachesResponseDataCachesItemTagsItem>>,
    /// 事件列表。
    #[serde(rename = "Events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<DescribeDataCachesResponseDataCachesItemEventsItem>>,
    /// 地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 快照ID。
    #[serde(rename = "SnapshotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    /// 本地快照ID。
    #[serde(rename = "FlashSnapshotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flash_snapshot_id: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 数据缓存大小。单位为GiB。
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    /// 数据缓存名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 数据源信息。
    #[serde(rename = "DataSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<DescribeDataCachesResponseDataCachesItemDataSource>,
    /// 数据缓存Bucket。
    #[serde(rename = "Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    /// 数据缓存对应Virtual Host的目录。
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

impl DescribeDataCachesResponseDataCachesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.data_cache_id {
            params.push(("DataCacheId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.creation_time {
            params.push(("CreationTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.progress {
            params.push(("Progress".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.last_matched_time {
            params.push(("LastMatchedTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.expire_date_time {
            params.push(("ExpireDateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.container_group_id {
            params.push(("ContainerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tags.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.events {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Events.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.snapshot_id {
            params.push(("SnapshotId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.flash_snapshot_id {
            params.push(("FlashSnapshotId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size {
            params.push(("Size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.data_source {
            for (k, v2) in v.to_query_params() {
                params.push((format!("DataSource.{}", k), v2));
            }
        }
        if let Some(ref v) = self.bucket {
            params.push(("Bucket".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.path {
            params.push(("Path".to_string(), v.to_string()));
        }
        params
    }
}

/// 数据源信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateDataCacheRequestDataSource {
    /// 数据源类型。取值范围：
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 数据源的配置参数。
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
}

impl UpdateDataCacheRequestDataSource {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        // 跳过: Options (serde_json::Value)
        params
    }
}

/// 标签列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateDataCacheRequestTagItem {
    /// 标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl UpdateDataCacheRequestTagItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key {
            params.push(("Key".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        params
    }
}

/// 创建一个弹性公网IP（EIP）并绑定。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateDataCacheRequestEipCreateParam {
    /// EIP带宽。单位为Mbps。默认为5 Mbps。
    #[serde(rename = "Bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<i32>,
    /// 绑定已有的共享带宽包。
    #[serde(rename = "CommonBandwidthPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_bandwidth_package: Option<String>,
    /// EIP的计量方式。取值范围：
    #[serde(rename = "InternetChargeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internet_charge_type: Option<String>,
    /// IP地址池ID。
    #[serde(rename = "PublicIpAddressPoolId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ip_address_pool_id: Option<String>,
    /// EIP的线路类型。取值范围：
    #[serde(rename = "ISP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp: Option<String>,
}

impl UpdateDataCacheRequestEipCreateParam {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.bandwidth {
            params.push(("Bandwidth".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.common_bandwidth_package {
            params.push(("CommonBandwidthPackage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.internet_charge_type {
            params.push(("InternetChargeType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.public_ip_address_pool_id {
            params.push(("PublicIpAddressPoolId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.isp {
            params.push(("ISP".to_string(), v.to_string()));
        }
        params
    }
}

/// 数据缓存标签列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CopyDataCacheRequestTagItem {
    /// 标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CopyDataCacheRequestTagItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key {
            params.push(("Key".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        params
    }
}

/// 虚拟节点标签信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateVirtualNodeRequestTagItem {
    /// 标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreateVirtualNodeRequestTagItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key {
            params.push(("Key".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        params
    }
}

/// 虚拟节点污点信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateVirtualNodeRequestTaintItem {
    /// 污点的Key。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 污点的Value。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 污点的Effect。取值范围：
    #[serde(rename = "Effect")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
}

impl CreateVirtualNodeRequestTaintItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key {
            params.push(("Key".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.effect {
            params.push(("Effect".to_string(), v.to_string()));
        }
        params
    }
}

/// 虚拟节点标签信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateVirtualNodeRequestTagItem {
    /// 标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl UpdateVirtualNodeRequestTagItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key {
            params.push(("Key".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        params
    }
}

/// 虚拟节点绑定的标签列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeVirtualNodesRequestTagItem {
    /// 标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DescribeVirtualNodesRequestTagItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key {
            params.push(("Key".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        params
    }
}

/// 标签信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeVirtualNodesResponseVirtualNodesItemTagsItem {
    /// 标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DescribeVirtualNodesResponseVirtualNodesItemTagsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key {
            params.push(("Key".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        params
    }
}

/// 事件信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeVirtualNodesResponseVirtualNodesItemEventsItem {
    /// 事件类型。可能值：
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 事件结束时间。
    #[serde(rename = "LastTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_timestamp: Option<String>,
    /// 事件消息。
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 事件的归属对象名。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 事件名。
    #[serde(rename = "Reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// 事件数。
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// 事件开始时间。
    #[serde(rename = "FirstTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_timestamp: Option<String>,
}

impl DescribeVirtualNodesResponseVirtualNodesItemEventsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.last_timestamp {
            params.push(("LastTimestamp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message {
            params.push(("Message".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.reason {
            params.push(("Reason".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.count {
            params.push(("Count".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.first_timestamp {
            params.push(("FirstTimestamp".to_string(), v.to_string()));
        }
        params
    }
}

/// 虚拟节点信息集合。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeVirtualNodesResponseVirtualNodesItem {
    /// 虚拟节点状态。可能值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 虚拟节点创建时间。UTC时间，RFC 3339标准。
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// 虚拟节点所属的VPC ID。
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 虚拟节点的公网IP。
    #[serde(rename = "InternetIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internet_ip: Option<String>,
    /// 虚拟节点绑定的标签信息。
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<DescribeVirtualNodesResponseVirtualNodesItemTagsItem>>,
    /// 事件信息。
    #[serde(rename = "Events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<DescribeVirtualNodesResponseVirtualNodesItemEventsItem>>,
    /// 虚拟节点ID。
    #[serde(rename = "VirtualNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_node_id: Option<String>,
    /// 虚拟节点的内网IP。
    #[serde(rename = "IntranetIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intranet_ip: Option<String>,
    /// 虚拟节点所属的地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 虚拟节点所属的交换机ID。
    #[serde(rename = "VirtualNodeVSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_node_v_switch_id: Option<String>,
    /// 虚拟节点所属的安全组ID。
    #[serde(rename = "VirtualNodeSecurityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_node_security_group_id: Option<String>,
    /// 虚拟节点所属的资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 虚拟节点的名称。
    #[serde(rename = "VirtualNodeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_node_name: Option<String>,
}

impl DescribeVirtualNodesResponseVirtualNodesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.creation_time {
            params.push(("CreationTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_id {
            params.push(("VpcId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.internet_ip {
            params.push(("InternetIp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tags.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.events {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Events.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.virtual_node_id {
            params.push(("VirtualNodeId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.intranet_ip {
            params.push(("IntranetIp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.virtual_node_v_switch_id {
            params.push(("VirtualNodeVSwitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.virtual_node_security_group_id {
            params.push(("VirtualNodeSecurityGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.virtual_node_name {
            params.push(("VirtualNodeName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupMetricResponseRecordsItemNetworkInterfacesItem {
    /// 累计接收错误数。
    #[serde(rename = "RxErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rx_errors: Option<i64>,
    /// 累计发送丢包数。
    #[serde(rename = "TxDrops")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_drops: Option<i64>,
    /// 累计发送字节数。
    #[serde(rename = "TxBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_bytes: Option<i64>,
    /// 累计接收包数量。
    #[serde(rename = "RxPackets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rx_packets: Option<i64>,
    /// 累计发送包数量。
    #[serde(rename = "TxPackets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_packets: Option<i64>,
    /// 网卡名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 累计发送错误数。
    #[serde(rename = "TxErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_errors: Option<i64>,
    /// 累计接收字节数。
    #[serde(rename = "RxBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rx_bytes: Option<i64>,
    /// 累计接收丢包数。
    #[serde(rename = "RxDrops")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rx_drops: Option<i64>,
}

impl DescribeContainerGroupMetricResponseRecordsItemNetworkInterfacesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.rx_errors {
            params.push(("RxErrors".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tx_drops {
            params.push(("TxDrops".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tx_bytes {
            params.push(("TxBytes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rx_packets {
            params.push(("RxPackets".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tx_packets {
            params.push(("TxPackets".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tx_errors {
            params.push(("TxErrors".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rx_bytes {
            params.push(("RxBytes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rx_drops {
            params.push(("RxDrops".to_string(), v.to_string()));
        }
        params
    }
}

/// 网络数据。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupMetricResponseRecordsItemNetwork {
    /// 网卡数据。
    #[serde(rename = "Interfaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interfaces: Option<Vec<DescribeContainerGroupMetricResponseRecordsItemNetworkInterfacesItem>>,
}

impl DescribeContainerGroupMetricResponseRecordsItemNetwork {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.interfaces {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Interfaces.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// CPU数据。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupMetricResponseRecordsItemCPU {
    /// CPU在采样窗口内的使用量（纳秒）。
    #[serde(rename = "UsageNanoCores")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_nano_cores: Option<i64>,
    /// CPU使用上限（CPU核数*1000）。
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// CPU历史使用总量。
    #[serde(rename = "UsageCoreNanoSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_core_nano_seconds: Option<i64>,
    /// 最近10秒的平均负载情况。
    #[serde(rename = "Load")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load: Option<i64>,
}

impl DescribeContainerGroupMetricResponseRecordsItemCPU {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.usage_nano_cores {
            params.push(("UsageNanoCores".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.limit {
            params.push(("Limit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.usage_core_nano_seconds {
            params.push(("UsageCoreNanoSeconds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.load {
            params.push(("Load".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupMetricResponseRecordsItemDiskItem {
    /// 磁盘写入的数据量，单位为Byte。
    #[serde(rename = "WriteBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_bytes: Option<i64>,
    /// 该参数暂未开放使用。
    #[serde(rename = "WriteIO")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_io: Option<i64>,
    /// 磁盘名称。
    #[serde(rename = "Device")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    /// 该参数暂未开放使用。
    #[serde(rename = "ReadIO")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_io: Option<i64>,
    /// 磁盘读取的数据量，单位为Byte。
    #[serde(rename = "ReadBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_bytes: Option<i64>,
}

impl DescribeContainerGroupMetricResponseRecordsItemDiskItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.write_bytes {
            params.push(("WriteBytes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.write_io {
            params.push(("WriteIO".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.device {
            params.push(("Device".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.read_io {
            params.push(("ReadIO".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.read_bytes {
            params.push(("ReadBytes".to_string(), v.to_string()));
        }
        params
    }
}

/// 内存数据。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupMetricResponseRecordsItemMemory {
    /// 常驻内存集，即实际使用的物理内存。
    #[serde(rename = "Rss")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rss: Option<i64>,
    /// 已使用内存。
    #[serde(rename = "UsageBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_bytes: Option<i64>,
    /// 当前内存工作集使用量。
    #[serde(rename = "WorkingSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_set: Option<i64>,
    /// 可用内存。
    #[serde(rename = "AvailableBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_bytes: Option<i64>,
    /// 缓存。
    #[serde(rename = "Cache")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<i64>,
}

impl DescribeContainerGroupMetricResponseRecordsItemMemory {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.rss {
            params.push(("Rss".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.usage_bytes {
            params.push(("UsageBytes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.working_set {
            params.push(("WorkingSet".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.available_bytes {
            params.push(("AvailableBytes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cache {
            params.push(("Cache".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupMetricResponseRecordsItemFilesystemItem {
    /// 空间总量。
    #[serde(rename = "Capacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
    /// 可用空间。
    #[serde(rename = "Available")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available: Option<i64>,
    /// 分区名称。
    #[serde(rename = "FsName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_name: Option<String>,
    /// 已使用空间。
    #[serde(rename = "Usage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<i64>,
    /// 分区类型。可能值：
    #[serde(rename = "Category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
}

impl DescribeContainerGroupMetricResponseRecordsItemFilesystemItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.capacity {
            params.push(("Capacity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.available {
            params.push(("Available".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.fs_name {
            params.push(("FsName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.usage {
            params.push(("Usage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.category {
            params.push(("Category".to_string(), v.to_string()));
        }
        params
    }
}

/// 容器的CPU数据。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupMetricResponseRecordsItemContainersItemCPU {
    /// CPU在采样窗口内的使用量（纳秒）。
    #[serde(rename = "UsageNanoCores")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_nano_cores: Option<i64>,
    /// CPU使用上限（CPU核数*1000）。
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// CPU历史使用总量。
    #[serde(rename = "UsageCoreNanoSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_core_nano_seconds: Option<i64>,
    /// 最近10秒的平均负载情况。
    #[serde(rename = "Load")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load: Option<i64>,
}

impl DescribeContainerGroupMetricResponseRecordsItemContainersItemCPU {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.usage_nano_cores {
            params.push(("UsageNanoCores".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.limit {
            params.push(("Limit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.usage_core_nano_seconds {
            params.push(("UsageCoreNanoSeconds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.load {
            params.push(("Load".to_string(), v.to_string()));
        }
        params
    }
}

/// 容器的内存数据。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupMetricResponseRecordsItemContainersItemMemory {
    /// 常驻内存集，即实际使用的物理内存。
    #[serde(rename = "Rss")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rss: Option<i64>,
    /// 已使用内存。
    #[serde(rename = "UsageBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_bytes: Option<i64>,
    /// 当前内存工作集使用量。
    #[serde(rename = "WorkingSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_set: Option<i64>,
    /// 可用内存。
    #[serde(rename = "AvailableBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_bytes: Option<i64>,
    /// 缓存。
    #[serde(rename = "Cache")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<i64>,
}

impl DescribeContainerGroupMetricResponseRecordsItemContainersItemMemory {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.rss {
            params.push(("Rss".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.usage_bytes {
            params.push(("UsageBytes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.working_set {
            params.push(("WorkingSet".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.available_bytes {
            params.push(("AvailableBytes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cache {
            params.push(("Cache".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupMetricResponseRecordsItemContainersItem {
    /// 容器的CPU数据。
    #[serde(rename = "CPU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<DescribeContainerGroupMetricResponseRecordsItemContainersItemCPU>,
    /// 容器的内存数据。
    #[serde(rename = "Memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<DescribeContainerGroupMetricResponseRecordsItemContainersItemMemory>,
    /// 容器名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl DescribeContainerGroupMetricResponseRecordsItemContainersItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cpu {
            for (k, v2) in v.to_query_params() {
                params.push((format!("CPU.{}", k), v2));
            }
        }
        if let Some(ref v) = self.memory {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Memory.{}", k), v2));
            }
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupMetricResponseRecordsItem {
    /// 网络数据。
    #[serde(rename = "Network")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<DescribeContainerGroupMetricResponseRecordsItemNetwork>,
    /// CPU数据。
    #[serde(rename = "CPU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<DescribeContainerGroupMetricResponseRecordsItemCPU>,
    /// 磁盘数据。
    #[serde(rename = "Disk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk: Option<Vec<DescribeContainerGroupMetricResponseRecordsItemDiskItem>>,
    /// 每条监控数据对应的统计时间。格式为RFC3339时间格式。
    #[serde(rename = "Timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// 内存数据。
    #[serde(rename = "Memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<DescribeContainerGroupMetricResponseRecordsItemMemory>,
    /// 文件系统分区数据。
    #[serde(rename = "Filesystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filesystem: Option<Vec<DescribeContainerGroupMetricResponseRecordsItemFilesystemItem>>,
    /// 容器的监控数据详情。
    #[serde(rename = "Containers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<DescribeContainerGroupMetricResponseRecordsItemContainersItem>>,
}

impl DescribeContainerGroupMetricResponseRecordsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.network {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Network.{}", k), v2));
            }
        }
        if let Some(ref v) = self.cpu {
            for (k, v2) in v.to_query_params() {
                params.push((format!("CPU.{}", k), v2));
            }
        }
        if let Some(ref v) = self.disk {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Disk.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.timestamp {
            params.push(("Timestamp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.memory {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Memory.{}", k), v2));
            }
        }
        if let Some(ref v) = self.filesystem {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Filesystem.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.containers {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Containers.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeMultiContainerGroupMetricResponseMonitorDatasItemRecordsItemNetworkInterfacesItem {
    /// 累计接收错误数。
    #[serde(rename = "RxErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rx_errors: Option<i64>,
    /// 累计发送丢包数。
    #[serde(rename = "TxDrops")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_drops: Option<i64>,
    /// 累计发送字节数。
    #[serde(rename = "TxBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_bytes: Option<i64>,
    /// 累计接收包数量。
    #[serde(rename = "RxPackets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rx_packets: Option<i64>,
    /// 累计发送包数量。
    #[serde(rename = "TxPackets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_packets: Option<i64>,
    /// 网卡名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 累计发送错误数。
    #[serde(rename = "TxErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_errors: Option<i64>,
    /// 累计接收字节数。
    #[serde(rename = "RxBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rx_bytes: Option<i64>,
    /// 累计接收丢包数。
    #[serde(rename = "RxDrops")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rx_drops: Option<i64>,
}

impl DescribeMultiContainerGroupMetricResponseMonitorDatasItemRecordsItemNetworkInterfacesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.rx_errors {
            params.push(("RxErrors".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tx_drops {
            params.push(("TxDrops".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tx_bytes {
            params.push(("TxBytes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rx_packets {
            params.push(("RxPackets".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tx_packets {
            params.push(("TxPackets".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tx_errors {
            params.push(("TxErrors".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rx_bytes {
            params.push(("RxBytes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rx_drops {
            params.push(("RxDrops".to_string(), v.to_string()));
        }
        params
    }
}

/// 网络数据。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeMultiContainerGroupMetricResponseMonitorDatasItemRecordsItemNetwork {
    /// 网卡数据。
    #[serde(rename = "Interfaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interfaces: Option<Vec<DescribeMultiContainerGroupMetricResponseMonitorDatasItemRecordsItemNetworkInterfacesItem>>,
}

impl DescribeMultiContainerGroupMetricResponseMonitorDatasItemRecordsItemNetwork {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.interfaces {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Interfaces.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// CPU数据。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeMultiContainerGroupMetricResponseMonitorDatasItemRecordsItemCPU {
    /// CPU在采样窗口内的使用量（纳秒）。
    #[serde(rename = "UsageNanoCores")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_nano_cores: Option<i64>,
    /// CPU使用上限（CPU核数*1000）。
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// CPU历史使用总量。
    #[serde(rename = "UsageCoreNanoSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_core_nano_seconds: Option<i64>,
    /// 最近10秒的平均负载情况。
    #[serde(rename = "Load")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load: Option<i64>,
}

impl DescribeMultiContainerGroupMetricResponseMonitorDatasItemRecordsItemCPU {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.usage_nano_cores {
            params.push(("UsageNanoCores".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.limit {
            params.push(("Limit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.usage_core_nano_seconds {
            params.push(("UsageCoreNanoSeconds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.load {
            params.push(("Load".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeMultiContainerGroupMetricResponseMonitorDatasItemRecordsItemDiskItem {
    /// 磁盘写入的数据量，单位为Byte。
    #[serde(rename = "WriteBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_bytes: Option<i64>,
    /// 磁盘名称。
    #[serde(rename = "Device")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    /// 该参数暂未开放使用。
    #[serde(rename = "WriteIo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_io: Option<i64>,
    /// 磁盘读取的数据量，单位为Byte。
    #[serde(rename = "ReadBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_bytes: Option<i64>,
    /// 该参数暂未开放使用。
    #[serde(rename = "ReadIo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_io: Option<i64>,
}

impl DescribeMultiContainerGroupMetricResponseMonitorDatasItemRecordsItemDiskItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.write_bytes {
            params.push(("WriteBytes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.device {
            params.push(("Device".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.write_io {
            params.push(("WriteIo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.read_bytes {
            params.push(("ReadBytes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.read_io {
            params.push(("ReadIo".to_string(), v.to_string()));
        }
        params
    }
}

/// 内存数据。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeMultiContainerGroupMetricResponseMonitorDatasItemRecordsItemMemory {
    /// 常驻内存集，即实际使用的物理内存。
    #[serde(rename = "Rss")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rss: Option<i64>,
    /// 已使用内存。
    #[serde(rename = "UsageBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_bytes: Option<i64>,
    /// 当前内存工作集使用量。
    #[serde(rename = "WorkingSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_set: Option<i64>,
    /// 可用内存。
    #[serde(rename = "AvailableBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_bytes: Option<i64>,
    /// 缓存。
    #[serde(rename = "Cache")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<i64>,
}

impl DescribeMultiContainerGroupMetricResponseMonitorDatasItemRecordsItemMemory {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.rss {
            params.push(("Rss".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.usage_bytes {
            params.push(("UsageBytes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.working_set {
            params.push(("WorkingSet".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.available_bytes {
            params.push(("AvailableBytes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cache {
            params.push(("Cache".to_string(), v.to_string()));
        }
        params
    }
}

/// 容器的CPU数据。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeMultiContainerGroupMetricResponseMonitorDatasItemRecordsItemContainersItemCPU {
    /// CPU在采样窗口内的使用量（纳秒）。
    #[serde(rename = "UsageNanoCores")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_nano_cores: Option<i64>,
    /// CPU使用上限（CPU核数*1000）。
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// CPU历史使用总量。
    #[serde(rename = "UsageCoreNanoSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_core_nano_seconds: Option<i64>,
    /// 最近10秒的平均负载情况。
    #[serde(rename = "Load")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load: Option<i64>,
}

impl DescribeMultiContainerGroupMetricResponseMonitorDatasItemRecordsItemContainersItemCPU {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.usage_nano_cores {
            params.push(("UsageNanoCores".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.limit {
            params.push(("Limit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.usage_core_nano_seconds {
            params.push(("UsageCoreNanoSeconds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.load {
            params.push(("Load".to_string(), v.to_string()));
        }
        params
    }
}

/// 容器的内存数据。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeMultiContainerGroupMetricResponseMonitorDatasItemRecordsItemContainersItemMemory {
    /// 常驻内存集，即实际使用的物理内存。
    #[serde(rename = "Rss")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rss: Option<i64>,
    /// 已使用内存。
    #[serde(rename = "UsageBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_bytes: Option<i64>,
    /// 当前内存工作集使用量。
    #[serde(rename = "WorkingSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_set: Option<i64>,
    /// 可用内存。
    #[serde(rename = "AvailableBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_bytes: Option<i64>,
    /// 缓存。
    #[serde(rename = "Cache")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<i64>,
}

impl DescribeMultiContainerGroupMetricResponseMonitorDatasItemRecordsItemContainersItemMemory {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.rss {
            params.push(("Rss".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.usage_bytes {
            params.push(("UsageBytes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.working_set {
            params.push(("WorkingSet".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.available_bytes {
            params.push(("AvailableBytes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cache {
            params.push(("Cache".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeMultiContainerGroupMetricResponseMonitorDatasItemRecordsItemContainersItem {
    /// 容器的CPU数据。
    #[serde(rename = "CPU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<DescribeMultiContainerGroupMetricResponseMonitorDatasItemRecordsItemContainersItemCPU>,
    /// 容器的内存数据。
    #[serde(rename = "Memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<DescribeMultiContainerGroupMetricResponseMonitorDatasItemRecordsItemContainersItemMemory>,
    /// 名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl DescribeMultiContainerGroupMetricResponseMonitorDatasItemRecordsItemContainersItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cpu {
            for (k, v2) in v.to_query_params() {
                params.push((format!("CPU.{}", k), v2));
            }
        }
        if let Some(ref v) = self.memory {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Memory.{}", k), v2));
            }
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeMultiContainerGroupMetricResponseMonitorDatasItemRecordsItemFilesystemItem {
    /// 空间总量。
    #[serde(rename = "Capacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
    /// 可用空间。
    #[serde(rename = "Available")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available: Option<i64>,
    /// 分区名称。
    #[serde(rename = "FsName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_name: Option<String>,
    /// 已使用空间。
    #[serde(rename = "Usage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<i64>,
}

impl DescribeMultiContainerGroupMetricResponseMonitorDatasItemRecordsItemFilesystemItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.capacity {
            params.push(("Capacity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.available {
            params.push(("Available".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.fs_name {
            params.push(("FsName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.usage {
            params.push(("Usage".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeMultiContainerGroupMetricResponseMonitorDatasItemRecordsItem {
    /// 网络数据。
    #[serde(rename = "Network")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<DescribeMultiContainerGroupMetricResponseMonitorDatasItemRecordsItemNetwork>,
    /// CPU数据。
    #[serde(rename = "CPU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<DescribeMultiContainerGroupMetricResponseMonitorDatasItemRecordsItemCPU>,
    /// 磁盘数据。
    #[serde(rename = "Disk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk: Option<Vec<DescribeMultiContainerGroupMetricResponseMonitorDatasItemRecordsItemDiskItem>>,
    /// 监控数据对应的统计时间。格式为RFC3339时间格式。
    #[serde(rename = "Timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// 内存数据。
    #[serde(rename = "Memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<DescribeMultiContainerGroupMetricResponseMonitorDatasItemRecordsItemMemory>,
    /// 容器的监控数据详情。
    #[serde(rename = "Containers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<DescribeMultiContainerGroupMetricResponseMonitorDatasItemRecordsItemContainersItem>>,
    /// 文件系统分区数据。
    #[serde(rename = "Filesystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filesystem: Option<Vec<DescribeMultiContainerGroupMetricResponseMonitorDatasItemRecordsItemFilesystemItem>>,
}

impl DescribeMultiContainerGroupMetricResponseMonitorDatasItemRecordsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.network {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Network.{}", k), v2));
            }
        }
        if let Some(ref v) = self.cpu {
            for (k, v2) in v.to_query_params() {
                params.push((format!("CPU.{}", k), v2));
            }
        }
        if let Some(ref v) = self.disk {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Disk.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.timestamp {
            params.push(("Timestamp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.memory {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Memory.{}", k), v2));
            }
        }
        if let Some(ref v) = self.containers {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Containers.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.filesystem {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Filesystem.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeMultiContainerGroupMetricResponseMonitorDatasItem {
    /// 监控数据详情。
    #[serde(rename = "Records")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<DescribeMultiContainerGroupMetricResponseMonitorDatasItemRecordsItem>>,
    /// 实例ID，即容器组ID。
    #[serde(rename = "ContainerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_group_id: Option<String>,
}

impl DescribeMultiContainerGroupMetricResponseMonitorDatasItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.records {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Records.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.container_group_id {
            params.push(("ContainerGroupId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeInstanceOpsRecordsResponseRecordsItem {
    /// 运维任务创建时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 运维任务过期时间。
    #[serde(rename = "ExpireTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<String>,
    /// 运维任务类型。
    #[serde(rename = "OpsType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_type: Option<String>,
    /// 运维任务状态。可能值：
    #[serde(rename = "OpsStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_status: Option<String>,
    /// 运维结果类型。当OpsStatus为Success时返回该参数。
    #[serde(rename = "ResultType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_type: Option<String>,
    /// 运维结果内容，即运维任务所生成文件在的下载地址。当OpsStatus为Success时返回该参数。
    #[serde(rename = "ResultContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_content: Option<String>,
}

impl DescribeInstanceOpsRecordsResponseRecordsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.expire_time {
            params.push(("ExpireTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ops_type {
            params.push(("OpsType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ops_status {
            params.push(("OpsStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.result_type {
            params.push(("ResultType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.result_content {
            params.push(("ResultContent".to_string(), v.to_string()));
        }
        params
    }
}

/// 标签列表。一个资源最多可以绑定20个标签。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TagResourcesRequestTagItem {
    /// 标签键。一旦传入该值，则不允许为空字符串。最多支持 128 个字符，不能以`aliyun`和`acs:`开头，不能包含`http://`或者`https://`。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签键。一旦传入该值，可以为空字符串。最多支持 128 个字符，不能以`aliyun`和`acs:`开头，不能包含`http://`或者`https://`。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl TagResourcesRequestTagItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key {
            params.push(("Key".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        params
    }
}

/// 标签列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListTagResourcesRequestTagItem {
    /// 标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ListTagResourcesRequestTagItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key {
            params.push(("Key".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        params
    }
}

/// 资源列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListTagResourcesResponseTagResourcesItem {
    /// 资源类型。可能值：
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// 资源绑定标签的标签值。
    #[serde(rename = "TagValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
    /// 资源ID。
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// 资源绑定标签的标签键。
    #[serde(rename = "TagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
}

impl ListTagResourcesResponseTagResourcesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_type {
            params.push(("ResourceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_value {
            params.push(("TagValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_id {
            params.push(("ResourceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_key {
            params.push(("TagKey".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupPriceResponsePriceInfoSpotPricesSpotPriceItem {
    /// 可用区。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 抢占式实例的价格。
    #[serde(rename = "SpotPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_price: Option<f32>,
    /// 实例规格。
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// 原价。
    #[serde(rename = "OriginPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_price: Option<f32>,
}

impl DescribeContainerGroupPriceResponsePriceInfoSpotPricesSpotPriceItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.spot_price {
            params.push(("SpotPrice".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_type {
            params.push(("InstanceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.origin_price {
            params.push(("OriginPrice".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupPriceResponsePriceInfoSpotPrices {
    /// 抢占式实例价格信息。
    #[serde(rename = "SpotPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_price: Option<Vec<DescribeContainerGroupPriceResponsePriceInfoSpotPricesSpotPriceItem>>,
}

impl DescribeContainerGroupPriceResponsePriceInfoSpotPrices {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.spot_price {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("SpotPrice.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupPriceResponsePriceInfoPriceDetailInfosDetailInfoItemRulesRuleItem {
    /// 规则描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 规则ID。
    #[serde(rename = "RuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<i64>,
}

impl DescribeContainerGroupPriceResponsePriceInfoPriceDetailInfosDetailInfoItemRulesRuleItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rule_id {
            params.push(("RuleId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupPriceResponsePriceInfoPriceDetailInfosDetailInfoItemRules {
    /// 定价规则子集。
    #[serde(rename = "Rule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<Vec<DescribeContainerGroupPriceResponsePriceInfoPriceDetailInfosDetailInfoItemRulesRuleItem>>,
}

impl DescribeContainerGroupPriceResponsePriceInfoPriceDetailInfosDetailInfoItemRules {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.rule {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Rule.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupPriceResponsePriceInfoPriceDetailInfosDetailInfoItem {
    /// 资源名称。
    #[serde(rename = "Resource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    /// 折扣价。
    #[serde(rename = "DiscountPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_price: Option<f32>,
    /// 成交价。
    #[serde(rename = "TradePrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_price: Option<f32>,
    /// 原价。
    #[serde(rename = "OriginalPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_price: Option<f32>,
    #[serde(rename = "Rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<DescribeContainerGroupPriceResponsePriceInfoPriceDetailInfosDetailInfoItemRules>,
}

impl DescribeContainerGroupPriceResponsePriceInfoPriceDetailInfosDetailInfoItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource {
            params.push(("Resource".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.discount_price {
            params.push(("DiscountPrice".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.trade_price {
            params.push(("TradePrice".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.original_price {
            params.push(("OriginalPrice".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rules {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Rules.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupPriceResponsePriceInfoPriceDetailInfos {
    /// 价格的详细信息。
    #[serde(rename = "DetailInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_info: Option<Vec<DescribeContainerGroupPriceResponsePriceInfoPriceDetailInfosDetailInfoItem>>,
}

impl DescribeContainerGroupPriceResponsePriceInfoPriceDetailInfos {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.detail_info {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("DetailInfo.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 价格。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupPriceResponsePriceInfoPrice {
    /// 折扣价。
    #[serde(rename = "DiscountPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_price: Option<f32>,
    /// 最终价，为原价减去折扣。
    #[serde(rename = "TradePrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_price: Option<f32>,
    /// 原价。
    #[serde(rename = "OriginalPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_price: Option<f32>,
    #[serde(rename = "DetailInfos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_infos: Option<DescribeContainerGroupPriceResponsePriceInfoPriceDetailInfos>,
    /// 货币单位。可能值：
    #[serde(rename = "Currency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

impl DescribeContainerGroupPriceResponsePriceInfoPrice {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.discount_price {
            params.push(("DiscountPrice".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.trade_price {
            params.push(("TradePrice".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.original_price {
            params.push(("OriginalPrice".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.detail_infos {
            for (k, v2) in v.to_query_params() {
                params.push((format!("DetailInfos.{}", k), v2));
            }
        }
        if let Some(ref v) = self.currency {
            params.push(("Currency".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupPriceResponsePriceInfoRulesRuleItem {
    /// 活动规则描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 活动ID。
    #[serde(rename = "RuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<i64>,
}

impl DescribeContainerGroupPriceResponsePriceInfoRulesRuleItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rule_id {
            params.push(("RuleId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupPriceResponsePriceInfoRules {
    /// 活动规则。
    #[serde(rename = "Rule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<Vec<DescribeContainerGroupPriceResponsePriceInfoRulesRuleItem>>,
}

impl DescribeContainerGroupPriceResponsePriceInfoRules {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.rule {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Rule.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 价格信息类型（Price），包括价格和优惠规则。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeContainerGroupPriceResponsePriceInfo {
    #[serde(rename = "SpotPrices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_prices: Option<DescribeContainerGroupPriceResponsePriceInfoSpotPrices>,
    /// 价格。
    #[serde(rename = "Price")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<DescribeContainerGroupPriceResponsePriceInfoPrice>,
    #[serde(rename = "Rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<DescribeContainerGroupPriceResponsePriceInfoRules>,
}

impl DescribeContainerGroupPriceResponsePriceInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.spot_prices {
            for (k, v2) in v.to_query_params() {
                params.push((format!("SpotPrices.{}", k), v2));
            }
        }
        if let Some(ref v) = self.price {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Price.{}", k), v2));
            }
        }
        if let Some(ref v) = self.rules {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Rules.{}", k), v2));
            }
        }
        params
    }
}

/// 要查询的资源信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAvailableResourceRequestDestinationResource {
    /// 资源类型。取值范围：
    #[serde(rename = "Category")]
    pub category: String,
    /// 实例规格族或者具体实例规格。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// vCPU核数。当Category取值为InstanceType时，可以设置。
    #[serde(rename = "Cores")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cores: Option<f32>,
    /// 内存大小。单位为GiB。当Category取值为InstanceType时，可以设置。
    #[serde(rename = "Memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<f32>,
}

impl DescribeAvailableResourceRequestDestinationResource {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Category".to_string(), self.category.to_string()));
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cores {
            params.push(("Cores".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.memory {
            params.push(("Memory".to_string(), v.to_string()));
        }
        params
    }
}

/// 要查询的抢占式实例的资源信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAvailableResourceRequestSpotResource {
    /// 实例的抢占策略。取值范围：
    #[serde(rename = "SpotStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_strategy: Option<String>,
    /// 抢占式实例的每小时最高价格，最多精确到小数点后3位。当SpotStrategy取值为SpotWithPriceLimit时，必须设置SpotPriceLimit。
    #[serde(rename = "SpotPriceLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_price_limit: Option<f64>,
    /// 抢占式实例的保护期。单位为小时。默认为1。可设置为0，表示无保护期。
    #[serde(rename = "SpotDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_duration: Option<i32>,
}

impl DescribeAvailableResourceRequestSpotResource {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.spot_strategy {
            params.push(("SpotStrategy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.spot_price_limit {
            params.push(("SpotPriceLimit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.spot_duration {
            params.push(("SpotDuration".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemAvailableResourcesAvailableResourceItemSupportedResourcesSupportedResourceItem {
    /// 支持的ECS实例规格信息。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 根据库存详细分类资源类别。可能值：
    #[serde(rename = "StatusCategory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_category: Option<String>,
}

impl DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemAvailableResourcesAvailableResourceItemSupportedResourcesSupportedResourceItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status_category {
            params.push(("StatusCategory".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemAvailableResourcesAvailableResourceItemSupportedResources {
    /// 资源信息组成的数组。
    #[serde(rename = "SupportedResource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_resource: Option<Vec<DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemAvailableResourcesAvailableResourceItemSupportedResourcesSupportedResourceItem>>,
}

impl DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemAvailableResourcesAvailableResourceItemSupportedResources {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.supported_resource {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("SupportedResource.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemAvailableResourcesAvailableResourceItem {
    /// 资源类型。可能值：
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "SupportedResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_resources: Option<DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemAvailableResourcesAvailableResourceItemSupportedResources>,
}

impl DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemAvailableResourcesAvailableResourceItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.supported_resources {
            for (k, v2) in v.to_query_params() {
                params.push((format!("SupportedResources.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemAvailableResources {
    /// 该可用区支持的资源信息集合。
    #[serde(rename = "AvailableResource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_resource: Option<Vec<DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemAvailableResourcesAvailableResourceItem>>,
}

impl DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemAvailableResources {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.available_resource {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("AvailableResource.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAvailableResourceResponseAvailableZonesAvailableZoneItem {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 可用区ID。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    #[serde(rename = "AvailableResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_resources: Option<DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemAvailableResources>,
}

impl DescribeAvailableResourceResponseAvailableZonesAvailableZoneItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.available_resources {
            for (k, v2) in v.to_query_params() {
                params.push((format!("AvailableResources.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAvailableResourceResponseAvailableZones {
    /// 可用区级别的资源信息集合。
    #[serde(rename = "AvailableZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_zone: Option<Vec<DescribeAvailableResourceResponseAvailableZonesAvailableZoneItem>>,
}

impl DescribeAvailableResourceResponseAvailableZones {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.available_zone {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("AvailableZone.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// DescribeRegions 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeRegionsRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
}

impl DescribeRegionsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回信息。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeRegionsResponse {
    /// 请求ID，唯一标识。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 地域信息集合。
    #[serde(rename = "Regions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<DescribeRegionsResponseRegionsItem>>,
}

/// CreateContainerGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateContainerGroupRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 实例所属的可用区。如果取值为空，则表示由系统选择。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 实例所属的安全组ID。同一个安全组内的实例之间可以互相访问。
    #[serde(rename = "SecurityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_id: Option<String>,
    /// 实例所属的交换机ID。支持指定多个交换机ID（单次最多10个），各交换机ID之间可以用半角逗号（,）进行分割，例如`vsw-***,vsw-***`。
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// ECI实例名称，即容器组名称。格式要求如下：
    #[serde(rename = "ContainerGroupName")]
    pub container_group_name: String,
    /// 实例重启策略。取值范围：
    #[serde(rename = "RestartPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_policy: Option<String>,
    /// 弹性公网IP ID。
    #[serde(rename = "EipInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eip_instance_id: Option<String>,
    /// 实例级别vCPU大小。单位：核。
    #[serde(rename = "Cpu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<f32>,
    /// 实例级别内存大小。单位：GiB。
    #[serde(rename = "Memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<f32>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// DNS策略。取值范围：
    #[serde(rename = "DnsPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_policy: Option<String>,
    /// 保证请求幂等性。从您的客户端生成一个参数值，确保不同请求间该参数值唯一。只支持ASCII字符，且不能超过64个字符。更多详情，请参阅[如何保证幂等性](~~25693~~)。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// 指定的ECS实例规格，支持多规格。更多信息，请参见[指定ECS规格创建实例](~~114664~~)。
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// 镜像缓存ID。更多信息，请参见[使用镜像缓存加速创建实例](~~141281~~)。
    #[serde(rename = "ImageSnapshotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_snapshot_id: Option<String>,
    /// 实例RAM角色名称。ECI与ECS共用实例RAM角色，更多信息，请参见[通过API使用实例RAM角色](~~61178~~)。
    #[serde(rename = "RamRoleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ram_role_name: Option<String>,
    /// 程序的缓冲时间，用于处理关闭之前的操作。单位为秒。
    #[serde(rename = "TerminationGracePeriodSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_grace_period_seconds: Option<i64>,
    /// 是否自动匹配镜像缓存。默认为false。
    #[serde(rename = "AutoMatchImageCache")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_match_image_cache: Option<bool>,
    /// IPv6地址数。固定为1，即一个ECI实例支持绑定一个IPv6地址。
    #[serde(rename = "Ipv6AddressCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_address_count: Option<i32>,
    /// ECI实例的有效期限，超出该时间后，实例会被强制退出。单位为秒。
    #[serde(rename = "ActiveDeadlineSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_deadline_seconds: Option<i64>,
    /// 实例的抢占策略。取值范围：
    #[serde(rename = "SpotStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_strategy: Option<String>,
    /// 设置抢占式实例的每小时最高价格，最多精确到小数点后3位。
    #[serde(rename = "SpotPriceLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_price_limit: Option<f32>,
    /// 配置了多可用区（通过VSwitchId参数指定多个交换机）时，ECI实例的调度策略。取值范围：
    #[serde(rename = "ScheduleStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_strategy: Option<String>,
    /// 自定义设置coredump生成的core文件的保存目录。更多信息，请参见[设置core文件保存到数据卷](~~167801~~)。
    #[serde(rename = "CorePattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_pattern: Option<String>,
    /// 是否使用共享命名空间。默认为false。
    #[serde(rename = "ShareProcessNamespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_process_namespace: Option<bool>,
    /// 是否自动创建一个EIP，并绑定到ECI实例上。
    #[serde(rename = "AutoCreateEip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_create_eip: Option<bool>,
    /// EIP的带宽，默认为5 Mbps。
    #[serde(rename = "EipBandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eip_bandwidth: Option<i32>,
    /// 设置EIP的线路类型。取值范围：
    #[serde(rename = "EipISP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eip_isp: Option<String>,
    /// 绑定已有的共享带宽包。
    #[serde(rename = "EipCommonBandwidthPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eip_common_bandwidth_package: Option<String>,
    /// 主机名称。
    #[serde(rename = "HostName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    /// 入方向带宽限制。单位：Bps。
    #[serde(rename = "IngressBandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress_bandwidth: Option<i64>,
    /// 出方向带宽限制。单位：Bps。
    #[serde(rename = "EgressBandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_bandwidth: Option<i64>,
    /// CPU物理核心数。仅部分规格支持自定义设置。
    #[serde(rename = "CpuOptionsCore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_options_core: Option<i32>,
    /// 每核线程数。仅部分规格支持自定义设置。配置为1时表示关闭超线程。
    #[serde(rename = "CpuOptionsThreadsPerCore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_options_threads_per_core: Option<i32>,
    /// 该参数暂不支持设置。
    #[serde(rename = "CpuOptionsNuma")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_options_numa: Option<String>,
    /// 增加的临时存储空间大小。单位：GiB。
    #[serde(rename = "EphemeralStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral_storage: Option<i32>,
    /// 标签列表。最多可以绑定20个。更多信息，请参见[使用标签管理实例](~~146608~~)。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<CreateContainerGroupRequestTagItem>>,
    /// 镜像仓库信息。
    #[serde(rename = "ImageRegistryCredential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_registry_credential: Option<Vec<CreateContainerGroupRequestImageRegistryCredentialItem>>,
    /// 容器信息。
    #[serde(rename = "Container")]
    pub container: Vec<CreateContainerGroupRequestContainerItem>,
    /// 数据卷信息。
    #[serde(rename = "Volume")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<Vec<CreateContainerGroupRequestVolumeItem>>,
    /// Init容器列表。
    #[serde(rename = "InitContainer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_container: Option<Vec<CreateContainerGroupRequestInitContainerItem>>,
    /// DNS服务器的IP地址列表。
    #[serde(rename = "DnsConfig.NameServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_config_name_server: Option<Vec<String>>,
    /// DNS搜索域列表。
    #[serde(rename = "DnsConfig.Search")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_config_search: Option<Vec<String>>,
    /// 对象的选项列表。
    #[serde(rename = "DnsConfig.Option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_config_option: Option<Vec<CreateContainerGroupRequestDnsConfigOptionItem>>,
    /// 添加一个ECI的别名。
    #[serde(rename = "HostAliase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_aliase: Option<Vec<CreateContainerGroupRequestHostAliaseItem>>,
    /// 通过安全上下文修改安全sysctl参数。更多信息，请参见[配置Security Context](~~462313~~)。
    #[serde(rename = "SecurityContext.Sysctl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_context_sysctl: Option<Vec<CreateContainerGroupRequestSecurityContextSysctlItem>>,
    /// 通过安全上下文修改非安全sysctl参数。更多信息，请参见[配置Security Context](~~462313~~)。
    #[serde(rename = "HostSecurityContext.Sysctl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_security_context_sysctl: Option<Vec<CreateContainerGroupRequestHostSecurityContextSysctlItem>>,
    /// NTP服务器。
    #[serde(rename = "NtpServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ntp_server: Option<Vec<String>>,
    /// ACR企业版实例信息列表。更多信息，请参见[免密拉取ACR镜像](~~194250~~)。
    #[serde(rename = "AcrRegistryInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acr_registry_info: Option<Vec<CreateContainerGroupRequestAcrRegistryInfoItem>>,
    /// 抢占式实例的保护期。单位为小时。默认为1。可设置为0，表示无保护期。
    #[serde(rename = "SpotDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_duration: Option<i64>,
    /// 是否周期执行：
    #[serde(rename = "StrictSpot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict_spot: Option<bool>,
    /// 自建镜像仓库地址。使用HTTP协议的自建镜像仓库中的镜像创建ECI实例时，需配置该参数，使得ECI使用HTTP协议拉取镜像，避免因协议不同而导致镜像拉取失败。
    #[serde(rename = "PlainHttpRegistry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plain_http_registry: Option<String>,
    /// 自建镜像仓库地址。使用自签发证书的自建镜像仓库中的镜像创建ECI实例时，需配置该参数来跳过证书认证，避免因证书认证失败而导致镜像拉取失败。
    #[serde(rename = "InsecureRegistry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insecure_registry: Option<String>,
    /// 镜像加速模式。取值范围：
    #[serde(rename = "ImageAccelerateMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_accelerate_mode: Option<String>,
    /// 是否开通ECI的IPv6公网通信能力。
    #[serde(rename = "Ipv6GatewayBandwidthEnable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_gateway_bandwidth_enable: Option<bool>,
    /// 当Ipv6GatewayBandwidthEnable配置为true时，配置IPv6地址的公网带宽峰值。取值如下：
    #[serde(rename = "Ipv6GatewayBandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_gateway_bandwidth: Option<String>,
    /// 当ECI规格比申请规格大时，可以开启该配置，确保容器内看到的资源跟申请的资源保持一致。
    #[serde(rename = "ContainerResourceView")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_resource_view: Option<bool>,
    /// 配置为true表示实例启用固定IP地址。具体请参见[配置ECI实例使用固定IP地址](~~2381086~~)。
    #[serde(rename = "FixedIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_ip: Option<String>,
    /// 固定IP地址空闲后的保留时长，即启用固定IP地址的实例释放后，其固定IP地址的保留时长，单位为小时。默认值为48。
    #[serde(rename = "FixedIpRetainHour")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_ip_retain_hour: Option<i32>,
    /// 数据缓存Bucket。
    #[serde(rename = "DataCacheBucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_cache_bucket: Option<String>,
    /// 数据缓存使用的云盘的性能等级。
    #[serde(rename = "DataCachePL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_cache_pl: Option<String>,
    /// 数据缓存使用ESSD AutoPL云盘时，ESSD AutoPL云盘预配置的读写IOPS。
    #[serde(rename = "DataCacheProvisionedIops")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_cache_provisioned_iops: Option<i64>,
    /// 数据缓存使用ESSD AutoPL云盘时，是否开启Burst（性能突发）。更多信息，请参见[ESSD AutoPL云盘](~~368372~~)。
    #[serde(rename = "DataCacheBurstingEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_cache_bursting_enabled: Option<bool>,
    /// 是否只预检此次请求。取值范围：
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// 指定ECI实例的私网IP地址。目前仅支持指定IPv4地址，请确保该IP地址没有被占用。
    #[serde(rename = "PrivateIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    /// 指定ECI实例的操作系统。取值范围：
    #[serde(rename = "OsType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    /// 指定ECI实例的CPU架构。取值范围：
    #[serde(rename = "CpuArchitecture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_architecture: Option<String>,
    /// 指定算力类别。更多信息，请参见[指定算力类别创建实例](~~2638061~~)。
    #[serde(rename = "ComputeCategory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_category: Option<Vec<String>>,
    /// 指定GPU驱动版本。
    #[serde(rename = "GpuDriverVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu_driver_version: Option<String>,
    #[serde(rename = "MaxPendingMinute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_pending_minute: Option<i32>,
}

impl CreateContainerGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.security_group_id {
            params.push(("SecurityGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("VSwitchId".to_string(), v.to_string()));
        }
        params.push(("ContainerGroupName".to_string(), self.container_group_name.to_string()));
        if let Some(ref v) = self.restart_policy {
            params.push(("RestartPolicy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.eip_instance_id {
            params.push(("EipInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cpu {
            params.push(("Cpu".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.memory {
            params.push(("Memory".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dns_policy {
            params.push(("DnsPolicy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_type {
            params.push(("InstanceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.image_snapshot_id {
            params.push(("ImageSnapshotId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ram_role_name {
            params.push(("RamRoleName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.termination_grace_period_seconds {
            params.push(("TerminationGracePeriodSeconds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_match_image_cache {
            params.push(("AutoMatchImageCache".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ipv6_address_count {
            params.push(("Ipv6AddressCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.active_deadline_seconds {
            params.push(("ActiveDeadlineSeconds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.spot_strategy {
            params.push(("SpotStrategy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.spot_price_limit {
            params.push(("SpotPriceLimit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.schedule_strategy {
            params.push(("ScheduleStrategy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.core_pattern {
            params.push(("CorePattern".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.share_process_namespace {
            params.push(("ShareProcessNamespace".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_create_eip {
            params.push(("AutoCreateEip".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.eip_bandwidth {
            params.push(("EipBandwidth".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.eip_isp {
            params.push(("EipISP".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.eip_common_bandwidth_package {
            params.push(("EipCommonBandwidthPackage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.host_name {
            params.push(("HostName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ingress_bandwidth {
            params.push(("IngressBandwidth".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.egress_bandwidth {
            params.push(("EgressBandwidth".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cpu_options_core {
            params.push(("CpuOptionsCore".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cpu_options_threads_per_core {
            params.push(("CpuOptionsThreadsPerCore".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cpu_options_numa {
            params.push(("CpuOptionsNuma".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ephemeral_storage {
            params.push(("EphemeralStorage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tag.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.image_registry_credential {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("ImageRegistryCredential.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        for (i, item) in self.container.iter().enumerate() {
            let prefix = format!("Container.{}", i + 1);
            for (k, v) in item.to_query_params() {
                params.push((format!("{}.{}", prefix, k), v));
            }
        }
        if let Some(ref v) = self.volume {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Volume.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.init_container {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("InitContainer.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.dns_config_name_server {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("DnsConfig.NameServer.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.dns_config_search {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("DnsConfig.Search.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.dns_config_option {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("DnsConfig.Option.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.host_aliase {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("HostAliase.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.security_context_sysctl {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("SecurityContext.Sysctl.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.host_security_context_sysctl {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("HostSecurityContext.Sysctl.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.ntp_server {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("NtpServer.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.acr_registry_info {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("AcrRegistryInfo.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.spot_duration {
            params.push(("SpotDuration".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.strict_spot {
            params.push(("StrictSpot".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.plain_http_registry {
            params.push(("PlainHttpRegistry".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.insecure_registry {
            params.push(("InsecureRegistry".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.image_accelerate_mode {
            params.push(("ImageAccelerateMode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ipv6_gateway_bandwidth_enable {
            params.push(("Ipv6GatewayBandwidthEnable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ipv6_gateway_bandwidth {
            params.push(("Ipv6GatewayBandwidth".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.container_resource_view {
            params.push(("ContainerResourceView".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.fixed_ip {
            params.push(("FixedIp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.fixed_ip_retain_hour {
            params.push(("FixedIpRetainHour".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.data_cache_bucket {
            params.push(("DataCacheBucket".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.data_cache_pl {
            params.push(("DataCachePL".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.data_cache_provisioned_iops {
            params.push(("DataCacheProvisionedIops".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.data_cache_bursting_enabled {
            params.push(("DataCacheBurstingEnabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dry_run {
            params.push(("DryRun".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.private_ip_address {
            params.push(("PrivateIpAddress".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.os_type {
            params.push(("OsType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cpu_architecture {
            params.push(("CpuArchitecture".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.compute_category {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("ComputeCategory.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.gpu_driver_version {
            params.push(("GpuDriverVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_pending_minute {
            params.push(("MaxPendingMinute".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateContainerGroupResponse {
    /// 请求ID，唯一标识。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 实例ID，即容器组ID。
    #[serde(rename = "ContainerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_group_id: Option<String>,
}

/// UpdateContainerGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateContainerGroupRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 指定需要更新的ECI实例ID，即容器组ID。
    #[serde(rename = "ContainerGroupId")]
    pub container_group_id: String,
    /// 实例重启策略。取值范围：
    #[serde(rename = "RestartPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_policy: Option<String>,
    /// 保证请求幂等性。从您的客户端生成一个参数值，确保不同请求间该参数值唯一。只支持ASCII字符，且不能超过64个字符。更多详情，请参阅[如何保证幂等性](~~25693~~)。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// 实例级别（容器组）的vCPU数。
    #[serde(rename = "Cpu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<f32>,
    /// 实例级别的（容器组）的内存数。单位为GiB。
    #[serde(rename = "Memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<f32>,
    /// 所属资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 实例绑定的标签列表。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<UpdateContainerGroupRequestTagItem>>,
    /// 数据卷列表。
    #[serde(rename = "Volume")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<Vec<UpdateContainerGroupRequestVolumeItem>>,
    /// DNS搜索域列表。
    #[serde(rename = "DnsConfig.Search")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_config_search: Option<Vec<String>>,
    /// DNS服务器的IP地址列表。
    #[serde(rename = "DnsConfig.NameServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_config_name_server: Option<Vec<String>>,
    /// DNS配置信息。
    #[serde(rename = "DnsConfig.Option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_config_option: Option<Vec<UpdateContainerGroupRequestDnsConfigOptionItem>>,
    /// 指定新的容器组配置信息。
    #[serde(rename = "Container")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<Vec<UpdateContainerGroupRequestContainerItem>>,
    /// 指定新的Init容器信息。
    #[serde(rename = "InitContainer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_container: Option<Vec<UpdateContainerGroupRequestInitContainerItem>>,
    /// 镜像仓库凭证信息列表。
    #[serde(rename = "ImageRegistryCredential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_registry_credential: Option<Vec<UpdateContainerGroupRequestImageRegistryCredentialItem>>,
    /// ACR企业版实例信息。
    #[serde(rename = "AcrRegistryInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acr_registry_info: Option<Vec<UpdateContainerGroupRequestAcrRegistryInfoItem>>,
    /// 更新类型。取值范围：
    #[serde(rename = "UpdateType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_type: Option<String>,
}

impl UpdateContainerGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("ContainerGroupId".to_string(), self.container_group_id.to_string()));
        if let Some(ref v) = self.restart_policy {
            params.push(("RestartPolicy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cpu {
            params.push(("Cpu".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.memory {
            params.push(("Memory".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tag.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.volume {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Volume.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.dns_config_search {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("DnsConfig.Search.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.dns_config_name_server {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("DnsConfig.NameServer.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.dns_config_option {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("DnsConfig.Option.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.container {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Container.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.init_container {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("InitContainer.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.image_registry_credential {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("ImageRegistryCredential.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.acr_registry_info {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("AcrRegistryInfo.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.update_type {
            params.push(("UpdateType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateContainerGroupResponse {
    /// 请求ID，唯一标识。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeContainerGroups 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeContainerGroupsRequest {
    /// 实例所属的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 实例所属的可用区。如果取值为空，则表示由系统选择。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 实例所属的虚拟交换机ID。
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 下一个查询开始的Token，NextToken为空表示没有下一个。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 限定此次返回资源的数量。如果不设置，默认返回20个，最大不能超过20个。
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// 实例ID序列。最多20个，字符串需按照JSON格式传入。
    #[serde(rename = "ContainerGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_group_ids: Option<String>,
    /// 实例名称，即容器组名称。
    #[serde(rename = "ContainerGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_group_name: Option<String>,
    /// 实例状态。取值范围：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 实例所属的资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 是否返回事件信息。
    #[serde(rename = "WithEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_event: Option<bool>,
    /// 实例标签信息。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<DescribeContainerGroupsRequestTagItem>>,
    /// 指定算力类别。取值范围：
    #[serde(rename = "ComputeCategory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_category: Option<String>,
    /// 实例所属的安全组ID。
    #[serde(rename = "SecurityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_id: Option<String>,
}

impl DescribeContainerGroupsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("VSwitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.next_token {
            params.push(("NextToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.limit {
            params.push(("Limit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.container_group_ids {
            params.push(("ContainerGroupIds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.container_group_name {
            params.push(("ContainerGroupName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.with_event {
            params.push(("WithEvent".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tag.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.compute_category {
            params.push(("ComputeCategory".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.security_group_id {
            params.push(("SecurityGroupId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeContainerGroupsResponse {
    /// 实例数量。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 下一个查询开始的Token。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 请求ID，唯一标识。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 实例信息列表。
    #[serde(rename = "ContainerGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_groups: Option<Vec<DescribeContainerGroupsResponseContainerGroupsItem>>,
}

/// DescribeContainerGroupStatus 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeContainerGroupStatusRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 可用区ID。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 交换机ID。
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// ECI实例ID序列。最多20个，字符串需按照JSON格式传入。
    #[serde(rename = "ContainerGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_group_ids: Option<String>,
    /// 返回最近几秒内有状态更新的ECI实例状态，用于轮询查询状态。
    #[serde(rename = "SinceSecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_second: Option<i32>,
    /// ECI实例绑定的标签。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<DescribeContainerGroupStatusRequestTagItem>>,
    /// 下一个查询开始的Token，NextToken为空表示没有下一个。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 限定此次返回资源（ECI实例）的数量。默认为200个。
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

impl DescribeContainerGroupStatusRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("VSwitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.container_group_ids {
            params.push(("ContainerGroupIds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.since_second {
            params.push(("SinceSecond".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tag.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.next_token {
            params.push(("NextToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.limit {
            params.push(("Limit".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeContainerGroupStatusResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 下一个查询开始的Token。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 结果条数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// ECI实例状态信息集合。
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<DescribeContainerGroupStatusResponseDataItem>>,
}

/// DescribeContainerGroupEvents 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeContainerGroupEventsRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 可用区ID。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 交换机ID。
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// ECI实例ID序列。最多20个，字符串需按照JSON格式传入。
    #[serde(rename = "ContainerGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_group_ids: Option<String>,
    /// ECI实例绑定的标签。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<DescribeContainerGroupEventsRequestTagItem>>,
    /// 下一个查询开始的Token，NextToken为空表示没有下一个。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 事件源。取值范围：
    #[serde(rename = "EventSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source: Option<String>,
    /// 返回最近几秒内新增的事件，用于轮询增量事件。
    #[serde(rename = "SinceSecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_second: Option<i32>,
    /// 限定此次返回资源（ECI实例）的数量。默认为200个。
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

impl DescribeContainerGroupEventsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("VSwitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.container_group_ids {
            params.push(("ContainerGroupIds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tag.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.next_token {
            params.push(("NextToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.event_source {
            params.push(("EventSource".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.since_second {
            params.push(("SinceSecond".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.limit {
            params.push(("Limit".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeContainerGroupEventsResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 返回的事件条数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 事件信息列表。
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<DescribeContainerGroupEventsResponseDataItem>>,
}

/// ResizeContainerGroupVolume 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ResizeContainerGroupVolumeRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 要扩容数据卷的ECI实例ID，即容器组ID。
    #[serde(rename = "ContainerGroupId")]
    pub container_group_id: String,
    /// 保证请求幂等性。从您的客户端生成一个参数值，确保不同请求间该参数值唯一。ClientToken只支持ASCII字符，且不能超过64个字符。更多信息，请参见[如何保证幂等性](~~25693~~)。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// ECI实例挂载的数据卷名称。目前仅支持扩容云盘。
    #[serde(rename = "VolumeName")]
    pub volume_name: String,
    /// 扩容后的云盘容量大小。单位为GiB。取值范围：
    #[serde(rename = "NewSize")]
    pub new_size: i64,
}

impl ResizeContainerGroupVolumeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("ContainerGroupId".to_string(), self.container_group_id.to_string()));
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        params.push(("VolumeName".to_string(), self.volume_name.to_string()));
        params.push(("NewSize".to_string(), self.new_size.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResizeContainerGroupVolumeResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// RestartContainerGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct RestartContainerGroupRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// ECI实例ID，即容器组ID。
    #[serde(rename = "ContainerGroupId")]
    pub container_group_id: String,
    /// 保证请求幂等性。从您的客户端生成一个参数值，确保不同请求间该参数值唯一。只支持ASCII字符，且不能超过64个字符。更多信息，请参见[如何保证幂等性](~~25693~~)。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

impl RestartContainerGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("ContainerGroupId".to_string(), self.container_group_id.to_string()));
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RestartContainerGroupResponse {
    /// 请求ID，唯一标识。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteContainerGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteContainerGroupRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// ECI实例ID，即容器组ID。
    #[serde(rename = "ContainerGroupId")]
    pub container_group_id: String,
    /// 保证请求幂等性。从您的客户端生成一个参数值，确保不同请求间该参数值唯一。只支持ASCII字符，且不能超过64个字符。更多信息，请参见[如何保证幂等性](~~25693~~)。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// 是否强制清理资源。取值范围：
    #[serde(rename = "Force")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

impl DeleteContainerGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("ContainerGroupId".to_string(), self.container_group_id.to_string()));
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.force {
            params.push(("Force".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteContainerGroupResponse {
    /// 请求ID，唯一标识。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ExecContainerCommand 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ExecContainerCommandRequest {
    /// 实例所属的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 实例ID，即容器组ID。
    #[serde(rename = "ContainerGroupId")]
    pub container_group_id: String,
    /// 容器名称。
    #[serde(rename = "ContainerName")]
    pub container_name: String,
    /// 要在容器内执行的命令序列。最多20个命令，单个命令最长256个字符。
    #[serde(rename = "Command")]
    pub command: String,
    /// 是否开启交互。默认为false。
    #[serde(rename = "TTY")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,
    /// 是否标准输入。默认为true。
    #[serde(rename = "Stdin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdin: Option<bool>,
    /// 是否立即执行命令并同步返回结果。默认为false。
    #[serde(rename = "Sync")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync: Option<bool>,
}

impl ExecContainerCommandRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("ContainerGroupId".to_string(), self.container_group_id.to_string()));
        params.push(("ContainerName".to_string(), self.container_name.to_string()));
        params.push(("Command".to_string(), self.command.to_string()));
        if let Some(ref v) = self.tty {
            params.push(("TTY".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.stdin {
            params.push(("Stdin".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sync {
            params.push(("Sync".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExecContainerCommandResponse {
    /// 请求ID，唯一标识。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// WebSocket URL。您可以利用WebSocket URL建立与容器的连接。
    #[serde(rename = "WebSocketUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_socket_uri: Option<String>,
    /// HTTP URL。在30秒内访问该地址可以进入到容器。更多信息，请参见[使用并集成ECI Terminal](~~202846~~)。
    #[serde(rename = "HttpUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_url: Option<String>,
    /// 命令的返回结果。当Sync设置为true时返回该参数。
    #[serde(rename = "SyncResponse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_response: Option<String>,
}

/// DescribeContainerLog 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeContainerLogRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// ECI实例ID，即容器组ID。
    #[serde(rename = "ContainerGroupId")]
    pub container_group_id: String,
    /// 容器名称。
    #[serde(rename = "ContainerName")]
    pub container_name: String,
    /// UTC时间，RFC3339标准。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 查询最新多少行的日志内容。默认500行，最大2000行 。
    #[serde(rename = "Tail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tail: Option<i32>,
    /// 是否查询上一个容器（如果容器退出重启了）。取值范围：
    #[serde(rename = "LastTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_time: Option<bool>,
    /// 查询最近多少秒内的日志。例如：10s、20s、30s。
    #[serde(rename = "SinceSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_seconds: Option<i32>,
    /// 日志总大小的限制。单位为Byte，取值范围为1~1048576（1 MB）。
    #[serde(rename = "LimitBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_bytes: Option<i64>,
    /// 是否返回日志时间戳。取值范围：
    #[serde(rename = "Timestamps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamps: Option<bool>,
}

impl DescribeContainerLogRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params.push(("ContainerGroupId".to_string(), self.container_group_id.to_string()));
        params.push(("ContainerName".to_string(), self.container_name.to_string()));
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tail {
            params.push(("Tail".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.last_time {
            params.push(("LastTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.since_seconds {
            params.push(("SinceSeconds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.limit_bytes {
            params.push(("LimitBytes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.timestamps {
            params.push(("Timestamps".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeContainerLogResponse {
    /// 请求ID，唯一标识。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 容器名称。
    #[serde(rename = "ContainerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    /// 日志内容。
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

/// CommitContainer 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CommitContainerRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 要创建CommitContainer任务的ECI实例。
    #[serde(rename = "ContainerGroupId")]
    pub container_group_id: String,
    /// 要制作镜像的容器的名称。
    #[serde(rename = "ContainerName")]
    pub container_name: String,
    /// 容器镜像信息。
    #[serde(rename = "Image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<CommitContainerRequestImage>,
    /// ACR企业版实例的访问凭证配置信息。
    #[serde(rename = "AcrRegistryInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acr_registry_info: Option<CommitContainerRequestAcrRegistryInfo>,
    /// 授权需要的ARN信息。
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<CommitContainerRequestArn>,
}

impl CommitContainerRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("ContainerGroupId".to_string(), self.container_group_id.to_string()));
        params.push(("ContainerName".to_string(), self.container_name.to_string()));
        if let Some(ref v) = self.image {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Image.{}", k), v2));
            }
        }
        if let Some(ref v) = self.acr_registry_info {
            for (k, v2) in v.to_query_params() {
                params.push((format!("AcrRegistryInfo.{}", k), v2));
            }
        }
        if let Some(ref v) = self.arn {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Arn.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CommitContainerResponse {
    /// 请求ID，唯一标识。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 任务ID。
    #[serde(rename = "TaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

/// DescribeCommitContainerTask 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCommitContainerTaskRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 查询凭证（Token），取值为上一次API调用返回的NextToken参数值。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 分页查询时每页行数。
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// 执行CommitContainer任务的ECI实例ID。
    #[serde(rename = "ContainerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_group_id: Option<String>,
    /// 任务状态。取值范围：
    #[serde(rename = "TaskStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_status: Option<String>,
    /// 任务ID。
    #[serde(rename = "TaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<Vec<String>>,
}

impl DescribeCommitContainerTaskRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.next_token {
            params.push(("NextToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_results {
            params.push(("MaxResults".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.container_group_id {
            params.push(("ContainerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_status {
            params.push(("TaskStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_id {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("TaskId.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeCommitContainerTaskResponse {
    /// 符合查询条件的结果条数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 本次调用返回的查询凭证值。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 每页显示行数。
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// 任务详细信息。
    #[serde(rename = "CommitTasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_tasks: Option<Vec<DescribeCommitContainerTaskResponseCommitTasksItem>>,
}

/// CreateImageCache 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateImageCacheRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 可用区。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 安全组ID。
    #[serde(rename = "SecurityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_id: Option<String>,
    /// 交换机ID。支持指定多个交换机ID（单次最多10个），各交换机ID之间可以用半角逗号（,）进行分割，例如`vsw-***,vsw-***`。
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 镜像缓存名称。
    #[serde(rename = "ImageCacheName")]
    pub image_cache_name: String,
    /// 弹性公网IP。如果需要拉取公网镜像，需要确保ECI实例能够访问公网，您可以配置EIP或者NAT网关实现公网访问。
    #[serde(rename = "EipInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eip_instance_id: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 保证请求幂等性。从您的客户端生成一个参数值，确保不同请求间该参数值唯一。只支持ASCII字符，且不能超过64个字符。更多信息，请参见[如何保证幂等性](~~25693~~)。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// 镜像缓存的大小。默认为20 GiB。
    #[serde(rename = "ImageCacheSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_cache_size: Option<i32>,
    /// 镜像缓存保留时间，过期将会被清理，默认永不过期。
    #[serde(rename = "RetentionDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i32>,
    /// 是否开启镜像缓存复用。开启后，新创建的镜像缓存可以复用已有镜像缓存的镜像层，加快镜像缓存的制作速度。取值范围：
    #[serde(rename = "AutoMatchImageCache")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_match_image_cache: Option<bool>,
    /// 镜像仓库信息。
    #[serde(rename = "ImageRegistryCredential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_registry_credential: Option<Vec<CreateImageCacheRequestImageRegistryCredentialItem>>,
    /// 用于制作镜像缓存的容器镜像。
    #[serde(rename = "Image")]
    pub image: Vec<String>,
    /// 镜像缓存标签信息，最多20个。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<CreateImageCacheRequestTagItem>>,
    /// 是否开启极速镜像缓存功能，开启后，可以加速镜像缓存创建。取值范围：
    #[serde(rename = "Flash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flash: Option<bool>,
    /// ACR实例信息。更多信息，请参见[免密拉取ACR镜像](~~194250~~)。
    #[serde(rename = "AcrRegistryInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acr_registry_info: Option<Vec<CreateImageCacheRequestAcrRegistryInfoItem>>,
    /// 注解。该参数暂不对外使用。
    #[serde(rename = "Annotations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<String>,
    /// 自建镜像仓库地址。使用HTTP协议的自建镜像仓库中的镜像创建镜像缓存时，需配置该参数，使得ECI使用HTTP协议拉取镜像，避免因协议不同而导致镜像拉取失败。
    #[serde(rename = "PlainHttpRegistry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plain_http_registry: Option<String>,
    /// 自建镜像仓库地址。
    #[serde(rename = "InsecureRegistry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insecure_registry: Option<String>,
    /// 标准快照副本数。默认情况下，一个镜像缓存对应一个快照。如果该镜像缓存将用于批量创建多个ECI实例，建议您配置该参数实现快照多副本。推荐每1000个ECI实例增加一个快照副本。
    #[serde(rename = "StandardCopyCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_copy_count: Option<i32>,
    /// 本地快照副本数。默认情况下，一个镜像缓存对应一个快照。如果该镜像缓存将用于批量创建多个ECI实例，建议您配置该参数实现快照多副本。推荐每1000个ECI实例增加一个快照副本。
    #[serde(rename = "FlashCopyCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flash_copy_count: Option<i32>,
    /// 镜像缓存的淘汰策略。默认为空，表示一直保留。
    #[serde(rename = "EliminationStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elimination_strategy: Option<String>,
    /// 容器镜像的操作系统。取值范围：
    #[serde(rename = "OsType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
}

impl CreateImageCacheRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.security_group_id {
            params.push(("SecurityGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("VSwitchId".to_string(), v.to_string()));
        }
        params.push(("ImageCacheName".to_string(), self.image_cache_name.to_string()));
        if let Some(ref v) = self.eip_instance_id {
            params.push(("EipInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.image_cache_size {
            params.push(("ImageCacheSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.retention_days {
            params.push(("RetentionDays".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_match_image_cache {
            params.push(("AutoMatchImageCache".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.image_registry_credential {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("ImageRegistryCredential.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        for (i, item) in self.image.iter().enumerate() {
            params.push((format!("Image.{}", i + 1), item.to_string()));
        }
        if let Some(ref v) = self.tag {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tag.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.flash {
            params.push(("Flash".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.acr_registry_info {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("AcrRegistryInfo.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.annotations {
            params.push(("Annotations".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.plain_http_registry {
            params.push(("PlainHttpRegistry".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.insecure_registry {
            params.push(("InsecureRegistry".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.standard_copy_count {
            params.push(("StandardCopyCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.flash_copy_count {
            params.push(("FlashCopyCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.elimination_strategy {
            params.push(("EliminationStrategy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.os_type {
            params.push(("OsType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateImageCacheResponse {
    /// 请求ID，唯一标识。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 镜像缓存ID。
    #[serde(rename = "ImageCacheId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_cache_id: Option<String>,
    /// 用于中转创建镜像缓存的ECI实例ID。
    #[serde(rename = "ContainerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_group_id: Option<String>,
}

/// DeleteImageCache 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteImageCacheRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 镜像缓存ID。
    #[serde(rename = "ImageCacheId")]
    pub image_cache_id: String,
    /// 保证请求幂等性。从您的客户端生成一个参数值，确保不同请求间该参数值唯一。只支持ASCII字符，且不能超过64个字符。更多信息，请参见[如何保证幂等性](~~25693~~)。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

impl DeleteImageCacheRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("ImageCacheId".to_string(), self.image_cache_id.to_string()));
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteImageCacheResponse {
    /// 请求ID，唯一标识。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// UpdateImageCache 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateImageCacheRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 镜像缓存ID。
    #[serde(rename = "ImageCacheId")]
    pub image_cache_id: String,
    /// 安全组ID。
    #[serde(rename = "SecurityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_id: Option<String>,
    /// 交换机ID。
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 镜像缓存名称。
    #[serde(rename = "ImageCacheName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_cache_name: Option<String>,
    /// 弹性公网IP。如果需要拉取公网镜像，需要确保ECI实例能够访问公网，您可以配置EIP或者NAT网关实现公网访问。
    #[serde(rename = "EipInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eip_instance_id: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 保证请求幂等性。从您的客户端生成一个参数值，确保不同请求间该参数值唯一。只支持ASCII字符，且不能超过64个字符。更多信息，请参见[如何保证幂等性](~~25693~~)。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// 镜像缓存的大小。默认为20 GiB。
    #[serde(rename = "ImageCacheSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_cache_size: Option<i32>,
    /// 镜像缓存保留时间，过期将会被清理，默认永不过期。
    #[serde(rename = "RetentionDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i32>,
    /// 是否开启镜像缓存复用。开启后，新创建的镜像缓存可以复用已有镜像缓存的镜像层，加快镜像缓存的制作速度。取值范围：
    #[serde(rename = "AutoMatchImageCache")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_match_image_cache: Option<bool>,
    /// 镜像仓库信息。
    #[serde(rename = "ImageRegistryCredential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_registry_credential: Option<Vec<UpdateImageCacheRequestImageRegistryCredentialItem>>,
    /// 用于制作镜像缓存的容器镜像列表。
    #[serde(rename = "Image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Vec<String>>,
    /// 镜像缓存绑定的标签信息，最多20个。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<UpdateImageCacheRequestTagItem>>,
    /// 是否开启极速镜像缓存功能，开启后，可以加速镜像缓存创建。取值范围：
    #[serde(rename = "Flash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flash: Option<bool>,
    /// ACR企业版实例信息。
    #[serde(rename = "AcrRegistryInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acr_registry_info: Option<Vec<UpdateImageCacheRequestAcrRegistryInfoItem>>,
    /// 标准快照副本数。默认情况下，一个镜像缓存对应一个快照。如果该镜像缓存将用于批量创建多个ECI实例，建议您配置该参数实现快照多副本。推荐每1000个ECI实例增加一个快照副本。
    #[serde(rename = "StandardCopyCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_copy_count: Option<i32>,
    /// 本地快照副本数。默认情况下，一个镜像缓存对应一个快照。如果该镜像缓存将用于批量创建多个ECI实例，建议您配置该参数实现快照多副本。推荐每1000个ECI实例增加一个快照副本。
    #[serde(rename = "FlashCopyCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flash_copy_count: Option<i32>,
    /// 镜像缓存的淘汰策略。默认为空，表示一直保留。
    #[serde(rename = "EliminationStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elimination_strategy: Option<String>,
}

impl UpdateImageCacheRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("ImageCacheId".to_string(), self.image_cache_id.to_string()));
        if let Some(ref v) = self.security_group_id {
            params.push(("SecurityGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("VSwitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.image_cache_name {
            params.push(("ImageCacheName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.eip_instance_id {
            params.push(("EipInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.image_cache_size {
            params.push(("ImageCacheSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.retention_days {
            params.push(("RetentionDays".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_match_image_cache {
            params.push(("AutoMatchImageCache".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.image_registry_credential {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("ImageRegistryCredential.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.image {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Image.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.tag {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tag.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.flash {
            params.push(("Flash".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.acr_registry_info {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("AcrRegistryInfo.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.standard_copy_count {
            params.push(("StandardCopyCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.flash_copy_count {
            params.push(("FlashCopyCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.elimination_strategy {
            params.push(("EliminationStrategy".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateImageCacheResponse {
    /// 请求ID，唯一标识。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeImageCaches 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeImageCachesRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 镜像缓存ID。
    #[serde(rename = "ImageCacheId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_cache_id: Option<String>,
    /// 镜像缓存名称。
    #[serde(rename = "ImageCacheName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_cache_name: Option<String>,
    /// 镜像缓存对应的快照ID。
    #[serde(rename = "SnapshotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    /// 容器镜像。
    #[serde(rename = "Image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// 所属资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 镜像缓存绑定的标签信息。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<DescribeImageCachesRequestTagItem>>,
    /// 指定容器镜像，用于查询最佳匹配的镜像缓存，最多100个。
    #[serde(rename = "MatchImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_image: Option<Vec<String>>,
    /// 查询结果条数上限。
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// 查询凭证（Token），取值为上一次API调用返回的NextToken参数值。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 是否完全匹配容器镜像。
    #[serde(rename = "ImageFullMatch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_full_match: Option<bool>,
    /// 完全匹配容器镜像的镜像缓存个数。
    #[serde(rename = "ImageMatchCountRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_match_count_request: Option<i32>,
}

impl DescribeImageCachesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.image_cache_id {
            params.push(("ImageCacheId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.image_cache_name {
            params.push(("ImageCacheName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.snapshot_id {
            params.push(("SnapshotId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.image {
            params.push(("Image".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tag.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.match_image {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("MatchImage.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.limit {
            params.push(("Limit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.next_token {
            params.push(("NextToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.image_full_match {
            params.push(("ImageFullMatch".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.image_match_count_request {
            params.push(("ImageMatchCountRequest".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeImageCachesResponse {
    /// 请求ID，唯一标识。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 镜像缓存信息列表。
    #[serde(rename = "ImageCaches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_caches: Option<Vec<DescribeImageCachesResponseImageCachesItem>>,
    /// 本次调用返回的查询凭证值。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 查询结果总条数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
}

/// CreateDataCache 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateDataCacheRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 创建数据缓存过程中生成的ECI实例所属的安全组ID。
    #[serde(rename = "SecurityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_id: Option<String>,
    /// 创建数据缓存过程中生成的ECI实例所属的交换机ID。
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 数据的存储空间。默认为default。支持自定义，以便进行业务分组和避免路径冲突。
    #[serde(rename = "Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    /// 数据存储的路径。
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// 数据缓存名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 数据缓存大小。单位为GiB，默认为20 GiB。请根据实际数据量评估所需大小。
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    /// 数据源。
    #[serde(rename = "DataSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<CreateDataCacheRequestDataSource>,
    /// 保留天数。过期会被清理。默认不过期。
    #[serde(rename = "RetentionDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i32>,
    /// 数据缓存绑定的标签列表。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<CreateDataCacheRequestTagItem>>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 保证请求幂等性。从您的客户端生成一个参数值，确保不同请求间该参数值唯一。只支持ASCII字符，且不能超过64个字符。更多信息，请参见[如何保证幂等性](~~25693~~)。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// 绑定已有的弹性公网IP（EIP）。
    #[serde(rename = "EipInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eip_instance_id: Option<String>,
    /// 创建一个弹性公网IP（EIP）并绑定。
    #[serde(rename = "EipCreateParam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eip_create_param: Option<CreateDataCacheRequestEipCreateParam>,
}

impl CreateDataCacheRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.security_group_id {
            params.push(("SecurityGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("VSwitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bucket {
            params.push(("Bucket".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.path {
            params.push(("Path".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size {
            params.push(("Size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.data_source {
            for (k, v2) in v.to_query_params() {
                params.push((format!("DataSource.{}", k), v2));
            }
        }
        if let Some(ref v) = self.retention_days {
            params.push(("RetentionDays".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tag.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.eip_instance_id {
            params.push(("EipInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.eip_create_param {
            for (k, v2) in v.to_query_params() {
                params.push((format!("EipCreateParam.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateDataCacheResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 数据缓存ID。
    #[serde(rename = "DataCacheId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_cache_id: Option<String>,
}

/// DescribeDataCaches 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDataCachesRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 数据缓存ID。
    #[serde(rename = "DataCacheId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_cache_id: Option<Vec<String>>,
    /// 数据缓存Bucket。默认为default。
    #[serde(rename = "Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    /// 数据缓存对应Virtual Host的目录。
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// 标签列表。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<DescribeDataCachesRequestTagItem>>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 查询结果条数上限。默认值和最大值为20。
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// 查询凭证（Token），取值为上一次API调用返回的NextToken参数值。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl DescribeDataCachesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.data_cache_id {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("DataCacheId.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.bucket {
            params.push(("Bucket".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.path {
            params.push(("Path".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tag.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.limit {
            params.push(("Limit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.next_token {
            params.push(("NextToken".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDataCachesResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 数据缓存信息。
    #[serde(rename = "DataCaches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_caches: Option<Vec<DescribeDataCachesResponseDataCachesItem>>,
    /// 查询凭证（Token），取值为上一次API调用返回的NextToken参数值。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 符合查询条件的结果条数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
}

/// UpdateDataCache 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateDataCacheRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 数据缓存ID。
    #[serde(rename = "DataCacheId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_cache_id: Option<String>,
    /// 安全组ID。
    #[serde(rename = "SecurityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_id: Option<String>,
    /// 交换机ID。
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 数据缓存Bucket。默认为default。
    #[serde(rename = "Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    /// 数据缓存名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 数据缓存大小。
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    /// 数据源信息。
    #[serde(rename = "DataSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<UpdateDataCacheRequestDataSource>,
    /// 保留天数。过期会被清理。默认不过期。
    #[serde(rename = "RetentionDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i32>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 标签列表。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<UpdateDataCacheRequestTagItem>>,
    /// 保证请求幂等性。从您的客户端生成一个参数值，确保不同请求间该参数值唯一。只支持ASCII字符，且不能超过64个字符。更多信息，请参见[如何保证幂等性](~~25693~~)。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// 弹性公网IP。如果VPC没有配置NAT网关，可以绑定弹性公网IP，以便拉取公网数据。
    #[serde(rename = "EipInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eip_instance_id: Option<String>,
    /// 创建一个弹性公网IP（EIP）并绑定。
    #[serde(rename = "EipCreateParam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eip_create_param: Option<UpdateDataCacheRequestEipCreateParam>,
}

impl UpdateDataCacheRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.data_cache_id {
            params.push(("DataCacheId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.security_group_id {
            params.push(("SecurityGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("VSwitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bucket {
            params.push(("Bucket".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size {
            params.push(("Size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.data_source {
            for (k, v2) in v.to_query_params() {
                params.push((format!("DataSource.{}", k), v2));
            }
        }
        if let Some(ref v) = self.retention_days {
            params.push(("RetentionDays".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tag.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.eip_instance_id {
            params.push(("EipInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.eip_create_param {
            for (k, v2) in v.to_query_params() {
                params.push((format!("EipCreateParam.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateDataCacheResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CopyDataCache 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CopyDataCacheRequest {
    /// 已有数据缓存所在地域。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 数据缓存Bucket。
    #[serde(rename = "Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    /// 数据缓存路径
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// 数据缓存名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 数据缓存保留天数。
    #[serde(rename = "RetentionDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i32>,
    /// 数据缓存标签列表。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<CopyDataCacheRequestTagItem>>,
    /// 数据缓存所属资源组 。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 保证请求幂等性。从您的客户端生成一个参数值，确保不同请求间该参数值唯一。只支持ASCII字符，且不能超过64个字符。更多信息，请参见[如何保证幂等性](~~25693~~)。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// 数据缓存ID。
    #[serde(rename = "DataCacheId")]
    pub data_cache_id: String,
    /// 要拷贝到的目的地域。
    #[serde(rename = "DestinationRegionId")]
    pub destination_region_id: String,
}

impl CopyDataCacheRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.bucket {
            params.push(("Bucket".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.path {
            params.push(("Path".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.retention_days {
            params.push(("RetentionDays".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tag.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        params.push(("DataCacheId".to_string(), self.data_cache_id.to_string()));
        params.push(("DestinationRegionId".to_string(), self.destination_region_id.to_string()));
        params
    }
}

/// 返回参数列表。
#[derive(Debug, Clone, Deserialize)]
pub struct CopyDataCacheResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 拷贝到新的地域后，生成的数据缓存ID。
    #[serde(rename = "DataCacheId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_cache_id: Option<String>,
}

/// DeleteDataCache 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteDataCacheRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 数据缓存ID。
    #[serde(rename = "DataCacheId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_cache_id: Option<String>,
    /// 保证请求幂等性。从您的客户端生成一个参数值，确保不同请求间该参数值唯一。只支持ASCII字符，且不能超过64个字符。更多信息，请参见[如何保证幂等性](~~25693~~)。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// 数据缓存Bucket。默认为default。
    #[serde(rename = "Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    /// 数据缓存对应Virtual Host的目录。
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

impl DeleteDataCacheRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.data_cache_id {
            params.push(("DataCacheId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bucket {
            params.push(("Bucket".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.path {
            params.push(("Path".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteDataCacheResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreateVirtualNode 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateVirtualNodeRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 可用区ID。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 安全组ID。虚拟节点以及该虚拟节点下的ECI实例将加入到该安全组中。
    #[serde(rename = "SecurityGroupId")]
    pub security_group_id: String,
    /// 交换机ID。虚拟节点以及虚拟节点下的ECI实例所使用的交换机。
    #[serde(rename = "VSwitchId")]
    pub v_switch_id: String,
    /// 虚拟节点的名称。长度为2~128个英文字符，可以包含小写英文字符、数字、半角句号（.）或者短划线（-）。
    #[serde(rename = "VirtualNodeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_node_name: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 保证请求幂等性。从您的客户端生成一个参数值，确保不同请求间该参数值唯一。只支持ASCII字符，且不能超过64个字符。更多详情，请参见[如何保证幂等性](~~25693~~)。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// 是否支持公网。默认为false。
    #[serde(rename = "EnablePublicNetwork")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_public_network: Option<bool>,
    /// 弹性公网IP的ID。
    #[serde(rename = "EipInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eip_instance_id: Option<String>,
    /// 虚拟节点要连接的Kubernetes集群的KubeConfig。需进行Base64编码后传入。
    #[serde(rename = "KubeConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kube_config: Option<String>,
    /// 虚拟节点支持的自定义资源。如果ECI Pod的request中声明了相应的自定义资源，则该Pod会被调度到VNode。
    #[serde(rename = "CustomResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_resources: Option<String>,
    /// 集群的域名。配置后，除了主机的搜索域外，Kubelet会配置所有容器来搜索该域名。
    #[serde(rename = "ClusterDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_domain: Option<String>,
    /// DNS服务器的IP地址。如果ECI Pod中设置了`dnsPolicy=ClusterFirst`，则使用该配置值为容器提供DNS服务。
    #[serde(rename = "ClusterDNS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_dns: Option<String>,
    /// 虚拟节点标签信息。最多可以配置20个。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<CreateVirtualNodeRequestTagItem>>,
    /// 虚拟节点污点信息。最多可以配置20个。
    #[serde(rename = "Taint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taint: Option<Vec<CreateVirtualNodeRequestTaintItem>>,
    /// 是否启用TLS启动引导。启用后，请使用TLS启动引导的KubeConfig证书。取值范围：
    #[serde(rename = "TlsBootstrapEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_bootstrap_enabled: Option<bool>,
}

impl CreateVirtualNodeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        params.push(("SecurityGroupId".to_string(), self.security_group_id.to_string()));
        params.push(("VSwitchId".to_string(), self.v_switch_id.to_string()));
        if let Some(ref v) = self.virtual_node_name {
            params.push(("VirtualNodeName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_public_network {
            params.push(("EnablePublicNetwork".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.eip_instance_id {
            params.push(("EipInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.kube_config {
            params.push(("KubeConfig".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.custom_resources {
            params.push(("CustomResources".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_domain {
            params.push(("ClusterDomain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_dns {
            params.push(("ClusterDNS".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tag.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.taint {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Taint.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.tls_bootstrap_enabled {
            params.push(("TlsBootstrapEnabled".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回信息。
#[derive(Debug, Clone, Deserialize)]
pub struct CreateVirtualNodeResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 虚拟节点ID。
    #[serde(rename = "VirtualNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_node_id: Option<String>,
}

/// DeleteVirtualNode 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteVirtualNodeRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 虚拟节点ID。
    #[serde(rename = "VirtualNodeId")]
    pub virtual_node_id: String,
    /// 保证请求幂等性。从您的客户端生成一个参数值，确保不同请求间该参数值唯一。只支持ASCII字符，且不能超过64个字符。更多详情，请参阅[如何保证幂等性](~~25693~~)。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

impl DeleteVirtualNodeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("VirtualNodeId".to_string(), self.virtual_node_id.to_string()));
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteVirtualNodeResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// UpdateVirtualNode 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateVirtualNodeRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 虚拟节点ID。
    #[serde(rename = "VirtualNodeId")]
    pub virtual_node_id: String,
    /// 虚拟节点名称。
    #[serde(rename = "VirtualNodeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_node_name: Option<String>,
    /// 虚拟节点支持的自定义资源。如果ECI Pod的request中声明了相应的自定义资源，则该Pod会被调度到VNode。
    #[serde(rename = "CustomResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_resources: Option<String>,
    /// 集群的域名。配置后，除了主机的搜索域外，Kubelet会配置所有容器来搜索该域名。
    #[serde(rename = "ClusterDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_domain: Option<String>,
    /// DNS服务器的IP地址。如果ECI Pod中设置了`dnsPolicy=ClusterFirst`，则使用该配置值为容器提供DNS服务。
    #[serde(rename = "ClusterDNS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_dns: Option<String>,
    /// 保证请求幂等性。从您的客户端生成一个参数值，确保不同请求间该参数值唯一。只支持ASCII字符，且不能超过64个字符。更多详情，请参阅[如何保证幂等性](~~25693~~)。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// 虚拟节点标签信息。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<UpdateVirtualNodeRequestTagItem>>,
}

impl UpdateVirtualNodeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("VirtualNodeId".to_string(), self.virtual_node_id.to_string()));
        if let Some(ref v) = self.virtual_node_name {
            params.push(("VirtualNodeName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.custom_resources {
            params.push(("CustomResources".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_domain {
            params.push(("ClusterDomain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_dns {
            params.push(("ClusterDNS".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tag.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateVirtualNodeResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeVirtualNodes 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeVirtualNodesRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 虚拟节点名称。
    #[serde(rename = "VirtualNodeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_node_name: Option<String>,
    /// 虚拟节点ID序列。最多20个，字符串需按照JSON格式传入。
    #[serde(rename = "VirtualNodeIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_node_ids: Option<String>,
    /// 保证请求幂等性。从您的客户端生成一个参数值，确保不同请求间该参数值唯一。只支持ASCII字符，且不能超过64个字符。更多详情，请参阅[如何保证幂等性](~~25693~~)。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// 虚拟节点状态。取值范围：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 下一个查询开始的Token，NextToken为空表示没有下一个。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 限制本次查询返回的资源数量。默认返回20个，最大不能超过20个。
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// 虚拟节点绑定的标签列表。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<DescribeVirtualNodesRequestTagItem>>,
}

impl DescribeVirtualNodesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.virtual_node_name {
            params.push(("VirtualNodeName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.virtual_node_ids {
            params.push(("VirtualNodeIds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.next_token {
            params.push(("NextToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.limit {
            params.push(("Limit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tag.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeVirtualNodesResponse {
    /// 查询到的虚拟节点数量。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 下一个查询开始的Token。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 虚拟节点信息集合。
    #[serde(rename = "VirtualNodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_nodes: Option<Vec<DescribeVirtualNodesResponseVirtualNodesItem>>,
}

/// DescribeContainerGroupMetric 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeContainerGroupMetricRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// ECI实例ID，即容器组ID。
    #[serde(rename = "ContainerGroupId")]
    pub container_group_id: String,
    /// 查询某个时间段内的监控数据时，设置的开始时间。取值必须为30天之内，默认为EndTime减去5分钟。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 查询某个时间段内的监控数据时，设置的结束时间。默认为当前时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 数据聚合周期，单位为秒。目前只支持15、30、60和600秒。默认为60秒。
    #[serde(rename = "Period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
}

impl DescribeContainerGroupMetricRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("ContainerGroupId".to_string(), self.container_group_id.to_string()));
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.period {
            params.push(("Period".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeContainerGroupMetricResponse {
    /// 请求ID，唯一标识。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// ECI实例ID，即容器组ID。
    #[serde(rename = "ContainerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_group_id: Option<String>,
    /// ECI实例的监控数据详情。
    #[serde(rename = "Records")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<DescribeContainerGroupMetricResponseRecordsItem>>,
}

/// DescribeMultiContainerGroupMetric 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeMultiContainerGroupMetricRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// ECI实例ID，即容器组ID。格式为JSON数组，一次最多支持20个ID。
    #[serde(rename = "ContainerGroupIds")]
    pub container_group_ids: String,
    /// 实例所属的资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 监控信息类型。目前仅支持配置为summary，表示返回Records。
    #[serde(rename = "MetricType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_type: Option<String>,
}

impl DescribeMultiContainerGroupMetricRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("ContainerGroupIds".to_string(), self.container_group_ids.to_string()));
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.metric_type {
            params.push(("MetricType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeMultiContainerGroupMetricResponse {
    /// 请求ID，唯一标识。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 监控信息集合。
    #[serde(rename = "MonitorDatas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_datas: Option<Vec<DescribeMultiContainerGroupMetricResponseMonitorDatasItem>>,
}

/// CreateInstanceOpsTask 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateInstanceOpsTaskRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// ECI实例ID。
    #[serde(rename = "ContainerGroupId")]
    pub container_group_id: String,
    /// 运维任务类型。取值范围：
    #[serde(rename = "OpsType")]
    pub ops_type: String,
    /// 运维任务参数。根据OpsType的取值，可设置相应参数。
    #[serde(rename = "OpsValue")]
    pub ops_value: String,
}

impl CreateInstanceOpsTaskRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("ContainerGroupId".to_string(), self.container_group_id.to_string()));
        params.push(("OpsType".to_string(), self.ops_type.to_string()));
        params.push(("OpsValue".to_string(), self.ops_value.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateInstanceOpsTaskResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 运维任务执行结果。
    #[serde(rename = "Result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}

/// DescribeInstanceOpsRecords 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeInstanceOpsRecordsRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// ECI实例ID。
    #[serde(rename = "ContainerGroupId")]
    pub container_group_id: String,
    /// 运维任务类型。取值范围：
    #[serde(rename = "OpsType")]
    pub ops_type: String,
}

impl DescribeInstanceOpsRecordsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("ContainerGroupId".to_string(), self.container_group_id.to_string()));
        params.push(("OpsType".to_string(), self.ops_type.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeInstanceOpsRecordsResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 运维任务信息合集。
    #[serde(rename = "Records")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<DescribeInstanceOpsRecordsResponseRecordsItem>>,
}

/// TagResources 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct TagResourcesRequest {
    /// 资源所属的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 资源ID列表。
    #[serde(rename = "ResourceId")]
    pub resource_id: Vec<String>,
    /// 标签列表。一个资源最多可以绑定20个标签。
    #[serde(rename = "Tag")]
    pub tag: Vec<TagResourcesRequestTagItem>,
    /// 资源类型。取值范围：
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
}

impl TagResourcesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        for (i, item) in self.resource_id.iter().enumerate() {
            params.push((format!("ResourceId.{}", i + 1), item.to_string()));
        }
        for (i, item) in self.tag.iter().enumerate() {
            let prefix = format!("Tag.{}", i + 1);
            for (k, v) in item.to_query_params() {
                params.push((format!("{}.{}", prefix, k), v));
            }
        }
        params.push(("ResourceType".to_string(), self.resource_type.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct TagResourcesResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListTagResources 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListTagResourcesRequest {
    /// 资源所属的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 资源ID列表。
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<Vec<String>>,
    /// 标签列表。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<ListTagResourcesRequestTagItem>>,
    /// 资源类型。取值范围：
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// 查询结果条数上限。
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    /// 查询凭证（Token），取值为上一次API调用返回的NextToken参数值。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl ListTagResourcesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.resource_id {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("ResourceId.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.tag {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tag.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params.push(("ResourceType".to_string(), self.resource_type.to_string()));
        if let Some(ref v) = self.limit {
            params.push(("Limit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.next_token {
            params.push(("NextToken".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListTagResourcesResponse {
    /// 本次调用返回的查询凭证值。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 资源列表。
    #[serde(rename = "TagResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_resources: Option<Vec<ListTagResourcesResponseTagResourcesItem>>,
}

/// UntagResources 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UntagResourcesRequest {
    /// 资源所属的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 资源ID列表。
    #[serde(rename = "ResourceId")]
    pub resource_id: Vec<String>,
    /// 要解绑的标签键列表。最多可输入20个标签键。
    #[serde(rename = "TagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<Vec<String>>,
    /// 资源类型。取值范围：
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// 是否解绑资源上全部的标签。当请求中未设置 `TagKey`时，该参数才有效。取值范围：
    #[serde(rename = "All")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
    /// 保证请求幂等性。从您的客户端生成一个参数值，确保不同请求间该参数值唯一。只支持ASCII字符，且不能超过64个字符。更多信息，请参见[如何保证幂等性](~~25693~~)。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

impl UntagResourcesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        for (i, item) in self.resource_id.iter().enumerate() {
            params.push((format!("ResourceId.{}", i + 1), item.to_string()));
        }
        if let Some(ref v) = self.tag_key {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("TagKey.{}", i + 1), item.to_string()));
            }
        }
        params.push(("ResourceType".to_string(), self.resource_type.to_string()));
        if let Some(ref v) = self.all {
            params.push(("All".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UntagResourcesResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListUsage 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListUsageRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
}

impl ListUsageRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListUsageResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 指定地域权益配额的已使用量和使用上限的信息集合。包含以下几项：
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<serde_json::Value>,
}

/// DescribeContainerGroupPrice 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeContainerGroupPriceRequest {
    /// 地域ID。您可以调用[DescribeRegions](~~146965~~)查询最新的地域可用区信息。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// vCPU大小。ECI支持的vCPU和内存规格请参见[vCPU和内存规格说明](~~114662~~)。
    #[serde(rename = "Cpu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<f32>,
    /// 内存大小。单位为GiB。ECI支持的vCPU和内存规格请参见[vCPU和内存规格说明](~~114662~~)。
    #[serde(rename = "Memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<f32>,
    /// 指定的ECS实例规格。ECI支持指定的ECS规格请参见[ECI支持的ECS规格说明](~~114664~~)。
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// 实例的抢占策略。取值范围：
    #[serde(rename = "SpotStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_strategy: Option<String>,
    /// 可用区。您可以调用[DescribeRegions](~~146965~~)查询最新的地域可用区信息。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 设置抢占式实例的每小时最高价格，最多精确到小数点后3位。当SpotStrategy取值为SpotWithPriceLimit时，必须设置SpotPriceLimit。
    #[serde(rename = "SpotPriceLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_price_limit: Option<f32>,
    /// 临时存储空间大小。单位为GiB.。
    #[serde(rename = "EphemeralStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral_storage: Option<i32>,
    /// 抢占式实例的保护期。单位为小时。默认为1。可设置为0，表示无保护期。
    #[serde(rename = "SpotDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_duration: Option<i32>,
    /// 算力类别。取值为economy时，表示查询经济型规格的价格。
    #[serde(rename = "ComputeCategory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_category: Option<String>,
}

impl DescribeContainerGroupPriceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.cpu {
            params.push(("Cpu".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.memory {
            params.push(("Memory".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_type {
            params.push(("InstanceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.spot_strategy {
            params.push(("SpotStrategy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.spot_price_limit {
            params.push(("SpotPriceLimit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ephemeral_storage {
            params.push(("EphemeralStorage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.spot_duration {
            params.push(("SpotDuration".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.compute_category {
            params.push(("ComputeCategory".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeContainerGroupPriceResponse {
    /// 请求ID，唯一标识。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 价格信息类型（Price），包括价格和优惠规则。
    #[serde(rename = "PriceInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_info: Option<DescribeContainerGroupPriceResponsePriceInfo>,
}

/// DescribeAvailableResource 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeAvailableResourceRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 可用区ID。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 要查询的资源信息。
    #[serde(rename = "DestinationResource")]
    pub destination_resource: DescribeAvailableResourceRequestDestinationResource,
    /// 要查询的抢占式实例的资源信息。
    #[serde(rename = "SpotResource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_resource: Option<DescribeAvailableResourceRequestSpotResource>,
}

impl DescribeAvailableResourceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        for (k, v) in self.destination_resource.to_query_params() {
            params.push((format!("DestinationResource.{}", k), v));
        }
        if let Some(ref v) = self.spot_resource {
            for (k, v2) in v.to_query_params() {
                params.push((format!("SpotResource.{}", k), v2));
            }
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeAvailableResourceResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "AvailableZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_zones: Option<DescribeAvailableResourceResponseAvailableZones>,
}
