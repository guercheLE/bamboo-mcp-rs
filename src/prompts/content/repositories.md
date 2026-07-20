# Bamboo repositories

Covers linked-repository lifecycle: registering a new linked repository, testing its connection,
scanning it for build specs, and granting cross-repository or cross-project access to it.

Search for what you want to do — e.g. `search for "how to register a linked repository?"` or
`search for "how to test a repository's connection?"` — and read the schema `get` returns before
relying on any response field name, since this crate only warns (never fails) on an output-schema
mismatch precisely because the underlying spec's response documentation is sometimes wrong.

One real gotcha: registering a repository and successfully *connecting* to it are separate steps —
a repository can be created with bad credentials or an unreachable URL and still "succeed" at the
create call. Always follow a create with a connection-test search-and-call, and don't tell the
user the repository is ready until that test passes.

If the goal is to grant a specific user or group access to a repository (rather than manage the
repository resource itself), see `bamboo_workflow_permissions`, which covers the same
list/grant/revoke pattern for repository-scoped permissions specifically.
