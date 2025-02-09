use std::sync::{Arc, RwLock};

use axum::{
    routing::{delete, get, post, put},
    Router,
};

use super::{
    controller::{add_todo, delete_todo, get_all_todos, get_todo, update_todo},
    repository::TodoRepository,
};

pub fn todos_router(todo_repository: TodoRepository) -> Router {
    let state = RwLock::new(todo_repository);
    let state = Arc::new(state);
    Router::new()
        .route("/", get(get_all_todos))
        .route("/", post(add_todo))
        .route("/{id}", get(get_todo))
        .route("/{id}", put(update_todo))
        .route("/{id}", delete(delete_todo))
        .with_state(state)
}
