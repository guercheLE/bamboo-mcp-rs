# Bamboo deployments: project → environment → version → trigger

This sub-workflow is designed to be run as an isolated sub-task where possible — if you were
delegated here from `bamboo_workflow`'s routing, or your environment otherwise supports running
this as its own sub-task, everything you need is in this prompt's own text plus the parameters
already listed above. Report back only a short summary when done (what was triggered/confirmed,
and anything still needed from the user) rather than the full step-by-step trace.

Every operation below should be reached by searching for what you need to do in plain language —
e.g. `search for "how to get a deployment project by plan key?"` — never by calling a specific
`operationId` directly. This crate's `call` tool only warns, never fails, on an output-schema
mismatch precisely because the underlying spec's documented response shapes are sometimes wrong,
and this store's `operationId`s frequently disambiguate near-identical operations with a bare
numeric suffix (e.g. `listGroupPermissions` through `listGroupPermissions_6`) that carries no
meaning on its own. Always read the schema `get` returns for the operationId `search` resolves to
before relying on any field name in its response.

## Step 0 — gather required parameters

Check the "Context already provided" section above first; only ask the user for whichever of
`deployment_project`, `environment`, and `plan_key` (the build plan whose artifacts get deployed)
is still listed as missing. Don't proceed to Step 1 until all three are known.

## Step 1 — does a deployment project already exist for this plan?

This is a genuine fork, not a guess — ask the user if it isn't already obvious from context:

- **(A) It already exists.** Search for how to look up deployment projects for a plan, resolve its
  id, and confirm the named environment exists on it.
- **(B) It doesn't exist yet.** It must be created — linked to `plan_key` — along with at least one
  environment before anything else in this workflow can proceed.

## Step 2 — configure the environment (parallelizable, delegate if possible)

Once the deployment project and environment are confirmed to exist, configuring the environment's
permissions, its variables, and its agent/requirement constraints are three independent sub-steps —
none depends on the others, only later steps depend on all three being settled. Do them
concurrently rather than one at a time.

If your environment provides a way to run a sub-task in its own context (e.g. an agent/task tool),
delegate "set environment permissions", "set environment variables", and "set environment
requirements" as three separate sub-tasks and have each return only a short confirmation — don't
pull full listing/config payloads into this conversation. If no such sub-task mechanism is
available, just do the three calls directly here.

Gate: don't proceed until each configured setting is confirmed via a follow-up search-and-call, not
just "the call didn't error."

## Step 3 — resolve the deployment version

Another fork: (A) reuse the latest completed build result of `plan_key` as the version's source, or
(B) point the version at a specific, user-named build result. Ask which applies if it isn't already
implied by context. Gated on confirming the version now exists for the deployment project.

## Step 4 — trigger the deployment

Using the environment (Step 1/2) and version (Step 3) resolved above, search for how to trigger a
deployment and call it. Don't proceed until both are confirmed to exist — a trigger against an
unresolved environment or version will fail or, worse, target the wrong one.

## Step 5 — poll the result and summarize

A deployment result takes time to finish. Poll its status until it reaches a terminal state (don't
report success from "the trigger call returned" alone), then summarize the outcome for the user and
offer next actions — retry, or promote to the next environment.

## Composing with other workflows

The source plan and its build artifacts come from `bamboo_workflow_projects_plans` and
`bamboo_workflow_builds`; environment permission scopes overlap with `bamboo_workflow_permissions`.
Fetch those prompts by name for more detail rather than assuming their content here.
