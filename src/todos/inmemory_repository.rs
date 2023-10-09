use std::collections::HashMap;

use super::model::Todo;

#[derive(Default)]
pub struct Repository {
    todos: HashMap<u32, Todo>,
}

impl Repository {
    pub fn new(hashmap: HashMap<u32, Todo>) -> Self {
        Self { todos: hashmap }
    }

    pub fn add(&mut self, todo: Todo) {
        self.todos.insert(todo.id(), todo);
    }

    pub fn all(&self) -> Vec<Todo> {
        self.todos.values().cloned().collect::<Vec<Todo>>()
    }
}
