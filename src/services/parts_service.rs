use crate::models::pieza::Part;
use crate::repositories::parts_repository::PartsRepository;
use std::sync::Mutex;

pub struct PartsService {
    repository: Mutex<PartsRepository>,
}

impl PartsService {
    pub fn new() -> Self {
        PartsService {
            repository: Mutex::new(PartsRepository::new()),
        }
    }

    pub fn get_all_parts(&self) -> Vec<Part> {
        let repo = self.repository.lock().unwrap();
        repo.get_parts()
    }

    pub fn get_part_by_id(&self, id: i32) -> Option<Part> {
        let repo = self.repository.lock().unwrap();
        repo.find_by_id(id)
    }

    pub fn create_part(&self, name: String, stock: u32) -> Part {
        let mut repo = self.repository.lock().unwrap();
        let id = repo.parts.len() as i32 + 1;
        let part = Part {
            id: Some(id),
            name,
            stock,
        };
        repo.add(part.clone());
        part
    }
}
