#!/usr/bin/env bash
set -euo pipefail

project_dir="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
asset_dir="$project_dir/soundfonts"
bank="$asset_dir/MuseScore_General.sf2"
license_file="$asset_dir/MuseScore_General_License.md"
source_base="https://ftp.osuosl.org/pub/musescore/soundfont/MuseScore_General"
bank_sha256=ee51d2c4b1525e70f19a45909c4fd7a2e26d91d115fa89dbf5a6bc413d8b9bf3
license_sha256=5ad8d737e13c7f01f5b9674872a82a92b4ba253603e8ed14b9db12293550b4b9
verifier="$project_dir/tools/verify-sha256.sh"
force=false

case "${1:-}" in
    "") ;;
    --force) force=true ;;
    -h|--help)
        printf '%s\n' 'usage: ./install-soundfont.sh [--force]'
        printf '%s\n' 'Downloads the free MIT-licensed MuseScore General SF2 bank (about 206 MiB).'
        exit 0
        ;;
    *)
        printf '%s\n' 'error: usage: ./install-soundfont.sh [--force]' >&2
        exit 2
        ;;
esac

if [ -f "$bank" ] && [ "$force" = false ]; then
    printf '%s\n' "SoundFont already installed: $bank"
    printf '%s\n' 'Use --force to replace it.'
    printf '%s\n' 'Try: cargo run --release -- run programs/examples/soundfont.bas'
    exit 0
fi

if ! command -v sha256sum >/dev/null 2>&1 || [ ! -x "$verifier" ]; then
    printf '%s\n' 'error: sha256sum and the project checksum verifier are required.' >&2
    exit 1
fi

if command -v curl >/dev/null 2>&1; then
    download() {
        curl --fail --location --retry 3 --output "$2" "$1"
    }
elif command -v wget >/dev/null 2>&1; then
    download() {
        wget --tries=3 --output-document="$2" "$1"
    }
else
    printf '%s\n' 'error: curl or wget is required to download the SoundFont.' >&2
    exit 1
fi

download_tmp="$(mktemp -d)"
trap 'rm -rf -- "$download_tmp"' EXIT HUP INT TERM

printf '%s\n' 'Downloading MuseScore General SF2 (about 206 MiB)...'
download "$source_base/MuseScore_General.sf2" "$download_tmp/MuseScore_General.sf2"
download "$source_base/MuseScore_General_License.md" "$download_tmp/MuseScore_General_License.md"
"$verifier" "$bank_sha256" "$download_tmp/MuseScore_General.sf2" MuseScore_General.sf2
"$verifier" "$license_sha256" "$download_tmp/MuseScore_General_License.md" \
    MuseScore_General_License.md

if [ "$(dd if="$download_tmp/MuseScore_General.sf2" bs=1 count=4 2>/dev/null)" != 'RIFF' ] || \
   [ "$(dd if="$download_tmp/MuseScore_General.sf2" bs=1 skip=8 count=4 2>/dev/null)" != 'sfbk' ]; then
    printf '%s\n' 'error: the downloaded file is not a valid SF2 RIFF bank.' >&2
    exit 1
fi

mkdir -p -- "$asset_dir"
mv -- "$download_tmp/MuseScore_General.sf2" "$bank"
mv -- "$download_tmp/MuseScore_General_License.md" "$license_file"

printf '%s\n' "Installed: $bank"
printf '%s\n' "License:   $license_file"
printf '%s\n' 'Try: cargo run --release -- run programs/examples/soundfont.bas'
