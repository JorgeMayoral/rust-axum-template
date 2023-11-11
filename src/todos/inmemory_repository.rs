use std::collections::HashMap;

use super::{model::Todo, repository::Repository};

#[derive(Default)]
pub struct InMemoryRepository {
    todos: HashMap<u32, Todo>,
}

impl InMemoryRepository {
    pub fn new(hashmap: HashMap<u32, Todo>) -> Self {
        Self { todos: hashmap }
    }
}

impl Repository for InMemoryRepository {
    fn add(&mut self, todo: Todo) {
        self.todos.insert(todo.id(), todo);
    }

    fn all(&self) -> Vec<Todo> {
        self.todos.values().cloned().collect::<Vec<Todo>>()
    }

    fn get(&self, id: u32) -> Option<Todo> {
        self.todos.get(&id).cloned()
    }

    fn update(&mut self, todo: Todo) {
        self.todos.insert(todo.id(), todo);
    }

    fn delete(&mut self, id: u32) {
        self.todos.remove(&id);
    }
}
