#!/usr/bin/env bash
set -euo pipefail

project_dir="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"

case "${1:-}" in
    '') ;;
    --verify)
        "$project_dir/verify.sh"
        ;;
    *)
        printf '%s\n' 'usage: ./install.sh [--verify]' >&2
        exit 2
        ;;
esac

"$project_dir/build-native.sh"

printf '%s\n' 'ModernBASIC is installed in this directory.'
printf '%s\n' 'Start it with: ./run.sh'
