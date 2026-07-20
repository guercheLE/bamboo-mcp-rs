# Bamboo server administration

Covers server-wide configuration: general/security/mail/IM server settings, global variables,
artifact handlers (local, S3, SFTP, etc.), dark features, pausing/resuming the server, and node
status.

Search for what you want to do — e.g. `search for "how to set a global variable?"` or `search for
"how to pause the Bamboo server?"` — and read the schema `get` returns before relying on any
response field name.

This domain is server-wide, not scoped to any one project or plan — double-check with the user
before making a change here, since it can affect every build on the instance (e.g. pausing the
server stops all queued and running builds instance-wide, not just one plan's). If a request could
plausibly be scoped narrower (a single plan's variables rather than a global variable, a single
repository's artifact handling rather than the server default), confirm which the user actually
means before calling a server-wide operation.

**Build/branch retention (expiry).** Search for how to view or set the expiry configuration
(how long old build results and inactive branches are kept before automatic cleanup). There's also
an on-demand "run now" operation — treat it as destructive and irreversible (it deletes old build
data immediately rather than waiting for the schedule) and confirm with the user before calling it.

**Background scheduler.** Bamboo runs its own internal maintenance jobs; search for how to list
them or trigger one on demand if the user is troubleshooting something that only a scheduled job
would normally handle.

**Search index.** If search results elsewhere seem stale or wrong, search for how to check or
trigger a reindex — this is the operation `bamboo_workflow_search_reporting` points back here for.

**Elevated session required for some actions ("websudo").** Certain security-sensitive admin
calls (e.g. changing server security settings or user management) may be rejected until the
caller has an active elevated re-authentication session — this looks like a permissions error but
isn't one. If an admin-scoped call that should work is unexpectedly rejected, search for how to
check or refresh the elevated session before assuming the user lacks permission.

For agent-specific configuration, see `bamboo_workflow_agents_capabilities`; for who's allowed to
change server settings, see `bamboo_workflow_permissions` — permission changes are themselves one
of the actions the elevated-session requirement above tends to gate.
