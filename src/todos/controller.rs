use std::sync::{Arc, RwLock};

use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};

use super::{model::Todo, router::TodosState};

#[tracing::instrument(skip(payload, state))]
pub async fn add_todo(
    State(state): State<Arc<RwLock<TodosState>>>,
    Json(payload): Json<Todo>,
) -> StatusCode {
    match state.write() {
        Ok(mut state) => {
            state.repository.add(payload);
            StatusCode::CREATED
        }
        Err(e) => {
            tracing::error!("Failed to get mutable reference to repository: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

#[tracing::instrument(skip(state))]
pub async fn get_all_todos(
    State(state): State<Arc<RwLock<TodosState>>>,
) -> Result<(StatusCode, Json<Vec<Todo>>), StatusCode> {
    match state.read() {
        Ok(state) => {
            let todos = state.repository.all();
            Ok((StatusCode::OK, Json(todos)))
        }
        Err(e) => {
            tracing::error!("Failed to get reference to repository: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[tracing::instrument(skip(state))]
pub async fn get_todo(
    Path(id): Path<u32>,
    State(state): State<Arc<RwLock<TodosState>>>,
) -> Result<(StatusCode, Json<Todo>), StatusCode> {
    match state.read() {
        Ok(state) => match state.repository.get(id) {
            Some(todo) => Ok((StatusCode::OK, Json(todo))),
            None => Err(StatusCode::NOT_FOUND),
        },
        Err(e) => {
            tracing::error!("Failed to get reference to repository: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[tracing::instrument(skip(payload, state))]
pub async fn update_todo(
    Path(id): Path<u32>,
    State(state): State<Arc<RwLock<TodosState>>>,
    Json(payload): Json<Todo>,
) -> StatusCode {
    match state.write() {
        Ok(mut state) => {
            state.repository.update(payload);
            StatusCode::OK
        }
        Err(e) => {
            tracing::error!("Failed to get mutable reference to repository: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

#[tracing::instrument(skip(state))]
pub async fn delete_todo(
    Path(id): Path<u32>,
    State(state): State<Arc<RwLock<TodosState>>>,
) -> StatusCode {
    match state.write() {
        Ok(mut state) => {
            state.repository.delete(id);
            StatusCode::NO_CONTENT
        }
        Err(e) => {
            tracing::error!("Failed to get mutable reference to repository: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
