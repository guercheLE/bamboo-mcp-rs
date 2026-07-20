# Bamboo projects and plans

Covers the project/plan lifecycle: create/get/delete a project or plan, enable/disable a plan,
mark favourites, manage labels, list/create branches, set plan or project variables, and export a
plan's spec.

For each of these, search for what you want to do in plain language — e.g. `search for "how to
create a new plan in a project?"` or `search for "how to set a plan variable?"` — and read the
schema `get` returns for the resolved operationId before relying on any field name in its
response. Never call an operationId directly from memory or from this prompt: Bamboo's REST
surface repeats many list/create/delete shapes across resource kinds with numerically-suffixed
`operationId`s (e.g. `_1`, `_2`, ...) that only mean something once resolved via `search`.

One real gotcha: a plan's key is immutable once created and is derived from its project key plus a
short plan key you choose at creation time — confirm both with the user before calling create,
since there's no rename operation for it afterward.

If the user wants to list or inspect many plans/projects at once and your environment supports
running an isolated sub-task, delegate that listing and bring back only the resolved names/keys
that matter, rather than pulling a large listing payload into this conversation.

For triggering or monitoring an actual build, or for deployment setup, see
`bamboo_workflow_builds` and `bamboo_workflow_deployments` instead — this prompt only covers the
project/plan resources themselves.
