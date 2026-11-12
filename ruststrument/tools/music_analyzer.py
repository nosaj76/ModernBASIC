#!/usr/bin/env python3
"""Compatibility front end for Ruststrument analysis and transcription.

The former script duplicated several DSP pipelines and produced contradictory
results. This version delegates analysis to the tested Rust implementation.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path
import tempfile

from ruststrument_cli import run


def _format_beats(value: float) -> str:
    return f"{max(0.03125, min(64.0, value)):.5f}".rstrip("0").rstrip(".")


def _basic_from_json(path: Path, tempo: int) -> str:
    data = json.loads(path.read_text(encoding="utf-8"))
    notes = sorted(data.get("notes", []), key=lambda item: (item["start_s"], item["midi"]))
    voices: list[tuple[float, list[str]]] = []
    seconds_per_beat = 60.0 / tempo
    for note in notes:
        start = max(0.0, float(note["start_s"]))
        end = max(start, float(note["end_s"]))
        selected = next((i for i, (cursor, _) in enumerate(voices) if cursor <= start + 0.001), None)
        if selected is None:
            if len(voices) >= 16:
                continue
            selected = len(voices)
            voices.append((0.0, []))
        cursor, tokens = voices[selected]
        if start > cursor + 0.001:
            tokens.append(f"R:{_format_beats((start - cursor) / seconds_per_beat)}")
        duration = _format_beats((end - start) / seconds_per_beat)
        velocity = max(1, min(127, int(note.get("velocity", 96))))
        tokens.append(f"{note['note']}:{duration}@{velocity}")
        voices[selected] = (end, tokens)

    lines = ["10 REM RUSTSTRUMENT TRANSCRIPTION", f"20 TEMPO {tempo}"]
    line_number = 30
    for voice, (_, tokens) in enumerate(voices, 1):
        for start in range(0, len(tokens), 24):
            sequence = " ".join(tokens[start : start + 24])
            lines.append(f'{line_number} PLAY {voice},"{sequence}"')
            line_number += 10
    lines.append(f"{line_number} END")
    return "\n".join(lines) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("audio", nargs="?", type=Path)
    parser.add_argument("mode", nargs="?", default="all")
    parser.add_argument("extra", nargs="*")
    parser.add_argument("--compare", nargs=2, metavar=("AUDIO_A", "AUDIO_B"))
    parser.add_argument("--tempo", type=int, default=120)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()

    if args.compare:
        run(["compare", *args.compare])
        return 0
    if args.audio is None:
        parser.error("an audio path is required")
    if not args.audio.is_file():
        parser.error(f"audio file does not exist: {args.audio}")

    mode = args.mode.lower()
    if mode in {"all", "analyze", "key", "chords", "tempo", "pitch", "spectrum"}:
        run(["analyze", str(args.audio)])
    elif mode in {"instrument", "classify"}:
        run(["classify", str(args.audio)])
    elif mode == "basic":
        with tempfile.TemporaryDirectory(prefix="ruststrument-transcribe-") as directory:
            transcription = Path(directory) / "notes.json"
            run(["transcribe", str(args.audio), str(transcription)])
            basic = _basic_from_json(transcription, max(20, min(400, args.tempo)))
            if args.output:
                args.output.write_text(basic, encoding="utf-8")
            else:
                print(basic, end="")
    else:
        parser.error("mode must be all, analyze, instrument, classify, or basic")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

