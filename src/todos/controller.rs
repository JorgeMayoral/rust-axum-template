use std::sync::{Arc, RwLock};

use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};

use super::{model::Todo, repository::TodoRepository};

#[tracing::instrument(skip(payload, repo))]
pub async fn add_todo(
    State(repo): State<Arc<RwLock<TodoRepository>>>,
    Json(payload): Json<Todo>,
) -> StatusCode {
    match repo.write() {
        Ok(mut repo) => {
            repo.add(payload);
            StatusCode::CREATED
        }
        Err(e) => {
            tracing::error!("Failed to get mutable reference to repository: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

#[tracing::instrument(skip(repo))]
pub async fn get_all_todos(
    State(repo): State<Arc<RwLock<TodoRepository>>>,
) -> Result<(StatusCode, Json<Vec<Todo>>), StatusCode> {
    match repo.read() {
        Ok(repo) => {
            let todos = repo.all();
            Ok((StatusCode::OK, Json(todos)))
        }
        Err(e) => {
            tracing::error!("Failed to get reference to repository: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[tracing::instrument(skip(repo))]
pub async fn get_todo(
    Path(id): Path<u32>,
    State(repo): State<Arc<RwLock<TodoRepository>>>,
) -> Result<(StatusCode, Json<Todo>), StatusCode> {
    match repo.read() {
        Ok(repo) => match repo.get(id) {
            Some(todo) => Ok((StatusCode::OK, Json(todo))),
            None => Err(StatusCode::NOT_FOUND),
        },
        Err(e) => {
            tracing::error!("Failed to get reference to repository: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[tracing::instrument(skip(payload, repo))]
pub async fn update_todo(
    Path(id): Path<u32>,
    State(repo): State<Arc<RwLock<TodoRepository>>>,
    Json(payload): Json<Todo>,
) -> StatusCode {
    match repo.write() {
        Ok(mut repo) => {
            repo.update(payload);
            StatusCode::OK
        }
        Err(e) => {
            tracing::error!("Failed to get mutable reference to repository: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

#[tracing::instrument(skip(repo))]
pub async fn delete_todo(
    Path(id): Path<u32>,
    State(repo): State<Arc<RwLock<TodoRepository>>>,
) -> StatusCode {
    match repo.write() {
        Ok(mut repo) => {
            repo.delete(id);
            StatusCode::NO_CONTENT
        }
        Err(e) => {
            tracing::error!("Failed to get mutable reference to repository: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
