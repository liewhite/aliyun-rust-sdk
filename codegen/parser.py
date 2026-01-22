"""API文档JSON解析器"""
import json
import re
from pathlib import Path
from .models import (
    ApiDef, ProductDef, StructDef, FieldDef, ParamLocation
)


RUST_KEYWORDS = {
    "as", "break", "const", "continue", "crate", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod",
    "move", "mut", "pub", "ref", "return", "self", "Self", "static", "struct",
    "super", "trait", "true", "type", "unsafe", "use", "where", "while",
    "async", "await", "dyn", "abstract", "become", "box", "do", "final",
    "macro", "override", "priv", "typeof", "unsized", "virtual", "yield", "try"
}


def to_snake_case(name: str) -> str:
    """将驼峰命名转为蛇形命名"""
    # 先把点号和连字符替换为下划线
    name = name.replace(".", "_").replace("-", "_")
    s1 = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', name)
    result = re.sub('([a-z0-9])([A-Z])', r'\1_\2', s1).lower()
    # 清理连续下划线
    while '__' in result:
        result = result.replace('__', '_')
    # 处理Rust关键字
    if result in RUST_KEYWORDS:
        return f"r#{result}"
    return result


def to_pascal_case(name: str) -> str:
    """将蛇形命名或混合命名转为大驼峰"""
    # 先将点号替换为下划线
    name = name.replace(".", "_")
    # 按下划线分割，每个部分首字母大写
    parts = name.split('_')
    result = []
    for part in parts:
        if part:
            result.append(part[0].upper() + part[1:])
    return ''.join(result)


def sanitize_struct_name(name: str) -> str:
    """清理结构体名称，确保是有效的Rust标识符"""
    # 替换点号
    name = name.replace(".", "")
    # 如果已经是PascalCase格式，直接返回
    # 否则转换
    if '_' in name:
        return to_pascal_case(name)
    return name


def map_json_type_to_rust(schema: dict, name: str = "") -> tuple[str, bool, str | None]:
    """
    将JSON schema类型映射到Rust类型
    返回: (rust_type, is_vec, vec_item_type)
    """
    json_type = schema.get("type", "string")
    format_type = schema.get("format", "")

    # 清理名字中的点号
    name = sanitize_struct_name(name) if name else ""

    if json_type == "string":
        return ("String", False, None)
    elif json_type == "integer":
        if format_type == "int64":
            return ("i64", False, None)
        return ("i32", False, None)
    elif json_type == "number":
        if format_type == "float":
            return ("f32", False, None)
        return ("f64", False, None)
    elif json_type == "boolean":
        return ("bool", False, None)
    elif json_type == "array":
        items = schema.get("items", {})
        item_type = items.get("type", "string")
        if item_type == "object":
            # 检查对象是否有properties
            if items.get("properties"):
                struct_name = f"{name}Item"
                return (f"Vec<{struct_name}>", True, struct_name)
            else:
                # 无properties的object用serde_json::Value
                return ("Vec<serde_json::Value>", True, "serde_json::Value")
        elif item_type == "array":
            # 嵌套数组：递归处理内部数组
            inner_items = items.get("items", {})
            inner_item_type = inner_items.get("type", "string")
            if inner_item_type == "object" and inner_items.get("properties"):
                struct_name = f"{name}Item"
                return (f"Vec<Vec<{struct_name}>>", True, struct_name)
            elif inner_item_type == "object":
                return ("Vec<Vec<serde_json::Value>>", True, "serde_json::Value")
            else:
                inner_type, _, _ = map_json_type_to_rust(inner_items)
                return (f"Vec<Vec<{inner_type}>>", True, inner_type)
        else:
            inner_type, _, _ = map_json_type_to_rust(items)
            return (f"Vec<{inner_type}>", True, inner_type)
    elif json_type == "object":
        # 检查是否有properties定义
        if schema.get("properties"):
            return (name if name else "serde_json::Value", False, None)
        else:
            # 无properties的object用serde_json::Value
            return ("serde_json::Value", False, None)
    else:
        return ("String", False, None)


