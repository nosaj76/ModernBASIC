#!/usr/bin/env bash
set -euo pipefail

project_dir="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
guitar_dir="$project_dir/soundfonts/FreePats-FSBS-Distorted-Guitar"
lead_dir="$project_dir/soundfonts/FreePats-FSBS-Clean-Guitar"
direct_dir="$project_dir/soundfonts/FreePats-FSBS-Direct-Guitar"
bass_dir="$project_dir/soundfonts/FreePats-Picked-Bass-YR"
drum_dir="$project_dir/soundfonts/AVL-Red-Zeppelin"
guitar_bank="$guitar_dir/EGuitarFSBS-dist2 bridge 20220911.sf2"
full_lead_bank="$lead_dir/EGuitarFSBS-clean bridge 20260807.sf2"
lead_bank="$lead_dir/EGuitarFSBS-clean bridge small 20260807.sf2"
direct_bank="$direct_dir/EGuitarFSBS-direct bridge 20220911.sf2"
bass_bank="$bass_dir/PickedBassYR 20190930.sf2"
drum_bank="$drum_dir/Red_Zeppelin_5-1.1.sf2"
guitar_url="https://github.com/freepats/electric-guitar-FSBS-dist2/releases/download/2022-09-11/EGuitarFSBS-dist2-SF2-20220911.7z"
lead_url="https://github.com/freepats/electric-guitar-FSBS-clean/releases/download/2026-08-07/EGuitarFSBS-clean-bridge-small-SF2-20260807.7z"
direct_url="https://github.com/freepats/electric-guitar-FSBS-direct/releases/download/2022-09-11/EGuitarFSBS-direct-SF2-20220911.7z"
bass_url="https://github.com/freepats/electric-bass-YR/releases/download/2019-09-30/PickedBassYR-SF2-20190930.7z"
drum_url="https://download.linuxaudio.org/musical-instrument-libraries/sf2/AVL-Drumkits-1.1-SF2-splitted/Red_Zeppelin_5-1.1.tar.7z"
guitar_sha256=0ed99135d955940c1200e09ca9e36e261dfebbe8767ae58da6f298fe29fdd146
lead_sha256=bcea7d4c92a1cc6a26e545a218b34e59037d4bab6cf0fce24396428e6934d69c
direct_sha256=dcdadeab31c7aff67ace2556ede4021897743b7ae88c187f2d12988483403295
bass_sha256=e6bff767c352cd27de2de8cfff326d0849a76bb38d876075eecd5ffd2dc5afd0
drum_sha256=f591a6acd1b136cef6eedbca0a6b2e0251eeec45c27272b3c8efc4d042ac10a2
verifier="$project_dir/tools/verify-sha256.sh"
force=false
play_command="cargo run --release -- run programs/music/rock/steel_thunder.bas"

case "${1:-}" in
    "") ;;
    --force) force=true ;;
    -h|--help)
        printf "%s\n" "usage: ./install-rock-soundfonts.sh [--force]"
        printf "%s\n" "Installs CC0 FreePats processed/raw guitar and bass banks plus CC-BY-SA AVL drums."
        exit 0
        ;;
    *)
        printf "%s\n" "error: usage: ./install-rock-soundfonts.sh [--force]" >&2
        exit 2
        ;;
esac

for tool in curl 7z sha256sum; do
    if ! command -v "$tool" >/dev/null 2>&1; then
        printf "%s\n" "error: $tool is required to install the rock SoundFonts." >&2
        exit 1
    fi
done
[ -x "$verifier" ] || {
    printf '%s\n' 'error: the project checksum verifier is not executable.' >&2
    exit 1
}

