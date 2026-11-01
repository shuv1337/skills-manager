# Skills Manager cutover and optional categories

Current status: full cutover subsequently authorized by the user; see [FULL-CUTOVER.md](FULL-CUTOVER.md). The gates below document the earlier stages.
Status: implementation and sandbox rehearsal approved in Plannotator; live cutover remains unapproved. See CUTOVER-REHEARSAL.md for current limits. 2026-10-29 PDT. Source inspected at d523742.
Approval of this document authorizes implementation and sandbox rehearsal only; live cutover requires a subsequent review of the exact target manifest and rehearsal evidence.

## Recommendation

Make Skills Manager the deployment owner and managed library. Retain the existing catalog as a versioned source and rollback asset. Start by mapping optional categories to existing tags; do not require a new directory layout or database category system before migrating.

The catalog need not remain the permanent authoring system. Initially, avoid simultaneous editing in both places: treat imported library copies as managed artifacts and make source edits in the catalog. After migration stabilizes, decide explicitly whether individual skills become locally authored library entries or track Git sources.

## Verified facts and limits

- Existing skill_tags storage supports multiple tags, global rename/delete, and per-skill assignment (core/migrations.rs and core/skill_store.rs). BatchTagDialog, MySkills, AddSkillsSheet, SkillPickerRow and SkillDetailPanel already expose tags. CLI has tag operations.
- Local installation copies directories. PreparedSource accepts a directory without requiring SKILL.md, and archive discovery can fall back to the archive root (core/installer.rs). The prior spike's category-as-one-skill result is consistent with this code.
- CLI SetSource can preserve identity, tags, presets and deployments while assigning a Git URL and subpath. Actual update behavior must still be rehearsed for this catalog.
- The old tool supports categorized roots, explicit category on single-skill roots, and separate flat versus category-shaped target layouts. Its example config is NOT evidence of current live configuration.
- Live ~/.agents/skills and ~/.claude/skills currently are distinct directories; ~/.skills-manager is absent. The assumed ~/.config/skills-tui/config.yaml does not exist. Resolve effective config before selecting targets.
- The earlier sandbox spike reported safe refusal of conflicting links, 21 adopted skills, and a bogus devops import. Those counts are historical, not a migration inventory. Do not reuse that sandbox as the migration seed.
- Existing package-lock.json modification is unrelated and must remain untouched.

## Optional categories: smallest useful implementation

Interpret category as optional organization, not permission to enable a skill. Import the old top-level category as a normal tag; unclassified skills remain untagged. Existing tags remain intact. Presets continue to represent deployment selections, independently of categories.

Build a bounded catalog import operation with preview and apply modes:

1. Read explicitly selected catalog roots and optional category hints. Discover actual SKILL.md packages using the old catalog rules: stop descending once a package is found, omit generated/vendor trees and excluded entries, handle category-level packages and nested packages. Do not scan the whole home directory.
2. Preview source path, canonical source identity, category/tag, proposed library name, existing match/conflict, and intended deployment targets (initially none). Report duplicate basenames and aliases for explicit resolution; never silently merge them.
3. Import only selected valid packages through existing installation services. Preserve supporting files, executable bits and safe package links; detect external link dependencies and resolve their policy before migration. Record import mapping so rerunning does not create duplicates or retag unrelated entries.
4. Add category tag without replacing unrelated tags. For future refresh, preserve manual tag changes; category inference is an import-time default rather than an automatic retagging policy.
5. Expose the preview in the install flow and CLI. Reuse current tag controls for organization; document that categories are tags for this first version.

Likely touchpoints: core/installer.rs (package validation), a small core catalog-import service, commands/skills.rs and CLI bindings, shared API types, InstallSkills view, locales and focused fixtures. Tag storage needs no migration. Investigate existing source deduplication before adding new persistent mapping fields.

If a dedicated single category becomes valuable later: add nullable category metadata with an additive migration, Uncategorized filter, bulk assignment and independent tag filtering. Keep physical library and target paths flat. Hierarchical categories, category descriptions and automatic folder moves are deferred. Category-shaped harness targets are a separate compatibility concern, not implied by category tags.

Relative effort: tag mapping and import preview are small-to-medium; a distinct category feature is medium across database/API/UI/CLI; target compatibility and safe migration are the larger work. Estimate calendar time after the inventory and importer spike.

## Staged cutover

### 1. Inventory and freeze scope

