#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
ENTRY="$PROJECT_ROOT/src/main.hyp"

if [[ ! -f "$ENTRY" ]]; then
  echo "Entry script not found at $ENTRY" >&2
  exit 1
fi

if ! command -v hypnoscript >/dev/null 2>&1; then
  echo "hypnoscript CLI not found. Please install it or build via cargo." >&2
  exit 1
fi

if [[ $# -eq 0 ]]; then
  set -- help
fi

COMMAND="$1"
shift || true

PREV_COMMAND="$HYPNO_COMMAND"
PREV_PAYLOAD="$HYPNO_PAYLOAD"

cleanup() {
  export HYPNO_COMMAND="$PREV_COMMAND"
  export HYPNO_PAYLOAD="$PREV_PAYLOAD"
}
trap cleanup EXIT

export HYPNO_COMMAND="$COMMAND"
if [[ $# -gt 0 ]]; then
  export HYPNO_PAYLOAD="$*"
else
  unset HYPNO_PAYLOAD
fi

exec hypnoscript run "$ENTRY"
