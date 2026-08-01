# Documentation gaps

**Scope note:** this repo does not have a full PRD or architecture doc set.
`docs/` holds exactly three files — [`SCHEMA_VERSIONS.md`](SCHEMA_VERSIONS.md)
(a one-table reference for the API spec source and generated store/schema
files), [`mcp-prompts-workflow-plan.md`](mcp-prompts-workflow-plan.md), and
[`mcp-prompts-coverage-review-plan.md`](mcp-prompts-coverage-review-plan.md)
(both scoped narrowly to the MCP-prompts feature shipped in v0.7.0–v0.7.1).
Nothing in this repo ever documented a design for auth, the embedded store/
asset lifecycle, CI/release infrastructure, credential storage, or Docker
packaging before building them — those areas simply have **no local doc to
compare against at all**.

**Methodology (revised):** for every gap tagged **(shared mcpify-template
gap)** below, "Doc gap" now cross-references mcpify's own planning docs —
`product-brief.md`, `prd.md`, `architecture.md`, and the versioned
`vN-implementation-plan.md` files at
`/Users/lucianoguerche/Documents/GitHub/mcpify/docs/` — since the actual root
cause of a shared gap lives in what *those* docs specified or missed, not in
this repo's own (mostly nonexistent) local docs. Each such entry also states
whether the same root cause is already documented in mcpify's own
`docs/DOC-GAPS.md` (in which case it points there instead of re-deriving a
differently-worded version) or whether it appears to be a genuinely new
finding not yet captured there. Entries tagged **(repo-specific)** are
compared only against this repo's own local docs, since they concern
Bamboo's actual API domain, not the generator.

One pattern falls out of doing this exhaustively: the large majority of gaps
below are **(shared mcpify-template gap)** — bugs and missing design
decisions in the generic transport/store/auth/CI substrate mcpify generates
for every project, not anything about Bamboo's API domain. Only PAT auth
support and the hand-authored MCP-prompts feature are genuinely
**(repo-specific)**. That's a direct consequence of this server's
architecture: `search`/`get`/`call` front all 404 Bamboo operations
generically, so almost none of this repo's own code is Bamboo-specific
business logic.

A second pattern, visible only after reading mcpify's own `docs/DOC-GAPS.md`
and `CHANGELOG.md` side by side with this repo's history: **roughly half of
the shared gaps below have no counterpart entry in mcpify's own
`docs/DOC-GAPS.md` at all**, even though the matching template fix clearly
landed in mcpify's own `CHANGELOG.md` (often on the same date, sometimes
citing this exact repo's own incident by name). Those are flagged explicitly
below as **new findings**, since mcpify's own gap analysis, thorough as it
is, evidently didn't walk its full `CHANGELOG.md` the same way this pass
walked `bamboo-mcp-rs`'s.

## Lessons for future docs

1. **Design the concurrency/shared-state model before first implementation,
   not release-by-release.** The embedded-store extraction path alone was
   patched across at least six separate releases (v0.3.1, v0.3.2, v0.3.3,
   v0.5.3, v0.6.3, v0.6.8) — first-write races, non-atomic overwrite, a
   single-process mutex gap, a Windows-only rename-over-open-file failure,
   then a cross-process race. mcpify's own `docs/DOC-GAPS.md` reaches the
   same conclusion independently (its lesson 5: "concurrency and
   platform-specific I/O semantics belong in the architecture doc for any
   embedded local datastore, not just in Rust doc-comments") — worth citing
   directly rather than restating, since it's the same finding from the
   generator side.

2. **Give every CI/release job an explicit timeout, and say why.** Neither
   `release.yml` nor `ci.yml` set `timeout-minutes` on their jobs, so
   GitHub's 6-hour default was the only real ceiling — and it's what let a
   mutex-poisoning bug (v0.8.1) turn one flaky test into a 6-hour stuck CI
   run before anyone noticed. mcpify fixed this in its own Rust-target
   templates within days (v0.11.3/v0.11.4), with the commit body explicitly
   naming *this repo's* incident as the reason — but as of this writing,
   mcpify's own `docs/DOC-GAPS.md` has no dedicated entry for it at all (see
   the v0.8.2/v0.8.3 entries below). A gap doesn't stop being worth
   recording just because the fix already shipped; recording it is what
   stops the same class of gap from recurring in a target that hasn't hit it
   yet.

3. **Cross-platform (Windows vs. POSIX) *process-spawning and TTY* behavior
   needs its own explicit review pass, separate from the store's file-lock
   semantics.** mcpify's `docs/DOC-GAPS.md` already has a thorough entry for
   Windows file-locking/timing on the embedded store (nine releases,
   v0.10.5–v0.11.11) — but a *different* Windows-only failure mode recurred
   at least four more times (`inquire::Select` needing a real TTY, a child
   process's `stdin` not surfacing EOF the way POSIX does, both for the
   setup wizard and for the stdio transport smoke test) with no entry of its
   own anywhere. See the v0.6.1/v0.6.5/v0.6.6/v0.6.8/v0.8.5 entries below —
   this looks like a genuinely new finding worth its own `docs/DOC-GAPS.md`
   entry in mcpify, distinct from the file-lock one.

4. **Document exactly what a "re-sync from the generator" is allowed to
   touch — this is currently missing from mcpify's own gap analysis too.**
   The v0.6.0 sync against updated mcpify templates silently reset
   `Cargo.toml`'s version to the generator's `0.1.0` placeholder and dropped
   hand-added `.gitignore` entries and README sections. mcpify's own
   `CHANGELOG.md` shows the matching upstream fixes (v0.10.0, v0.10.2), but
   — checked directly — mcpify's `docs/DOC-GAPS.md` never turned this into
   a dated entry or a "Lessons" bullet, despite it being exactly the kind of
   silently-regressed-hand-owned-field problem that lesson would generalize
   well. Keep an explicit, versioned "hand-owned, never regenerate" list
   (crate identity, version, local `.gitignore` additions, README sections
   written after generation) and diff against it on every sync.

5. **Verify a published artifact's stated guarantee, don't just name the
   step after it.** The GHCR image was intended as a normal Docker image
   from its first release (v0.2.0) but the `docker/build-push-action` step
   never set `platforms:` — so it silently stayed amd64-only for 8 releases
   until someone tried to pull it on arm64. mcpify's own `docs/DOC-GAPS.md`
   already covers the *fix* for this well (multi-arch entry, v0.11.12–
   v0.11.14) but not the earlier root cause: the original GHCR-publish step
   (added back in v0.2.0, closing the separate gap that `docker-build.yml`
   only ever validated and never published) was never itself checked
   against an "actually multi-arch" acceptance criterion. Whenever a doc
   asserts an artifact's guarantee, pair that assertion with an automated
   check in CI, not just a step name that implies it.

