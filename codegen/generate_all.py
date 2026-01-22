#!/usr/bin/env python3
"""
阿里云 SDK 批量生成器

下载 API 文档并生成独立的 Rust crates
"""
import argparse
import sys
from pathlib import Path

from .download import download_all
from .parser import ApiDocParser
from .rust_generator import RustGenerator


def generate_from_file(json_path: Path, output_dir: Path) -> tuple[bool, str]:
    """从单个 JSON 文件生成 SDK crate"""
    try:
        parser = ApiDocParser(json_path)
        product = parser.parse()

        if not product.apis:
            return False, ""

        generator = RustGenerator(product, output_dir)
        generator.generate()
        return True, generator.crate_name

    except Exception as e:
        print(f"  生成失败: {e}", file=sys.stderr)
        return False, ""


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
    批量下载并生成 SDK crates

    Args:
        output_dir: 输出目录 (crates 将生成在 output_dir/crates/)
        api_docs_dir: API 文档存放目录
        filter_codes: 只处理指定产品
        filter_groups: 只处理指定分组
        style: 只处理指定风格 (RPC/ROA)
        skip_download: 跳过下载，只从已有文件生成
        force_download: 强制重新下载
    """
    api_docs_dir.mkdir(parents=True, exist_ok=True)

    if skip_download:
        json_files = list(api_docs_dir.glob("*.json"))
        if filter_codes:
            filter_codes_lower = [c.lower() for c in filter_codes]
            json_files = [f for f in json_files if f.stem.lower() in filter_codes_lower]
        print(f"找到 {len(json_files)} 个 JSON 文件")
    else:
        results = download_all(
            output_dir=api_docs_dir,
            filter_codes=filter_codes,
            filter_groups=filter_groups,
            style=style,
            skip_existing=not force_download,
        )
        json_files = [path for _, path in results]

    print(f"\n开始生成 SDK crates...")
    success_count = 0
    failed = []
    generated_crates = []

    for i, json_path in enumerate(json_files, 1):
        product_name = json_path.stem
        print(f"[{i}/{len(json_files)}] 生成 {product_name}...")

        success, crate_name = generate_from_file(json_path, output_dir)
        if success:
            success_count += 1
            generated_crates.append(crate_name)
            print(f"  完成: crates/{crate_name}/")
        else:
            failed.append(product_name)

    print(f"\n生成完成:")
    print(f"  成功: {success_count}")
    print(f"  失败: {len(failed)}")

    if failed:
        print(f"  失败列表: {', '.join(failed)}")

    if generated_crates:
        print(f"\n生成的 crates:")
        for crate in sorted(generated_crates):
            print(f"  - {crate}")


def main():
    parser = argparse.ArgumentParser(description="阿里云 SDK 批量生成器")

    parser.add_argument(
        "--output", "-o",
        type=Path,
        default=Path("."),
        help="输出目录 (默认当前目录)"
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
