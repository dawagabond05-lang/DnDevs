#!/usr/bin/env bash
# DnDevs seat 1: KING.VEX_88 + LHLAVA_13 (Claude Code, dawa.gabond05, low token).
cd /var/home/VVgbon/Umbra-Keep_Vex-Lhlava || exit 1
exec env -u CLAUDE_CONFIG_DIR ANTHROPIC_MODEL=sonnet CLAUDE_CODE_EFFORT_LEVEL=low \
  claude --append-system-prompt "$(cat .avalhla/persona.live.md)" \
  "$(cat ~/.config/dndevs/seats/boot.txt)"
