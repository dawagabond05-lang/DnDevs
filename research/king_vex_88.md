# King.Vex_88 lens: pruning the Sage skills

Research lens, not a vote. Dawa chooses.

## Baseline (VERIFIED)
- Only name + description load at startup (~100 tokens). The body should stay under 5k tokens and 500 lines; references cost nothing until read [A]. Description max is 1,024 chars [A]. After compaction Claude Code keeps only a skill's first 5k tokens, so key rules go at the top [C]. Aim for the "smallest possible set of high-signal tokens" [D][E].
- Old.Sage SKILL.md: 231 lines, 15.1 KB (~3.8k tokens, INFERENCE); description 964/1,024 chars. Young.Sage: 102 lines, 5.9 KB.

## 1. Old.Sage_20
- **SKILL.md L3**: trim to ~500 chars. Drop "all in the warm voice…" and "with real dice and dungeons…". Drop the bare trigger "Sage", which collides with Young.Sage (INFERENCE).
- **L15-20**: the roster and the 11/01/26 date repeat dndevs-canon.md L15-28 and will go stale [B]. Keep one line.
- **L98-161** (Three Histories + Run the table, 64 lines): play mode only. Move to `references/run-the-table.md` and leave a 3-line pointer.
- **L163-192** (model routing): engineering, not game; the model names will go stale. Move to working-agreement.md or `references/bulk-jobs.md`.
- **L54-56**: repeats sage_lookup.py L12. Cut.
- **L63-66 + L207-209**: two licence/version passages. Merge.
- **L199-201, L205-206**: repeat binarymagix.md L4-7 and L130-131. Cut from the reference.
- **binarymagix.md L112-117**: duplicates database.md L29-37. Keep one copy.
- **binarymagix.md L118-137**: an audit fix list that will go stale. Move it out. The file has 137 lines, so add a table of contents [B].
- **database.md L10-12, L50-51**: dated snapshot counts. Point to `sage_lookup.py sources`.
- working-agreement.md was approved by Dawa, so editing it needs `~]{OK`.

## 2. Young.Sage_02
- **L3**: the triggers "~]{OK", "the Forge", "the RoundTable" and "Old.Sage_20" will over-fire. Narrow them to "Young.Sage_02", "young-sage/ branch", and "prepare work for Old.Sage_20".
- **L71**: "Read before you write" is something Claude already does. Cut [B].
- **L84-86**: the Forge's internal pipeline. Cut to one line.
- **L92-96**: duplicates templates.md L62-69. Also, VERDICT·EVIDENCE·NEXT·VERIFY here differs from VERDICT·EVIDENCE·COUNCIL·NEXT·VERIFY in working-agreement.md L11. Pick one.
- **L29-32**: duplicates templates.md L71-73. Keep a pointer.
- **L26-27 vs L33-34**: the first says never touch the Forge, the second says write to `forge/stack/`; the host is also spelled VVgBazz and VVgbon. This needs Dawa's ruling.

## 3. GitLab minimum
- VERIFIED: hosted cloud sessions need GitHub to clone and open PRs. GitLab works only as a `CCR_FORCE_BUNDLE=1` bundle, which **cannot push back** [F]. gitlab.com is on the default allowlist [G].
- **Option A (recommended):** no GitLab credential. Bundle in, `git format-patch` → `young-<topic>.mbox` + sha256 out, which L33-34 already describe. INFERENCE.
- **Option B (only if a push is required):**
  - One **project access token**: role **Developer**, scope **`write_repository` only**, short expiry. It is single-project [H] and Git-only, with no API [I]. No `api`, `read_api` or Maintainer.
  - `main` stays **Fully protected** (the default: Developers cannot push, nobody can force-push) [J]. "Never commit to main" is then enforced, not just written down.
  - Store it as a **network secret** (Username and password, host gitlab.com), never as an env var, since env vars are readable by anyone on the environment [G]. UNKNOWN whether git push picks it up; test with `git ls-remote` first.
  - Ban `-o merge_request.auto_merge`; use `merge_request.create` only on Dawa's ask [K]. Protect the CI variables (INFERENCE from [J]).
  - At most 3 lines in the skill, after L24.

## Sources (read 2026-11-02)
- [A] https://platform.claude.com/docs/en/agents-and-tools/agent-skills/overview
- [B] https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices
- [C] https://code.claude.com/docs/en/skills
- [D] https://www.anthropic.com/engineering/effective-context-engineering-for-ai-agents (2025-09-29)
- [E] https://trychroma.com/research/context-rot (2025-07-14, search summary only)
- [F] https://code.claude.com/docs/en/claude-code-on-the-web
- [G] https://code.claude.com/docs/en/cloud-environments
- [H] https://gitlab.com/gitlab-org/gitlab/-/raw/master/doc/user/project/settings/project_access_tokens.md
- [I] https://gitlab.com/gitlab-org/gitlab/-/raw/master/doc/security/tokens/access_token_scopes.md
- [J] https://gitlab.com/gitlab-org/gitlab/-/raw/master/doc/user/project/repository/branches/default.md
- [K] https://gitlab.com/gitlab-org/gitlab/-/raw/master/doc/topics/git/commit.md
