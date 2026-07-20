# Bamboo workflows: start here

This is the menu for guided, multi-step Bamboo management workflows. Match the user's goal (or
the `goal` argument above, if supplied) to one of the sub-workflows below, then fetch that
prompt by name (`prompts/get`).

**Delegate the whole matched sub-workflow if you can.** If your environment provides a way to run
a sub-task/agent in an isolated context, delegate the entire matched sub-workflow to it — hand the
sub-task the prompt name and whatever parameters are already known, let it fetch that prompt
itself and carry out every one of its steps (including all of its own `search`/`get`/`call`
traffic) entirely within its own context, and have it report back only a short summary: what was
accomplished/confirmed, and anything it still needs from the user. This is what keeps a
multi-step workflow's full tool-call trace out of this conversation. Only run the sub-workflow's
steps directly here if no such delegation mechanism is available.

## Sub-workflows

- **`bamboo-projects-plans`** — create, inspect, or manage a build project or plan
  (enable/disable, favourites, labels, branches, variables, spec export).
- **`bamboo-builds`** — trigger a plan build, check its result, comment/label it, or
  manage broken-build responsibility.
- **`bamboo-deployments`** — set up or run a deployment: project → environment →
  version → trigger, including first-time environment setup.
- **`bamboo-agents-capabilities`** — manage local/remote/elastic/ephemeral build agents
  and their capabilities, or assign agents to jobs/environments.
- **`bamboo-permissions`** — grant, revoke, or list permissions for users/groups/roles
  on any resource kind (global, project, plan, deployment, environment, repository).
- **`bamboo-repositories`** — register or manage a linked repository, test its
  connection, scan for build specs, or grant cross-repository/project access.
- **`bamboo-users-groups`** — administer users and groups, access tokens, group
  membership, or sessions.
- **`bamboo-server-admin`** — server-wide configuration: general/security/mail/IM
  settings, global variables, artifact handlers, dark features, pause/resume, node status.
- **`bamboo-search-reporting`** — find the right read-only signal: search across
  plans/projects/branches/deployments/jobs/stages/users/versions/authors, quick filters, or
  charts/reports.

If the user's goal doesn't clearly match one sub-workflow (e.g. it spans several, or is genuinely
ambiguous), ask a short clarifying question rather than guessing which one to delegate to.