skip_guitar=false
skip_full_lead=false
skip_lead=false
skip_direct=false
skip_bass=false
skip_drums=false
if [ -f "$guitar_bank" ] && [ "$force" = false ]; then skip_guitar=true; fi
if [ -f "$full_lead_bank" ] && [ "$force" = false ]; then skip_full_lead=true; fi
if [ -f "$lead_bank" ] && [ "$force" = false ]; then skip_lead=true; fi
if [ -f "$direct_bank" ] && [ "$force" = false ]; then skip_direct=true; fi
if [ -f "$bass_bank" ] && [ "$force" = false ]; then skip_bass=true; fi
if [ -f "$drum_bank" ] && [ "$force" = false ]; then skip_drums=true; fi
if [ "$skip_guitar" = true ] && [ "$skip_lead" = true ] && \
   [ "$skip_direct" = true ] && [ "$skip_full_lead" = true ] && \
   [ "$skip_bass" = true ] && [ "$skip_drums" = true ]; then
    printf "%s\n" "Rock SoundFonts are already installed. Use --force to replace them."
    printf "%s\n" "Play BASIC: $play_command"
    exit 0
fi

download_tmp="$(mktemp -d)"
cleanup() { rm -rf -- "$download_tmp"; }
trap cleanup EXIT HUP INT TERM

verify_sf2() {
    local bank_path="$1"
    if [ "$(dd if="$bank_path" bs=1 count=4 2>/dev/null)" != "RIFF" ] || \
       [ "$(dd if="$bank_path" bs=1 skip=8 count=4 2>/dev/null)" != "sfbk" ]; then
        printf "%s\n" "error: downloaded asset is not a valid SF2 bank: $bank_path" >&2
        exit 1
    fi
}

if [ "$skip_guitar" = false ]; then
    printf "%s\n" "Downloading FreePats FSBS distorted guitar (about 121 MiB)..."
    curl --fail --location --retry 3 --output "$download_tmp/guitar.7z" "$guitar_url"
    "$verifier" "$guitar_sha256" "$download_tmp/guitar.7z" \
        EGuitarFSBS-dist2-SF2-20220911.7z
    mkdir -p -- "$download_tmp/guitar"
    7z e "$download_tmp/guitar.7z" -o"$download_tmp/guitar" -y >/dev/null
    verify_sf2 "$download_tmp/guitar/EGuitarFSBS-dist2 bridge 20220911.sf2"
    install -d -m 0755 "$guitar_dir"
    install -m 0644 "$download_tmp/guitar/EGuitarFSBS-dist2 bridge 20220911.sf2" "$guitar_bank"
    install -m 0644 "$download_tmp/guitar/LICENSE.txt" "$guitar_dir/LICENSE.txt"
    install -m 0644 "$download_tmp/guitar/README.txt" "$guitar_dir/README.txt"
    printf "%s\n" "Installed: $guitar_bank"
fi

if [ "$skip_full_lead" = false ]; then
    printf '%s\n' 'Downloading FreePats FSBS full clean guitar...'
    curl --fail --location --retry 3 --output "$download_tmp/full-lead.7z" \
        'https://github.com/freepats/electric-guitar-FSBS-clean/releases/download/2026-08-07/EGuitarFSBS-clean-SF2-20260807.7z'
    "$verifier" 6d338afefe0efb629afd9ba28a5c8c129fc28b97ef1b57a31ebceb7bf0e3bf65 \
        "$download_tmp/full-lead.7z" EGuitarFSBS-clean-SF2-20260807.7z
    mkdir -p -- "$download_tmp/full-lead"
    7z e "$download_tmp/full-lead.7z" -o"$download_tmp/full-lead" -y >/dev/null
    verify_sf2 "$download_tmp/full-lead/EGuitarFSBS-clean bridge 20260807.sf2"
    install -d -m 0755 "$lead_dir"
    install -m 0644 "$download_tmp/full-lead/EGuitarFSBS-clean bridge 20260807.sf2" "$full_lead_bank"
    install -m 0644 "$download_tmp/full-lead/LICENSE.txt" "$lead_dir/LICENSE-full.txt"
    install -m 0644 "$download_tmp/full-lead/README.txt" "$lead_dir/README-full.txt"
fi

