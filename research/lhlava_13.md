# Lhlava_13 lens: scripts, tests, CI for the two sages

Research only; no vote. Clock: `date -u` 2026-11-02T08:14Z.

## Sources
- S1 platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices · 2026-11-02 · scripts for deterministic work, validate→fix loops, verbose errors, "at least three evaluations" (VERIFIED).
- S2 code.claude.com/docs/en/plugin-evals.md · 2026-11-02 · `claude plugin eval`: skills-directory plugins OK; graders regex/tool_used/tool_order/file_exists/llm/baseline; "no custom-code graders"; exit 0/1/2 (VERIFIED).
- S3 code.claude.com/docs/en/claude-code-on-the-web · 2026-11-02 · hosted cloning/push "require GitHub"; GitLab bundle "can't push results back" (VERIFIED).
- S4 code.claude.com/docs/en/cloud-environments.md · 2026-11-02 · rustc/cargo preinstalled; gitlab.com, crates.io allowlisted (VERIFIED).
- S5 this cloud sandbox · 2026-11-02 · cargo 1.97.0, rustfmt 1.9.0, clippy 0.1.97, Python 3.13 present (VERIFIED, ran `--version`).
- S6 docs.gitlab.com auto_merge and job_rules (snippets only; fetch blocked) · 2026-11-02 · "Pipelines must succeed" needs a successful pipeline to exist; `=~ /^young-sage\//` rule form (INFERENCE until the pages are read).

## 1. Old.Sage_20 (dndevs-old-sage)
- `scripts/rt_check.py FILE...`: stdlib-only validator. Checks: first line `RoundTable/v1`, required headers, Message-ID = file name, no leftover `<...>`, SmartStamp block, `Document:` = path, last line `Signature:`. One error per line, exit 1. INFERENCE: share one rules file with the real audit so they cannot drift. UNKNOWN: what the audit's 14 checks actually are.
- `scripts/rt_new.py --kind HANDOFF --to ...`: builds the skeleton. Message-ID from `secrets.token_hex(5)`, real clock, Dawa's words as an argument. It creates the file with `O_EXCL` (never overwrites), then runs `rt_check.py` on it.
- `scripts/handoff.py BASE`: builds the paste-ready HANDOFF from git facts (`rev-parse`, `log BASE..HEAD`, `diff --stat`, `format-patch --stdout | sha256sum`), so hashes and commit lists are never typed by hand.
- `tests/test_rt_check.py` (unittest): good and bad fixtures, including copies of Young.Sage_02's two rejected files (Dawa OK first). Also `roll.py` and `sage_lookup.py` on a fixture DB.
- `evals/` (S2): about 3 cases per behaviour. Graders: `tool_used: Skill`, `tool_used` Bash `input_match: rt_check`, and `regex` on the validator output written to a file (no code graders). Add a negative case: an unrelated prompt with `min: 0, max: 0, arm: both`.

## 2. Young.Sage_02
- Bundle the same `rt_check.py` and `rt_new.py` (byte-identical, sha256 noted) and make "run rt_check before any inbox write" a required step.
- `scripts/check_rust.sh`: runs fmt, clippy `-D warnings` and `test --locked`, and records each command with its exit code. If cargo is missing it prints `UNVERIFIED (no cargo)` and never `TESTED`. In Claude Code cloud sessions cargo is present (S4, S5). Claude.ai chat is a different sandbox: whether it has cargo is UNKNOWN; S1 says it can install from PyPI/npm, so keep the scripts stdlib-only.
- Fix the templates: template 2 begins with `# UPGRADE PROPOSAL`, and template 4 has `Kind: HANDOFF (or NOTE…)`. Both would probably fail a strict first-line check (INFERENCE).
- Evals: `tool_used` Bash with `input_match: "push.*main"` and `max: 0`, which asserts it never pushes to main.

## 3. GitLab CI
- Blocker (S3, VERIFIED): an Anthropic-hosted cloud session cannot push to GitLab. Options: Dawa pushes the mbox; a GitLab token as a network secret (whether git push works that way is UNKNOWN); or a self-hosted environment.
- `.gitlab-ci.yml`: `workflow:rules` set to `merge_request_event` to avoid duplicate pipelines; a pinned `rust:<x.y>` image; jobs `fmt` (`cargo fmt --all -- --check`), `clippy` (`--all-targets -- -D warnings`) and `test` (`--locked`); no `allow_failure` anywhere. Add a `roundtable-lint` job running `rt_check.py` on changed `memory/roundtable/**` files, if the repo holds the RoundTable at all (UNKNOWN).
- Fail closed: run the jobs on every MR into main, not only on `young-sage/*` ones. In project settings turn on "Pipelines must succeed", leave "Skipped pipelines are considered successful" off, and make main protected with no direct pushes (S6, INFERENCE).
- Optional: a second job running `claude plugin eval . --trust-plugin --threshold 0.8 --max-cost-usd N`, only on MRs that touch `skills/`. Costs API money (S2).
