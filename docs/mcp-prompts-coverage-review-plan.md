# MCP prompts: operation-coverage review and README documentation

## Context

The guided-workflow MCP prompts feature (`docs/mcp-prompts-workflow-plan.md`) shipped in `v0.7.0`
with a master `bamboo_workflow` prompt plus nine domain sub-workflows. The user has now used it
and asked for two follow-ups: (1) review the full 404-operation catalog against those nine
sub-workflows to see whether any domain is under-covered or missing a workflow entirely, staying
agnostic to the fact that this crate's `call`/`search`/`get` tools front whichever API version is
configured (currently only `default`, but the design must not assume that stays true — no prompt
may hardcode an `operationId` or assume a response shape, exactly as `docs/mcp-prompts-workflow-plan.md`
already established); and (2) document the prompts feature in `README.md`, which currently has no
mention of it at all.

## Coverage review

Grouping all 404 operations by their real top-level path segment (verified directly against the
decompressed `mcp_store.db.zst`, not guessed) and mapping each group to whichever of the nine
existing sub-workflows already claims it:

| Path group | Ops | Covered by |
|---|---:|---|
| `/api/deploy` | 58 | `bamboo_workflow_deployments` |
| `/api/permissions` | 77 | `bamboo_workflow_permissions` |
| `/api/plan`, `/api/project`, `/api/clone`, `/api/buildNumber`, `/api/planDirectoryInfo` | 47 | `bamboo_workflow_projects_plans` |
| `/api/result`, `/api/queue`, `/responsibility/brokenBuild` | 27 | `bamboo_workflow_builds` |
| `/admin/config`, `/admin/artifactHandlers`, `/admin/darkFeatures`, `/admin/globalVariables`, `/admin/security`, `/admin/systemInfo`, `/api/server`, `/api/status` | 55 | `bamboo_workflow_server_admin` |
| `/api/agent`, `/api/capability`, `/api/elasticConfiguration`, `/api/elasticInstances`, `/admin/elastic`, `/api/ephemeral`, `/admin/ephemeral` | 42 | `bamboo_workflow_agents_capabilities` |
| `/api/repository` | 15 | `bamboo_workflow_repositories` |
| `/api/admin`, `/admin/user`, `/api/access-token`, `/admin/session` | 30 | `bamboo_workflow_users_groups` |
| `/api/search`, `/api/quickFilter`, `/api/quicksearch`, `/api/chart` | 24 | `bamboo_workflow_search_reporting` |

That accounts for 375 of 404 operations under a clearly-matching existing sub-workflow. The
remaining ~29 operations, checked individually (full path + operationId, not just the group
count), are all small (1–4 operations each) and are real, distinct capabilities that happen to
fall through the cracks of every sub-workflow's current prose rather than a genuinely new domain:

- **`/admin/expiry`** (4 ops) — build/branch retention & cleanup config, plus an on-demand `run`
  that's genuinely destructive (deletes old build data) and irreversible.
- **`/admin/scheduler`** (2 ops) — Bamboo's internal background maintenance jobs (list/trigger).
- **`/api/reindex`** (2 ops) — rebuild the search index.
- **`/api/websudo-session`** (3 ops) — an elevated re-authentication session that some
  security-sensitive admin actions require before they'll succeed; a real, non-obvious gotcha
  worth documenting since a rejected call here looks like a permissions problem but isn't one.
- **`/api/clone`** (1 op) — duplicate an existing plan into a new one.
- **`/api/dependency`** (2 ops) — search for parent/child plan dependencies (chained builds — "run
  plan B automatically after plan A succeeds").
- **`/api/encrypt`** (1 op) — encrypt a value before storing it as a masked plan/project variable.
- **`/api/job`** (2 ops, per-job Docker pipeline config) and **`/api/config/job/.../agent-assignment`**
  (4 ops, per-job agent assignment) — job-level settings that `bamboo_workflow_agents_capabilities`'s
  own description already promises ("agent assignment to jobs") but its content never actually
  covered.
- **`/triggers/remote`** (1 op) — manually trigger repository change-detection instead of waiting
  for the next scheduled poll.
- **`/api/currentUser`, `/api/avatar`, `/api/info`** (5 ops) — small user/server-identity lookups.

**Decision: no new top-level prompt.** Every one of these is either (a) squarely inside an
existing sub-workflow's resource scope once named explicitly, or (b) too small on its own (1–4
operations) to justify the `prompts/list` overhead and the "keep content proportional" principle
`docs/mcp-prompts-workflow-plan.md` already set as a hard review criterion — padding a thin domain
into its own prompt is exactly the anti-pattern that plan warned against. Instead, each gets folded
into whichever existing sub-workflow already owns its resource, as a short paragraph or gotcha —
real content, not a stub, but proportional to how small the domain actually is. The one
cross-cutting item (websudo) gets a single canonical explanation in `bamboo_workflow_server_admin`
with one-line cross-references from `bamboo_workflow_users_groups` and
`bamboo_workflow_permissions`, rather than restating it three times.

