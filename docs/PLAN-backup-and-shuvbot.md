# Plan: shuvskills → Skills Manager backup, then shuvbot cutover

2026-10-30 PDT. Follows the shuvdev full cutover (FULL-CUTOVER.md) and today's fleet cleanup (9 connected agents, every skills dir fully managed).

## What the backup actually carries

`skills-manager-cli git init` turns `~/.skills-manager/skills/` (410 skills, 112 MB) into a git repo on branch **`main`** (hard-coded) and commits skill dirs + `.skills-manager/` metadata. On another machine `git clone <url>` restores the library and reindexes the DB from that metadata.

| Travels in the backup | Does NOT travel |
|---|---|
| Skill files, skill ids, tags, source type/ref/branch/subpath for git skills | Direct `skills deploy` links (sync_targets rows) |
| Presets, preset membership, per-preset per-agent toggles | Custom agents (`shared-agents`, `claude-synced-a96c090e`) |
| | Disabled-agent list, source paths of `local`/`import` skills |

Consequence: today's deployments (26 / 22 / 18 / impeccable×6 / manage-skills×3) are all **direct** deploys with no preset, so a bare restore on shuvbot would yield 410 library skills and zero links. Step 1 fixes that by modeling the fleet as presets.

## Step 1 — Model the fleet as presets (shuvdev)

Organization only; no files change. Membership mirrors current `deployed_to` exactly.

| Preset | Members | Deploy target(s) |
|---|---|---|
| `shared-agents` | the 26 skills currently in `~/.agents/skills` | `--agent shared-agents` |
| `claude-code` | the 22 skills in `~/.claude/skills` (incl. `-claude` variants, `tldraw-offline`, `manage-skills`) | `--agent claude_code` |
| `claude-synced` | the 18 skills in `~/.claude/skills/synced/…` | `--agent claude-synced-a96c090e` |
| `all-harnesses` | `impeccable` (generic, `b8e0bef8`) | `--agent codex cursor opencode grok antigravity pi` (shared-agents/claude already have it) |
| `tooling` | `manage-skills` | `--agent codex opencode` (claude_code via `claude-code`) |

Then `presets deploy <p> --agent … --dry-run` on shuvdev must report **0 changed pairs** for every preset — proof the presets reproduce the live layout. The existing empty `Default` preset is left alone.

:::question
How should the fleet layout travel to shuvbot?

The backup carries presets but not direct deploy links. Five presets mirroring today's links make the layout visible in the GUI and restorable with five commands; a script does the same from outside the product.

- [ ] Five presets as in the table above — in the backup, visible in the GUI
- [ ] A `restore-deployments.sh` of `skills deploy <id>` calls kept in the fork's docs — no preset bookkeeping

Recommended: Five presets as in the table above
:::

## Step 2 — Archive shuvskills `main`, replace with the library

Repo: `git@github.com:shuv1337/shuvskills.git` (private, 11 branches, `main` unprotected). Local checkout parked at `~/repos/shuvskills.stop`.

