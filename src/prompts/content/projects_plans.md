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

**Cloning a plan.** If the user wants a new plan that starts out the same as an existing one,
search for how to clone a plan rather than recreating it field by field.

**Chained builds.** If the user wants one plan to build automatically after another one succeeds,
this is a plan dependency, not a plan setting — search for how to find or set parent/child plan
dependencies rather than assuming it's configured on the triggering plan alone.

**Masked/secret variables.** Before storing a secret as a plan or project variable, search for how
to encrypt a value first — plain-text variables are visible to anyone who can view the plan's
configuration.

For triggering or monitoring an actual build, or for deployment setup, see
`bamboo_workflow_builds` and `bamboo_workflow_deployments` instead — this prompt only covers the
project/plan resources themselves.
