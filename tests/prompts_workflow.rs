// Protocol-level `prompts/list`/`prompts/get` integration tests, kept out
// of `src/core/mcp_server.rs`'s existing `#[cfg(test)] mod tests` (which
// stays scoped to `search`/`get`/`call`) — mirrors the same
// duplex-transport + `ClientHandler`-stub pattern
// `mcp_protocol_routes_search_get_and_call_requests` already uses there,
// promoted to its own top-level integration-test file/compilation unit.

use std::sync::Arc;

use bamboo_mcp::auth::auth_manager::AuthManager;
use bamboo_mcp::core::config_schema::{AuthMethod, Config};
use bamboo_mcp::core::mcp_server::McpifyServer;
use rmcp::model::{ContentBlock, GetPromptRequestParams, ProtocolVersion};
use rmcp::{ClientHandler, ServiceExt};
use tokio::sync::Mutex;

#[derive(Debug, Clone, Default)]
struct TestClient;

impl ClientHandler for TestClient {}

fn server() -> McpifyServer {
    let config: Config = serde_json::from_value(serde_json::json!({
        "url": "https://api.example.test",
        "auth_method": "basic"
    }))
    .unwrap();
    McpifyServer::new(
        "default".to_string(),
        config,
        Arc::new(Mutex::new(AuthManager::new(AuthMethod::Basic))),
    )
}

#[test]
fn server_info_advertises_the_prompts_capability() {
    use rmcp::ServerHandler;
    let info = server().get_info();
    assert_eq!(info.protocol_version, ProtocolVersion::V_2024_11_05);
    assert!(info.capabilities.prompts.is_some());
}

#[tokio::test]
async fn prompts_list_advertises_all_ten_prompts_and_deployments_arguments() {
    let (server_transport, client_transport) = tokio::io::duplex(64 * 1024);
    let server_task = tokio::spawn(async move {
        server().serve(server_transport).await?.waiting().await?;
        anyhow::Ok(())
    });
    let client = TestClient.serve(client_transport).await.unwrap();

    let prompts = client.list_all_prompts().await.unwrap();
    let mut names: Vec<&str> = prompts.iter().map(|p| p.name.as_str()).collect();
    names.sort_unstable();
    assert_eq!(
        names,
        [
            "bamboo",
            "bamboo-agents-capabilities",
            "bamboo-builds",
            "bamboo-deployments",
            "bamboo-permissions",
            "bamboo-projects-plans",
            "bamboo-repositories",
            "bamboo-search-reporting",
            "bamboo-server-admin",
            "bamboo-users-groups",
        ]
    );

    let deployments = prompts
        .iter()
        .find(|p| p.name == "bamboo-deployments")
        .expect("bamboo-deployments must be advertised");
    let arguments = deployments
        .arguments
        .as_ref()
        .expect("bamboo-deployments must advertise its arguments");
    let mut argument_names: Vec<&str> = arguments.iter().map(|a| a.name.as_str()).collect();
    argument_names.sort_unstable();
    assert_eq!(
        argument_names,
        ["deployment_project", "environment", "plan_key"]
    );
    assert!(
        arguments.iter().all(|a| a.required == Some(false)),
        "every prompt argument must be optional, never required — a strict client would \
         otherwise refuse prompts/get until it's filled, defeating the ask-if-missing design"
    );

    drop(client);
    tokio::time::timeout(std::time::Duration::from_secs(2), server_task)
        .await
        .unwrap()
        .unwrap()
        .unwrap();
}

fn first_text(result: &rmcp::model::GetPromptResult) -> &str {
    match &result
        .messages
        .first()
        .expect("prompt result must have at least one message")
        .content
    {
        ContentBlock::Text(text_content) => text_content.text.as_str(),
        _ => panic!("expected a text content block"),
    }
}

#[tokio::test]
async fn master_prompt_links_to_the_deployments_sub_workflow() {
    let (server_transport, client_transport) = tokio::io::duplex(64 * 1024);
    let server_task = tokio::spawn(async move {
        server().serve(server_transport).await?.waiting().await?;
        anyhow::Ok(())
    });
    let client = TestClient.serve(client_transport).await.unwrap();

    let result = client
        .get_prompt(GetPromptRequestParams::new("bamboo"))
        .await
        .unwrap();
    assert!(first_text(&result).contains("bamboo-deployments"));

    drop(client);
    tokio::time::timeout(std::time::Duration::from_secs(2), server_task)
        .await
        .unwrap()
        .unwrap()
        .unwrap();
}

#[tokio::test]
async fn deployments_prompt_echoes_supplied_args_and_lists_the_missing_one() {
    let (server_transport, client_transport) = tokio::io::duplex(64 * 1024);
    let server_task = tokio::spawn(async move {
        server().serve(server_transport).await?.waiting().await?;
        anyhow::Ok(())
    });
    let client = TestClient.serve(client_transport).await.unwrap();

    let result = client
        .get_prompt(
            GetPromptRequestParams::new("bamboo-deployments").with_arguments(
                serde_json::json!({
                    "deployment_project": "payments",
                    "plan_key": "PAY-MAIN"
                })
                .as_object()
                .unwrap()
                .clone(),
            ),
        )
        .await
        .unwrap();
    let text = first_text(&result);
    assert!(text.contains("deployment_project: payments"));
    assert!(text.contains("plan_key: PAY-MAIN"));
    assert!(text.contains("environment"));

    drop(client);
    tokio::time::timeout(std::time::Duration::from_secs(2), server_task)
        .await
        .unwrap()
        .unwrap()
        .unwrap();
}
