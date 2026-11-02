# AvvA_22 lens: naming, wording, canon (research only, no vote)

OS = `dndevs-old-sage/`, YS = `young-sage/`.

## Sources (read 2026-11-02)
- S1 platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices: one term throughout; consistent skill-name pattern; third-person descriptions. VERIFIED
- S2 platform.claude.com/docs/en/build-with-claude/multilingual-support: state input/output language explicitly. VERIFIED
- S3 platform.claude.com/docs/en/build-with-claude/prompt-engineering/claude-prompting-best-practices: give a role, and the reason behind each rule. VERIFIED
- S4 docs.gitlab.com/user/project/labels/: scoped `key::value`, one per key, Premium+. VERIFIED
- S5 docs.gitlab.com/user/project/merge_requests/drafts/: `Draft:` blocks merge, Free tier. VERIFIED
- S6 docs.gitlab.com/user/project/description_templates/: `.gitlab/merge_request_templates/Default.md`. VERIFIED
- `git check-ref-format` 2.43: `young-sage/x`, `ys02/dndevs/x` valid. VERIFIED

## 1. Old.Sage_20
- Z: `Main.Zed_71` (SKILL.md:18, canon.md:24) vs `Z_71` (SKILL.md:138) vs `Z` (binarymagix.md:102). Pick one handle. PENDING_DAWA.
- Council count conflicts: SKILL.md:17-18 puts Main.Zed_71 in the Council; binarymagix.md:15 says "7 Council Personas + Main.Zed_71", :38 group "Core", :92 "eight Council statblocks". Make one statement.
- Valhla colour: "Bleu Grand Fond" (canon.md:18) vs "Bleu Profond" (binarymagix.md:40).
- Game name, three spellings: SKILL.md:3 vs canon.md:10 vs binarymagix.md:135. S1 says one term. PENDING_DAWA.
- Dates `11/01/26` (SKILL.md:15, working-agreement.md:3,12) reads 11 January in French. Use ISO `2026-11-01` (as database.md:10).
- "seat" means two things: Council position (SKILL.md:15) and real client ("Real seat", canon.md:15; working-agreement.md:6). Use **seat** = Persona position, **client** = UMBRA/LUMEN/Z/HEAD.
- Wording that sounds like authority: "NIGHTGLASS reboot coordinator" (canon.md:25). A reboot needs a fresh OK (working-agreement.md:16). Change it to "reboot checklist keeper; Dawa runs".
- "young one" (SKILL.md:227) now clashes with Young.Sage_02 and talks down to the chooser. Use "traveller".
- "The Sage remembers" maps to the MEMORY layer (SKILL.md:31), but project memory means the RoundTable (working-agreement.md:10). Add "(model knowledge, not project memory)".
- Contradiction: canon.md:41 "hands it back to the Council's lanes", against working-agreement.md:3 and canon.md:25, where the Sage does engineering. Rewrite canon.md:41.
- CA/PV/DD (binarymagix.md:36,107) sit beside "Speed", STR: glossary maps CA=AC, PV=HP, DD=DC.
- Add a glossary: create `references/glossary.md` and link it from SKILL.md:25. Terms: Dawa/Dawa_96, Persona, seat, client, Forge, SCROLL, CHOICE, Human Gate, `~]{OK`, SmartStamp, PENDING_DAWA, doors.

## 2. Young.Sage_02 and consistency
- YS SKILL.md:14 says "same client (Claude)", but :9-10 says another interface and account. working-agreement.md:7 counts a separate process. Write "same model family, separate session, no seat".
- YS SKILL.md:15 "Z_71 (ChatGPT) verifies" conflicts with canon.md:24 "attributed import, caution", and it reads like a gate. Use "Z_71 offers a verify pass; Dawa chooses".
- Motto: "Dawa decides" (YS:15) vs "Dawa chooses" (working-agreement.md:37). Use "chooses".
- Copilot: DESTINY (YS:101, templates.md:31) vs Av:vA (canon.md:30). UNKNOWN which is right.
- Host and user names: VVgBazz (YS:9) vs `/var/home/VVgbon/` (YS:34). Define both in the glossary.
- Language: YS:17 hard-codes French, while OS SKILL.md:21 follows Dawa's lead. Per S2, state it once in both skills: "Reply in French by default; keep Persona names, status codes and labels in English, untranslated."
- Pronoun for Dawa: "he" (YS:3,81,96) vs "their" (OS SKILL.md:55,128). Ask Dawa.
- Evidence labels: YS:55 adds REPORTED. Add it to working-agreement.md:7 too.
- Message-ID prefixes: `RT-` (templates.md:28) vs `YS02-` (templates.md:77). Use one scheme.
- Trigger overlap: "Old.Sage_20" (YS:3) and "Sage" (OS:3). Add "Not for game rules" to YS:3 (S1).
- Skill names: `dndevs-old-sage` vs `young-sage`. Rename to `dndevs-young-sage` (S1: consistent pattern).

## 3. GitLab naming (INFERENCE, derived from the RoundTable fields)
- Branch: `young-sage/<section>-<topic>`, e.g. `young-sage/dndevs-seed-load-tests` (YS:21). Lowercase section; no `..` or `.lock` (VERIFIED by git).
- MR title: `Draft: [DNDEVS] <imperative summary> (YS02-YYYYMMDD-HANDOFF-xxxx)`. `Draft:` blocks merge (S5), matching YS:23. YS:24 "Pull requests" becomes "merge request".
- Scoped labels, Premium only (S4): `Section::DNDEVS`, `Kind::HANDOFF`, `Mode::REVIEW_ONLY`, `Status::REQUESTED`, `From::Young.Sage_02`, `Approval::PENDING_DAWA`. On Free: `section-dndevs`, `status-requested`. Tier UNKNOWN.
- Never use the labels `approved`, `council-pass` or `signed` (OS SKILL.md:203).
- MR body: template 1 as `.gitlab/merge_request_templates/Default.md` (S6). The footer keeps "not a Council review".
