use std::sync::{Arc, RwLock};

use axum::{
    routing::{delete, get, post, put},
    Router,
};

use super::{
    controller::{add_todo, delete_todo, get_all_todos, get_todo, update_todo},
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
        .route("/:id", get(get_todo))
        .route("/:id", put(update_todo))
        .route("/:id", delete(delete_todo))
        .with_state(state)
}
