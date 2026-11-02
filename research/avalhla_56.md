# Avalhla_56 lens: one team, many clients (research, no vote)

Sources, read 2026-11-02 via Firecrawl (WebFetch blocked for agentskills.io and opencode.ai).
- S1 agentskills.io/specification
- S2 opencode.ai/docs/skills, /agents, /rules
- S3 learn.chatgpt.com/docs/build-skills (Codex; search excerpt)
- S4 docs.github.com/en/copilot/concepts/agents/about-agent-skills
- S5 code.claude.com/docs/en/skills, /gitlab-ci-cd
- S6 docs.gitlab.com/user/duo_agent_platform/customize/agent_skills, /agents_md, /agents/custom, /agents/external

## Findings
- VERIFIED (S1): required fields are `name` (1-64 characters, a-z0-9 and hyphens, same as the folder) and `description` (at most 1024 characters). Optional: `license`, `compatibility`, `metadata`, `allowed-tools`.
- VERIFIED: `.agents/skills/` is read by OpenCode (S2), Codex (S3), Copilot (S4) and GitLab Duo (S6). Claude Code reads `.claude/skills/` and claude.ai uploads (S5); its docs do not mention `.agents/skills`. OpenCode and Copilot also read `.claude/skills/`.
- VERIFIED (S5): Claude adds fields of its own (`model`, `context`, `hooks`…). OpenCode recognizes only the standard ones (S2).
- VERIFIED (local measurement): the dndevs-old-sage description is 963 of 1024 characters, so any generated suffix would break it. Persona IDs such as `Old.Sage_20` are not valid skill names, so they belong in `metadata`.
- INFERENCE: parts of the bodies are tied to Claude: Old.Sage's "Which model does the work" and "a voice of UMBRA / Claude Code" are wrong under other models; so are Young.Sage's "Replies in the app" and VVgBazz paths in GitLab.

## 1. Old.Sage_20: one source of truth, generated files
- One repository, `dndevs-personas/`, with `personas/old-sage-20/`: `persona.yaml` (persona ID, skill name, seat, clients), a client-neutral `core.md`, `overlays/<client>.md`, `references/`, `scripts/`.
- `shared/covenant.md` holds what both skills and WORKING_AGREEMENT repeat: `~]{OK` scope, VERIFIED/INFERENCE/UNKNOWN, "Dawa chooses". It is generated into `AGENTS.md` (read by OpenCode, Codex, GitLab; S2, S6) and into `CLAUDE.md`.
- A build script writes:
  - `.agents/skills/dndevs-old-sage/` (standard fields only)
  - `.claude/skills/dndevs-old-sage/` (core plus the Claude overlay with model routing)
  - `.opencode/agents/old-sage.md`, with `mode`, `permission.skill: {"dndevs-old-sage": allow}` and denial for the other seats' skills (S2)
- Generated files start with `GENERATED from <path>@<commit>; do not edit`. CI reruns the build, fails on any diff, and runs skills-ref validate (S1).
- Harmony rule (INFERENCE, working agreement item 7): each overlay makes the persona state which client and model really ran it ("Old.Sage_20 lens, run by OpenCode/<model>"), never UMBRA/Claude Code when Claude did not. The RoundTable SmartStamp gets `Client:` and `Model: observed|configured` lines.
- Shorten the description to about 700 characters.

## 2. Young.Sage_02
- Remove "not the terminal CLI" from the description. The surface goes in `compatibility:` and `metadata.surface`.
- The core keeps the lane rules (a `young-sage/*` branch, never `main`, handoffs, evidence labels). Overlays:
  - `claude-app`: phone reply shape; zip upload to claude.ai (S5).
  - `opencode`
  - `gitlab`: no VVgBazz paths; the handoff goes in the MR or issue description, using template 1.
- The build also produces `young-sage.zip` for the second account, so nobody hand-edits it.

## 3. Young.Sage_02 inside GitLab: options ranked by persona fidelity
1. **External agent running Claude Code.** VERIFIED (S6): available on Premium/Ultimate, with `injectGatewayToken: true` (GitLab-managed Claude credentials), triggered by mentioning a service account in an issue or MR. Closest to the same model and files. UNKNOWN: whether it loads `.claude/skills`, because S5 says only "follows CLAUDE.md".
2. **Claude Code in GitLab CI** (beta, `ANTHROPIC_API_KEY`, `claude -p … --allowedTools`; S5). Proposals:
   - Narrow `--allowedTools`; protected `main` (INFERENCE).
   - Run only when the note author is Dawa and it contains `~]{OK` (INFERENCE).
3. **Project skill in `.agents/skills/young-sage/`.** Used by local Duo Chat, flows and the Duo CLI, from GitLab 18.10 (VERIFIED, S6). GitLab's model, so the persona says so.
4. **AI Catalog custom agent.** Fields: name, description, visibility, system prompt, tools (VERIFIED, S6). UI-entered prompt, no scripts: generate `gitlab/young-sage.system-prompt.md` from the core and paste it as a new version.
- A dedicated GitLab service account for Young.Sage_02 matches its separate-account identity (INFERENCE).

UNKNOWN: Copilot CLI and `AGENTS.md` (not checked).