if [ "$skip_lead" = false ]; then
    printf "%s\n" "Downloading FreePats FSBS compact clean lead guitar (about 6.3 MiB)..."
    curl --fail --location --retry 3 --output "$download_tmp/lead.7z" "$lead_url"
    "$verifier" "$lead_sha256" "$download_tmp/lead.7z" \
        EGuitarFSBS-clean-bridge-small-SF2-20260807.7z
    mkdir -p -- "$download_tmp/lead"
    7z e "$download_tmp/lead.7z" -o"$download_tmp/lead" -y >/dev/null
    verify_sf2 "$download_tmp/lead/EGuitarFSBS-clean bridge small 20260807.sf2"
    install -d -m 0755 "$lead_dir"
    install -m 0644 "$download_tmp/lead/EGuitarFSBS-clean bridge small 20260807.sf2" "$lead_bank"
    install -m 0644 "$download_tmp/lead/LICENSE.txt" "$lead_dir/LICENSE.txt"
    install -m 0644 "$download_tmp/lead/README.txt" "$lead_dir/README.txt"
    printf "%s\n" "Installed: $lead_bank"
fi

if [ "$skip_direct" = false ]; then
    printf "%s\n" "Downloading FreePats FSBS raw direct Fender guitar (about 60 MiB)..."
    curl --fail --location --retry 3 --output "$download_tmp/direct.7z" "$direct_url"
    "$verifier" "$direct_sha256" "$download_tmp/direct.7z" \
        EGuitarFSBS-direct-SF2-20220911.7z
    mkdir -p -- "$download_tmp/direct"
    7z e "$download_tmp/direct.7z" -o"$download_tmp/direct" -y >/dev/null
    verify_sf2 "$download_tmp/direct/EGuitarFSBS-direct bridge 20220911.sf2"
    install -d -m 0755 "$direct_dir"
    install -m 0644 "$download_tmp/direct/EGuitarFSBS-direct bridge 20220911.sf2" "$direct_bank"
    install -m 0644 "$download_tmp/direct/LICENSE.txt" "$direct_dir/LICENSE.txt"
    install -m 0644 "$download_tmp/direct/README.txt" "$direct_dir/README.txt"
    printf "%s\n" "Installed: $direct_bank"
fi

if [ "$skip_bass" = false ]; then
    printf "%s\n" "Downloading FreePats Yamaha picked bass (about 2.2 MiB)..."
    curl --fail --location --retry 3 --output "$download_tmp/bass.7z" "$bass_url"
    "$verifier" "$bass_sha256" "$download_tmp/bass.7z" PickedBassYR-SF2-20190930.7z
    mkdir -p -- "$download_tmp/bass"
    7z e "$download_tmp/bass.7z" -o"$download_tmp/bass" -y >/dev/null
    verify_sf2 "$download_tmp/bass/PickedBassYR 20190930.sf2"
    install -d -m 0755 "$bass_dir"
    install -m 0644 "$download_tmp/bass/PickedBassYR 20190930.sf2" "$bass_bank"
    install -m 0644 "$download_tmp/bass/LICENSE.txt" "$bass_dir/LICENSE.txt"
    install -m 0644 "$download_tmp/bass/README.txt" "$bass_dir/README.txt"
    printf "%s\n" "Installed: $bass_bank"
fi

if [ "$skip_drums" = false ]; then
    printf "%s\n" "Downloading AVL Red Zeppelin drums (about 14 MiB)..."
    curl --fail --location --retry 3 --output "$download_tmp/drums.7z" "$drum_url"
    "$verifier" "$drum_sha256" "$download_tmp/drums.7z" Red_Zeppelin_5-1.1.tar.7z
    mkdir -p -- "$download_tmp/drum-outer" "$download_tmp/drums"
    7z x "$download_tmp/drums.7z" -o"$download_tmp/drum-outer" -y >/dev/null
    7z e "$download_tmp/drum-outer/Red_Zeppelin_5-1.1.tar" -o"$download_tmp/drums" -y >/dev/null
    verify_sf2 "$download_tmp/drums/Red_Zeppelin_5-1.1.sf2"
    install -d -m 0755 "$drum_dir"
    install -m 0644 "$download_tmp/drums/Red_Zeppelin_5-1.1.sf2" "$drum_bank"
    install -m 0644 "$download_tmp/drums/AVL-Drumkits CC-BY-SA License.pdf" "$drum_dir/AVL-Drumkits CC-BY-SA License.pdf"
    printf "%s\n" "Installed: $drum_bank"
fi

printf "%s\n" "Play BASIC: $play_command"
