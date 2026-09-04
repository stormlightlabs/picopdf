"""Command-line boundary used by the picopdf Rust process client."""

from __future__ import annotations

import argparse
from collections.abc import Sequence

from picopdf_docling.protocol import PROTOCOL_VERSION


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(prog="picopdf-docling")
    parser.add_argument(
        "--protocol-version",
        action="store_true",
        help="print the process protocol version and exit",
    )
    return parser


def main(argv: Sequence[str] | None = None) -> int:
    args = build_parser().parse_args(argv)
    if args.protocol_version:
        print(PROTOCOL_VERSION)
        return 0

    build_parser().print_help()
    return 0
