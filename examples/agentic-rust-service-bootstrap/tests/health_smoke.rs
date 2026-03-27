use parcel_locker_operations_service::build_router;

#[tokio::test]
async fn health_endpoint_returns_ok() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind test listener");
    let address = listener.local_addr().expect("read listener address");
    let server = tokio::spawn(async move {
        axum::serve(listener, build_router())
            .await
            .expect("run test server");
    });

    let response = reqwest::get(format!("http://{address}/health"))
        .await
        .expect("call /health");

    assert!(response.status().is_success());
    assert_eq!(
        response.text().await.expect("read response body"),
        r#"{"status":"ok"}"#
    );

    server.abort();
}