6. **State completeness claims with their verification method, not just the
   claim.** `mcp-prompts-workflow-plan.md` (v0.7.0) designed all nine domain
   sub-workflows without first enumerating the full 404-operation catalog
   against them; ~29 operations fell through the cracks until a dedicated
   follow-up did that enumeration (v0.7.1). This one is repo-specific (the
   prompts feature is hand-authored, not part of mcpify's generator), but
   the underlying discipline — record the enumeration method, not just the
   coverage claim — generalizes directly to mcpify's own REQ-2.5.1
   "zero-placeholder" bar, which is defined purely as "the generated test
   suite passes" (PRD §2.5, §2.3.6) and says nothing about whether the
   generated *documentation* (README completeness, subcommand coverage) is
   itself complete — see the v0.3.8/v0.3.11 entries below, neither of which
   a passing test suite would have caught.

7. **A design invariant belongs in the planning doc, not only a source
   comment — and a downstream fix belongs back in the doc that missed it,
   not only in `CHANGELOG.md`.** Several invariants (schema self-containment,
   store extraction happening "once per process", output-schema mismatches
   being non-fatal) were first written down only as a doc comment in the
   code. mcpify's own lesson 7 makes exactly this point about doc-vs-code
   sync ("a retrofitted requirement should get a retrofitted line in the
   PRD, immediately, not just a code fix") — but by mcpify's own admission
   in that same lesson, most of *its* gaps never got a doc update either.
   This pass on `bamboo-mcp-rs` independently confirms that observation from
   the downstream side: at least a dozen fixes below trace to a template
   change with a `CHANGELOG.md` line but no matching `docs/DOC-GAPS.md`
   entry, meaning the underlying doc gap in mcpify's own `prd.md`/
   `architecture.md` is still open even though the symptom was patched.

8. **Treat a fix found in one generated repo as a fleet-wide signal — and
   check both directions, not just downstream-of-mcpify.** Most fixes here
   trace cleanly from an mcpify template gap down to this repo. But at least
   one ran the other way: `bamboo-mcp-rs` fixed its own `cargo-dist`
   cross-platform release issues (v0.3.10 — ONNX Runtime's dropped
   x64-macOS support, Windows static-CRT linking, glibc floor) four days
   *before* mcpify's own Rust-target template picked up the same fix
   (folded into an unrelated-sounding `feat: add manifest-driven generator
   parity`, 2026-07-14). A fix discovered downstream doesn't reliably
   announce itself upstream just because it's the "right" direction of
   propagation — it still needs an explicit "check mcpify's own templates
   for the same bug" step, in either direction.

## Dated entries

## 2026-07-26 — v0.8.8 (shared mcpify-template gap)

### Doc gap

No doc weighed QEMU-emulated cross-architecture Docker builds against
native-runner-per-architecture-then-merge for multi-arch publishing up
front. `docs/prd.md` REQ-2.3.7 requires "a CI/CD pipeline with automated
versioning/publishing" but never specifies an architecture-coverage
requirement or evaluates QEMU's reliability for it — confirmed absent from
`prd.md`, `architecture.md`, and every `vN-implementation-plan.md`.

**mcpify cross-reference:** exactly matches mcpify's own `docs/DOC-GAPS.md`
entry "2026-07-26 - multi-arch container images not specified (v0.11.12 -
v0.11.14)" — this repo's v0.8.8 is the direct downstream consequence of
mcpify's v0.11.14 template fix (same root cause, same day). No new finding
here; mcpify's own doc already covers this well.

### Resulting work

Replaced the single QEMU cross-build job with a matrix job that builds
`linux/amd64` on `ubuntu-24.04` and `linux/arm64` natively on
`ubuntu-24.04-arm` (no emulation), pushing each by digest, then merges both
into one multi-arch manifest via `docker buildx imagetools create`.

## 2026-07-26 — v0.8.7 (shared mcpify-template gap)

### Doc gap

No doc specified that the GHCR publish step (added back in v0.2.0, the very
first release) needed an explicit `platforms:` argument to actually build
more than the CI runner's native architecture. `prd.md` REQ-2.3.7 requires
the publishing pipeline but never an architecture-coverage acceptance
check for what it publishes.

**mcpify cross-reference:** matches mcpify's own `docs/DOC-GAPS.md` entry
"2026-07-26 - multi-arch container images not specified" (the v0.11.12 half
of it) exactly — same root cause, same fix, same day. Not a new finding.
The one thing mcpify's entry doesn't mention, and worth adding here: the
*original* single-arch GHCR step traces back to v0.2.0 (see that entry
below), eight releases before anyone checked it actually did what its name
implied — see Lesson 5 above.

### Resulting work

Added `docker/setup-qemu-action` and `platforms: linux/amd64,linux/arm64` to
the release workflow's `build-push-action` step (superseded one release
later by v0.8.8's native-runner approach).

## 2026-07-26 — v0.8.6 (shared mcpify-template gap)

### Doc gap

`SCHEMA_VERSIONS.md` documents *where* the schema/store data comes from but
never states the resolution invariant it must satisfy (every stored schema
must be self-contained — no dangling `$ref`). The actual root cause is
upstream: `v1-implementation-plan.md` made a deliberate, documented decision
to embed a full standalone copy of the component-schema library into every
operation's schemas ("simpler and more robust than cross-schema `$ref`
resolution, at the cost of some file size") but never bounded that cost or
specified what "self-contained" should mean once real specs hit it —
`prd.md` REQ-2.4.1 and `architecture.md` §2 both praise `mcp_store.db` as
"self-contained" the same way, with the same silence.

**mcpify cross-reference:** exactly matches mcpify's own `docs/DOC-GAPS.md`
entry "2026-07-21 to 2026-07-26 - self-contained schema/`$ref` handling took
three iterations (v0.5.12, v0.11.5, v0.11.7)" — this repo's v0.8.6 is the
direct downstream consequence of mcpify's third-and-final iteration,
v0.11.7. No new finding; mcpify's own entry already correctly frames this as
having taken three rounds to converge.

### Resulting work

Recursively inlined every `$ref` in the stored/generated schemas that isn't
part of a genuine reference cycle, matching a companion fix landed in the
mcpify generator itself. Confirmed remaining `$ref` occurrences are real
cycles in Bamboo's own domain model (e.g. `RestPlanList` → `RestPlan` →
`RestProject` → `RestPlanList`), which have no finite fully-inlined form.

