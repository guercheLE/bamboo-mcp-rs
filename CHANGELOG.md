# Changelog

All notable changes to this project are documented here. Format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/). This project releases
via a dedicated `chore(release): bump version to X.Y.Z` commit tagged `vX.Y.Z`;
each section below corresponds to one such release.

## [0.8.8] - 2026-07-26

### Fixed

- Built the multi-arch (amd64+arm64) GHCR image on native runners
  (`ubuntu-24.04` / `ubuntu-24.04-arm`) instead of QEMU emulation, using
  Docker's build-by-digest-then-merge pattern for reliability.

## [0.8.7] - 2026-07-26

### Fixed

- Actually published a multi-arch (amd64+arm64) GHCR image — the release
  workflow's `docker/build-push-action` step never set `platforms:`, so only
  a `linux/amd64` image had ever been built and pushed, breaking `docker
  pull` on arm64 hosts (Apple Silicon, arm64 CI runners).

## [0.8.6] - 2026-07-26

### Fixed

- Fully inlined every non-cyclic `$ref` in the operation schemas stored in
  `mcp_store.db` and `generated_schemas.json`, replacing a leftover
  reference to a locally-embedded `$defs` block. Genuinely cyclic schemas
  (e.g. Bamboo's Plan → Chain → Project model) are left as confirmed cycles.

## [0.8.5] - 2026-07-21

### Changed

- Stabilized a flaky MCP protocol test in CI.

## [0.8.4] - 2026-07-21

### Fixed

- Regenerated the store and schemas with fully-expanded `$ref`s so every
  operation's `input_schema`/`output_schema` is self-contained — a
  component `$ref` inside a `get`-tool response could previously point at
  nothing in the returned snippet. Also repopulated semantic-search
  embeddings for the refreshed store.

## [0.8.3] - 2026-07-21

### Changed

- Capped the CI test job at 20 minutes instead of relying on GitHub's
  6-hour default job ceiling.

## [0.8.2] - 2026-07-21

### Changed

- Capped the release workflow's build job at 45 minutes (and its
  artifact-upload job at 10 minutes) instead of relying on GitHub's 6-hour
  default job ceiling.

## [0.8.1] - 2026-07-21

### Fixed

- Recovered from mutex poisoning on the shared store connection: a panic
  while holding the process-wide connection lock no longer permanently
  breaks every subsequent `search`/`get`/`call`. Also wrapped the MCP
  protocol test in a 30-second timeout so a future hang fails fast.

## [0.8.0] - 2026-07-20

### Changed

- **BREAKING:** Renamed every MCP prompt identifier to hyphenated,
  "workflow"-free names (e.g. `bamboo_workflow_deployments` →
  `bamboo-deployments`, the master prompt `bamboo_workflow` → `bamboo`).
  Any client already configured against the old prompt names needs
  updating.

## [0.7.1] - 2026-07-20

### Added

- Extended the guided workflow prompts with ~29 previously-uncovered
  operations found by a full audit of the 404-operation catalog: build/
  branch retention & expiry, the background scheduler, search reindexing,
  the websudo elevated-session requirement, plan cloning, chained-build
  dependencies, encrypted variables, per-job agent assignment and Docker
  pipeline config, and manual change-detection triggering.

### Documentation

- Documented the guided-workflow MCP prompts feature in the README, which
  previously had no mention of it.

## [0.7.0] - 2026-07-20

### Added

- Guided Bamboo workflow MCP prompts: a master `bamboo_workflow` menu
  prompt plus nine per-domain sub-workflow prompts (projects/plans,
  builds, deployments, agents/capabilities, permissions, repositories,
  users/groups, server admin, search/reporting) that walk the calling LLM
  step by step through common multi-step Bamboo management tasks, gate
  progression on verified outcomes, and describe operations by capability
  rather than a specific `operationId`.

## [0.6.8] - 2026-07-19

### Fixed

- Retried the store-file rename on a cross-process race (short backoff,
  5 attempts) instead of failing outright when another process briefly
  holds the destination file open.
- Stopped the setup-wizard smoke test from hanging indefinitely on GitHub
  Actions Windows runners.

### Changed

- Installed `cargo-dist` from a prebuilt binary in CI instead of compiling
  it from source, cutting release-workflow time substantially.

## [0.6.7] - 2026-07-19

### Documentation

- Added a sponsorship callout and `FUNDING.yml`.

## [0.6.6] - 2026-07-19

### Fixed

- Stopped the stdio smoke test from hanging indefinitely on GitHub Actions
  Windows runners — the server process itself never returned there because
  Windows' console/pipe I/O doesn't surface an immediately-closed child
  stdin as EOF the way Unix does. Skipped via a runtime check rather than a
  blanket `#[cfg(windows)]`, so real Windows machines keep full coverage.

## [0.6.5] - 2026-07-19

### Fixed