Resolve actual old-tool config, environment overrides, catalog roots, exclusions and every target root. Record root aliases, per-entry lstat type, exact symlink text, resolved destination, complete package hashes and modes, enabled matrix, source revision/dirty content, and external dependencies. Include project-local roots only if selected. Identify MCP servers, materializers, timers and hooks that may still depend on the old tool; replacing its UI does not authorize deleting those integrations.

Map each live target to an explicit Skills Manager target. Shared agents is a required capability gate: verify custom-target support and alias deduplication or implement a narrow adapter before cutover. Verify category-shaped targets separately if inventory finds any. Never substitute per-harness defaults for the shared folder implicitly.

### 2. Build and rehearse in fresh isolation

Use a disk-backed sandbox under ~/.cache/agent-ws. Redirect HOME and all effective XDG locations, explicitly configure tool paths, and assert every write target stays inside the sandbox. Copy live state without following symlinks for rollback fixtures; create separate redirected source fixtures for mutation tests so rehearsal cannot write through to live content.

Implement package validation and category import. Import selected packages directly from canonical sources, not adopted deployment links. Verify files, modes, category tags, exclusions, collisions, repeat import, and intended per-target enabled sets. Keep auto-update, auto-sync and background backup writers disabled during rehearsal.

For Git-backed skills, test SetSource with the actual branch/subpath contract: unchanged content, changed upstream, local modifications, unavailable remote, and persistence of tags/presets. Local-only skills must be labeled as such; never promise source tracking without demonstrated behavior.

### 3. Backup and restore rehearsal — required gate

Produce a timestamped manifest and non-dereferencing backup of the selected live roots, old-tool config, source content including uncommitted changes, and preexisting Skills Manager state if any. Capture absence as well as presence. Preserve permissions and literal link targets. Include package dependencies outside catalog roots when required. Back up SQLite consistently with writers stopped or SQLite backup support; do not copy an active database alone.

Restore into a second sandbox, compare hashes/modes/link texts/enabled matrix, and exercise both an interrupted partial cutover and rollback after a library edit. A backup that merely exists is not acceptance.

### 4. Live canary — separate approval

Present the exact manifest: target and skill counts, exclusions, unresolved conflicts (must be zero), source mappings, backup location, command sequence and verified restore results. Pause only identified competing skill writers, retaining their previous states. Do not stop active agents indiscriminately.

Import the library without deployment first; verify content. Select a low-risk skill/target pair. Move its exact old entry into the backup area, then deploy its verified library copy using existing conflict checks. Do not force-overwrite real directories or unrelated links. Record completion after every pair so a failure can restore the moved entry. Open a fresh harness session and demonstrate actual discovery and loading of the expected skill content, not just a filesystem check.

### 5. Expand and observe

After canary success, migrate the remaining approved pairs in bounded batches. Verify exact enabled sets, no duplicate discovery, content parity, usable assets/scripts, tags and source-update behavior. Run real smoke checks in every affected harness. Preserve the old executable/config/catalog and backups through at least one normal work cycle and one successful update cycle. Keep old deployment writers disabled while Skills Manager owns the roots.

## Rollback runbook

Trigger on missing/duplicate skills, wrong enabled matrix, broken assets, unexpected writes, update-source failures or user request.

1. Stop Skills Manager and its identified background writers. Preserve current library and metadata first so post-cutover edits are not lost.
2. For each completed or partially completed manifest entry, remove only the new link whose destination still matches the recorded Skills Manager deployment. If it changed independently, stop on that entry and reconcile; do not delete unknown files.
3. Restore the original entry with exact link text/type/mode, or restore absence if no entry existed. Restore root aliases and scoped configuration as recorded. Avoid replacing entire agent directories containing unrelated new work.
4. Restore prior manager metadata only with writers stopped; if originally absent, archive the newly created state outside active discovery rather than deleting it. Restore old writer states and resume the old TUI.
5. Compare the manifest and run fresh-session discovery/load checks in each affected harness. Preserve all new content for reconciliation into the catalog. Report any unresolved differences; rollback is not complete until acceptance passes.

## Required tests and exit criteria

Package-with-assets import; invalid category directory rejection; category-level/nested discovery; exclusions; duplicate names; repeat imports; tags preserved through updates; flat/shared/category-shaped target cases actually in scope; conflict refusal; failure between moving an old link and deploying its replacement; full restore equality; real harness loading. Run relevant Rust and UI tests plus build/lint for touched surfaces. Do not infer live success from sandbox tests.

Decision requested: approve the existing-tags approach and implementation/rehearsal scope. Dedicated categories remain an optional follow-up. No live mutation is authorized by this plan approval.
