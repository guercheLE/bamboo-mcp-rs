# Bamboo users and groups

Covers user and group administration: creating/inspecting/deleting users and groups, managing
group membership, issuing or revoking personal access tokens, and managing active sessions.

Search for what you want to do — e.g. `search for "how to add a user to a group?"` or `search for
"how to create a personal access token for a user?"` — and read the schema `get` returns before
relying on any response field name.

One real gotcha: removing a user from a group doesn't revoke permissions granted to that user
*directly* (outside the group) — if the goal is "fully lock this user out," check both the user's
direct permission grants and their group memberships, not just one or the other. Direct-permission
review is `bamboo-permissions`'s job, not this prompt's — treat the two as complementary
steps for a full lockout, not duplicated work.

For access tokens specifically, confirm with the user whether they want the token scoped to their
own account or to a service/bot user before creating one — Bamboo doesn't let you widen a token's
scope after creation, only revoke and reissue it.

This domain also covers the current-authenticated-user lookup, avatar upload/retrieval, and
admin-forced session invalidation (ending a specific named user's active sessions, e.g. after
disabling their account) — search for these the same way as any other operation here.

If a user- or group-management call that should work is unexpectedly rejected, this may be the
elevated-session ("websudo") requirement described in `bamboo-server-admin`, not an
actual permissions gap — check there before assuming the caller lacks permission.
