use banking_feature_slices_example::{AppState, run};
use factstore_memory::MemoryStore;
use std::sync::Arc;
use tokio::{net::TcpListener, sync::Mutex, task::JoinHandle};

#[tokio::test]
async fn health_endpoint_returns_ok() {
    let (address, handle) = spawn_app().await;

    let response = reqwest::get(format!("http://{address}/health"))
        .await
        .expect("health request should succeed");

    assert!(response.status().is_success());
    assert_eq!(
        response
            .text()
            .await
            .expect("response body should be readable"),
        r#"{"status":"ok","service":"banking-feature-slices-example"}"#
    );

    handle.abort();
}

async fn spawn_app() -> (std::net::SocketAddr, JoinHandle<()>) {
    let app_state = AppState::from_memory_store(Arc::new(Mutex::new(MemoryStore::new())));
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("ephemeral listener should bind");
    let address = listener
        .local_addr()
        .expect("local address should be available");

    let handle = tokio::spawn(async move {
        run(listener, app_state).await.expect("server should run");
    });

    (address, handle)
}
