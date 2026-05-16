#!/usr/bin/env bash

set -euo pipefail

ROOT=$(git rev-parse --show-toplevel 2>/dev/null || pwd)
GSTACK_BIN="$HOME/.codex/skills/gstack/bin"
if [ -d "$ROOT/.agents/skills/gstack/bin" ]; then
  GSTACK_BIN="$ROOT/.agents/skills/gstack/bin"
fi

slug=$(basename "$ROOT")
if [ -x "$GSTACK_BIN/gstack-slug" ]; then
  eval "$("$GSTACK_BIN/gstack-slug" 2>/dev/null)" 2>/dev/null || true
  if [ -n "${SLUG:-}" ]; then
    slug="$SLUG"
  fi
fi

branch=$(git rev-parse --abbrev-ref HEAD 2>/dev/null | tr '/' '-' || echo no-branch)
stamp=$(date +%Y%m%d-%H%M%S)
session_dir="${GSTACK_HOME:-$HOME/.gstack}/projects/$slug/planning-sessions/${branch}-${stamp}"
mkdir -p "$session_dir"

printf 'SESSION_DIR=%q\n' "$session_dir"
printf 'PRIOR_MESSAGE_FILE=%q\n' "$session_dir/prior-implementation-last-message.md"
printf 'NEXT_MILESTONE_FILE=%q\n' "$session_dir/next-milestone.md"
printf 'DESIGN_DOC_SUMMARY_FILE=%q\n' "$session_dir/design-doc-summary.md"
printf 'PLAN_PASS1_FILE=%q\n' "$session_dir/plan-pass-1.md"
printf 'PLAN_PASS2_FILE=%q\n' "$session_dir/plan-pass-2.md"
printf 'SESSION_LOG_FILE=%q\n' "$session_dir/session-log.md"
