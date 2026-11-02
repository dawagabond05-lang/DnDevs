#!/usr/bin/env bash
# DnDevs seat 3: Lumen-Council (LUMEN): OpenCode, own headless server, DC MCP.
cd /var/home/VVgbon/Lumen-Castle_Lux-Valhla || exit 1
exec env DNDEVS_AGENT=dndevs-lumen /var/home/VVgbon/.opencode/bin/opencode --standalone \
  --prompt "$(cat ~/.config/dndevs/seats/boot.txt)" /var/home/VVgbon/Lumen-Castle_Lux-Valhla
