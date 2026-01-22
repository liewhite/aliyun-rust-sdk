#!/usr/bin/env python3
"""
阿里云 API 文档下载器

从阿里云 API 元数据服务下载产品列表和 API 文档
"""
import json
import urllib.request
import urllib.error
from pathlib import Path
from dataclasses import dataclass
from typing import Optional
import time
import sys


PRODUCTS_URL = "https://api.aliyun.com/meta/v1/products.json"
API_DOCS_URL_TEMPLATE = "https://api.aliyun.com/meta/v1/products/{code}/versions/{version}/api-docs.json"


@dataclass
class Product:
    """产品信息"""
    code: str
    name: str
    short_name: str
    group: str
    style: str  # RPC 或 ROA
    versions: list[str]
    default_version: str
    category_name: str
    category2_name: str

    @classmethod
    def from_dict(cls, data: dict) -> "Product":
        return cls(
            code=data.get("code", ""),
            name=data.get("name", ""),
            short_name=data.get("shortName", ""),
            group=data.get("group", ""),
            style=data.get("style", "RPC"),
            versions=data.get("versions", []),
            default_version=data.get("defaultVersion", ""),
            category_name=data.get("categoryName", ""),
            category2_name=data.get("category2Name", ""),
        )


def fetch_json(url: str, retries: int = 3, delay: float = 1.0) -> Optional[dict]:
    """获取 JSON 数据，带重试"""
    for attempt in range(retries):
        try:
            req = urllib.request.Request(
                url,
                headers={"User-Agent": "aliyun-sdk-codegen/1.0"}
            )
            with urllib.request.urlopen(req, timeout=30) as response:
                return json.loads(response.read().decode("utf-8"))
        except urllib.error.HTTPError as e:
            if e.code == 404:
                return None
            print(f"  HTTP错误 {e.code}: {url}", file=sys.stderr)
        except urllib.error.URLError as e:
            print(f"  网络错误: {e.reason}", file=sys.stderr)
        except Exception as e:
            print(f"  未知错误: {e}", file=sys.stderr)

        if attempt < retries - 1:
            time.sleep(delay)

    return None


def fetch_products() -> list[Product]:
    """获取所有产品列表"""
    print("获取产品列表...")
    data = fetch_json(PRODUCTS_URL)
    if not data:
        raise RuntimeError("无法获取产品列表")

    products = [Product.from_dict(item) for item in data]
    print(f"找到 {len(products)} 个产品")
    return products


def download_api_docs(
    product: Product,
    output_dir: Path,
    version: Optional[str] = None
) -> Optional[Path]:
    """下载产品的 API 文档"""
    # 优先使用指定版本，否则取最新版本（versions 列表最后一个）
    if not version:
        version = product.versions[-1] if product.versions else product.default_version
    if not version:
        print(f"  跳过 {product.code}: 无可用版本")
        return None

    url = API_DOCS_URL_TEMPLATE.format(code=product.code, version=version)
    data = fetch_json(url)

    if not data:
        print(f"  跳过 {product.code}: 无法获取 API 文档")
        return None

    # 检查是否有 API
    apis = data.get("apis", {})
    if not apis:
        print(f"  跳过 {product.code}: 无 API 定义")
        return None

    output_file = output_dir / f"{product.code.lower()}.json"
    output_file.write_text(json.dumps(data, ensure_ascii=False, indent=2), encoding="utf-8")

    return output_file


def download_all(
    output_dir: Path,
    filter_codes: Optional[list[str]] = None,
    filter_groups: Optional[list[str]] = None,
    style: Optional[str] = None,
    skip_existing: bool = True
) -> list[tuple[Product, Path]]:
    """
    下载所有产品的 API 文档

    Args:
        output_dir: 输出目录
        filter_codes: 只下载指定的产品代码
        filter_groups: 只下载指定分组的产品
        style: 只下载指定风格的产品 (RPC/ROA)
        skip_existing: 跳过已存在的文件

    Returns:
        成功下载的 (产品, 文件路径) 列表
    """
    output_dir.mkdir(parents=True, exist_ok=True)

    products = fetch_products()

    # 过滤
    if filter_codes:
        filter_codes_lower = [c.lower() for c in filter_codes]
        products = [p for p in products if p.code.lower() in filter_codes_lower]

    if filter_groups:
        filter_groups_lower = [g.lower() for g in filter_groups]
        products = [p for p in products if p.group.lower() in filter_groups_lower]

    if style:
        products = [p for p in products if p.style.upper() == style.upper()]

    print(f"准备下载 {len(products)} 个产品的 API 文档")

    results = []
    for i, product in enumerate(products, 1):
        output_file = output_dir / f"{product.code.lower()}.json"

        if skip_existing and output_file.exists():
            print(f"[{i}/{len(products)}] 跳过 {product.code}: 文件已存在")
            results.append((product, output_file))
            continue

        print(f"[{i}/{len(products)}] 下载 {product.code} ({product.name})...")
        path = download_api_docs(product, output_dir)

        if path:
            results.append((product, path))
            print(f"  保存到 {path}")

        # 避免请求过快
        time.sleep(0.1)

    print(f"\n完成，成功下载 {len(results)} 个产品")
    return results


def list_products(
    filter_groups: Optional[list[str]] = None,
    style: Optional[str] = None
):
    """列出所有产品"""
    products = fetch_products()

    if filter_groups:
        filter_groups_lower = [g.lower() for g in filter_groups]
        products = [p for p in products if p.group.lower() in filter_groups_lower]

    if style:
        products = [p for p in products if p.style.upper() == style.upper()]

    # 按分组排序
    products.sort(key=lambda p: (p.group, p.code))

    current_group = None
    for p in products:
        if p.group != current_group:
            current_group = p.group
            print(f"\n=== {current_group} ===")
        print(f"  {p.code:20} {p.name:30} [{p.style}] v{p.default_version}")

    print(f"\n共 {len(products)} 个产品")


if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser(description="阿里云 API 文档下载器")
    subparsers = parser.add_subparsers(dest="command", help="命令")

    # list 命令
    list_parser = subparsers.add_parser("list", help="列出所有产品")
    list_parser.add_argument("--group", "-g", type=str, help="按分组过滤")
    list_parser.add_argument("--style", "-s", type=str, choices=["RPC", "ROA"], help="按 API 风格过滤")

    # download 命令
    dl_parser = subparsers.add_parser("download", help="下载 API 文档")
    dl_parser.add_argument("--output", "-o", type=Path, default=Path("api-docs"), help="输出目录")
    dl_parser.add_argument("--codes", "-c", type=str, nargs="+", help="只下载指定产品")
    dl_parser.add_argument("--group", "-g", type=str, help="按分组过滤")
    dl_parser.add_argument("--style", "-s", type=str, choices=["RPC", "ROA"], help="按 API 风格过滤")
    dl_parser.add_argument("--force", "-f", action="store_true", help="强制重新下载")

    args = parser.parse_args()

    if args.command == "list":
        groups = [args.group] if args.group else None
        list_products(filter_groups=groups, style=args.style)

    elif args.command == "download":
        groups = [args.group] if args.group else None
        download_all(
            output_dir=args.output,
            filter_codes=args.codes,
            filter_groups=groups,
            style=args.style,
            skip_existing=not args.force
        )

    else:
        parser.print_help()
