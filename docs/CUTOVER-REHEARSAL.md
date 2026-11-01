# Skills Manager readiness checkpoint

Current status: full cutover subsequently authorized by the user; see [FULL-CUTOVER.md](FULL-CUTOVER.md). The gates below document the earlier stages.
2026-10-29 PDT. **One-entry live canary approved and deployed successfully; batch expansion remains unapproved.** The reviewer accepted proceeding with the Shuvcode limitation; disable its existing catalog plugin only if it causes issues. Changes are uncommitted. The approved Shared agents `discord-release-notify` link now points to the new manager library. Other live target entries, installed binaries and source repositories are unchanged.

## Implemented and verified

- Catalog preview/import in UI and CLI maps optional categories to existing tags. Imports require explicit source selection and do not deploy skills.
- One package copy policy now serves installation and copy-mode deployment: preserve files, executable/directory modes, internal relative links and nested Git fixture repositories; omit only the package root's own `.git`. Absolute, external and dangling links fail before replacement.
- Git cache-to-checkout copying preserves symlinks. A separate installed-content baseline prevents refresh from overwriting local modifications, including same-path edits after a new CLI process reindexes the library. Tags and preset membership survive updates.
- CLI `--target-name` preserves original harness folder names for separate library variants. Five same-named shared/Claude variants remain distinct.
- The `executor-v2` migration snapshot materializes its 11 supporting-file dependencies. Original paths, hashes and source backups remain recorded; this is an explicit local snapshot, not automatic external-link following.

## Exact rehearsal scope

32 distinct library snapshots represent **47 existing top-level deployments: 26 Shared agents and 21 Claude Code**. All source identities, source revisions/dirty status, original target types/link text, snapshot paths, categories and dependencies are in the source map below.

Claude's nested `synced/` collection, hidden trash and backup files remain in place. Other catalog packages are not enabled by this migration. Existing source repositories and read-only MCP integrations remain available. No service was stopped. The consumer inventory found configuration references but no running consumer in its bounded scan; refresh this inventory immediately before any live action.

## Evidence

Evidence root: `/home/shuv/.cache/agent-ws/skills-readiness-20261029/`.

| Check | Result and limit |
| --- | --- |
| Full package comparison | All 32 imported snapshots match, including modes, internal links and `wtfm` nested Git fixtures. |
| Target matrix | All 47 original target paths deployed in isolation; variants and shared-folder aliases checked. |
| Backup/restore | Both target roots restored independently with exact hashes, modes and literal link text. |
| Failure and rollback | Conflict refusal, interruption between old-entry move and deployment, full 47-path rollback and retention of new library edits passed. |
| Git updates | Branch/subpath fixture: unchanged and changed upstream, unavailable remote, local edits and tag/preset retention passed. This proves update mechanics; production entries are still local snapshots, not enabled remote tracking. |
| Local refresh | Fixtures, links, local-edit refusal and tag/preset retention passed. |
| Codex | Real CLI/provider discovered a temporary deployed proof skill and returned its random proof value. |
| Claude | Real CLI loaded `wtfm`, `executor-v2` and `anatomy` bodies; read an executor support file. Used a deterministic local endpoint because provider quota was exhausted: no claim of provider inference. Nested synced commands remained discoverable. |
| Shuvpi | Runtime `get_commands` discovered all 26 Shared agents skills; no provider-backed inference claimed. |
| Shuvcode | Installed CLI skill-list probes return an empty list, including explicit sandbox paths. Reviewer accepted this as non-blocking. Keep the catalog plugin enabled; disable it only if it causes issues. This remains a limit on evidence, not a successful native discovery test. |
| Rust | 631 passed, 7 ignored; host Git configuration isolated. CLI: 7 passed. |
| Frontend | Earlier importer checks: 9 tests, production build and lint passed. GUI visual acceptance remains outstanding. |
| Candidate build | Release GUI and CLI built. Copies and SHA-256 values retained under `candidate/` and `handoff-verification.json`; installed executables untouched. |
| Final read-only comparison | Both live target roots still match rehearsal backups; all 32 sources match source backups; live `~/.skills-manager` remains absent. |

