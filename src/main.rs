use {tower_http::trace::TraceLayer, tracing_subscriber::EnvFilter};

mod api_types;
mod error;
mod game_state;
mod router;
mod planner;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("blaze_haskell=debug,tower_http=info"))
                .unwrap(),
        )
        .init();
    let app = router::router().layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
