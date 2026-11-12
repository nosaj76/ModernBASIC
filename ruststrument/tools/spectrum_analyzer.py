#!/usr/bin/env python3
"""Compatibility spectrum/transcription launcher for Ruststrument."""

from __future__ import annotations

import argparse
from pathlib import Path
import tempfile

from music_analyzer import _basic_from_json
from ruststrument_cli import run


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("audio", type=Path)
    parser.add_argument("max_seconds", nargs="?", type=int, default=120,
                        help="retained for compatibility; Ruststrument analyzes the bounded file")
    parser.add_argument("mode", nargs="?", default="all", choices=["all", "melody", "basic"])
    parser.add_argument("tempo", nargs="?", type=int, default=120)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    if not args.audio.is_file():
        parser.error(f"audio file does not exist: {args.audio}")

    if args.mode == "all":
        run(["analyze", str(args.audio)])
        return 0
    with tempfile.TemporaryDirectory(prefix="ruststrument-spectrum-") as directory:
        output = Path(directory) / "notes.json"
        run(["transcribe", str(args.audio), str(output), "--polyphony", "1"])
        if args.mode == "basic":
            basic = _basic_from_json(output, max(20, min(400, args.tempo)))
            if args.output:
                args.output.write_text(basic, encoding="utf-8")
            else:
                print(basic, end="")
        else:
            print(output.read_text(encoding="utf-8"), end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

