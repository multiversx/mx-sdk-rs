#!/usr/bin/env python3

from typing import Dict
from pathlib import Path
import io

Sizes = Dict[str, int]

RED_CIRCLE = "&#x1F534;"
GREEN_CIRCLE = "&#x1F7E2;"
YELLOW_CIRCLE = "&#x1F7E1;"
CHECK = "&#x2713;"


def load_sizes(path: Path) -> Sizes:
    sizes: Dict[str, int] = {}
    with open(path, 'r') as file:
        lines = file.readlines()
        for line in lines:
            name, size = line.split(" ")
            sizes[name] = int(size)
    return sizes


def format_size(size: int) -> str:
    if size == 0:
        return "-"
    return f"{size}"


def format_diff_absolute(before: int, after: int) -> str:
    if before == 0 or after == 0:
        return "-"
    diff = after - before
    return f"{diff:+}"


def format_diff_relative(before: int, after: int) -> str:
    if before == 0:
        return f"added {YELLOW_CIRCLE}"
    if after == 0:
        return f"removed {YELLOW_CIRCLE}"
    if before < after:
        symbol = RED_CIRCLE
    elif before > after:
        symbol = GREEN_CIRCLE
    else:
        symbol = CHECK
    diff = (after - before) / before * 100
    return f"{diff:+.2f}% {symbol}"


def generate_size_report(before: Sizes, after: Sizes) -> str:
    """
    >>> before = { \
            "foo": 42, \
            "bar": 50, \
            "same": 50, \
            "biz": 15, \
        }
    >>> after = { \
            "foo": 45, \
            "bar": 30, \
            "same": 50, \
            "baz": 5, \
        }
    >>> print(size_diff(before, after))
    Contract file size comparison (bytes):
    Contract | Previous | Current | Difference | Percentage
    --:|--:|--:|--:|--:|
    bar | 50 | 30 | -20 | -40.00% &#x1F7E2;
    baz | - | 5 | - | added &#x1F7E1;
    biz | 15 | - | - | removed &#x1F7E1;
    foo | 42 | 45 | +3 | +7.14% &#x1F534;
    same | 50 | 50 | +0 | +0.00% &#x2713;
    """
    with io.StringIO() as output:
        names = sorted(set(before).union(after))
        print("Contract file size comparison (bytes):")
        print("Contract | Previous | Current | Difference | Percentage", file=output)
        print("--:|--:|--:|--:|--:|", file=output, end='')
        for name in names:
            before_size = before.get(name, 0)
            after_size = after.get(name, 0)
            diff_absolute = format_diff_absolute(before_size, after_size)
            diff_relative = format_diff_relative(before_size, after_size)
            print(f"\n{name} | {format_size(before_size)} | {format_size(after_size)} | {diff_absolute} | {diff_relative}", file=output, end='')
        return output.getvalue()


def generate_size_report_from_paths(before: Path, after: Path) -> str:
    return generate_size_report(load_sizes(before), load_sizes(after))


if __name__ == "__main__":
    import argparse
    parser = argparse.ArgumentParser(description='Compares two file size tables.')
    parser.add_argument('before', help='foo help', type=Path)
    parser.add_argument('after', help='foo help', type=Path)
    args = parser.parse_args()
    size_diff_table = generate_size_report_from_paths(args.before, args.after)
    print(size_diff_table)
