#!/usr/bin/env python3
"""
阿里云 SDK 批量生成器

下载 API 文档并生成 Rust SDK
"""
import argparse
import sys
from pathlib import Path

from .download import download_all, fetch_products, download_api_docs
from .parser import ApiDocParser
from .rust_generator import RustGenerator


def generate_from_file(json_path: Path, output_dir: Path) -> bool:
    """从单个 JSON 文件生成 SDK"""
    try:
        parser = ApiDocParser(json_path)
        product = parser.parse()

        if not product.apis:
            return False

        generator = RustGenerator(product, output_dir)
        generator.generate()
        return True

    except Exception as e:
        print(f"  生成失败: {e}", file=sys.stderr)
        return False


def generate_all(
    output_dir: Path,
    api_docs_dir: Path,
    filter_codes: list[str] | None = None,
    filter_groups: list[str] | None = None,
    style: str | None = None,
    skip_download: bool = False,
    force_download: bool = False,
):
    """
    批量下载并生成 SDK

    Args:
        output_dir: SDK 输出目录 (包含 src/)
        api_docs_dir: API 文档存放目录
        filter_codes: 只处理指定产品
        filter_groups: 只处理指定分组
        style: 只处理指定风格 (RPC/ROA)
        skip_download: 跳过下载，只从已有文件生成
        force_download: 强制重新下载
    """
    api_docs_dir.mkdir(parents=True, exist_ok=True)

    if skip_download:
        # 从已有文件生成
        json_files = list(api_docs_dir.glob("*.json"))
        if filter_codes:
            filter_codes_lower = [c.lower() for c in filter_codes]
            json_files = [f for f in json_files if f.stem.lower() in filter_codes_lower]
        print(f"找到 {len(json_files)} 个 JSON 文件")
    else:
        # 下载 API 文档
        results = download_all(
            output_dir=api_docs_dir,
            filter_codes=filter_codes,
            filter_groups=filter_groups,
            style=style,
            skip_existing=not force_download,
        )
        json_files = [path for _, path in results]

    # 生成 SDK
    print(f"\n开始生成 SDK...")
    success_count = 0
    failed = []

    for i, json_path in enumerate(json_files, 1):
        product_name = json_path.stem
        print(f"[{i}/{len(json_files)}] 生成 {product_name}...")

        if generate_from_file(json_path, output_dir):
            success_count += 1
            print(f"  完成: src/{product_name}/")
        else:
            failed.append(product_name)

    # 更新 lib.rs
    update_lib_rs(output_dir)

    print(f"\n生成完成:")
    print(f"  成功: {success_count}")
    print(f"  失败: {len(failed)}")

    if failed:
        print(f"  失败列表: {', '.join(failed)}")


def update_lib_rs(output_dir: Path):
    """更新 lib.rs，添加所有模块声明"""
    src_dir = output_dir / "src"
    lib_rs_path = src_dir / "lib.rs"

    # 找出所有生成的模块目录
    modules = []
    for item in src_dir.iterdir():
        if item.is_dir() and (item / "mod.rs").exists():
            modules.append(item.name)

    modules.sort()

    # 读取现有 lib.rs
    if lib_rs_path.exists():
        content = lib_rs_path.read_text(encoding="utf-8")
        lines = content.split("\n")
    else:
        lines = [
            "//! 阿里云SDK",
            "//!",
            "//! 统一的阿里云API客户端库",
            "",
            "pub mod client;",
            "pub mod error;",
            "",
            "pub use client::{Client, ClientConfig, ApiRequest, HttpMethod};",
            "pub use error::SdkError;",
        ]

    # 找出已有的模块声明
    existing_modules = set()
    for line in lines:
        if line.startswith("pub mod ") and line.endswith(";"):
            mod_name = line[8:-1].strip()
            if mod_name not in ("client", "error"):
                existing_modules.add(mod_name)

    # 添加新模块
    new_modules = set(modules) - existing_modules

    if new_modules:
        # 在文件末尾添加新模块声明
        if not content.endswith("\n"):
            lines.append("")

        for mod in sorted(new_modules):
            lines.append(f"pub mod {mod};")

        lib_rs_path.write_text("\n".join(lines), encoding="utf-8")
        print(f"\n更新 lib.rs，添加 {len(new_modules)} 个模块")


def main():
    parser = argparse.ArgumentParser(description="阿里云 SDK 批量生成器")

    parser.add_argument(
        "--output", "-o",
        type=Path,
        default=Path("."),
        help="SDK 输出目录 (默认当前目录)"
    )
    parser.add_argument(
        "--api-docs", "-d",
        type=Path,
        default=Path("api-docs"),
        help="API 文档存放目录 (默认 api-docs/)"
    )
    parser.add_argument(
        "--codes", "-c",
        type=str,
        nargs="+",
        help="只处理指定产品代码"
    )
    parser.add_argument(
        "--group", "-g",
        type=str,
        help="只处理指定分组"
    )
    parser.add_argument(
        "--style", "-s",
        type=str,
        choices=["RPC", "ROA"],
        help="只处理指定 API 风格"
    )
    parser.add_argument(
        "--skip-download",
        action="store_true",
        help="跳过下载，只从已有 JSON 生成"
    )
    parser.add_argument(
        "--force-download", "-f",
        action="store_true",
        help="强制重新下载 API 文档"
    )

    args = parser.parse_args()

    groups = [args.group] if args.group else None

    generate_all(
        output_dir=args.output,
        api_docs_dir=args.api_docs,
        filter_codes=args.codes,
        filter_groups=groups,
        style=args.style,
        skip_download=args.skip_download,
        force_download=args.force_download,
    )


if __name__ == "__main__":
    main()
