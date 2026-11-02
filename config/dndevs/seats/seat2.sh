#!/usr/bin/env bash
# DnDevs seat 2: Young.Sage_02 (Claude Code, dawa.gabond05, low token).
cd /var/home/VVgbon/Young-Sage_02 || exit 1
exec env -u CLAUDE_CONFIG_DIR ANTHROPIC_MODEL=sonnet CLAUDE_CODE_EFFORT_LEVEL=low \
  claude --permission-mode default \
  --append-system-prompt "$(cat ~/.config/dndevs/young-sage.persona.md)" \
  "$(cat ~/.config/dndevs/seats/boot.txt)"
