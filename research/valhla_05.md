# Valhla_05 lens: knowledge cartography (no vote)

Clock: 2026-11-02 08:14 UTC (`date -u`). Skills read in full, read-only.

## Sources (read 2026-11-02)
- S1 https://code.claude.com/docs/en/skills (S1–S3: Anthropic docs, no licence stated; summarised)
- S2 https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices
- S3 https://platform.claude.com/docs/en/agents-and-tools/agent-skills/overview
- S4 https://docs.gitlab.com/user/model_context_protocol/mcp_server/ (v19.6; licence UNKNOWN)

## What the docs say (VERIFIED)
- Description: ≤1,024 chars, third person, what + when (S2, S3). Claude Code cuts description + `when_to_use` at 1,536 chars (S1).
- Outside Claude Code (claude.ai uploads), only six frontmatter keys are allowed: `name`, `description`, `license`, `compatibility`, `metadata`, `allowed-tools`. Others are a hard error (S1).
- SKILL.md under 500 lines; references one level deep; a table of contents in reference files over 100 lines; one term per concept; no time-sensitive facts (S2).
- Custom skills do not sync across surfaces (S3). `${CLAUDE_SKILL_DIR}` points at the skill's own folder in Claude Code (S1).

## Measured (VERIFIED)
- Old.Sage: description 963/1,024 chars, all D&D; SKILL.md 228 lines; `binarymagix.md` 137 lines, no table of contents. Young.Sage: description 474 chars; SKILL.md 99 lines.
- Old.Sage never mentions Young.Sage, a HANDOFF, the Forge or the label `REPORTED`. Neither skill contains `PRE_REVIEW_ONLY`; Young uses `Mode: REVIEW_ONLY`.

## 1. Old.Sage_20
- `SKILL.md` frontmatter: the description covers only the DM role. Coordinator work (RoundTable, NIGHTGLASS, Forge, Young.Sage handoffs) has no trigger words, so it will likely under-trigger (INFERENCE). Split into `dndevs-old-sage` (DM) and `dndevs-old-sage-forge` (coordinator), per S2's "organise by domain". If one skill stays, trim the D&D triggers and add coordinator ones.
- `SKILL.md`: move "Run the table" + "Three Histories" (lines 98–161) to `references/run-the-table.md`, and "Which model does the work" to `references/model-routing.md`; leave one-line pointers.
- `SKILL.md`: call scripts as `python3 ${CLAUDE_SKILL_DIR}/scripts/sage_lookup.py`. A relative path likely breaks outside the skill folder (INFERENCE). Optional: `allowed-tools: Bash(python3 ${CLAUDE_SKILL_DIR}/scripts/*)`, a portable key.
- `references/binarymagix.md`: add a table of contents (S2 rule).
- `references/database.md`, `references/dndevs-canon.md`: move dated facts into a "Last checked" footer.
- New `references/handoff-intake.md`: receiving a Young.Sage HANDOFF (check sha256 and branch; `REPORTED` until re-run).
- `references/working-agreement.md`: add `REPORTED` to the evidence labels so they match Young's.

## 2. Young.Sage_02
- `SKILL.md` frontmatter: lead with what the skill does ("Prepares research, drafts and tests on a young-sage/ branch for Old.Sage_20"), then the triggers. Add only portable keys (`license`, `metadata.version`), because the skill runs on claude.ai (S1).
- `SKILL.md`: the Forge path `/var/home/VVgbon/NIGHTGLASS/forge/stack/` is machine-specific. Move it to `references/forge-paths.md`.
- `SKILL.md` and `templates.md`: give the templates names instead of numbers ("template 4" becomes "RoundTable HANDOFF"). Settle `REVIEW_ONLY` vs `PRE_REVIEW_ONLY` as one term (PENDING_DAWA).
- Both skills: the approval and evidence rules exist in two divergent copies. Skills don't sync (S3), so ship one identical `references/protocol.md` in each skill, with a version line and sha256, so drift is visible (INFERENCE).
- Naming: `dndevs-young-sage` would match `dndevs-old-sage` (S2 "consistent pattern"); renaming changes the command (Dawa chooses).

## 3. Young.Sage_02 with GitLab (lens only)
- GitLab's MCP server is GA (GitLab 19.5) on Free, Premium and Ultimate, at `https://<host>/api/v4/mcp`, with OAuth DCR (S4, VERIFIED).
- Put GitLab knowledge in a new `references/gitlab.md` that is read only when the repo host is GitLab (S2 "conditional details"). It holds a vocabulary map (PR → merge request, Actions → CI pipelines), so SKILL.md stays host-neutral.
- `templates.md`: replace `Repository: <owner/repo>` with `Host: <github|gitlab> · Project: <full namespace path>`. GitLab subgroups make `owner/repo` too narrow (INFERENCE).
- Tool names: S4 supports the header `X-Gitlab-Mcp-Server-Tool-Name-Prefix: gitlab_`, and S2 asks for fully qualified tool names, so GitLab and GitHub tools never collide.
- Least surface: the header `X-Gitlab-Enabled-Mcp-Server-Toolsets: core,merge_requests,repository` limits which tools are listed. "Listing a tool does not grant access" (S4). Writes still need Dawa's `~]{OK`.
- GitLab as a custom connector in the Claude app: UNKNOWN.
