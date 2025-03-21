use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Part {
    pub id: Option<i32>, // Cambiar de i32 a Option<i32> para permitir valores nulos
    pub name: String,
    pub stock: u32,
}