Relevant evidence files:

- `source-map.json`: exact source/target mapping and dependency provenance.
- `rehearsal-latest.json`: full matrix, parity and rollback results.
- `update-latest.json`: fresh-process Git/local update results.
- `consumers.json`: bounded consumer inventory.
- `handoff-verification.json`: final live/source equality and candidate hashes.
- `rehearsal-1793305644213437773/backup/` and `restore/`: non-dereferencing target backups and independent restore.
- `source-backups/`: original package contents; `sources/`: self-contained import snapshots.
- `rehearsal-1793305644213437773/claude-loader-results.json`, `codex-smoke-result.json`, `shuvpi-commands.jsonl` and `shuvcode-isolated-skills.json`: harness evidence and limits.

## Remaining gate

1. Shuvcode disposition accepted in review: proceed with the existing catalog plugin retained; disable only if it causes issues. Do not call the empty API result a pass.
2. Visually verify the import UI. Claude provider inference was unavailable; its local-endpoint loader test must remain clearly distinguished.
3. The one-entry CLI canary is now packaged and rehearsed; see `CANARY-APPROVAL.md`. Live preflight passed without writes. Refresh competing-writer checks immediately before execution. No live canary has been executed or approved.
4. Present that final manifest in Plannotator for the separate live approval required by `PLAN-live-cutover.md`.

This checkpoint is implementation and test evidence for review. Approval of this checkpoint or code review does not authorize live migration.

## Source/target manifest

