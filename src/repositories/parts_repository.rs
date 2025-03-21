use crate::models::pieza::Part;

pub struct PartsRepository {
    pub parts: Vec<Part>,
}

impl PartsRepository {
    pub fn new() -> Self {
        let initial_parts = vec![
            Part {
                id: Some(1), // Asegurando que el `id` es `Some`
                name: "filtro de aire".to_string(),
                stock: 50,
            },
            Part {
                id: Some(2), // Asegurando que el `id` es `Some`
                name: "llanta 205".to_string(),
                stock: 50,
            },
        ];
        PartsRepository {
            parts: initial_parts,
        }
    }

    // Devuelve una lista de partes
    pub fn get_parts(&self) -> Vec<Part> {
        self.parts.clone() // Clonamos para evitar mutabilidad externa
    }

    // Buscar una parte por `id`
    pub fn find_by_id(&self, id: i32) -> Option<Part> {
        self.parts.iter().find(|part| part.id == Some(id)).cloned() // Asegurando que el `id` coincida
    }

    // Agregar una nueva parte
    pub fn add(&mut self, part: Part) {
        self.parts.push(part); // Empujar la nueva parte al repositorio
    }
}