- Relaxed a Windows-fragile stdio-transport error-message assertion in
  tests to check the platform-independent behavior (handshake never
  completes, non-zero exit) instead of an exact error string that only
  ever held on Linux/macOS.

## [0.6.4] - 2026-07-19

### Changed

- Closed the remaining production test-coverage gap (`AuthManager`
  credential normalization, `ApiClient`'s full happy-path pipeline, the
  `call` tool's known-`operationId` path), raising production line
  coverage from ~83% to ~87%.

## [0.6.3] - 2026-07-19

### Fixed

- Eliminated a Windows-only race in `resolve_store_path`: concurrent calls
  could previously rename a freshly-extracted store over a destination
  file another thread already had open. The resolved path is now cached
  per API version behind a mutex, so extraction genuinely happens once per
  process.

### Changed

- Raised production test coverage toward the 85% gate with real tests for
  previously-uncovered branches (`QueryCache`, `CircuitBreaker`,
  `RateLimiter`, config-file parsing, the sanitizer, and the validator's
  schema-compilation-failure fallback).

## [0.6.2] - 2026-07-19

### Fixed

- Marked the hand-simplified release workflow `allow-dirty` so
  `cargo-dist`'s drift check no longer fails a real release run against
  this repo's deliberately non-standard single-job `release.yml`.

## [0.6.1] - 2026-07-19

### Fixed

- Stopped a setup-wizard test from requiring a real TTY in CI — with more
  than one auth method configured, prompting routed through
  `inquire::Select`, which panics without a real terminal.

## [0.6.0] - 2026-07-19

### Added

- Embedded `mcp_store.db` zstd-compressed instead of raw, shrinking the
  bytes shipped in the compiled binary/published crate by roughly
  two-thirds.

### Fixed

- Restored `.gitignore`'s cargo boilerplate, `Cargo.toml`'s version, and
  several README sections that a prior template re-sync had silently
  regressed back to generator defaults.

### Changed

- Re-synced against current mcpify Rust-target templates: hardened auth/
  validation/schema handling in the MCP server, API client, and search
  tool; a simpler hand-rolled release workflow; new coverage/profiling
  scaffolding (cold-start vs. warm-search profiling split, 80%+ coverage
  gate, embeddings row-count parity test).

### Documentation

- Re-added a "Connect an MCP client" README section with real stdio/HTTP
  `mcpServers` JSON configuration examples.

## [0.5.4] - 2026-07-17

### Fixed

- Completed the credential-derivation cascade: a credential blob with
  neither a usable access token nor a refresh token now falls back to a
  fresh `authenticate()` exchange instead of failing with a generic "no
  active credentials" error.
- Sent an explicit `Content-Length: 0` on body-less PUT/POST/DELETE
  requests, so strict APIs that reject a missing `Content-Length` with
  411 no longer block calls whose arguments are entirely query/path
  parameters.

## [0.5.3] - 2026-07-16

### Fixed

- Extracted `mcp_store.db` atomically (write to a sibling file, then
  rename into place) to close a concurrent-call race where one tool call
  could read the store mid-truncate from another call's in-progress
  write and see a spuriously empty database.

## [0.5.2] - 2026-07-16

### Fixed

- Stopped rejecting an otherwise-successful `call` purely because the live
  response didn't match its documented output schema; the response is now
  returned with a logged warning instead, since upstream OpenAPI specs are
  frequently wrong about response shape. Input validation is unchanged.
- Surfaced the actual JSON Schema validation violations in error messages
  instead of a generic "invalid input"/"unexpected response shape" string.

## [0.5.1] - 2026-07-16

_No user-facing changes (formatting only)._

## [0.5.0] - 2026-07-15

### Fixed

- Wired up the `BAMBOO_MCP_TOKEN`/`BAMBOO_MCP_API_KEY` (and username/
  password) environment-variable credential overrides, which were
  previously documented but had no effect at all.
- Stopped the setup wizard from writing a `config.json` the config loader
  never read; it now prompts for global vs. local persistence and writes
  YAML that's actually read back on startup.
- Fell back to the encrypted credential file when the OS keychain cleanly
  reports "not found", not only on a hard error.
- Resolved the home directory via `HOME` then `USERPROFILE` on Windows.
- `populate-embeddings` now defaults to populating and verifying every API
  version's store (row-count parity between endpoints and
  `semantic_endpoints`), failing loudly instead of silently
  under-populating; `search` now warns when a store is incomplete.

### Changed

- The `call` tool's `arguments` field now defaults to `{}` instead of
  `null`.

### Documentation

- Documented that `BAMBOO_MCP_URL` must include the `/rest` suffix.

## [0.4.0] - 2026-07-14

### Changed

- Adopted current mcpify Rust-target parity.

## [0.3.11] - 2026-07-13

### Documentation

- Documented real, worked `call` argument examples.

## [0.3.10] - 2026-07-10

