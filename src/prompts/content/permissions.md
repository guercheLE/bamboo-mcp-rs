# Bamboo permissions

Covers the same list/grant/revoke-for-users/groups/roles pattern, repeated across seven
permission-scoped resource kinds: global, project, plan, projectplan, deployment, environment, and
repository. It's not seven different operations to learn — it's one pattern, applied to whichever
resource kind and resource id the user names.

Always start by confirming **which resource kind and which specific resource** (e.g. "plan
permissions for plan `PROJ-PLAN`", not just "plan permissions") — this store's `operationId`s for
this exact pattern are disambiguated only by a bare numeric suffix (e.g. `listGroupPermissions`
through `listGroupPermissions_6`, one per resource kind), so guessing the wrong suffix silently
operates on the wrong resource kind. Search for the task in plain language including the resource
kind — e.g. `search for "how to list group permissions on a plan?"` or `search for "how to grant
a user permission on a deployment project?"` — and confirm the resolved operationId's schema (via
`get`) actually matches the resource kind you intend before calling it.

If the user wants to review permissions across many resources of the same kind, and your
environment supports running an isolated sub-task, delegate that listing and bring back only the
users/groups/roles that actually have access, not the full per-resource payload.

For agent-related permissions specifically, see `bamboo_workflow_agents_capabilities`; for
repository access grants, see `bamboo_workflow_repositories`.
