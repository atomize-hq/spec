#!/usr/bin/env bash

set -uo pipefail

ROOT=$(git rev-parse --show-toplevel 2>/dev/null || pwd)
cd "$ROOT" || exit 1

has_jq=false
if command -v jq >/dev/null 2>&1; then
  has_jq=true
fi

print_section() {
  printf '\n=== %s ===\n' "$1"
}

run_json_summary() {
  local label="$1"
  local mode="$2"
  shift 2
  local out err
  out=$(mktemp)
  err=$(mktemp)
  if "$@" >"$out" 2>"$err"; then
    print_section "$label"
    if [ "$has_jq" = true ]; then
      case "$mode" in
        coverage)
          jq -r '"total_function_units: \(.function_coverage.total_units)\npromoted_family_units: \(.function_coverage.promoted_family_units)\nunsupported_function_units: \(.function_coverage.unsupported_function_units)\nreal_example_sources: \([.sources[] | select(.kind == "real_example") | .id] | join(", "))"' "$out" 2>/dev/null || cat "$out"
          ;;
        recommendation)
          jq -r '"recommendation_status: \(.recommendation_status)\ndecision_status: \(.decision_summary.decision_status)\nsummary: \(.decision_summary.summary)\nblockers: \((.decision_summary.open_blockers // []) | join(", "))\nmissing_evidence: \((.evidence_summary.missing_evidence // []) | join(", "))\nstale_evidence: \((.evidence_summary.stale_evidence // []) | join(", "))"' "$out" 2>/dev/null || cat "$out"
          ;;
        contract)
          jq -r '"overall_verdict: \(.overall_verdict)\nfailed_checks: \([.checks | to_entries[] | select(.value.status != "pass") | .key] | join(", "))"' "$out" 2>/dev/null || cat "$out"
          ;;
        *)
          cat "$out"
          ;;
      esac
    else
      cat "$out"
    fi
  else
    local last_line
    last_line=$(awk 'NF { line=$0 } END { print line }' "$err")
    print_section "$label"
    if [ "$label" = "FAMILY_DECISION_CONTRACT" ] && [ "$last_line" = "invalid input" ]; then
      echo "status: unavailable"
      echo "reason: decision artifacts are missing or not mutually consistent for this branch snapshot"
    else
      echo "[command failed] $*"
      [ -n "$last_line" ] && echo "$last_line"
    fi
  fi
  rm -f "$out" "$err"
}

print_section "REPO"
echo "root: $ROOT"
echo "branch: $(git branch --show-current 2>/dev/null || echo unknown)"

print_section "DIRTY_STATUS"
git status --short 2>/dev/null || true

print_section "RECENT_COMMITS"
git log --oneline -8 2>/dev/null || true

slug="atomize-hq-spec"
if [ -x "$HOME/.codex/skills/gstack/bin/gstack-slug" ]; then
  eval "$("$HOME/.codex/skills/gstack/bin/gstack-slug" 2>/dev/null)" 2>/dev/null || true
  if [ -n "${SLUG:-}" ]; then
    slug="$SLUG"
  fi
fi

proj="${GSTACK_HOME:-$HOME/.gstack}/projects/$slug"
checkpoint_dir="$proj/checkpoints"
latest_checkpoint=""
if [ -d "$checkpoint_dir" ]; then
  latest_checkpoint=$(find "$checkpoint_dir" -maxdepth 1 -name "*.md" -type f 2>/dev/null | xargs ls -1t 2>/dev/null | head -1)
fi

print_section "LATEST_CHECKPOINT"
if [ -n "$latest_checkpoint" ] && [ -f "$latest_checkpoint" ]; then
  echo "path: $latest_checkpoint"
  sed -n 's/^## Working on: /title: /p' "$latest_checkpoint" | head -1
  awk '
    /^### Summary/ {flag=1; next}
    /^### / && flag {flag=0}
    flag {print}
  ' "$latest_checkpoint" | sed '/^$/d' | head -3
  awk '
    /^### Remaining Work/ {flag=1; next}
    /^### / && flag {flag=0}
    flag {print}
  ' "$latest_checkpoint" | sed -n 's/^[0-9]\+\. /next: /p' | head -2
else
  echo "path: [none]"
fi

run_json_summary \
  "FAMILY_COVERAGE" \
  coverage \
  cargo xtask family coverage --format json

run_json_summary \
  "FAMILY_RECOMMENDATION" \
  recommendation \
  cargo xtask family recommend --format json

run_json_summary \
  "FAMILY_DECISION_CONTRACT" \
  contract \
  cargo xtask family verify-decision-contract --format json
