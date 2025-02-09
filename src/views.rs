use axum::{extract::State, response::IntoResponse, routing::get, Router};
use axum_template::{engine::Engine, Key, RenderHtml};
use serde::Serialize;
use tera::Tera;

pub type ViewsEngine = Engine<Tera>;

#[derive(Clone)]
struct ViewsState {
    engine: ViewsEngine,
}

#[derive(Debug, Serialize)]
struct Data {}

#[tracing::instrument(skip(state))]
async fn get_index(State(state): State<ViewsState>, Key(key): Key) -> impl IntoResponse {
    let ViewsState { engine } = state;
    let data = Data {};
    RenderHtml(key, engine, data)
}

pub fn views_router() -> Router {
    let mut tera = Tera::default();
    tera.add_template_file("templates/index.html", Some("/"))
        .unwrap();
    let engine = Engine::from(tera);
    Router::new()
        .route("/", get(get_index))
        .with_state(ViewsState { engine })
}
