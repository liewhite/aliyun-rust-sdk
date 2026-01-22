"""数据模型定义"""
from dataclasses import dataclass, field
from typing import Optional
from enum import Enum


class ParamLocation(Enum):
    QUERY = "query"
    BODY = "body"
    PATH = "path"
    HEADER = "header"


class RustType(Enum):
    STRING = "String"
    I32 = "i32"
    I64 = "i64"
    F32 = "f32"
    F64 = "f64"
    BOOL = "bool"
    VEC = "Vec"
    OBJECT = "struct"


@dataclass
class FieldDef:
    """字段定义"""
    name: str
    rust_name: str
    rust_type: str
    required: bool
    description: str
    is_vec: bool = False
    vec_item_type: Optional[str] = None


@dataclass
class StructDef:
    """结构体定义"""
    name: str
    fields: list[FieldDef]
    description: str = ""


@dataclass
class ApiDef:
    """API定义"""
    name: str
    action: str
    summary: str
    methods: list[str]
    need_sign: bool
    request_struct: StructDef
    response_struct: StructDef
    query_params: list[FieldDef] = field(default_factory=list)
    body_params: list[FieldDef] = field(default_factory=list)


@dataclass
class ProductDef:
    """产品定义"""
    name: str
    version: str
    style: str
    apis: list[ApiDef]
    structs: list[StructDef] = field(default_factory=list)
