# Bamboo search and reporting

For any read-only lookup — searching across plans, projects, branches, deployments, jobs, stages,
users, versions, or authors; using a quick filter; or pulling a chart/report — search for the
specific thing you want in plain language (e.g. `search for "how to search for plans by name?"` or
`search for "how to get a build results chart?"`) and read the schema `get` returns before relying
on any response field name. If the search or report could return a large result set, and your
environment supports running an isolated sub-task, delegate the listing and bring back only the
specific items that answer the user's question rather than the full payload.

If results look stale or wrong rather than just empty, that's a search-index problem, not a query
problem — the reindex operation lives in `bamboo-server-admin`, not here.
