# Bamboo agents and capabilities

Covers the build-agent lifecycle across all four agent kinds Bamboo supports — local, remote,
elastic, and ephemeral — plus their capabilities and their assignment to jobs/environments.

Search for what you want to do rather than assuming an operation name — e.g. `search for "how to
list remote agents?"`, `search for "how to enable an elastic agent?"`, or `search for "how to
assign a capability requirement to a job?"` — then read the schema `get` returns before relying on
any field name. Which agent kind you're working with matters: an operation that applies to remote
agents often has a differently-scoped counterpart for elastic or ephemeral agents, and the two are
easy to confuse if you go by operation name alone rather than confirming via `search` + `get`.

One real gotcha: a job or environment's *capability requirements* (what it needs) are a separate
resource from an *agent's capabilities* (what it has) — matching them is how Bamboo decides which
agents are eligible to run a given job, but setting one side doesn't touch the other. If the user
wants a job to run on a specific agent, check whether the mismatch is on the job's requirements or
the agent's capabilities before changing either.

For granting a user or group permission to assign/manage agents, see `bamboo_workflow_permissions`
instead.
