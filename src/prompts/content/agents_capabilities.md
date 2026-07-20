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

**Assigning an agent to a specific job.** This is a separate, more direct mechanism than
capability matching above — search for how to find agents currently assigned to a job, find which
agents are eligible to be assigned, add an assignment, or remove one. If the user wants a job to
run on a specific agent every time rather than whichever eligible agent picks it up next, this is
the operation to use, not a capability requirement change.

**Per-job Docker pipeline configuration** is a related job-level setting — search for how to get
or set it if the user's job runs in a Docker-based build environment.

For granting a user or group permission to assign/manage agents, see `bamboo_workflow_permissions`
instead.
