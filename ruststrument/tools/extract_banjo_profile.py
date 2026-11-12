#!/usr/bin/env python3
"""Create a native RSTI banjo from a directory of labeled recordings.

This compatibility command replaces the former PCM16-only, O(N squared)
Python DFT. Ruststrument now performs decoding, pitch detection, envelopes,
harmonic analysis, region mapping, and physical-model fitting itself.
"""

from __future__ import annotations

import argparse
from pathlib import Path

from ruststrument_cli import run


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("samples", type=Path, help="directory containing labeled note recordings")
    parser.add_argument("output", nargs="?", type=Path, default=Path("banjo.rsti"))
    parser.add_argument("--name", default="Five-string Banjo")
    parser.add_argument("--quality-report", type=Path)
    args = parser.parse_args()
    if not args.samples.is_dir():
        parser.error(f"sample directory does not exist: {args.samples}")

    run(["create", str(args.samples), args.name, str(args.output)])
    run(["inspect", str(args.output)])
    if args.quality_report:
        run(["quality", str(args.output), str(args.quality_report)])
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

