# Aliyun SDK for Rust

阿里云 API Rust SDK，从官方 API 元数据自动生成。每个产品为独立的 crate，按需引入。

## 项目结构

```
aliyun-sdk/
├── crates/
│   ├── aliyun-sdk-core/    # 核心库（Client, Error）
│   ├── aliyun-ecs/         # 云服务器 ECS
│   ├── aliyun-rds/         # 云数据库 RDS
│   └── ...                 # 其他产品
├── api-docs/               # API 文档 JSON
└── codegen/                # 代码生成器
```

## 已支持的产品

| Crate | 产品名称 | 说明 |
|-------|---------|------|
| `aliyun-ecs` | 云服务器 ECS | 弹性计算 |
| `aliyun-rds` | 云数据库 RDS | 关系型数据库 |
| `aliyun-oss` | 对象存储 OSS | 基础存储服务 |
| `aliyun-vpc` | 专有网络 VPC | 网络 |
| `aliyun-slb` | 负载均衡 SLB | 网络 |
| `aliyun-cdn` | 内容分发 CDN | 加速 |
| `aliyun-nas` | 文件存储 NAS | 存储 |
| `aliyun-r-kvstore` | 云数据库 Redis | 缓存 |
| `aliyun-dds` | 云数据库 MongoDB | NoSQL |
| `aliyun-rocketmq` | 云消息队列 RocketMQ | 消息队列 |
| `aliyun-kms` | 密钥管理服务 KMS | 安全 |
| `aliyun-ram` | 访问控制 RAM | 身份管理 |
| `aliyun-sts` | 安全令牌 STS | 临时凭证 |
| `aliyun-cas` | 数字证书管理服务 | SSL 证书 |
| `aliyun-ddoscoo` | DDoS 高防 | 安全 |
| `aliyun-cms` | 云监控 | 监控 |
| `aliyun-arms` | 应用实时监控服务 | APM |
| `aliyun-actiontrail` | 操作审计 | 审计 |
| `aliyun-alidns` | 云解析 DNS | 域名 |
| `aliyun-dysmsapi` | 短信服务 | 通信 |
| `aliyun-cs` | 容器服务 ACK | Kubernetes |

## 使用方法

### 添加依赖

```toml
[dependencies]
aliyun-ecs = { path = "crates/aliyun-ecs" }
# 或使用 git
# aliyun-ecs = { git = "https://github.com/xxx/aliyun-sdk" }
```

### 示例代码

```rust
use aliyun_ecs::{EcsClient, ClientConfig, DescribeInstancesRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfig {
        access_key_id: "your-access-key-id".to_string(),
        access_key_secret: "your-access-key-secret".to_string(),
        endpoint: "ecs.aliyuncs.com".to_string(),
        region_id: Some("cn-hangzhou".to_string()),
    };

    let client = EcsClient::new(config)?;

    let mut request = DescribeInstancesRequest::default();
    request.region_id = Some("cn-hangzhou".to_string());

    let response = client.describe_instances(request).await?;
    println!("{:?}", response);

    Ok(())
}
```

## 代码生成器 (codegen)

SDK 代码从阿里云官方 API 元数据自动生成。

### 列出可用产品

```bash
python -m codegen.download list
python -m codegen.download list --group "弹性计算"
python -m codegen.download list --style RPC
```

### 下载 API 文档

```bash
python -m codegen.download download --codes Ecs Rds -o api-docs
python -m codegen.download download --group "数据库" -o api-docs
```

### 生成 SDK Crates

```bash
# 下载并生成指定产品
python -m codegen.generate_all --codes Ecs Rds -o . -d api-docs

# 从已下载的文件生成（跳过下载）
python -m codegen.generate_all --skip-download -o . -d api-docs

# 生成所有 RPC 风格产品
python -m codegen.generate_all --style RPC -o . -d api-docs
```

### 编译

```bash
# 编译所有 crates
cargo build --workspace

# 编译指定 crate
cargo build -p aliyun-ecs

# Release 编译
cargo build --workspace --release
```

## 注意事项

1. **独立 Crates**: 每个产品为独立 crate，只编译需要的产品
2. **多版本产品**: 自动使用最新版本（versions 列表最后一个）
3. **未定义字段**: 文档中未完整定义的 object 类型使用 `serde_json::Value`
4. **重新生成**: 会覆盖已有文件
