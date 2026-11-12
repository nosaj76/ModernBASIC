#!/usr/bin/env bash
set -euo pipefail

project_dir="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
# A bare interactive launch asks; scripts must choose a mode or BASIC command.
if [ "$#" -eq 0 ]; then
    if [ ! -t 0 ]; then
        printf '%s\n' 'Choose --desktop [workspace-directory] or --no-desktop [BASIC arguments].' >&2
        exit 2
    fi
    while :; do
        printf '%s\n' 'Choose your environment:' \
            '  1) Desktop (files, editor, Office and browser)' \
            '  2) No desktop (start BASIC directly)' \
            '  q) Cancel' >&2
        printf 'Choice: ' >&2
        IFS= read -r choice || exit 0
        case "$choice" in
            1|desktop) set -- --desktop; break ;;
            2|no-desktop) set -- --no-desktop; break ;;
            q|Q) exit 0 ;;
            *) printf '%s\n' 'Enter 1, 2 or q.' >&2 ;;
        esac
    done
fi
case "$1" in
    --desktop)
        shift
        exec "$project_dir/tools/run-desktop.sh" "$@"
        ;;
    --no-desktop)
        shift
        if [ "$#" -eq 0 ]; then set -- shell; fi
        ;;
esac

. "$project_dir/tools/test-binary.sh"
binary="$(modernbasic_build_binary "$project_dir")"

cd "$project_dir"
exec "$binary" "$@"
