use super::model::Todo;

pub trait Repository {
    fn add(&mut self, todo: Todo);
    fn all(&self) -> Vec<Todo>;
    fn get(&self, id: u32) -> Option<Todo>;
    fn update(&mut self, todo: Todo);
    fn delete(&mut self, id: u32);
}

pub type TodoRepository = Box<dyn Repository + Send + Sync + 'static>;
