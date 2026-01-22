"""Rust代码生成器 - 每个产品生成独立 crate"""
from pathlib import Path
from .models import ProductDef, StructDef, FieldDef, ApiDef
from .parser import to_snake_case


class RustGenerator:
    """Rust SDK代码生成器 - 生成独立 crate"""

    def __init__(self, product: ProductDef, output_dir: Path):
        self.product = product
        self.output_dir = output_dir
        # crate 名称: aliyun-{product}
        self.crate_name = f"aliyun-{product.name.lower().replace('_', '-')}"
        # 模块名转小写并将连字符替换为下划线
        self.module_name = product.name.lower().replace("-", "_")
        # 客户端名称
        name_parts = product.name.replace("-", " ").title().replace(" ", "")
        self.client_name = f"{name_parts}Client"

    def generate(self):
        """生成独立 crate"""
        crate_dir = self.output_dir / "crates" / self.crate_name
        src_dir = crate_dir / "src"
        src_dir.mkdir(parents=True, exist_ok=True)

        # 生成 Cargo.toml
        cargo_toml = self._generate_cargo_toml()
        (crate_dir / "Cargo.toml").write_text(cargo_toml, encoding="utf-8")

        # 生成 lib.rs
        lib_rs = self._generate_lib_rs()
        (src_dir / "lib.rs").write_text(lib_rs, encoding="utf-8")

        # 生成 types.rs
        types_code = self._generate_types()
        (src_dir / "types.rs").write_text(types_code, encoding="utf-8")

        # 生成 api.rs
        api_code = self._generate_api()
        (src_dir / "api.rs").write_text(api_code, encoding="utf-8")

    def _generate_cargo_toml(self) -> str:
        """生成 crate 的 Cargo.toml"""
        return f'''[package]
name = "{self.crate_name}"
version = "0.1.0"
edition = "2021"
description = "阿里云 {self.product.name} SDK"
license = "MIT OR Apache-2.0"

[dependencies]
aliyun-sdk-core = {{ path = "../aliyun-sdk-core" }}
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0"
'''

    def _generate_lib_rs(self) -> str:
        """生成 lib.rs"""
        return f'''//! 阿里云 {self.product.name} SDK
//!
//! API版本: {self.product.version}
//!
//! ## 使用示例
//!
//! ```rust,ignore
//! use {self.crate_name.replace("-", "_")}::{self.client_name};
//! use aliyun_sdk_core::ClientConfig;
//!
//! let config = ClientConfig {{
//!     access_key_id: "your-key".to_string(),
//!     access_key_secret: "your-secret".to_string(),
//!     endpoint: "{self.module_name}.aliyuncs.com".to_string(),
//!     region_id: Some("cn-hangzhou".to_string()),
//! }};
//!
//! let client = {self.client_name}::new(config)?;
//! ```

mod types;
mod api;

pub use types::*;
pub use api::{self.client_name};
pub use aliyun_sdk_core::{{ClientConfig, SdkError}};
'''

    def _generate_types(self) -> str:
        """生成类型定义"""
        lines = [
            "//! 类型定义 - 自动生成，请勿手动修改",
            "",
            "#![allow(unused_mut)]",
            "",
            "use serde::{Deserialize, Serialize};",
            "",
        ]

        # 生成嵌套结构体
        for struct_def in self.product.structs:
            lines.extend(self._generate_struct(struct_def, is_nested=True))
            lines.append("")

        # 生成请求和响应结构体
        for api in self.product.apis:
            lines.extend(self._generate_struct(api.request_struct, is_request=True))
            lines.append("")
            lines.extend(self._generate_struct(api.response_struct, is_response=True))
            lines.append("")

        return "\n".join(lines)

    def _generate_struct(
        self,
        struct_def: StructDef,
        is_request: bool = False,
        is_response: bool = False,
        is_nested: bool = False
    ) -> list[str]:
        """生成单个结构体"""
        lines = []

        # 文档注释
        if struct_def.description:
            desc = struct_def.description.split('\n')[0].strip()
            if len(desc) > 100:
                desc = desc[:97] + "..."
            if desc:
                lines.append(f"/// {desc}")

        # derive
        derives = ["Debug", "Clone"]
        if is_request:
            derives.extend(["Default", "Serialize"])
        elif is_response:
            derives.extend(["Deserialize"])
        elif is_nested:
            derives.extend(["Default", "Serialize", "Deserialize"])
        else:
            derives.extend(["Serialize", "Deserialize"])

        lines.append(f"#[derive({', '.join(derives)})]")
        lines.append(f"pub struct {struct_def.name} {{")

        for field in struct_def.fields:
            if field.description:
                desc = field.description.split('\n')[0].strip()
                if len(desc) > 100:
                    desc = desc[:97] + "..."
                if desc:
                    lines.append(f"    /// {desc}")

            lines.append(f'    #[serde(rename = "{field.name}")]')
            if not field.required:
                lines.append('    #[serde(skip_serializing_if = "Option::is_none")]')

            lines.append(f"    pub {field.rust_name}: {field.rust_type},")

        lines.append("}")

        if is_request or is_nested:
            lines.extend(self._generate_to_query_params(struct_def))

        return lines

    def _generate_to_query_params(self, struct_def: StructDef) -> list[str]:
        """生成请求参数转换方法"""
        lines = [
            "",
            f"impl {struct_def.name} {{",
            "    /// 转换为查询参数",
            "    pub fn to_query_params(&self) -> Vec<(String, String)> {",
            "        let mut params = Vec::new();",
        ]

        simple_types = {"String", "i32", "i64", "f32", "f64", "bool"}

        for field in struct_def.fields:
            field_access = f"self.{field.rust_name}"
            original_name = field.name

            actual_type = field.rust_type
            if actual_type.startswith("Option<"):
                actual_type = actual_type[7:-1]

            # 跳过无法序列化的类型
            if "serde_json::Value" in actual_type:
                lines.append(f"        // 跳过: {original_name} (serde_json::Value)")
                continue
            if actual_type.startswith("Vec<Vec<"):
                lines.append(f"        // 跳过: {original_name} (嵌套数组)")
                continue

            is_simple = actual_type in simple_types
            is_vec = "Vec<" in actual_type

            if field.required:
                if is_vec:
                    lines.extend(self._generate_vec_param_code(original_name, field_access, field))
                elif is_simple:
                    lines.append(f'        params.push(("{original_name}".to_string(), {field_access}.to_string()));')
                else:
                    lines.append(f"        for (k, v) in {field_access}.to_query_params() {{")
                    lines.append(f'            params.push((format!("{original_name}.{{}}", k), v));')
                    lines.append("        }")
            else:
                lines.append(f"        if let Some(ref v) = {field_access} {{")
                if is_vec:
                    lines.extend(self._generate_vec_param_code(original_name, "v", field, indent=12))
                elif is_simple:
                    lines.append(f'            params.push(("{original_name}".to_string(), v.to_string()));')
                else:
                    lines.append(f"            for (k, v2) in v.to_query_params() {{")
                    lines.append(f'                params.push((format!("{original_name}.{{}}", k), v2));')
                    lines.append("            }")
                lines.append("        }")

        lines.extend([
            "        params",
            "    }",
            "}",
        ])
        return lines

    def _generate_vec_param_code(self, name: str, var: str, field: FieldDef, indent: int = 8) -> list[str]:
        """生成数组参数的序列化代码"""
        sp = " " * indent
        lines = []

        simple_vec_types = ["String", "i32", "i64", "f32", "f64", "bool"]

        if field.vec_item_type == "serde_json::Value":
            lines.append(f"{sp}// 跳过: {name} (serde_json::Value)")
            return lines

        if field.vec_item_type and field.vec_item_type not in simple_vec_types:
            lines.append(f"{sp}for (i, item) in {var}.iter().enumerate() {{")
            lines.append(f'{sp}    let prefix = format!("{name}.{{}}", i + 1);')
            lines.append(f"{sp}    for (k, v) in item.to_query_params() {{")
            lines.append(f'{sp}        params.push((format!("{{}}.{{}}", prefix, k), v));')
            lines.append(f"{sp}    }}")
            lines.append(f"{sp}}}")
        else:
            lines.append(f"{sp}for (i, item) in {var}.iter().enumerate() {{")
            lines.append(f'{sp}    params.push((format!("{name}.{{}}", i + 1), item.to_string()));')
            lines.append(f"{sp}}}")

        return lines

    def _generate_api(self) -> str:
        """生成API调用代码"""
        lines = [
            "//! API 调用 - 自动生成，请勿手动修改",
            "",
            "use aliyun_sdk_core::{Client, ClientConfig, ApiRequest, HttpMethod, SdkError};",
            "use super::types::*;",
            "",
            f"/// {self.product.name} API 版本",
            f'pub const API_VERSION: &str = "{self.product.version}";',
            "",
            f"/// {self.product.name} 客户端",
            "#[derive(Debug, Clone)]",
            f"pub struct {self.client_name} {{",
            "    client: Client,",
            "}",
            "",
            f"impl {self.client_name} {{",
            "    /// 创建新客户端",
            "    pub fn new(config: ClientConfig) -> Result<Self, SdkError> {",
            "        Ok(Self {",
            "            client: Client::new(config)?,",
            "        })",
            "    }",
            "",
        ]

        for api in self.product.apis:
            lines.extend(self._generate_api_method(api))
            lines.append("")

        lines.append("}")

        return "\n".join(lines)

    def _generate_api_method(self, api: ApiDef) -> list[str]:
        """生成单个API方法"""
        method_name = to_snake_case(api.name)
        request_type = api.request_struct.name
        response_type = api.response_struct.name

        http_method = "Post" if "post" in api.methods else "Get"

        summary = api.summary.split('\n')[0].strip() if api.summary else ""
        if len(summary) > 100:
            summary = summary[:97] + "..."

        lines = [
            f"    /// {summary}",
            f"    pub async fn {method_name}(",
            f"        &self,",
            f"        request: {request_type},",
            f"    ) -> Result<{response_type}, SdkError> {{",
            f"        let api_request = ApiRequest {{",
            f'            action: "{api.action}",',
            f"            version: API_VERSION,",
            f"            method: HttpMethod::{http_method},",
            f"            query: request.to_query_params(),",
            f"            body: None,",
            f"            need_sign: {str(api.need_sign).lower()},",
            f"        }};",
            f"        self.client.request(api_request).await",
            f"    }}",
        ]

        return lines
