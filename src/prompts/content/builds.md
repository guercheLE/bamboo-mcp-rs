# Bamboo builds

Covers triggering and monitoring a single plan's builds: queue a build, check a build result,
add a comment or label to one, and manage broken-build responsibility (who's on the hook for a
failing plan).

Check the "Context already provided" section above for `plan_key` before asking the user for it.

Search for what you want to do — e.g. `search for "how to queue a build for a plan?"` or
`search for "how to get the result of a build?"` — and read the schema `get` returns before
relying on any response field name, for the same reasons as every other Bamboo workflow here:
response shapes in the underlying spec aren't always reliable, and near-duplicate operationIds are
disambiguated only by a numeric suffix that means nothing without resolving it through `search`
first.

**Don't report success from the queue call alone.** Queuing a build only starts it; the actual
result (successful/failed/still running) has to be polled separately. If the user wants to know
the outcome, search for how to get a build result and poll its status until it reaches a terminal
state before summarizing.

If the plan doesn't exist yet, or the user wants to change what a plan does rather than just build
it, see `bamboo_workflow_projects_plans`. To deploy what this build produces, see
`bamboo_workflow_deployments`, which takes this build's plan key as one of its own parameters.
