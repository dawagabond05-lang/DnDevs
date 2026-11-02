#!/usr/bin/env bash
# DnDevs seat 4: Av:vA_22 (DESTINY): OpenCode, own headless server, DC MCP.
cd /var/home/VVgbon/Destiny-Tower_AvvA-Avalhla || exit 1
exec env DNDEVS_AGENT=dndevs-avva /var/home/VVgbon/.opencode/bin/opencode --standalone \
  --prompt "$(cat ~/.config/dndevs/seats/boot.txt)" /var/home/VVgbon/Destiny-Tower_AvvA-Avalhla
