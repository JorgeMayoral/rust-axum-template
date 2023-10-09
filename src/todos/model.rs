use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

impl Todo {
    pub fn new(id: u32, title: String, completed: bool) -> Self {
        Self {
            id,
            title,
            completed,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}
