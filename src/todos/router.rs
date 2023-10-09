use std::sync::{Arc, RwLock};

use axum::{
    routing::{get, post},
    Router,
};

use super::{
    controller::{add_todo, get_all_todos},
    inmemory_repository::Repository,
};

#[derive()]
pub struct TodosState {
    pub repository: Repository,
}

pub fn todos_router() -> Router {
    let state = TodosState {
        repository: Repository::default(),
    };
    let state = RwLock::new(state);
    let state = Arc::new(state);
    Router::new()
        .route("/", get(get_all_todos))
        .route("/", post(add_todo))
        .with_state(state)
}