## 2026-07-21 — v0.8.5 (shared mcpify-template gap)

### Doc gap

No doc anywhere covers Windows CI process-spawning/timing flakiness for
*non-store* generated tests. This fix (switching the protocol test's
sentinel URL to an ephemeral local port, and widening its timeout from 30s
to 120s) is the same category of fix mcpify itself applied to
`tests/cli_smoke.rs.tera` in its own v0.11.8 (see mcpify's `docs/DOC-GAPS.md`
Windows-file-locking entry) — but confirmed directly against mcpify's
current template source
(`src/targets/rust/templates/core/mcp_server.rs.tera`, lines ~256–349): it
still hardcodes the old sentinel URL `https://api.example.test` and a 30s
timeout in the *protocol* test (as opposed to `cli_smoke.rs.tera`, which did
get the ephemeral-port treatment). This fix was made directly in
`bamboo-mcp-rs` and never made it back upstream.

**mcpify cross-reference:** **new finding, not covered.** mcpify's
Windows-file-locking `docs/DOC-GAPS.md` entry lists `v0.11.8` (the
`cli_smoke.rs` ephemeral-port fix) but its own template still has the
un-fixed sentinel-URL/30s-timeout pattern in `core/mcp_server.rs.tera`'s own
protocol test — meaning any other Rust-target project regenerated today
would still hit this. Worth flagging upstream directly, not just noting
here.

### Resulting work

Bound the test's own config to an ephemeral `127.0.0.1:0` port instead of an
external sentinel hostname, and widened the protocol-test timeout from 30s
to 120s.

## 2026-07-21 — v0.8.4 (shared mcpify-template gap)

### Doc gap

Same underlying invariant gap as v0.8.6 above (schema self-containment never
specified with a concrete acceptance shape) — this is the *second* of the
three iterations.