| Library entry | Category | Source | Target folders |
| --- | --- | --- | --- |
| anatomy | — | `/home/shuv/.agents/skills/anatomy` | `/home/shuv/.agents/skills/anatomy` |
| anatomy-claude | — | `/home/shuv/.claude/skills/anatomy` | `/home/shuv/.claude/skills/anatomy` |
| androidtv-control | smart-home | `/home/shuv/repos/shuvskills/skills/smart-home/androidtv-control` | `/home/shuv/.agents/skills/androidtv-control`, `/home/shuv/.claude/skills/androidtv-control` |
| clankstudio | — | `/home/shuv/repos/clankstudio/skills/clankstudio` | `/home/shuv/.agents/skills/clankstudio`, `/home/shuv/.claude/skills/clankstudio` |
| clankstudio-anatomy | — | `/home/shuv/repos/clankstudio/skills/clankstudio-anatomy` | `/home/shuv/.agents/skills/clankstudio-anatomy`, `/home/shuv/.claude/skills/clankstudio-anatomy` |
| clankstudio-fframes | — | `/home/shuv/repos/clankstudio/skills/clankstudio-fframes` | `/home/shuv/.agents/skills/clankstudio-fframes`, `/home/shuv/.claude/skills/clankstudio-fframes` |
| clankstudio-hyperframes | — | `/home/shuv/repos/clankstudio/skills/clankstudio-hyperframes` | `/home/shuv/.agents/skills/clankstudio-hyperframes`, `/home/shuv/.claude/skills/clankstudio-hyperframes` |
| clankstudio-pipelines | — | `/home/shuv/repos/clankstudio/skills/clankstudio-pipelines` | `/home/shuv/.agents/skills/clankstudio-pipelines`, `/home/shuv/.claude/skills/clankstudio-pipelines` |
| clankstudio-psychopomp | — | `/home/shuv/repos/clankstudio/skills/clankstudio-psychopomp` | `/home/shuv/.agents/skills/clankstudio-psychopomp`, `/home/shuv/.claude/skills/clankstudio-psychopomp` |
| discord-release-notify | devops | `/home/shuv/repos/shuvskills/skills/devops/discord-release-notify` | `/home/shuv/.agents/skills/discord-release-notify`, `/home/shuv/.claude/skills/discord-release-notify` |
| executor-v2 | — | `/home/shuv/.agents/skills/executor-v2` | `/home/shuv/.agents/skills/executor-v2`, `/home/shuv/.claude/skills/executor-v2` |
| impeccable | — | `/home/shuv/.agents/skills/impeccable` | `/home/shuv/.agents/skills/impeccable` |
| impeccable-claude | — | `/home/shuv/.claude/skills/impeccable` | `/home/shuv/.claude/skills/impeccable` |
| open-edit | — | `/home/shuv/.agents/skills/open-edit` | `/home/shuv/.agents/skills/open-edit` |
| plannotator | — | `/home/shuv/.agents/skills/plannotator` | `/home/shuv/.agents/skills/plannotator`, `/home/shuv/.claude/skills/plannotator` |
| plannotator-annotate | — | `/home/shuv/.agents/skills/plannotator-annotate` | `/home/shuv/.agents/skills/plannotator-annotate` |
| plannotator-annotate-claude | — | `/home/shuv/.claude/skills/plannotator-annotate` | `/home/shuv/.claude/skills/plannotator-annotate` |
| plannotator-compound | — | `/home/shuv/.agents/skills/plannotator-compound` | `/home/shuv/.agents/skills/plannotator-compound` |
| plannotator-last | — | `/home/shuv/.agents/skills/plannotator-last` | `/home/shuv/.agents/skills/plannotator-last` |
| plannotator-last-claude | — | `/home/shuv/.claude/skills/plannotator-last` | `/home/shuv/.claude/skills/plannotator-last` |
| plannotator-review | — | `/home/shuv/.agents/skills/plannotator-review` | `/home/shuv/.agents/skills/plannotator-review` |
| plannotator-review-claude | — | `/home/shuv/.claude/skills/plannotator-review` | `/home/shuv/.claude/skills/plannotator-review` |
| plannotator-setup-goal | — | `/home/shuv/.agents/skills/plannotator-setup-goal` | `/home/shuv/.agents/skills/plannotator-setup-goal` |
| plannotator-visual-explainer | — | `/home/shuv/.agents/skills/plannotator-visual-explainer` | `/home/shuv/.agents/skills/plannotator-visual-explainer` |
| playwriter | — | `/home/shuv/.agents/skills/playwriter` | `/home/shuv/.agents/skills/playwriter`, `/home/shuv/.claude/skills/playwriter` |
| quota-axi | — | `/home/shuv/.agents/skills/quota-axi` | `/home/shuv/.agents/skills/quota-axi` |
| shark | — | `/home/shuv/.agents/skills/shark` | `/home/shuv/.agents/skills/shark`, `/home/shuv/.claude/skills/shark` |
| shuv-fork-it | software-development | `/home/shuv/repos/shuvskills/skills/software-development/shuv-fork-it` | `/home/shuv/.agents/skills/shuv-fork-it`, `/home/shuv/.claude/skills/shuv-fork-it` |
| tldraw-offline | — | `/home/shuv/.claude/skills/tldraw-offline` | `/home/shuv/.claude/skills/tldraw-offline` |
| using-exe-dev | devops | `/home/shuv/repos/shuvskills/skills/devops/using-exe-dev` | `/home/shuv/.agents/skills/using-exe-dev`, `/home/shuv/.claude/skills/using-exe-dev` |
| visual-explainer | — | `/home/shuv/.agents/skills/visual-explainer` | `/home/shuv/.agents/skills/visual-explainer` |
| wtfm | software-development | `/home/shuv/repos/shuvskills/skills/software-development/wtfm` | `/home/shuv/.agents/skills/wtfm`, `/home/shuv/.claude/skills/wtfm` |

## Live canary update

2026-10-29 01:52 PM PDT: the separately approved one-entry canary passed live package parity and fresh Codex discovery/loading acceptance. Only the selected Shared agents link changed; Claude remained identical. See `CANARY-APPROVAL.md` for the exact runner, backup, evidence and rollback command. The original absence of manager state recorded above describes the pre-canary snapshot; live manager state now contains the single approved package. Leave deployed for observation; no further target pairs authorized.
