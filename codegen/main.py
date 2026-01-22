#!/usr/bin/env python3
"""
阿里云SDK代码生成器入口

用法:
    python -m codegen.main api-docs.json
    python -m codegen.main api-docs.json --output ./
"""
import argparse
import sys
from pathlib import Path

from .parser import ApiDocParser
from .rust_generator import RustGenerator


def main():
    parser = argparse.ArgumentParser(
        description="从阿里云API文档生成Rust SDK代码"
    )
    parser.add_argument(
        "api_docs",
        type=Path,
        help="API文档JSON文件路径"
    )
    parser.add_argument(
        "--output", "-o",
        type=Path,
        default=Path("."),
        help="输出目录，默认为当前目录"
    )

    args = parser.parse_args()

    if not args.api_docs.exists():
        print(f"错误: 文件不存在 {args.api_docs}", file=sys.stderr)
        sys.exit(1)

    print(f"解析API文档: {args.api_docs}")
    doc_parser = ApiDocParser(args.api_docs)
    product = doc_parser.parse()

    print(f"产品: {product.name}")
    print(f"版本: {product.version}")
    print(f"API数量: {len(product.apis)}")
    print(f"嵌套结构体数量: {len(product.structs)}")

    print(f"\n生成Rust代码到: {args.output}")
    generator = RustGenerator(product, args.output)
    generator.generate()

    output_module = args.output / "src" / product.name.lower()
    print(f"\n生成完成!")
    print(f"  - {output_module / 'types.rs'}")
    print(f"  - {output_module / 'api.rs'}")
    print(f"  - {output_module / 'mod.rs'}")


if __name__ == "__main__":
    main()
