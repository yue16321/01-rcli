use axum::Router;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::{info, warn};

#[derive(Clone, Debug)]
pub struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> anyhow::Result<()> {
    info!("Serving {:?} on port {}", path, port);
    let state = HttpServeState { path };

    let router = Router::new()
        .route("/{*path}", get(file_handler))
        .with_state(Arc::new(state));
    let listener = tokio::net::TcpListener::bind(("127.0.0.1", port)).await?;
    axum::serve(listener, router).await?;
    Ok(())
}
async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let p = std::path::Path::new(&state.path).join(path);
    info!("Reading file {:?}", p);
    if !p.exists() {
        return (
            StatusCode::NOT_FOUND,
            format!("File does not exist: {:?}", p.display()),
        );
    }
    match tokio::fs::read_to_string(&p).await {
        Ok(content) => {
            info!("Read {} bytes", content.len());
            (StatusCode::OK, content)
        }
        Err(e) => {
            warn!("Error reading file {:?}: {:?}", p, e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        }
    }
}
