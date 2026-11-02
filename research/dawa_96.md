# Dawa_96 lens: daily friction (research lens; Dawa chooses)

Sources (read 2026-11-02):
S1 code.claude.com/docs/en/claude-code-on-the-web · S2 code.claude.com/docs/en/cloud-environments ·
S3 code.claude.com/docs/en/skills · S4 support.claude.com/en/articles/12512180-using-skills-in-claude ·
S5 code.claude.com/docs/en/mobile · S6 docs.gitlab.com/auth/tokens/fine_grained_access_tokens/ (+ `_other/`) ·
S7 docs.gitlab.com/user/project/settings/project_access_tokens/ · S8 docs.gitlab.com/user/profile/personal_access_tokens/

## 1. Old.Sage_20: less friction

- First line: French verdict plus what Dawa must do ("Rien à faire" / "1 commande" / "1 décision"). INFERENCE: at 97x20 he sees the top first.
- Whole reply in 20 lines; the rest goes to a file plus a one-line path. That limit covers only command output today; apply it to replies too.
- One decision per reply, ending with the exact line to copy: `~]{OK apply young-seed-load-tests.mbox`. No scope drift.
- One paste-ready code block per reply, starting with `cd`, plus one line on what success looks like.
- Default report: VERDICT · NEXT · VERIFY. Add EVIDENCE/COUNCIL on request or before a signature.
- Put the engineering rules (now in `references/working-agreement.md`) in a short block at the top of SKILL.md; most of the file is game rules. INFERENCE: they are easy to miss.

## 2. Young.Sage_02: no lost work

- VERIFIED (S1, S2): an idle VM pauses, can be reclaimed, and reopening "provisions a fresh VM" from a fresh clone. Only the conversation returns. REPORTED: the patch was lost this way. Rule: **never end a turn with unpushed work.** Commit and push the branch after every meaningful change.
- The skill tells it to write to `/var/home/VVgbon/NIGHTGLASS/forge/stack/`. A cloud VM cannot reach that path. Replace it with "push the branch; Old.Sage_20 builds the .mbox". If push fails, paste the `git format-patch` plus sha256 in chat; the transcript survives.
- Mandatory **FILES RECEIVED** check first: each named path → present/absent plus sha256; if absent, say so in line one ("Je ne vois pas X; joins-le ou pousse-le"). VERIFIED (S2): cloud sessions see only the clone.
- One-step handoff: the handoff ends with one block Dawa pastes into Old.Sage_20's terminal: `cd ~/… && git fetch young young-sage/<topic> && git format-patch … -o ~/NIGHTGLASS/forge/stack/ && sha256sum …`.
- Add a `version:` line to both SKILL.md files; each sage states it on its first reply. VERIFIED (S4): on Pro/Max, skills are uploaded per account, and updating means delete then re-upload. The two accounts can drift apart silently. VERIFIED (S3): cloud sessions load that account's enabled skills.

## 3. A GitLab home for Young.Sage_02 (Dawa's side)

- VERIFIED (S1): built-in cloning and pushing in cloud sessions need GitHub. Non-GitHub repos go in as an upload only and "can't push results back". VERIFIED (S2): `gitlab.com` is on the default Trusted allowlist, and network secrets attach a token per host without the session seeing it.
- VERIFIED: project access tokens need Premium on GitLab.com (S7); fine-grained personal tokens are Free, GA since 19.2, limitable to one project, with "Code: Push" for git push (S6).

Steps:
1. gitlab.com → New project → blank, private, `young-sage-workshop`.
2. Avatar → Edit profile → Access → Personal access tokens → Generate token (fine-grained) → that project only → Code: Download + Push → 30–90 day expiry. Copy it.
3. In **Young.Sage_02's** Claude account: claude.ai/code → environment → Edit → Network secrets → Add secret → host `gitlab.com`, type Username and password (any username, token as password; S8).
4. Paste to Young.Sage_02: "Dépôt: https://gitlab.com/<toi>/young-sage-workshop.git. Teste `git ls-remote`, puis branche young-sage/<topic>, push après chaque commit."
5. Old.Sage_20 on VVgBazz: `git remote add young <url>` and fetch with Dawa's own credential.

- UNKNOWN: whether git push through the agent proxy works with this secret. Step 4's `git ls-remote` plus a test push answers it in a minute.
- Options: a separate GitLab user with Developer role would block `main` server-side (INFERENCE, defaults not checked); fewer steps: a GitHub repo, native in cloud sessions (S1).
