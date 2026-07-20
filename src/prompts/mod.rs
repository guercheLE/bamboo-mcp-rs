pub mod router;

use rmcp::schemars;
use serde::Deserialize;

/// Arguments for the `bamboo_workflow` master prompt. `goal` is free-text
/// so the calling LLM can describe what it's trying to do in its own
/// words; the prompt's own routing prose matches it to a sub-workflow.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct MasterWorkflowArgs {
    /// What you're trying to accomplish, in your own words (e.g. "set up
    /// a new deployment pipeline"). Omit to see the full menu instead.
    pub goal: Option<String>,
}

/// Arguments for `bamboo_workflow_builds`.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct BuildsArgs {
    /// The plan key whose build you want to trigger, inspect, or manage
    /// (e.g. `PROJ-PLAN`).
    pub plan_key: Option<String>,
}

/// Arguments for `bamboo_workflow_deployments`.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DeploymentsArgs {
    /// Name of the deployment project, if one already exists.
    pub deployment_project: Option<String>,
    /// Name of the target deployment environment.
    pub environment: Option<String>,
    /// The build plan key whose artifacts should be deployed.
    pub plan_key: Option<String>,
}

/// Renders a short "context already provided" section from whichever
/// prompt arguments the caller already supplied, so a sub-workflow's
/// static markdown body (pulled in via `include_str!`) never has to be
/// templated at runtime — this header is just prepended to it. Every
/// entry is `(argument_name, Some(value))` if the caller supplied it, or
/// `(argument_name, None)` if it's still missing and the prompt's own
/// prose should ask the user for it before proceeding.
pub(crate) fn render_context_header(args: &[(&str, Option<&str>)]) -> String {
    let supplied: Vec<_> = args
        .iter()
        .filter_map(|(name, value)| value.map(|v| (*name, v)))
        .collect();
    let missing: Vec<_> = args
        .iter()
        .filter(|(_, value)| value.is_none())
        .map(|(name, _)| *name)
        .collect();

    let mut sections = Vec::new();
    if !supplied.is_empty() {
        let mut lines = vec!["## Context already provided".to_string()];
        lines.extend(
            supplied
                .iter()
                .map(|(name, value)| format!("- {name}: {value}")),
        );
        sections.push(lines.join("\n"));
    }
    if !missing.is_empty() {
        let mut lines = vec!["## Still needed from the user".to_string()];
        lines.extend(missing.iter().map(|name| format!("- {name}")));
        sections.push(lines.join("\n"));
    }
    sections.join("\n\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_slice_renders_nothing() {
        assert_eq!(render_context_header(&[]), "");
    }

    #[test]
    fn all_supplied_renders_only_the_provided_section() {
        let header = render_context_header(&[("goal", Some("deploy my project"))]);
        assert_eq!(
            header,
            "## Context already provided\n- goal: deploy my project"
        );
    }

    #[test]
    fn all_missing_renders_only_the_needed_section() {
        let header = render_context_header(&[("plan_key", None)]);
        assert_eq!(header, "## Still needed from the user\n- plan_key");
    }

    #[test]
    fn mixed_renders_both_sections_supplied_first() {
        let header = render_context_header(&[
            ("deployment_project", Some("payments")),
            ("environment", None),
            ("plan_key", Some("PAY-MAIN")),
        ]);
        assert_eq!(
            header,
            "## Context already provided\n\
             - deployment_project: payments\n\
             - plan_key: PAY-MAIN\n\n\
             ## Still needed from the user\n\
             - environment"
        );
    }
}
