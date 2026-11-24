
use axum::{
    extract::{Path, Query, State},
    http::{header, Method, StatusCode},
    response::Json,
    routing::get,
    Router,
};
use moviebox_tui::{
    providers::models::ProviderKind,
    service::MovieBoxService,
};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
struct ApiState {
    service: Arc<MovieBoxService>,
}

#[derive(Deserialize)]
struct SearchQuery {
    q: String,
    #[serde(default = "default_provider")]
    provider: String,
    #[serde(default = "default_page")]
    page: usize,
}
fn default_provider() -> String { "moviebox".to_string() }
fn default_page() -> usize { 1 }

#[derive(Deserialize)]
struct HomeQuery {
    #[serde(default = "default_tab")]
    tab: String,
    #[serde(default = "default_page")]
    page: usize,
}
fn default_tab() -> String { "1".to_string() }

#[derive(Serialize)]
struct ApiError {
    error: String,
}

async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "service": "MovieBox Catalog API",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

async fn search(
    State(state): State<ApiState>,
    Query(params): Query<SearchQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ApiError>)> {
    let provider = ProviderKind::parse(&params.provider).ok_or_else(|| err(
        StatusCode::BAD_REQUEST, "Unknown provider. Try moviebox, 4khdhub, dramachi, bdix_circleftp or bdix_dhakaflix."
    ))?;
    let q = params.q.trim();
    if q.is_empty() {
        return Err(err(StatusCode::BAD_REQUEST, "The q search parameter is required."));
    }
    let page = params.page.clamp(1, 100);
    let items = state.service.search_typed(provider, q, page).await
        .map_err(|e| err(StatusCode::BAD_GATEWAY, &e.to_string()))?;
    Ok(Json(serde_json::json!({
        "provider": provider.cache_key(),
        "query": q,
        "page": page,
        "count": items.len(),
        "results": items
    })))
}

async fn home(
    State(state): State<ApiState>,
    Query(params): Query<HomeQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ApiError>)> {
    let page = params.page.clamp(1, 100);
    let (items, metrics) = state.service.homepage(&params.tab, page).await
        .map_err(|e| err(StatusCode::BAD_GATEWAY, &e))?;
    Ok(Json(serde_json::json!({
        "tab": params.tab,
        "page": page,
        "count": items.len(),
        "results": items,
        "metrics": metrics
    })))
}

async fn details(
    State(state): State<ApiState>,
    Path((provider_name, id)): Path<(String, String)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ApiError>)> {
    let provider = ProviderKind::parse(&provider_name).ok_or_else(|| err(
        StatusCode::BAD_REQUEST, "Unknown provider."
    ))?;
    let result = state.service.details_typed(provider, &id).await
        .map_err(|e| err(StatusCode::BAD_GATEWAY, &e.to_string()))?;
    Ok(Json(serde_json::json!({ "result": result })))
}

fn err(status: StatusCode, message: &str) -> (StatusCode, Json<ApiError>) {
    (status, Json(ApiError { error: message.to_string() }))
}

#[tokio::main]
async fn main() {
    let state = ApiState { service: Arc::new(MovieBoxService::new()) };
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE]);
    let app = Router::new()
        .route("/", get(health))
        .route("/health", get(health))
        .route("/v1/home", get(home))
        .route("/v1/search", get(search))
        .route("/v1/details/{provider}/{id}", get(details))
        .with_state(state)
        .layer(cors);

    let port = std::env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(8080);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await.expect("bind API listener");
    println!("MovieBox Catalog API listening on {addr}");
    axum::serve(listener, app).await.expect("serve API");
}