Every added sentence follows the same agnostic-phrasing rule as the original content: describe the
task to search for, never a specific `operationId`, and never assert a response field name without
telling the reader to confirm it via `get` first.

## Content changes

- **`src/prompts/content/server_admin.md`** — add: build/branch expiry & retention (with the
  destructive-action warning), the background scheduler, search-index reindex, and the websudo
  elevated-session gotcha.
- **`src/prompts/content/projects_plans.md`** — add: cloning a plan, plan-to-plan (chained-build)
  dependencies, and encrypting a value before storing it as a masked variable.
- **`src/prompts/content/agents_capabilities.md`** — add: per-job agent assignment (find/add/remove
  assigned agents, find eligible agents) and per-job Docker pipeline configuration — this is the
  one case that's a genuine gap against the prompt's own advertised description, not just an
  enrichment.
- **`src/prompts/content/repositories.md`** — add: manually triggering remote change-detection.
- **`src/prompts/content/users_groups.md`** — add: avatar and current-user lookups, admin-forced
  session invalidation, and a one-line cross-reference to the websudo gotcha.
- **`src/prompts/content/permissions.md`** — add a one-line cross-reference to the websudo gotcha
  (permission changes are exactly the kind of sensitive action it tends to gate).
- **`src/prompts/content/search_reporting.md`** — add a one-line cross-reference: if search results
  look stale, the reindex operation lives in `bamboo_workflow_server_admin`.

No changes to `src/prompts/router.rs`, `src/prompts/mod.rs`, or `src/core/mcp_server.rs` — prompt
names, descriptions, and arguments are unchanged, so `tests/prompts_workflow.rs`'s existing
assertions (all 10 names, `bamboo_workflow_deployments`'s three arguments) still hold as written.

## README changes

Add a new `### Guided workflows (MCP prompts)` subsection to `README.md`'s existing `## Usage`
section, placed after `### Harness Server` and before `### Connect an MCP client` (prompts are a
Harness-Server-side capability reached over the same MCP connection documented right after it).
Content: what the feature is (a master menu prompt plus nine domain sub-workflows), the full list
of prompt names with a one-line purpose each (mirroring `content/master.md`'s own menu so the two
don't drift), and one short `prompts/get`-shaped example. Note the delegation/parallelization
design and the agnostic-phrasing rule in one sentence each, since both are genuinely distinguishing
properties of this feature, not just implementation detail.

## Verification

- `cargo build`, `cargo test --locked`, `cargo fmt --check`, `cargo clippy --all-targets -- -D
  warnings` — this round is content-only (`.md` + `README.md`), so this mainly confirms the
  `include_str!`'d files still exist and compile and that `tests/prompts_workflow.rs`'s existing
  prompt-name/argument assertions are unaffected.
- Manual smoke check: re-run the same `initialize` → `prompts/get` stdio round-trip used for the
  original feature against `bamboo_workflow_server_admin` and `bamboo_workflow_agents_capabilities`
  (the two files gaining the most new content) to confirm the rendered text reads correctly.
- Read `README.md`'s new section back against `content/master.md` to confirm the prompt names and
  one-line descriptions match exactly — this is the one place drift between the two would be easy
  to introduce silently.

## Release

Same shape as the original feature's release, per this repo's own convention
(`chore(release): bump version to X.Y.Z` on a dedicated commit, tag-triggered `release.yml`):

1. `git commit` the content + README changes — `feat(prompts): extend workflow prompt content with
   previously uncovered operations` (still a `feat` under this repo's convention: it makes
   previously-undiscoverable operations reachable through the guided prompts, even though it adds
   no new prompt or tool).
2. `git commit` `docs/mcp-prompts-coverage-review-plan.md` separately — `docs: add MCP prompts
   coverage review plan`.
3. Bump `version` in `Cargo.toml` and commit as `chore(release): bump version to X.Y.Z`. Unlike the
   original feature (a new capability class, minor-bumped `0.6.8` → `0.7.0` matching this repo's
   `feat(auth)`/`feat(store)` precedent), this round adds no new prompt, tool, or module — it's
   additive content within an existing feature's existing surface, closer in kind to the many
   `fix`/`docs`/`style` commits this repo has consistently patch-bumped. Default to a patch bump,
   `0.7.0` → `0.7.1`.
4. `git tag v0.7.1` (or whatever `X.Y.Z` step 3 actually lands on) on the bump commit.
5. `git push` the branch, then push the tag.
