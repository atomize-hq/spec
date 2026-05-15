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
project_dir="${GSTACK_HOME:-$HOME/.gstack}/projects/$slug"

latest=""
if [ -d "$project_dir" ]; then
  latest=$(find "$project_dir" -maxdepth 1 -type f -name "*-${branch}-design-*.md" 2>/dev/null | xargs ls -1t 2>/dev/null | head -1 || true)
  if [ -z "$latest" ]; then
    latest=$(find "$project_dir" -maxdepth 1 -type f -name "*-design-*.md" 2>/dev/null | xargs ls -1t 2>/dev/null | head -1 || true)
  fi
fi

if [ -z "$latest" ]; then
  echo "no design doc found" >&2
  exit 1
fi

printf '%s\n' "$latest"