1. Fetch; push `origin/main` to `archive/catalog-2026-10-30` and annotated tag `archive/catalog-2026-10-30` (points at `5e9a9b3`'s lineage on main). Other branches untouched.
2. `gh api` → set default branch to `archive/catalog-2026-10-30`; `git push origin --delete main`.
   Why delete rather than force-push: Skills Manager's `set-remote` sets upstream tracking and later `pull` merges `origin/main`; the old catalog history must not be reachable from the new `main`.
3. shuvdev: `skills-manager-cli git init` → `git set-remote git@github.com:shuv1337/shuvskills.git` → `git push` (creates the new `main`; SSH key already works).
4. `gh api` → default branch back to `main`. Verify `git versions` shows the snapshot and the GUI Backup page shows Connected / shuvdev.
5. Leave the GUI's "Automatic backup" on; write a `BACKUP.md` note in `docs/` with the archive branch name.

:::question
Archive the old shuvskills catalog as `archive/catalog-2026-10-30` (branch + tag), leaving the other 10 feature branches untouched?

`main` gets deleted and recreated from the Skills Manager library. The old history stays reachable from the archive branch and tag; nothing is force-pushed.

- [ ] Yes, archive `main` as-is
- [ ] Merge the open feature branches into the archive first

Recommended: Yes, archive `main` as-is
:::

## Step 3 — Install Skills Manager on shuvbot (macOS 27.2, arm64)

shuvbot has git, gh, cargo (rustc 1.95), node 26, bun, Xcode 27.2β CLT; no Skills Manager, no `~/.skills-manager`.

1. Commit shuvdev's 24 uncommitted fork changes (catalog import, migrations, content-hash fixes) to branch `shuv/cutover-1.43.1` on `shuv1337/skills-manager` and push. shuvbot builds that exact commit so DB migrations and metadata schema match.
2. On shuvbot: clone, `npm ci`, `npm run tauri:build` (produces `Skills Manager.app` + `skills-manager-cli` for aarch64-darwin). Install `.app` to `/Applications`, CLI to `~/.local/bin`; the app publishes `~/.skills-manager/bin/skills-manager-cli` on first launch, which is what the `manage-skills` skill resolves.
3. Launch once (`open -a "Skills Manager"` over SSH works if the desktop session is logged in; otherwise CLI-only until next login).

:::question
Commit the 24 uncommitted fork changes to `shuv/cutover-1.43.1` on `shuv1337/skills-manager` and push, so shuvbot builds the identical code?

Your fork only; no upstream PR. Without this, shuvbot would have to rsync a dirty working tree or run a different build than shuvdev.

- [ ] Yes, commit and push to the fork branch
- [ ] No, rsync the working tree to shuvbot instead

Recommended: Yes, commit and push to the fork branch
:::

## Step 4 — Snapshot and nuke shuvbot's current skill setup

Current state on shuvbot (all unmanaged, mix of copies and symlinks into `~/repos/shuvskills`, `~/repos/clankstudio`, and a `/var/folders/...T/shuvcode/clank-eval/...` temp dir):

- `~/.agents/skills` 32 entries, `~/.claude/skills` 25 + `synced/`, `~/.codex/skills` 1 + `.system/`, `~/.config/opencode/skills` 11, `~/.cursor/skills` 11, `~/.grok/skills` 1, `~/.pi/agent/skills` empty, `~/.hermes/skills` (categorized, Hermes-owned).
- **19 skills exist on shuvbot but not in shuvdev's library:** `cloudflare-one`, `cloudflare-one-migrations`, `diagnosing-bugs`, `grill-me`, `grill-with-docs`, `hyperframes-{animation,audio,cli,core,creative,keyframes,registry,studio}`, `media-use`, `faceless-explainer`, `pr-fanout`, `rex`, `sqim`, `video-brief`. The hyperframes/media-use/faceless-explainer set lives in `~/repos/clankstudio/upstream/hyperframes/skills/`; `video-brief` in `~/repos/clankstudio/skills/`; the rest are plain copies in `~/.agents/skills`.

Procedure:
1. `tar` everything above (following symlinks, so copies survive) to `~/.local/share/skills-manager-cutover/shuvbot-pre-cutover-20261030.tar` on shuvbot and `scp` a copy to shuvdev's cutover evidence dir.
2. Remove the contents of the agent skills dirs listed above **except**: `~/.codex/skills/.system/` (Codex-owned), `~/.claude/skills/synced/<id>/manifest.json` (Claude's sync manifest; we replace the 18 skill dirs beside it, same as shuvdev), and `~/.hermes/skills` (see Decision 5).
3. Move `~/repos/shuvskills` → `~/repos/shuvskills.stop` and leave `~/.local/bin/skills` (old TUI) in place but note it's retired, mirroring shuvdev.

:::question
What happens to the 19 skills that exist only on shuvbot (hyperframes suite, cloudflare-one, grill-me, rex, sqim, pr-fanout, video-brief, …)?

A literal nuke-and-restore drops them (they stay in the tarball). Bringing them into shuvdev's library first means they ride the backup and can be deployed on either machine.

- [ ] Add them to shuvdev's library before the first backup, in a `shuvbot` preset — hyperframes set as git skills, the rest from the tarball
- [ ] Drop them; tarball only
- [ ] Adopt them on shuvbot after the restore and let the next backup carry them back

Recommended: Add them to shuvdev's library before the first backup, in a `shuvbot` preset
:::

:::question
Leave Hermes on shuvbot alone?

Hermes reads `~/.hermes/skills` in a category/skill layout; Skills Manager's `hermes` adapter deploys flat into the same dir. Mixing the two could confuse Hermes' discovery, and Hermes isn't part of the fleet on shuvdev.

- [ ] Leave Hermes alone — don't touch `~/.hermes/skills`, disable the `hermes` adapter
- [ ] Bring Hermes under management too

Recommended: Leave Hermes alone
:::

## Step 5 — Restore on shuvbot

1. `skills-manager-cli git clone git@github.com:shuv1337/shuvskills.git` → library at `~/.skills-manager/skills`, DB reindexed (expect 410 skills, 5 presets).
2. Recreate local agent setup: `agents add-custom shared-agents ~/.agents/skills --name "Shared agents"`, `agents add-custom claude-synced-a96c090e ~/.claude/skills/synced/35a21328-…_82dc40fa-… --name "Claude synced skills"`; `agents disable` whatever non-fleet agents shuvbot auto-detects (same 12-key list as shuvdev, plus any Mac-only extras — reported before disabling).
3. `presets deploy` each of the five presets to the agents in the Step 1 table (dry-run first, then real).
4. Verify: `agents list` shows the same 9 connected; every agent skills dir contains only managed symlinks; counts 26/22/18/+6/+2; `git status` clean, `ahead 0 behind 0`.
5. Auto-backup on shuvbot too; both devices push to the same `main` and Skills Manager merges metadata.

## Review decisions — 2026-10-31 PDT

Plannotator session 2 (annotated, 4/5 answered): presets (Q1 unanswered; implied by Q4), archive `main` as-is, commit+push fork branch, import the 19 shuvbot-only skills into shuvdev's library under a `shuvbot` preset before the first backup, and **remove Hermes from shuvbot entirely** (no longer in use) rather than leaving it alone.

## Executed — 2026-10-31 PDT

- **Presets** (shuvdev): `shared-agents` 27, `claude-code` 23, `claude-synced` 18, `all-harnesses` 1, `tooling` 1, `shuvbot` 20. Dry-run deploys reported 0 changed pairs against the live layout. The counts grew by one because Plannotator 0.28.40's self-update had replaced the managed `plannotator*` links with real dirs and added `plannotator-inbox`; the library entries were refreshed in place via `skills update` (ids kept), `plannotator-inbox` and `plannotator-inbox-claude` added, and the links redeployed. Papercut `pc_f85cc3214570`.
- **19 shuvbot-only skills** imported into shuvdev's library: `hyperframes-cli`, `hyperframes-registry`, `media-use` as git skills from `heygen-com/hyperframes@main`; the 7 retired workflow skills (`hyperframes-{animation,audio,core,creative,keyframes,studio}`, `faceless-explainer`) and the 9 plain copies as `local` snapshots from `~/.local/share/skills-manager-cutover/shuvbot-sources/` (hyperframes source commit `38cfcdd`, see `HYPERFRAMES-SOURCE-COMMIT.txt`). Upstream removed those 7 on 2026-10-21 (#5651).
- **shuvskills**: old `main` (`8563a31`) archived as branch + tag `archive/catalog-2026-10-30`; `main` deleted and recreated from the library (`493e495 Initial skill library snapshot`, 4,790 files, 16.7 MB pack); default branch back to `main`. Local catalog checkouts parked at `~/repos/shuvskills.stop` on both machines.
- **Fork**: `shuv/cutover-1.43.1` @ `10e990f` pushed to `shuv1337/skills-manager`; shuvbot built that commit (`npm run tauri:build`; updater `.tar.gz` signing fails without `TAURI_SIGNING_PRIVATE_KEY`, the `.app` is fine). Installed `/Applications/Skills Manager.app` + `~/.local/bin/skills-manager-cli`; the app published `~/.skills-manager/bin/skills-manager-cli` 1.43.1.
- **shuvbot nuke**: snapshot `~/.local/share/skills-manager-cutover/shuvbot-pre-cutover-20261031.tar` (5,021 entries, symlinks dereferenced; copy on shuvdev). Removed all agent skills dirs' contents (kept `~/.codex/skills/.system`, Claude's sync `manifest.json`), `~/.hermes` entirely (per review: Hermes no longer in use), and 31 stub dotdirs (`stub-dotdirs-removed-20261031.txt`).
- **shuvbot restore**: `git clone` → 431 skills, 7 presets; custom agents `shared-agents` and `claude-synced-a96c090e` re-added; presets deployed → 47 / 23 / 18 / codex 2 / opencode 2 / cursor 1 / pi 1. Connected agents: 7 (claude_code, codex, cursor, opencode, pi + 2 custom; grok/antigravity are not installed on shuvbot). Every skills-dir entry is a managed symlink. Both devices: `main`, clean, ahead 0 / behind 0.

## Follow-up — git sources for tool-written skills (2026-10-31 PDT)

Library entries that an external tool can overwrite were re-pointed at their upstream repos with `skills set-source` (ids, tags, preset membership and deployments kept). 25 entries, all now `source_type: git`; `skills check` reports `up_to_date`.

| Entries | Source | Why |
|---|---|---|
| `plannotator`, `-annotate`, `-inbox`, `-last`, `-review` | `backnotprop/plannotator` `apps/skills/core/<n>` | Plannotator's self-update rewrites agent skill dirs |
| `plannotator-{annotate,inbox,last,review}-claude` | … `apps/skills/claude/<n>` | |
| `plannotator-compound`, `-setup-goal`, `-visual-explainer` | … `apps/skills/extra/<n>` | |
| `shuvgeist` | `shuv1337/shuvgeist` `skills/shuvgeist` | CLI auto-installs into `~/.agents/skills` on every run; cpSync refuses to replace the managed symlink and the per-run path swallows the error, so the link wins. Re-added to `shared-agents`. |
| `impeccable`, `impeccable-claude` | `pbakaus/impeccable` `.agents/skills/impeccable` / `.claude/skills/impeccable` | `npx impeccable` writes agent dirs. Dropped the 22 MB `scripts/bin/linux-x64/impeccable` cache; the launcher re-fetches (checksummed). |
| `visual-explainer` | `nicobailon/visual-explainer` `plugins/visual-explainer` | was tracked by `npx skills` lock |
| `playwriter`, `quota-axi`, `anatomy`, `anatomy-claude` | `remorses/playwriter`, `kunchenguid/quota-axi`, `wheresryan22/anatomy` (root) | clear upstreams |
| `diagnosing-bugs`, `grill-me`, `grill-with-docs` | `mattpocock/skills` | sources recovered from shuvbot's `.skill-lock.json` |
| `cloudflare-one`, `cloudflare-one-migrations` | `cloudflare/skills` | same |

All content diffs were upstream-newer; pre-change library copies are in `~/.local/share/skills-manager-cutover/pre-setsource-20261031.tar`. `~/.agents/.skill-lock.json` retired on both hosts (shuvbot's copy kept at `skill-lock-retired-20261031.json`) so `npx skills` no longer claims them.

Left `local` on purpose: `open-edit` (library copy carries `hooks/` + `scripts/preflight.sh` that upstream `veedstudio/open-edit` no longer ships), the 7 retired hyperframes workflow skills, and the shuvskills-authored skills (`discord-release-notify`, `shuv-fork-it`, `using-exe-dev`, `androidtv-control`, `clankstudio-*`, `wtfm`, `executor-v2`, `shark`, `tldraw-offline`), whose canonical home is now this backup. Claude's `synced/` folder is Claude-owned and was not touched.

## Rollback

- shuvskills: `archive/catalog-2026-10-30` branch + tag; restoring old `main` is `git push origin archive/catalog-2026-10-30:main` + default-branch flip.
- shuvdev library: `git init` is local; `rm -rf ~/.skills-manager/skills/.git` returns to the pre-backup state (DB untouched).
- shuvbot: the pre-cutover tarball restores every dir byte-for-byte; `~/repos/shuvskills.stop` rename is reversible.

## Out of scope

- Upstream PRs of any kind.
- `dock` or any machine other than shuvdev/shuvbot.
- Re-pointing `local`/`import` library skills at git sources (separate cleanup).