class ApiDocParser:
    """API文档解析器"""

    def __init__(self, doc_path: Path):
        self.doc_path = doc_path
        self.nested_structs: dict[str, StructDef] = {}

    def parse(self) -> ProductDef:
        """解析API文档"""
        with open(self.doc_path, 'r', encoding='utf-8') as f:
            doc = json.load(f)

        info = doc.get("info", {})
        product_name = info.get("product", "unknown")
        version = info.get("version", "")
        style = info.get("style", "RPC")

        apis = []
        for api_name, api_spec in doc.get("apis", {}).items():
            api_def = self._parse_api(api_name, api_spec)
            if api_def:
                apis.append(api_def)

        return ProductDef(
            name=product_name,
            version=version,
            style=style,
            apis=apis,
            structs=list(self.nested_structs.values())
        )

    def _parse_api(self, name: str, spec: dict) -> ApiDef | None:
        """解析单个API定义"""
        summary = spec.get("summary", "")
        methods = spec.get("methods", ["post"])
        security = spec.get("security", [])
        need_sign = any("AK" in s for s in security)

        # 解析请求参数
        params = spec.get("parameters", [])
        query_params = []
        body_params = []
        seen_params = set()

        for param in params:
            param_name = param.get("name", "")
            if param_name in seen_params:
                continue
            seen_params.add(param_name)

            location = param.get("in", "query")
            schema = param.get("schema", {})
            field_def = self._parse_field(param_name, schema, f"{name}Request")

            if location == "query":
                query_params.append(field_def)
            elif location == "body":
                body_params.append(field_def)

        # 构建请求结构体
        request_struct = StructDef(
            name=f"{name}Request",
            fields=query_params + body_params,
            description=f"{name} 请求参数"
        )

        # 解析响应
        responses = spec.get("responses", {})
        response_schema = responses.get("200", {}).get("schema", {})
        response_struct = self._parse_response_struct(name, response_schema)

        return ApiDef(
            name=name,
            action=name,
            summary=summary,
            methods=methods,
            need_sign=need_sign,
            request_struct=request_struct,
            response_struct=response_struct,
            query_params=query_params,
            body_params=body_params
        )

    def _parse_field(self, name: str, schema: dict, parent_struct: str) -> FieldDef:
        """解析字段定义"""
        required = schema.get("required", False)
        description = schema.get("description", "")

        # 处理嵌套属性名（如 "ReadinessProbe.TimeoutSeconds"）
        rust_name = to_snake_case(name.replace(".", "_"))

        rust_type, is_vec, vec_item_type = map_json_type_to_rust(schema, f"{parent_struct}{to_pascal_case(name)}")

        json_type = schema.get("type", "string")

        # 如果是对象数组，解析内部结构
        if is_vec and json_type == "array":
            items = schema.get("items", {})
            if items.get("type") == "object" and items.get("properties"):
                self._parse_nested_struct(vec_item_type, items)
            elif items.get("type") == "array":
                # 嵌套数组：解析内部数组的元素
                inner_items = items.get("items", {})
                if inner_items.get("type") == "object" and inner_items.get("properties"):
                    self._parse_nested_struct(vec_item_type, inner_items)
        # 如果是单个对象，也要解析
        elif json_type == "object" and schema.get("properties"):
            struct_name = sanitize_struct_name(f"{parent_struct}{to_pascal_case(name)}")
            self._parse_nested_struct(struct_name, schema)
            rust_type = struct_name

        # 非必需字段用Option包装
        if not required:
            final_type = f"Option<{rust_type}>"
        else:
            final_type = rust_type

        return FieldDef(
            name=name,
            rust_name=rust_name,
            rust_type=final_type,
            required=required,
            description=description,
            is_vec=is_vec,
            vec_item_type=vec_item_type
        )

    def _parse_nested_struct(self, struct_name: str, schema: dict):
        """解析嵌套结构体

        注意：struct_name 应该已经是清理过的有效Rust标识符
        """
        # 跳过无效的结构体名
        if struct_name in self.nested_structs:
            return
        if struct_name == "serde_json::Value" or "::" in struct_name:
            return

        properties = schema.get("properties", {})
        fields = []
        seen_fields = set()

        for prop_name, prop_schema in properties.items():
            # 转换为 rust_name 进行去重
            rust_name = to_snake_case(prop_name.replace(".", "_"))
            if rust_name in seen_fields:
                continue
            seen_fields.add(rust_name)

            field_def = self._parse_field(prop_name, prop_schema, struct_name)
            fields.append(field_def)

        self.nested_structs[struct_name] = StructDef(
            name=struct_name,
            fields=fields,
            description=schema.get("description", "")
        )

    def _parse_response_struct(self, api_name: str, schema: dict) -> StructDef:
        """解析响应结构体"""
        struct_name = f"{api_name}Response"
        properties = schema.get("properties", {})
        fields = []
        seen_fields = set()

        for prop_name, prop_schema in properties.items():
            rust_name = to_snake_case(prop_name.replace(".", "_"))
            if rust_name in seen_fields:
                continue
            seen_fields.add(rust_name)

            field_def = self._parse_field(prop_name, prop_schema, struct_name)
            fields.append(field_def)

        return StructDef(
            name=struct_name,
            fields=fields,
            description=schema.get("description", "")
        )
