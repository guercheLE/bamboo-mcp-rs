use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{PromptMessage, Role};
use rmcp::prompt;
use rmcp::prompt_router;

use crate::core::mcp_server::McpifyServer;
use crate::prompts::{BuildsArgs, DeploymentsArgs, MasterWorkflowArgs, render_context_header};

/// One method per MCP prompt, mirroring the `#[tool_router]`-decorated
/// `search`/`get`/`call` block in `core::mcp_server` almost exactly. Kept
/// in its own `impl McpifyServer` block, never merged into the tool
/// block, so prompt and tool routing stay easy to tell apart at a glance.
/// `vis = pub`: unlike `#[tool_router]` (called from within the same
/// `mcp_server.rs` module it's defined in, where the macro's default
/// private visibility is enough), the generated `prompt_router()` fn is
/// called from `McpifyServer::new()` in a different module, so it needs
/// to be `pub` to be reachable from there.
#[prompt_router(vis = "pub")]
impl McpifyServer {
    #[prompt(
        name = "bamboo_workflow",
        description = "Start here. Presents the available Bamboo management workflows, \
                        routes to the right guided sub-workflow based on the user's goal, \
                        and — where the environment supports it — delegates that whole \
                        sub-workflow to an isolated sub-task to spare this conversation's \
                        context window."
    )]
    async fn bamboo_workflow_prompt(
        &self,
        Parameters(args): Parameters<MasterWorkflowArgs>,
    ) -> Vec<PromptMessage> {
        let header = render_context_header(&[("goal", args.goal.as_deref())]);
        vec![PromptMessage::new_text(
            Role::User,
            format!("{header}\n\n{}", include_str!("content/master.md")),
        )]
    }

    #[prompt(
        name = "bamboo_workflow_projects_plans",
        description = "Project and plan lifecycle: create/get/delete, enable/disable, \
                        favourites, labels, branches, plan/project variables, spec export."
    )]
    async fn bamboo_workflow_projects_plans_prompt(&self) -> Vec<PromptMessage> {
        vec![PromptMessage::new_text(
            Role::User,
            include_str!("content/projects_plans.md"),
        )]
    }

    #[prompt(
        name = "bamboo_workflow_builds",
        description = "Trigger a plan build via the queue, monitor its result, add \
                        comments/labels, manage broken-build responsibility."
    )]
    async fn bamboo_workflow_builds_prompt(
        &self,
        Parameters(args): Parameters<BuildsArgs>,
    ) -> Vec<PromptMessage> {
        let header = render_context_header(&[("plan_key", args.plan_key.as_deref())]);
        vec![PromptMessage::new_text(
            Role::User,
            format!("{header}\n\n{}", include_str!("content/builds.md")),
        )]
    }

    #[prompt(
        name = "bamboo_workflow_deployments",
        description = "Guided deployment-project → environment → version → trigger flow, \
                        including the fresh-vs-reused-version fork."
    )]
    async fn bamboo_workflow_deployments_prompt(
        &self,
        Parameters(args): Parameters<DeploymentsArgs>,
    ) -> Vec<PromptMessage> {
        let header = render_context_header(&[
            ("deployment_project", args.deployment_project.as_deref()),
            ("environment", args.environment.as_deref()),
            ("plan_key", args.plan_key.as_deref()),
        ]);
        vec![PromptMessage::new_text(
            Role::User,
            format!("{header}\n\n{}", include_str!("content/deployments.md")),
        )]
    }

    #[prompt(
        name = "bamboo_workflow_agents_capabilities",
        description = "Local/remote/elastic/ephemeral agent lifecycle, capabilities, agent \
                        assignment to jobs/environments."
    )]
    async fn bamboo_workflow_agents_capabilities_prompt(&self) -> Vec<PromptMessage> {
        vec![PromptMessage::new_text(
            Role::User,
            include_str!("content/agents_capabilities.md"),
        )]
    }

    #[prompt(
        name = "bamboo_workflow_permissions",
        description = "The repeated list/grant/revoke-for-users/groups/roles pattern across \
                        all seven permission-scoped resource kinds (global, project, plan, \
                        projectplan, deployment, environment, repository)."
    )]
    async fn bamboo_workflow_permissions_prompt(&self) -> Vec<PromptMessage> {
        vec![PromptMessage::new_text(
            Role::User,
            include_str!("content/permissions.md"),
        )]
    }

    #[prompt(
        name = "bamboo_workflow_repositories",
        description = "Linked-repository lifecycle: registration, connection test, specs \
                        scanning, cross-repository/project access grants."
    )]
    async fn bamboo_workflow_repositories_prompt(&self) -> Vec<PromptMessage> {
        vec![PromptMessage::new_text(
            Role::User,
            include_str!("content/repositories.md"),
        )]
    }

    #[prompt(
        name = "bamboo_workflow_users_groups",
        description = "User and group administration, access tokens, group membership, \
                        session management."
    )]
    async fn bamboo_workflow_users_groups_prompt(&self) -> Vec<PromptMessage> {
        vec![PromptMessage::new_text(
            Role::User,
            include_str!("content/users_groups.md"),
        )]
    }

    #[prompt(
        name = "bamboo_workflow_server_admin",
        description = "Server-wide configuration: general/security/mail/IM settings, global \
                        variables, artifact handlers, dark features, pause/resume, node status."
    )]
    async fn bamboo_workflow_server_admin_prompt(&self) -> Vec<PromptMessage> {
        vec![PromptMessage::new_text(
            Role::User,
            include_str!("content/server_admin.md"),
        )]
    }

    #[prompt(
        name = "bamboo_workflow_search_reporting",
        description = "Thin pointer to the right read-only signal (search across \
                        plans/projects/branches/deployments/jobs/stages/users/versions/authors, \
                        quick filters, charts/reports)."
    )]
    async fn bamboo_workflow_search_reporting_prompt(&self) -> Vec<PromptMessage> {
        vec![PromptMessage::new_text(
            Role::User,
            include_str!("content/search_reporting.md"),
        )]
    }
}
