# Aliyun SDK for Rust

阿里云 API Rust SDK，从官方 API 元数据自动生成。

## 已支持的产品

| 模块 | 产品名称 | 说明 |
|------|---------|------|
| `ecs` | 云服务器 ECS | 弹性计算 |
| `rds` | 云数据库 RDS | 关系型数据库 |
| `oss` | 对象存储 OSS | 基础存储服务 |
| `vpc` | 专有网络 VPC | 网络 |
| `slb` | 负载均衡 SLB | 网络 |
| `cdn` | 内容分发 CDN | 加速 |
| `nas` | 文件存储 NAS | 存储 |
| `r_kvstore` | 云数据库 Redis | 缓存 |
| `dds` | 云数据库 MongoDB | NoSQL |
| `rocketmq` | 云消息队列 RocketMQ | 消息队列 |
| `kms` | 密钥管理服务 KMS | 安全 |
| `ram` | 访问控制 RAM | 身份管理 |
| `sts` | 安全令牌 STS | 临时凭证 |
| `cas` | 数字证书管理服务 | SSL 证书 |
| `ddoscoo` | DDoS 高防 | 安全 |
| `cms` | 云监控 | 监控 |
| `arms` | 应用实时监控服务 | APM |
| `actiontrail` | 操作审计 | 审计 |
| `alidns` | 云解析 DNS | 域名 |
| `dysmsapi` | 短信服务 | 通信 |
| `cs` | 容器服务 ACK | Kubernetes |
| `cr` | 容器镜像服务 ACR | 镜像仓库 |
| `eci` | 弹性容器实例 ECI | Serverless 容器 |

## 使用方法

```rust
use aliyun_sdk::{ClientConfig, ecs::EcsClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfig {
        access_key_id: "your-access-key-id".to_string(),
        access_key_secret: "your-access-key-secret".to_string(),
        endpoint: "ecs.aliyuncs.com".to_string(),
        region_id: Some("cn-hangzhou".to_string()),
    };

    let client = EcsClient::new(config)?;

    // 调用 API
    let request = aliyun_sdk::ecs::DescribeInstancesRequest::default();
    let response = client.describe_instances(request).await?;

    println!("{:?}", response);
    Ok(())
}
```

## 代码生成器 (codegen)

SDK 代码从阿里云官方 API 元数据自动生成。

### 列出可用产品

```bash
# 列出所有产品
python -m codegen.download list

# 按分组过滤
python -m codegen.download list --group "弹性计算"

# 按 API 风格过滤 (RPC/ROA)
python -m codegen.download list --style RPC
```

### 下载 API 文档

```bash
# 下载指定产品
python -m codegen.download download --codes Ecs Rds -o api-docs

# 下载指定分组
python -m codegen.download download --group "数据库" -o api-docs

# 强制重新下载
python -m codegen.download download --codes Ecs -o api-docs --force
```

### 生成 SDK

```bash
# 下载并生成指定产品
python -m codegen.generate_all --codes Ecs Rds -o . -d api-docs

# 从已下载的文件生成（跳过下载）
python -m codegen.generate_all --skip-download -o . -d api-docs

# 生成所有 RPC 风格产品
python -m codegen.generate_all --style RPC -o . -d api-docs

# 强制重新下载并生成
python -m codegen.generate_all --codes Ecs -o . -d api-docs --force-download
```

### 生成单个产品

```bash
# 使用已有的 JSON 文件生成
python -m codegen.main api-docs/ecs.json --output .
```

### 命令行参数

**download.py**

| 参数 | 说明 |
|------|------|
| `list` | 列出所有产品 |
| `download` | 下载 API 文档 |
| `--codes`, `-c` | 指定产品代码 |
| `--group`, `-g` | 按分组过滤 |
| `--style`, `-s` | 按 API 风格过滤 (RPC/ROA) |
| `--output`, `-o` | 输出目录 |
| `--force`, `-f` | 强制重新下载 |

**generate_all.py**

| 参数 | 说明 |
|------|------|
| `--codes`, `-c` | 指定产品代码 |
| `--group`, `-g` | 按分组过滤 |
| `--style`, `-s` | 按 API 风格过滤 |
| `--output`, `-o` | SDK 输出目录 |
| `--api-docs`, `-d` | API 文档目录 |
| `--skip-download` | 跳过下载，从已有文件生成 |
| `--force-download`, `-f` | 强制重新下载 |

## 注意事项

1. **多版本产品**: 自动使用最新版本（versions 列表最后一个）
2. **未定义字段**: 文档中未完整定义的 object 类型使用 `serde_json::Value`
3. **重新生成**: 会覆盖已有文件，请注意备份手动修改的代码
