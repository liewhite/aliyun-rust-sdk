"""Rust代码生成器"""
from pathlib import Path
from .models import ProductDef, StructDef, FieldDef, ApiDef
from .parser import to_snake_case


class RustGenerator:
    """Rust SDK代码生成器"""

    def __init__(self, product: ProductDef, output_dir: Path):
        self.product = product
        self.output_dir = output_dir
        # 模块名转小写并将连字符替换为下划线（Rust 模块名不支持连字符）
        self.module_name = product.name.lower().replace("-", "_")

    def generate(self):
        """生成所有Rust代码"""
        module_dir = self.output_dir / "src" / self.module_name
        module_dir.mkdir(parents=True, exist_ok=True)

        # 生成types.rs
        types_code = self._generate_types()
        (module_dir / "types.rs").write_text(types_code, encoding="utf-8")

        # 生成api.rs
        api_code = self._generate_api()
        (module_dir / "api.rs").write_text(api_code, encoding="utf-8")

        # 生成mod.rs
        mod_code = self._generate_mod()
        (module_dir / "mod.rs").write_text(mod_code, encoding="utf-8")

        # 更新lib.rs
        self._update_lib_rs()

    def _generate_types(self) -> str:
        """生成类型定义"""
        lines = [
            "//! 类型定义 - 自动生成，请勿手动修改",
            "",
            "use serde::{Deserialize, Serialize};",
            "",
        ]

        # 生成嵌套结构体（可能用于请求或响应，同时支持Serialize和Deserialize）
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

        # 添加文档注释 - 只取第一行
        if struct_def.description:
            desc = struct_def.description.split('\n')[0].strip()
            if len(desc) > 100:
                desc = desc[:97] + "..."
            if desc:
                lines.append(f"/// {desc}")

        # 添加derive
        derives = ["Debug", "Clone"]
        if is_request:
            derives.extend(["Default", "Serialize"])
        elif is_response:
            derives.extend(["Deserialize"])
        elif is_nested:
            # 嵌套结构体可能用于请求或响应，同时支持两者
            derives.extend(["Default", "Serialize", "Deserialize"])
        else:
            derives.extend(["Serialize", "Deserialize"])

        lines.append(f"#[derive({', '.join(derives)})]")

        # 结构体定义
        lines.append(f"pub struct {struct_def.name} {{")

        for field in struct_def.fields:
            # 文档注释 - 只取第一行，清理后如果太长则截断
            if field.description:
                desc = field.description.split('\n')[0].strip()
                if len(desc) > 100:
                    desc = desc[:97] + "..."
                if desc:
                    lines.append(f"    /// {desc}")

            # serde属性
            lines.append(f'    #[serde(rename = "{field.name}")]')
            if not field.required:
                lines.append('    #[serde(skip_serializing_if = "Option::is_none")]')

            # 字段定义
            lines.append(f"    pub {field.rust_name}: {field.rust_type},")

        lines.append("}")

        # 为请求结构体和嵌套结构体生成to_query_params方法
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

            # 获取实际类型（去掉Option包装）
            actual_type = field.rust_type
            if actual_type.startswith("Option<"):
                actual_type = actual_type[7:-1]

            # 跳过无法序列化为查询参数的类型
            if "serde_json::Value" in actual_type:
                lines.append(f"        // 跳过: {original_name} 类型为 {actual_type}")
                continue
            # 跳过嵌套数组 Vec<Vec<...>>
            if actual_type.startswith("Vec<Vec<"):
                lines.append(f"        // 跳过: {original_name} 嵌套数组类型")
                continue

            is_simple = actual_type in simple_types
            is_vec = "Vec<" in actual_type

            if field.required:
                # 必需字段直接转换
                if is_vec:
                    lines.extend(self._generate_vec_param_code(original_name, field_access, field))
                elif is_simple:
                    lines.append(f'        params.push(("{original_name}".to_string(), {field_access}.to_string()));')
                else:
                    # 嵌套对象需要展开
                    lines.append(f"        for (k, v) in {field_access}.to_query_params() {{")
                    lines.append(f'            params.push((format!("{original_name}.{{}}", k), v));')
                    lines.append("        }")
            else:
                # 可选字段需要检查
                lines.append(f"        if let Some(ref v) = {field_access} {{")
                if is_vec:
                    lines.extend(self._generate_vec_param_code(original_name, "v", field, indent=12))
                elif is_simple:
                    lines.append(f'            params.push(("{original_name}".to_string(), v.to_string()));')
                else:
                    # 嵌套对象需要展开
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

        # 跳过 serde_json::Value 类型
        if field.vec_item_type == "serde_json::Value":
            lines.append(f"{sp}// 跳过: {name} 元素类型为 serde_json::Value")
            return lines

        # 检查是否是对象数组
        if field.vec_item_type and field.vec_item_type not in simple_vec_types:
            # 对象数组需要展开成 Name.N.Field 格式
            lines.append(f"{sp}for (i, item) in {var}.iter().enumerate() {{")
            lines.append(f'{sp}    let prefix = format!("{name}.{{}}", i + 1);')
            lines.append(f"{sp}    for (k, v) in item.to_query_params() {{")
            lines.append(f'{sp}        params.push((format!("{{}}.{{}}", prefix, k), v));')
            lines.append(f"{sp}    }}")
            lines.append(f"{sp}}}")
        else:
            # 简单类型数组用 Name.N 格式
            lines.append(f"{sp}for (i, item) in {var}.iter().enumerate() {{")
            lines.append(f'{sp}    params.push((format!("{name}.{{}}", i + 1), item.to_string()));')
            lines.append(f"{sp}}}")

        return lines

    def _generate_api(self) -> str:
        """生成API调用代码"""
        # 客户端名称：首字母大写，连字符转为空（如 R-kvstore -> RKvstoreClient）
        name_parts = self.product.name.replace("-", " ").title().replace(" ", "")
        client_name = f"{name_parts}Client"
        lines = [
            "//! API 调用 - 自动生成，请勿手动修改",
            "",
            "use crate::client::{Client, ClientConfig, ApiRequest, HttpMethod};",
            "use crate::error::SdkError;",
            "use super::types::*;",
            "",
            f"/// {self.product.name} API 版本",
            f'pub const API_VERSION: &str = "{self.product.version}";',
            "",
            f"/// {self.product.name} 客户端",
            "#[derive(Debug, Clone)]",
            f"pub struct {client_name} {{",
            "    client: Client,",
            "}",
            "",
            f"impl {client_name} {{",
            "    /// 创建新客户端",
            f"    pub fn new(config: ClientConfig) -> Result<Self, SdkError> {{",
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

        # 确定HTTP方法
        http_method = "Post" if "post" in api.methods else "Get"

        # 处理多行文档注释
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

    def _generate_mod(self) -> str:
        """生成模块入口文件"""
        name_parts = self.product.name.replace("-", " ").title().replace(" ", "")
        client_name = f"{name_parts}Client"
        lines = [
            f"//! {self.product.name} SDK - 自动生成，请勿手动修改",
            f"//!",
            f"//! API版本: {self.product.version}",
            "",
            "pub mod types;",
            "pub mod api;",
            "",
            "pub use types::*;",
            f"pub use api::{client_name};",
        ]
        return "\n".join(lines)

    def _update_lib_rs(self):
        """更新lib.rs，添加模块声明"""
        lib_rs_path = self.output_dir / "src" / "lib.rs"

        # 读取现有内容或创建新内容
        if lib_rs_path.exists():
            content = lib_rs_path.read_text(encoding="utf-8")
        else:
            content = ""

        # 检查是否已声明该模块
        module_decl = f"pub mod {self.module_name};"

        if module_decl not in content:
            # 在适当位置添加模块声明
            if content.strip():
                content = content.rstrip() + f"\n{module_decl}\n"
            else:
                content = f"{module_decl}\n"

            lib_rs_path.write_text(content, encoding="utf-8")
