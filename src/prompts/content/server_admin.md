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

For agent-specific configuration, see `bamboo_workflow_agents_capabilities`; for who's allowed to
change server settings, see `bamboo_workflow_permissions`.
