// Minimal axum-based server used by RustyWeb apps.
// This is the runtime the CLI runs during `rustyweb run`.
use axum::{
    routing::{get, post},
    Router, extract::Form, response::{Html, IntoResponse}, http::StatusCode,
};
use std::net::SocketAddr;
use tera::Tera;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    templates: Arc<Tera>,
}

#[tokio::main]
async fn main() {
    // basic template loader: loads templates from ./templates dir
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            std::process::exit(1);
        }
    };
    let state = AppState { templates: Arc::new(tera) };

    let app = Router::new()
        .route("/", get(index))
        .route("/api/add", post(api_add))
        .with_state(state);

    let addr = SocketAddr::from(([127,0,0,1], 3000));
    println!("Listening on http://{}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

async fn index(state: axum::extract::State<AppState>) -> impl IntoResponse {
    let mut ctx = tera::Context::new();
    ctx.insert("title", "Welcome to RustyWeb");
    let s = state.templates.render("index.html", &ctx).unwrap_or_else(|e| e.to_string());
    Html(s)
}

#[derive(Deserialize)]
struct AddForm {
    a: i32,
    b: i32,
}

async fn api_add(Form(payload): Form<AddForm>) -> impl IntoResponse {
    let sum = payload.a + payload.b;
    (StatusCode::OK, serde_json::json!({ "result": sum }))
}