### Fixed

- Fixed CI (`cargo fmt --check` drift) and cross-platform release-build
  failures: dropped the `x86_64-apple-darwin` release target (ONNX
  Runtime no longer ships prebuilt binaries for x64 macOS), disabled
  static CRT linking on Windows (incompatible with the prebuilt ONNX
  Runtime), and pinned Linux release builds to `ubuntu-24.04` for a
  new-enough glibc.

## [0.3.9] - 2026-07-10

### Changed

- Distributed prebuilt binaries via `cargo-dist`: cross-compiled release
  builds for macOS (x86_64/arm64), Linux (x86_64), and Windows, published
  to GitHub Releases with shell/PowerShell installer scripts. Split
  crates.io publishing and the GHCR image push into their own workflows.

## [0.3.8] - 2026-07-08

### Documentation

- Documented previously-undocumented subcommands (`test-connection`,
  `config`, `version`, `versions`), the `.env.example` configuration
  knobs, and the built-in observability/resilience stack (structured
  logging, OTel tracing, Prometheus metrics, circuit breaker/retry/rate
  limiting, health checks, OS-keychain credential storage).

## [0.3.7] - 2026-07-08

### Fixed

- Normalized stored credentials before use (deriving a usable
  `authorization_header` from a bare token where needed) and re-persisted
  the normalized form back to the credential store.

## [0.3.6] - 2026-07-06

### Fixed

- Wrote logs to stderr instead of stdout, so MCP clients strictly parsing
  stdout as JSON-RPC (e.g. VS Code) no longer tear down the connection on
  the first interleaved log line.

## [0.3.5] - 2026-07-06

### Fixed

- Installed the rustls crypto provider before first TLS use, fixing a
  panic ("No rustls crypto provider is configured") the first time the
  OTLP trace exporter built its HTTP client.

## [0.3.4] - 2026-07-06

### Fixed

- Backfilled embeddings for `mcp_store.db` so semantic search returns real
  ranked results instead of empty ones.

## [0.3.3] - 2026-07-06

### Fixed

- Store extraction now always overwrites the temp-dir copy on startup
  instead of skipping when a stale copy already exists, so a rebuild's
  updated embedded bytes are no longer silently ignored.
- `populate_embeddings` now accepts an explicit path, or `--all` to
  backfill every API version this project has a store for, instead of
  only ever touching `mcp_store.db`.

## [0.3.2] - 2026-07-06

### Fixed

- Dropped the store-path fallback chain (`CARGO_MANIFEST_DIR`, exe-dir
  guessing) in favor of resolving directly from the store bytes now
  embedded in the binary.

## [0.3.1] - 2026-07-06

### Fixed

- Embedded every supported version's store `.db` bytes directly into the
  compiled binary via `include_bytes!`, so an installed binary no longer
  depends on the build checkout it came from still existing on disk.
- Resolved the store db path relative to the running executable's
  directory when it isn't found in the current working directory.

## [0.3.0] - 2026-07-06

### Added

- Personal Access Token (PAT) bearer-token authentication, alongside the
  existing Basic auth, selectable in the setup wizard — Bamboo Data
  Center accepts a PAT even though the OpenAPI spec this server was
  generated from only declares Basic auth.

### Documentation

- Added a schema/db version source reference (`docs/SCHEMA_VERSIONS.md`).

## [0.2.3] - 2026-07-05

### Fixed

- Namespaced the `populate-embeddings` helper binary to
  `bamboo-mcp-populate-embeddings`, avoiding a name collision when
  multiple mcpify-generated servers are installed on the same `PATH`.

_(Note: 0.2.2 was never tagged/released — 0.2.3 is the next real release
after 0.2.1.)_

## [0.2.1] - 2026-07-05

_No user-facing changes (formatting only)._

## [0.2.0] - 2026-07-05

### Added

- Initial scaffold of the Bamboo MCP server, generated by mcpify: `search`/
  `get`/`call` tools backed by an embedded semantic database, stdio/HTTP
  transports, an interactive setup wizard, and coverage/profiling scripts.
- Published a versioned Docker image to GHCR on release.

### Fixed

- Resolved Docker build failures: switched `fastembed` to its rustls-TLS
  feature to drop a missing system OpenSSL dependency, moved the Docker
  build/runtime stages to Debian trixie for a new-enough glibc, and made
  `populate-embeddings`'s upsert idempotent against a `sqlite-vec` virtual
  table (which doesn't support `INSERT OR REPLACE`).
- Unblocked the crates.io publish: replaced the generator's placeholder
  license/repository metadata with real values, and renamed the package
  to `bamboo-mcp-rs` after discovering `bamboo-mcp` was already taken by
  an unrelated crate (the installed binary is still named `bamboo-mcp`).
  Also bumped several GitHub Actions to Node 24-native majors to silence
  Node 20 deprecation warnings.