**mcpify cross-reference:** matches mcpify's own `docs/DOC-GAPS.md` entry
"self-contained schema/`$ref` handling took three iterations", specifically
the `v0.11.5` bullet ("Embed `$defs` into the literal schemas the `get` tool
returns"). Not a new finding — already well covered there.

### Resulting work

Regenerated the store and schemas via `mcpify add-version --set-default
--force` against the same Bamboo OpenAPI source (picking up the generator
fix, mcpify 0.11.5) so every schema embeds a `$defs` map alongside any
remaining `$ref`. Repopulated `semantic_endpoints` embeddings for the
refreshed store. (Superseded by the fully-inlined follow-up in v0.8.6 above.)

## 2026-07-21 — v0.8.3 (shared mcpify-template gap)

### Doc gap

Neither `ci.yml` nor `release.yml` (see v0.8.2 below) set `timeout-minutes`
in mcpify's Rust-target templates, so GitHub's 6-hour default job ceiling
was the only real bound on either. `prd.md` REQ-2.3.7 requires "a CI/CD
pipeline (GitHub Actions)" but never a resource-bound requirement for the
jobs it generates — confirmed absent from `prd.md` and `architecture.md`.

**mcpify cross-reference:** **new finding, not covered in mcpify's own
`docs/DOC-GAPS.md`.** The matching upstream fix landed the same day —
mcpify commit `21f4e96` (`fix(rust): cap the CI test job at 20 minutes`,
touching `src/targets/rust/templates/.github/workflows/ci.yml.tera`
directly, released as mcpify `v0.11.4`) — and its sibling commit `adf0b3c`
(v0.8.2 below) explicitly names *this repo's own incident* in its commit
body ("the only thing that actually bounded bamboo-mcp-rs's real
incident..."). Despite that, mcpify's `docs/DOC-GAPS.md` has no dated entry
for either fix — a real, template-level, incident-driven gap that its own
gap analysis missed. See Lesson 2 above.

### Resulting work

Capped the CI test job at 20 minutes; it normally finishes in well under
that.

## 2026-07-21 — v0.8.2 (shared mcpify-template gap)

### Doc gap

Same gap as v0.8.3 above, for the release workflow's build job — and this
is the entry directly caused by the real incident (v0.8.1, below): a hung
test ran for 6 hours before GitHub's default ceiling finally killed it,
because nothing had ever bounded it tighter.

**mcpify cross-reference:** **new finding, not covered in mcpify's own
`docs/DOC-GAPS.md`.** Matching upstream fix: mcpify commit `adf0b3c`
(`fix(rust): cap the release workflow's build job at 45 minutes`,
`src/targets/rust/templates/.github/workflows/release.yml.tera`, mcpify
`v0.11.3`) — whose own commit body says, verbatim: *"the only thing that
actually bounded bamboo-mcp-rs's real incident, where a hung test... ran for
6 hours before GitHub killed the job."* mcpify's generator explicitly cites
this exact repository's own production incident as the reason for the fix,
yet that incident never became a `docs/DOC-GAPS.md` entry on the mcpify
side. This is the single clearest example in this whole review of a fix
that closed the loop in code but not in documentation — see Lesson 2 above.

### Resulting work

Capped the release workflow's build job at 45 minutes (headroom for a slow/
cold-cache run against a normal ~20-minute build) and its artifact-upload
job at 10 minutes.

## 2026-07-21 — v0.8.1 (shared mcpify-template gap)

### Doc gap

`architecture.md` §2 ("Data Layer") describes `mcp_store.db` extraction and
embedding without ever mentioning concurrency, and nothing in `prd.md`
states that a panic while holding a shared connection's lock could poison it
permanently for the rest of the process's life. This is exactly the
resilience-property gap mcpify's own lessons already generalize (see below)
— but it's worth noting the mutex-poisoning bug itself isn't actually
Windows-specific, only its "long tail of symptoms" cousin was.

**mcpify cross-reference:** matches mcpify's own `docs/DOC-GAPS.md` entry
"Windows-specific file-locking/timing semantics unaddressed for the embedded
store", specifically the `v0.11.2` bullet ("recover from mutex poisoning on
the shared store connection"). Not a new finding — but worth flagging a
minor imprecision in mcpify's own entry: mutex poisoning is a *general*
concurrency-correctness bug (any panic while holding the lock, on any OS),
folded into an entry titled specifically "Windows-specific." The 6-hour CI
incident it caused (see v0.8.2/v0.8.3 above) is itself the more clear-cut
Windows-CI-timeout story; the poisoning bug that triggered it isn't
platform-specific at all.

### Resulting work

Recovered from mutex poisoning at every lock site instead of propagating it
(safe here since all access through the lock is read-only), and wrapped the
MCP protocol test in a 30-second timeout so a future hang fails fast instead
of burning the CI budget.

## 2026-07-20 — v0.8.0 (repo-specific)

### Doc gap

`mcp-prompts-workflow-plan.md`'s prompt-inventory table (the design doc for
this hand-authored feature — not part of mcpify's generator, since prompts
aren't a generator capability at all) locked in verbose, redundant prompt
names without a naming-convention review step. This is compared against
this repo's own local plan doc, correctly — the prompts feature doesn't
exist in mcpify's generator or templates, so there's no mcpify PRD/
architecture section to check it against.

### Resulting work

Renamed every prompt's wire-visible name to a hyphenated, "workflow"-free
form (`bamboo_workflow` → `bamboo`, `bamboo_workflow_deployments` →
`bamboo-deployments`, etc.), updated every cross-reference in the prompt
content files, source doc comments, tests, and the README. Documented
explicitly as a breaking change.

## 2026-07-20 — v0.7.0 → v0.7.1 (repo-specific)

### Doc gap

`mcp-prompts-workflow-plan.md` designed and named all nine domain
sub-workflows up front but specified no process for verifying the full
404-operation catalog was actually covered by them. Compared against this
repo's own local plan doc, correctly — same reasoning as v0.8.0 above, this
is Bamboo-domain content with no mcpify-generator counterpart to check
against.

### Resulting work

A dedicated follow-up review (`mcp-prompts-coverage-review-plan.md`, v0.7.1)
grouped all 404 operations by path segment against the nine existing
sub-workflows to systematically find the gap, then folded each missing
capability into whichever existing sub-workflow already owned its resource
as proportional new content. Also documented the prompts feature in the
README, which had no mention of it at all through v0.7.0.

## 2026-07-19 — v0.6.8 (shared mcpify-template gap, two entries)

### Doc gap

**Cross-process store-rename retry:** continuation of the store-extraction
concurrency theme (see v0.3.1 through v0.6.3 below) — `architecture.md` §2
never addresses cross-process contention on the embedded store's temp-dir
copy. **Setup-wizard smoke-test Windows hang:** a separate, uncovered theme
— see Lesson 3 above and the v0.6.1/v0.6.5/v0.6.6/v0.8.5 entries.

**mcpify cross-reference:** the rename-retry half matches mcpify's own
`docs/DOC-GAPS.md` Windows-file-locking entry exactly (`v0.11.1` bullet,
"Added retry logic around the store-file rename"). The setup-wizard-hang
half does **not** appear in that entry, nor anywhere else in mcpify's
`docs/DOC-GAPS.md` — it's the same root cause as mcpify's `v0.11.1` CHANGELOG
bullet ("Stopped the Rust target's setup-wizard smoke test from hanging on
GitHub Actions Windows runners") but that fix was folded into the same
release as the (well-documented) store-rename fix without getting its own
gap write-up. See Lesson 3.

### Resulting work

Retried the store-file rename with a short backoff (5 attempts, 50ms ×
attempt) on a cross-process race, instead of adding cross-process locking.
Skipped the setup-wizard smoke test on GitHub Actions Windows via a runtime
check (`cfg!(windows) && GITHUB_ACTIONS`), matching the stdio smoke test's
own fix one release earlier (v0.6.6, below).

## 2026-07-19 — v0.6.6 & v0.6.5 (shared mcpify-template gap)

### Doc gap

No doc — not `architecture.md`, not `prd.md`, not any `vN-implementation-
plan.md` — states that Windows' console/pipe I/O model doesn't give the
same immediate-EOF-on-closed-stdin behavior POSIX does, so any generated
integration test that spawns the server binary and expects fast failure on
a closed stdin can hang indefinitely on GitHub Actions Windows runners
specifically (confirmed: one run had to be manually cancelled after ~5
hours).

**mcpify cross-reference:** **new finding, not covered in mcpify's own
`docs/DOC-GAPS.md`.** The matching upstream fixes are in mcpify's own
`CHANGELOG.md` — `v0.10.7` ("Relaxed a stdio-transport error assertion... too
strict/fragile on Windows") and `v0.10.8` ("Stopped the Rust target's
stdio-transport smoke test from hanging on GitHub Actions Windows
runners") — both real, dated, template-level fixes with no corresponding
`docs/DOC-GAPS.md` entry. This is the clearest instance of the theme in
Lesson 3 above: a Windows process-spawning/TTY gap, distinct from the
well-documented store-file-lock gap, spanning at least four mcpify releases
(`v0.10.1`, `v0.10.7`, `v0.10.8`, `v0.11.1`) with no dedicated write-up
anywhere.

### Resulting work

v0.6.5 relaxed the stdio-transport test to assert the platform-independent
property (handshake never completes, non-zero exit) rather than an exact
error string. v0.6.6 then skipped the whole hanging test on GitHub Actions
Windows via a runtime check, leaving it enabled on a real Windows machine
where the behavior hasn't been confirmed to reproduce.

## 2026-07-19 — v0.6.4 & v0.6.3 (shared mcpify-template gap)

### Doc gap

**v0.6.3 (Windows race in `resolve_store_path`):** `resolve_store_path`'s
own doc comment claimed extraction "happens once per process," but nothing
in `architecture.md` §2 enforced that as a real design invariant, and
nothing flagged Windows' stricter rename-over-open-file semantics. **v0.6.4
(coverage-gate closing):** the 85%-production-coverage gate (adopted from
mcpify's templates in the v0.6.0 sync) turned out not to hold automatically
for a spec this large — confirmed: no doc anywhere (`prd.md`, `architecture.
md`, or any `vN-implementation-plan.md`) states or justifies the specific
80%/85% threshold, so there's no documented basis for whether it should
scale with a target's generated-surface size.

**mcpify cross-reference:** v0.6.3 matches mcpify's own `docs/DOC-GAPS.md`
Windows-file-locking entry exactly (`v0.10.5` bullet, "eliminate Windows-only
race in `resolve_store_path`"). v0.6.4's coverage-gate calibration issue is
**a new finding, not covered** — mcpify's own `CHANGELOG.md` v0.10.6 records
"closed the remaining generated-project production-coverage gap" as
internal-only with no `docs/DOC-GAPS.md` entry, and no mcpify doc anywhere
states why 85% (or 80%, per `prd.md`'s "Coverage and profiling tooling"
mention) was chosen or whether it's meant to scale with spec size.

### Resulting work

v0.6.3 cached the resolved store path per API version behind a mutex, so
extraction genuinely happens once per process as documented, and added
targeted tests for previously-uncovered branches. v0.6.4 closed the
remaining gap with tests against `AuthManager`, `ApiClient`'s full happy
path, and the `call` tool's known-operation path, raising coverage from
~83% to ~87%.

## 2026-07-19 — v0.6.2 (shared mcpify-template gap)

### Doc gap

No doc noted that `cargo-dist`'s own drift check (`dist host
--steps=create`) would flag this repo's deliberately simplified,
hand-written `release.yml` as "out of date" relative to `dist`'s own
auto-generated multi-job shape. Confirmed absent from `prd.md`,
`architecture.md`, and every `vN-implementation-plan.md` — none of them
discuss `cargo-dist`'s own workspace-drift detection at all.

**mcpify cross-reference:** **new finding, not covered in mcpify's own
`docs/DOC-GAPS.md`.** The matching upstream fix took *two* mcpify releases
to land correctly: `v0.10.3` ("Generated Rust projects' release workflow now
allows a dirty git tree by default") followed immediately by `v0.10.4`
("Moved the `allow-dirty` `cargo-dist` setting to the correct `[dist]`
table (it had incorrectly been placed under
`[dist.github-custom-runners]`)") — i.e. even mcpify's own first attempt at
this fix had a placement bug. Neither release has a `docs/DOC-GAPS.md`
entry, despite the two-step correction being exactly the kind of pattern
("self-contained by design" needing more than one iteration) that entry
already generalizes for the schema/`$ref` saga elsewhere in the same file.

### Resulting work

Added `allow-dirty` to the dist config to tell it the divergence is
intentional. Fixed upstream in mcpify's `dist-workspace.toml.tera` template
too, per the commit message.

## 2026-07-19 — v0.6.1 (shared mcpify-template gap)

### Doc gap

No doc specified that any generated test exercising `inquire::Select`
(triggered whenever a project has more than one configured auth method)
needs to construct its inputs directly rather than driving the interactive
prompt, since `inquire` panics without a real TTY under CI. `prd.md`
REQ-1.6 ("Guided Setup Wizard") describes only where the wizard persists
values, never how a *test* of the wizard should be written to avoid a TTY
dependency. This was a latent gap in the generated test suite for every
project with 2+ auth strategies — invisible here until the repo-specific
PAT-auth addition (v0.3.0, below) pushed Bamboo's auth-method count past
that threshold.

**mcpify cross-reference:** **new finding, not covered in mcpify's own
`docs/DOC-GAPS.md`.** Matching upstream fix: mcpify `CHANGELOG.md` v0.10.1
("The generated Rust setup-wizard test no longer requires a real TTY, fixing
it in headless CI environments"). Part of the same uncovered Windows/TTY
theme as v0.6.5/v0.6.6/v0.6.8/v0.8.5 — see Lesson 3 above.

### Resulting work

Constructed the prompt's target values directly in the test instead of
invoking the interactive select, matching what the non-interactive output
path already hardcoded elsewhere. Fixed upstream in mcpify's
`setup_wizard.rs.tera` template too, per the commit message.

## 2026-07-19 — v0.6.0 (shared mcpify-template gap)

### Doc gap

No doc anywhere — repo-side or generator-side — records a contract for what
a "sync to current mcpify parity" commit is allowed to silently regenerate
versus what's hand-owned and must be preserved. `architecture.md` describes
the generator's `execute()` lifecycle (bootstrap → scaffolding → auth →
transports → tools → wizard/tests) in detail but never discusses what
happens when that same pipeline runs again against a project that already
has hand-added content. As a direct result, the sync in this release
silently regressed two things: `.gitignore` lost its cargo-default entries
and the `.mcpify/*` scratch-file guard, and `Cargo.toml`'s `version` field
reset to the generator's `0.1.0` placeholder instead of staying at the
repo's actual `0.5.4`.

**mcpify cross-reference:** **significant new finding, not covered in
mcpify's own `docs/DOC-GAPS.md` at all.** Checked directly: no entry, no
"Lessons" bullet, nothing matching "Cargo.toml", "placeholder", "gitignore",
or "preserve" anywhere in that file. The matching upstream fixes are real
and dated in mcpify's own `CHANGELOG.md` — `v0.10.0` ("Restored cargo-default
`.gitignore` entries and the `.mcpify` scratch-file guard in generated Rust
projects"; "Restored the Observability & Resilience, License, and install
sections to the generated Rust README") and `v0.10.2` ("Preserve an
existing project's name when re-syncing/regenerating a Rust project,
instead of silently overwriting it") — but neither ever became a
`docs/DOC-GAPS.md` entry or fed the "Lessons" section there. This is the
single most consequential gap in this whole review to flag back upstream:
a version-number regression on re-sync could silently ship a wrong version
number on a real release if it went unnoticed. See Lesson 4 above.

### Resulting work

Restored `.gitignore`'s cargo boilerplate and the `.mcpify/*` guard,
restored `Cargo.toml`'s version to `0.5.4`, and restored the README sections
(Observability & Resilience, License, install docs) the same sync had
dropped. A companion commit in the same release separately re-added a
"Connect an MCP client" README section mcpify's template had newly started
generating, closing a related README gap.

## 2026-07-17 — v0.5.4 (shared mcpify-template gap)

### Doc gap

`prd.md` REQ-1.2 (auth strategy generation) is silent on OAuth2-specific
mechanics beyond "emit one auth strategy module per scheme," and nothing in
`prd.md`/`architecture.md` specifies the credential-derivation cascade's
fallback order (must retry a fresh `authenticate()` exchange, not only
`refresh_token()`) or that body-less requests still need an explicit
`Content-Length: 0` for strict API compliance.

**mcpify cross-reference:** matches mcpify's own `docs/DOC-GAPS.md` entry
"2026-07-17 - OAuth2 setup wizard incompleteness (v0.9.0)" — same release
(mcpify `v0.9.0`, same date), same root cause. Not a new finding, though
worth noting mcpify's entry title emphasizes the *wizard* half of that
release (the PKCE addition) while this repo's v0.5.4 fix is specifically
the *Fixed* half of the same release (the credential-cascade completion and
`Content-Length` fix) — complementary detail on the same gap, not a
duplicate.

### Resulting work

Added the `authenticate()` fallback to the credential cascade; added an
explicit empty body with `Content-Length: 0` for body-less PUT/POST/DELETE
requests.

## 2026-07-16 — v0.5.3 (shared mcpify-template gap)

### Doc gap

No doc noted that MCP tool calls execute concurrently as separate tokio
tasks, and that a non-atomic write of the shared temp-dir store copy was
therefore unsafe under concurrency.

**mcpify cross-reference:** matches mcpify's own `docs/DOC-GAPS.md` Windows-
file-locking entry, specifically its `v0.8.2` bullet ("extract
`mcp_store.db` atomically to avoid concurrent-call races") — same fix, same
date. Not a new finding.

### Resulting work

Changed the store extraction to write to a uniquely-named sibling file and
rename it into place — atomic on both POSIX and Windows.

## 2026-07-16 — v0.5.2 (shared mcpify-template gap)

### Doc gap

Neither `prd.md` nor the original `architecture.md` "`call` Pipeline"
description distinguished input-validation strictness from
output-validation strictness before this fix.

**mcpify cross-reference:** matches mcpify's own `docs/DOC-GAPS.md` entry
"2026-07-16 - output-schema mismatch handling not specified up front
(v0.8.1)" exactly — same release, same root cause. Not a new finding. Worth
repeating mcpify's own good callout here: `architecture.md` §"The `call`
Pipeline" *was* updated after this fix to document the new behavior — one of
the few gaps on either side of this repo pair where the doc was actually
brought back in sync with the code (see Lesson 7 above).

### Resulting work

`call_operation` now logs a warning and still returns the live response on
an output-schema mismatch, rather than rejecting an otherwise-successful
call; input validation is unchanged and still rejects invalid arguments.
Error messages now include the actual jsonschema violation details instead
of a generic "invalid input" string.

## 2026-07-15 — v0.5.0 (shared mcpify-template gap)

### Doc gap

`prd.md` §2.2 defines the seven-tier config cascade and §2.3.5 requires
OS-keychain credential storage with an encrypted-file fallback, but neither
section (nor `architecture.md`) specifies the exact interaction contract
between them: that env-var overrides must actually be read before falling
back to stored credentials, that the setup wizard's persisted format must
match what the config loader reads back, or that a keychain "not found"
response must be handled differently from a hard keychain error. This was
the one gap in this repo's history that was **QA-reported by a real user**
rather than caught internally first.

**mcpify cross-reference:** no dedicated `docs/DOC-GAPS.md` entry matches
this bundle specifically — it's adjacent to, but distinct from, entry
"[Unreleased] - config command observability gap" (about the `config`
command not showing cascade provenance) and the OAuth2-wizard entry. The
likely upstream counterpart is mcpify `CHANGELOG.md` `v0.8.0` ("Hardened
auth, credential-handling, and embedding-population logic for parity across
all 5 language targets"), dated 2026-07-15 — the *same day* as this repo's
v0.5.0. **Flagging as a partial new finding:** the specific sub-bugs here
(dead `config.json`, keychain-fallback-only-on-hard-error, `HOME`/
`USERPROFILE` resolution) aren't individually itemized in mcpify's
`docs/DOC-GAPS.md` even though `v0.8.0`'s CHANGELOG summary strongly implies
they're the same fix — worth confirming and folding into the config-
observability entry or a new one there.

### Resulting work

Wired up the env-var overrides to actually take effect; had the setup
wizard write YAML the loader reads back on startup; fell back to the
encrypted file on a clean keychain "not found"; resolved home via `HOME`
then `USERPROFILE`; `populate-embeddings` now defaults to populating and
verifying every API version's store. Also documented that `BAMBOO_MCP_URL`
must include the `/rest` suffix.

## 2026-07-14 — v0.4.0 (shared mcpify-template gap)

### Doc gap

This release is a "sync to current mcpify parity" with no detailed commit
body — itself an instance of the same undocumented-sync-contract gap called
out at v0.6.0 above, just one that didn't happen to regress anything
visible this time.

**mcpify cross-reference:** same new finding as v0.6.0 above (no mcpify
`docs/DOC-GAPS.md` entry for the sync-contract gap) — recorded here for
completeness since the pattern recurs with real consequences two releases
later, not treated as a separate finding.

### Resulting work

Re-synced generated code against current mcpify Rust-target templates.

## 2026-07-13 — v0.3.11 (shared mcpify-template gap)

### Doc gap

No doc reviewed the generated README's `call` examples for realism before
shipping. `architecture.md` §"CLI invocation of generated output" requires
that each target document *how to invoke* the generated binary, but nothing
requires the generated README's worked examples to be checked against the
target project's actual operations/flags.

**mcpify cross-reference:** **new finding, not covered.** Same-day match in
mcpify's own `CHANGELOG.md`: `v0.5.16` ("Generated `call` examples in docs
now correctly use `--args`"), dated 2026-07-13 — same date as this repo's
fix. No `docs/DOC-GAPS.md` entry exists for it. Low-severity, but a clean
same-day example of the pattern in Lesson 7 above.

### Resulting work

Documented real, worked `call` argument examples grounded in this project's
actual operations.

## 2026-07-10 — v0.3.10 & v0.3.9 (shared mcpify-template gap)

### Doc gap

No doc anticipated that `ort`/ONNX Runtime's prebuilt binaries have hard
platform floors — no prebuilt library for x64 macOS, a documented Windows
static-vs-dynamic-CRT linking incompatibility, and a glibc floor
incompatible with `cargo-dist`'s default Ubuntu image. `v2-implementation-
plan.md` resolves `fastembed` as the Rust target's embedding dependency
(open decision 2) but never discusses its native-runtime platform
constraints, unlike `v1-implementation-plan.md`, which *did* explicitly
pin `reqwest`'s TLS feature (`rustls-tls`) for mcpify's own dependencies —
the same discipline was never carried over to what the Rust target ships.

**mcpify cross-reference:** **new finding, not covered in mcpify's own
`docs/DOC-GAPS.md`.** And unusually, the propagation direction ran
*backwards* here: this repo (`bamboo-mcp-rs`) discovered and fixed these
platform constraints on 2026-07-10, four days *before* mcpify's own
Rust-target template picked up the same fix — folded into the unrelated-
sounding `feat: add manifest-driven generator parity` (2026-07-14, mcpify
`v0.7.0`), confirmed directly in
`src/targets/rust/templates/dist-workspace.toml.tera`. See Lesson 8 above.

### Resulting work

Dropped the `x86_64-apple-darwin` release target; set `msvc-crt-static =
false` for Windows; pinned Linux release builds to `ubuntu-24.04`. Also
fixed unrelated `cargo fmt --check` drift in the same commit.

## 2026-07-08 — v0.3.8 (shared mcpify-template gap)

### Doc gap

The generated README omitted, entirely, four CLI subcommands, the
`.env.example` configuration knobs, and the entire observability/resilience
stack. `architecture.md` §"CLI invocation of generated output" only
requires documenting *how to invoke* the binary, and `prd.md` REQ-2.5.1's
"zero-placeholder" quality bar is defined purely via the generated test
suite passing (REQ-2.3.6) — nothing requires the generated README to
actually enumerate every capability REQ-2.3 (Enterprise-First) mandates be
present. A passing test suite is silent on whether the README describing
that same code is complete.

**mcpify cross-reference:** **new finding, not covered.** No
`docs/DOC-GAPS.md` entry addresses README completeness as a category at
all (the closest is the pre-commit-hooks entry, which is unrelated). Worth
a fresh entry there generalizing this alongside v0.3.11 and v0.6.0's
README-restoration case above — the recurring pattern is that
"zero-placeholder" only ever meant "the code works," never "the docs
describing the code are accurate."

### Resulting work

Added an "Observability & Resilience" README section grounded in the
actual source, documented every previously-missing subcommand, and added a
License section pointing at the existing `LICENSE` file.

## 2026-07-08 — v0.3.7 (shared mcpify-template gap)

### Doc gap

No doc specified the exact normalized shape credentials must be in before
`apply_auth_headers` can use them — a generic property of the `AuthManager`
credential-cascade design (`prd.md` §1.2/§1.3) that the PRD never details
past "select exactly one active strategy."

**mcpify cross-reference:** **new finding, not covered.** Same-day match in
mcpify's own `CHANGELOG.md`: `v0.5.12`'s "Normalize generated credentials
before use in the Rust target" bullet, dated 2026-07-08 — same date as this
repo's fix, folded into the same release as the schema-reachability fix but
not itself covered by that or any other `docs/DOC-GAPS.md` entry.

### Resulting work

Added a `normalize_credentials` step to the credential cascade and
re-persisted the normalized form back to the credential store.

## 2026-07-06 — v0.3.6 (shared mcpify-template gap)

### Doc gap

`prd.md` §2.3.1 ("Logging") specifies structured JSON logging with secret
redaction but never states that the log *destination* must depend on
transport mode, and `architecture.md`'s "Dual-Role Execution" section
describes `stdio`/`http` transport selection without connecting it to
logging at all.

**mcpify cross-reference:** matches mcpify's own `docs/DOC-GAPS.md` entry
"2026-08-01 - stdio vs HTTP logging stream" precisely and by name — its
`v0.5.10` bullet is this exact fix, same date, same repo pattern ("write
logs to stderr instead of stdout... across three targets"). Not a new
finding. **Important forward-pointer, though:** mcpify's own entry goes on
to say this original fix was "directionally correct for `stdio` mode but
wrong for `http` mode, where stdout is the conventional log destination,"
and records a newer, still-unreleased fix (2026-08-01) making log
destination conditional on transport. `bamboo-mcp-rs`'s current `README.md`
("Structured logs go to **stderr** (never stdout...)") still describes the
old, transport-*unconditional* behavior — this repo has not yet picked up
mcpify's corrected version, and won't until a future regeneration/sync.
Worth tracking as known pending debt, not a new discovery.

### Resulting work

Routed the logging fmt layer to stderr explicitly, and based the
pretty-vs-JSON TTY auto-detection on stderr rather than stdout.

## 2026-07-06 — v0.3.5 (shared mcpify-template gap)

### Doc gap

No doc noted that adding OpenTelemetry OTLP export (`prd.md` REQ-2.3.2)
requires explicitly installing a `rustls` crypto provider at process
start — `opentelemetry-otlp`'s HTTP exporter pulls in a second, transitive
`reqwest` that links `rustls` with no default provider selected.

**mcpify cross-reference:** **new finding, not covered.** Exact-date match
in mcpify's own `CHANGELOG.md`: `v0.5.9` ("Install the `rustls` crypto
provider before first TLS use in the Rust target, fixing a startup panic on
some platforms"), dated 2026-07-06 — same date as this repo's fix. No
`docs/DOC-GAPS.md` entry exists for it, despite REQ-2.3.2 (OpenTelemetry)
being a named, mandatory requirement whose interaction with the HTTP
client's TLS setup was never worked through in `architecture.md`.

### Resulting work

Installed the `aws-lc-rs` default crypto provider once at process start.

## 2026-07-06 — v0.3.4 & v0.3.3 (shared mcpify-template gap)

### Doc gap

No doc specified that store extraction must always overwrite any existing
temp-dir copy, nor that `populate_embeddings` needed to handle every API
version rather than being hardcoded to the default.

**mcpify cross-reference:** matches mcpify's own `docs/DOC-GAPS.md`
CWD-relative-paths entry, specifically its `v0.5.6` bullet ("further
store-path hardening (`VERSION_STORE_FILES`, always-refresh extraction)") —
same fix, same date (2026-07-06). Not a new finding. The multi-version
`--all` backfill half also has a sibling in mcpify's own `CHANGELOG.md`
`v0.5.7` ("Fixed multi-version embedding population across the TypeScript,
Python, C#, and Go targets") — confirming this propagated to every target,
not just Rust.

### Resulting work

v0.3.3 made store extraction always overwrite and added a test asserting
`VERSION_STORE_FILES`/`VERSION_STORE_BYTES` stay in sync; `populate_embeddings`
now accepts an explicit path or an `--all` flag. v0.3.4 re-ran the fixed
tool to backfill this project's own previously-incomplete embeddings.

## 2026-07-06 — v0.3.2 & v0.3.1 (shared mcpify-template gap)

### Doc gap

No doc specified where a compiled binary should locate its bundled
`mcp_store*.db` file when invoked from outside its build checkout — a
CWD-relative-path risk of exactly the shape mcpify's own lesson 2
generalizes (see below). `v2-implementation-plan.md`'s own embeddings
decision shows the same class of oversight existed elsewhere in the very
same target's plan: it documents the Rust target's `fastembed` cache
defaulting to a bare relative `.fastembed_cache` path (overridable via
`FASTEMBED_CACHE_DIR`/`HF_HOME`) without ever flagging that as a
CWD-relative risk when no override is set.

**mcpify cross-reference:** matches mcpify's own `docs/DOC-GAPS.md` entry
"2026-08-01 - filesystem paths must not depend on launch-time CWD" exactly —
its bullet list cites `v0.5.4` (this repo's v0.3.1, "store db path
resolution with exe-dir fallback") and `v0.5.5` (this repo's v0.3.2, "two
follow-up fixes moving from a path-fallback chain to embedded-only store
resolution") by name, same dates. Not a new finding — this is mcpify's own
best-documented gap, and correctly so; it's also the entry whose "Lessons"
bullet 2 is the direct, generalized fix for exactly this class of bug. One
open thread worth noting: that same mcpify entry's final bullet (fixing the
`fastembed` cache path itself, 2026-08-01, "unreleased") has **not yet
reached `bamboo-mcp-rs`** — this repo's own `fastembed` cache is still
presumably CWD-relative today, pending a future regeneration.

### Resulting work

Settled on embedding every supported version's store bytes via
`include_bytes!` and extracting to the OS temp dir on first use, with no
filesystem-guessing fallback chain.

## 2026-07-06 — v0.3.0 (repo-specific)

### Doc gap

No doc (this repo's own, or Bamboo's own published OpenAPI spec) captured
that Bamboo Data Center's real auth surface is broader than what the spec
declares. This is compared against this repo's own domain knowledge and
Bamboo's spec, correctly — `prd.md` REQ-1.2 already anticipates that a
generator can't know more than what `components.securitySchemes` declares
(REQ-1.2.4's ambiguous-scheme fallback exists for exactly this reason at
generation time), so the gap here is squarely in *this API's* own spec
accuracy, not in mcpify's generation logic.

### Resulting work

Added a `PatAuthStrategy` alongside the existing `BasicAuthStrategy`, a new
`AuthMethod::Pat` config value, and a setup-wizard choice between the two.

## 2026-07-05 — v0.2.3 (shared mcpify-template gap)

### Doc gap

No doc addressed binary-naming collisions when multiple mcpify-generated
MCP servers are installed side by side on the same machine.
`v2-implementation-plan.md` describes what the generated helper binary
(`populate-embeddings`-equivalent) does but never its naming convention
relative to PATH-collision risk against other generated projects' own
helper binaries.

**mcpify cross-reference:** **new finding, not covered.** Exact-date match
in mcpify's own `CHANGELOG.md`: `v0.5.3` ("Namespaced the Rust target's
generated helper binary to avoid collisions with other binaries"), dated
2026-07-05 — same date as this repo's fix. No `docs/DOC-GAPS.md` entry
exists for it.

### Resulting work

Renamed the helper binary to `bamboo-mcp-populate-embeddings`.

## 2026-07-05 — v0.2.0 (shared mcpify-template gap)

### Doc gap

No doc existed yet at this point in the repo's history. Two distinct root
causes, both in mcpify's generator design rather than anything Bamboo-
specific:

- **Docker build failure (openssl/glibc/vec0 upsert):** `v1-implementation-
  plan.md` explicitly pins `reqwest (json, rustls-tls)` for mcpify's own
  dependencies, but `v2-implementation-plan.md`'s `fastembed` decision for
  the *generated* Rust target never carries that same TLS-feature
  discipline forward, nor discusses the base Docker image's glibc floor
  against `ort`'s prebuilt binaries, nor the `sqlite-vec` `vec0` virtual
  table's lack of `INSERT OR REPLACE` support.
- **crates.io publish blockers (placeholder license/repo, name collision):**
  `v6-implementation-plan.md` (line ~153) *explicitly, deliberately* ships
  placeholder `license`/`repository` values in `Cargo.toml.tera` "plus a
  comment telling the user to fill them in; crates.io rejects a publish
  without a license" — a known, intentional gap, but one never paired with
  a pre-publish checklist or a generation-time check that would catch it
  before the first real release attempt.

**mcpify cross-reference:** **new finding, not covered in mcpify's own
`docs/DOC-GAPS.md`**, for both root causes — checked directly, no match for
"native-tls", "openssl", "glibc", "vec0", "placeholder license", or
"SPDX" anywhere in that file. The Docker-build fix is dated and unusually
well-documented on the mcpify side even without a `docs/DOC-GAPS.md` entry:
mcpify commit `62fc345` (`fix(rust): drop native-tls from fastembed and
match ort's glibc requirement`, mcpify `v0.4.0`) says, verbatim, *"Discovered
via a real Docker build failure in a generated Rust project (bamboo-mcp-rs
CI run 28736427046)"* — i.e. mcpify's own generator explicitly credits this
repository's own CI run as the discovery source. That the fix is dated and
attributed but still has no `docs/DOC-GAPS.md` entry is a clean example of
Lesson 7 above. Separately, `docs/v9-implementation-plan.md` on the mcpify
side documents a *related but distinct*, still-being-applied fix for this
same schema/store-embedding area (compressing `generated_schemas.json` to
fit crates.io's 10 MiB cap, plus loading `mcp_store*.db` into memory to stop
`populate-embeddings` from colliding with a live server's file lock) — as of
that plan's own "Outcome" section, both were implemented and validated
upstream, but `bamboo-mcp-rs` would only receive them via a future
regeneration (the plan explicitly says "no hand-patch track for either").
Worth watching for on the next sync.

### Resulting work

Switched `fastembed` to its rustls-TLS feature variants; moved Docker
build/runtime stages to Debian trixie; made the embeddings upsert
idempotent (delete-then-insert instead of `INSERT OR REPLACE`); set real
MIT license/repository metadata in `Cargo.toml`; renamed the package to
`bamboo-mcp-rs` after discovering `bamboo-mcp` was already taken on
crates.io. Separately in the same release, added the GHCR-publish step
itself (closing the pre-existing gap that `docker-build.yml` only ever
validated the image and never published it anywhere) — the step whose own
`platforms:` omission became a doc gap in its own right eight releases
later (v0.8.7, above).
