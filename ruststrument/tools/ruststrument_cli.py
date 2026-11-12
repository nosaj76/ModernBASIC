#!/usr/bin/env python3
"""Shared launcher for the legacy Python compatibility commands."""

from __future__ import annotations

import os
from pathlib import Path
import shutil
import subprocess
import sys


def find_binary() -> str:
    override = os.environ.get("RUSTSTRUMENT_BIN")
    candidates = []
    if override:
        candidates.append(Path(override).expanduser())
    root = Path(__file__).resolve().parents[1]
    candidates.extend(
        [root / "target" / "release" / "ruststrument", root / "target" / "debug" / "ruststrument"]
    )
    for candidate in candidates:
        if candidate.is_file() and os.access(candidate, os.X_OK):
            return str(candidate)
    installed = shutil.which("ruststrument")
    if installed:
        return installed
    raise SystemExit(
        "ruststrument executable not found; run 'cargo build --release' or set "
        "RUSTSTRUMENT_BIN"
    )


def run(arguments: list[str], *, capture: bool = False) -> subprocess.CompletedProcess[str]:
    command = [find_binary(), *arguments]
    return subprocess.run(command, check=True, text=True, capture_output=capture)


def main() -> int:
    if len(sys.argv) < 2:
        raise SystemExit("usage: ruststrument_cli.py <ruststrument arguments...>")
    run(sys.argv[1:])
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

