use axum :: {response::IntoResponse, routing::get, Json, Router};

async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Basic attempt to wrap my head around Rust backend structures";

    let json_response = serde_json :: json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

#[tokio::main]
async fn main(){
    let app = Router::new().route("/api/healthcheck", get(health_check_handler));

    println!("Server has started successfully");
    axum::Server::bind(&"0.0.0.0:5000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}