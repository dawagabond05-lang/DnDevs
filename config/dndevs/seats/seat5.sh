#!/usr/bin/env bash
# DnDevs seat 5: Z_71 DevilBro: OpenCode, own headless server, DC MCP.
cd /var/home/VVgbon/Avalhla || exit 1
exec env DNDEVS_AGENT=dndevs-z71 /var/home/VVgbon/.opencode/bin/opencode --standalone \
  --prompt "$(cat ~/.config/dndevs/seats/boot.txt)" /var/home/VVgbon/Avalhla
