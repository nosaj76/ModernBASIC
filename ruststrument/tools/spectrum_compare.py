#!/usr/bin/env python3
"""Compare pitch, dynamics, stereo, harmonic, rhythm, and spectral balance."""

from __future__ import annotations

import argparse

from ruststrument_cli import run


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("audio_a")
    parser.add_argument("audio_b")
    args = parser.parse_args()
    run(["compare", args.audio_a, args.audio_b])
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

