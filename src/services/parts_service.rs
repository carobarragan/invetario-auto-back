use crate::models::parts::{Part, PartsError};
use crate::repositories::parts_repository::{DynPartsRepository, PartsRepositoryTrait};
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait PartsServiceTrait {
    async fn get_all(&self) -> Result<Vec<Part>, PartsError>;
    async fn get_by_id(&self, id: u32) -> Result<Part, PartsError>;
    async fn add(&self, new_part: Part) -> Result<Part, PartsError>;
    async fn update(&self, id: u32, updated_part: Part) -> Result<Part, PartsError>;
    async fn delete(&self, id: u32) -> Result<(), PartsError>;
}
#[derive(Clone)]
pub struct PartsService {
    repository: DynPartsRepository,
}

impl PartsService {
    pub fn new(repository: DynPartsRepository) -> Self {
        PartsService { repository }
    }
}

impl Default for PartsService {
    fn default() -> Self {
        let repository = Arc::new(crate::repositories::parts_repository::PartsRepository::default())
            as DynPartsRepository;
        PartsService { repository }
    }
}

#[async_trait]
impl PartsServiceTrait for PartsService {
    async fn get_all(&self) -> Result<Vec<Part>, PartsError> {
        self.repository.get_all().await
    }

    async fn get_by_id(&self, id: u32) -> Result<Part, PartsError> {
        self.repository.get_by_id(id).await
    }

    async fn add(&self, mut new_part: Part) -> Result<Part, PartsError> {
        let parts = self.repository.get_all().await?;
        let new_id = parts.iter().map(|part| part.id).max().unwrap_or(0) + 1;
        new_part.id = new_id;

        // Validar los datos
        if new_part.name.trim().is_empty() {
            return Err(PartsError::InvalidData(
                "El nombre es obligatorio".to_string(),
            ));
        }
        if new_part.category.trim().is_empty() {
            return Err(PartsError::InvalidData(
                "La categoría es obligatoria".to_string(),
            ));
        }
        if new_part.price <= 0.0 {
            return Err(PartsError::InvalidData(
                "El precio debe ser mayor que 0".to_string(),
            ));
        }
        if new_part.brand.trim().is_empty() {
            return Err(PartsError::InvalidData(
                "La marca es obligatoria".to_string(),
            ));
        }
        if !["Nuevo", "Usado"].contains(&new_part.condition.as_str()) {
            return Err(PartsError::InvalidData(
                "El estado debe ser 'Nuevo' o 'Usado'".to_string(),
            ));
        }

        let mut parts = self.repository.get_all().await?;
        parts.push(new_part.clone());
        self.repository.save_all(parts).await?;
        Ok(new_part)
    }

    async fn update(&self, id: u32, updated_part: Part) -> Result<Part, PartsError> {
        let mut parts = self.repository.get_all().await?;
        let index = parts
            .iter()
            .position(|part| part.id == id)
            .ok_or(PartsError::NotFound)?;

        let mut part_to_update = updated_part;
        part_to_update.id = id;

        // Validar los datos
        if part_to_update.name.trim().is_empty() {
            return Err(PartsError::InvalidData(
                "El nombre es obligatorio".to_string(),
            ));
        }
        if part_to_update.category.trim().is_empty() {
            return Err(PartsError::InvalidData(
                "La categoría es obligatoria".to_string(),
            ));
        }
        if part_to_update.price <= 0.0 {
            return Err(PartsError::InvalidData(
                "El precio debe ser mayor que 0".to_string(),
            ));
        }
        if part_to_update.brand.trim().is_empty() {
            return Err(PartsError::InvalidData(
                "La marca es obligatoria".to_string(),
            ));
        }
        if !["Nuevo", "Usado"].contains(&part_to_update.condition.as_str()) {
            return Err(PartsError::InvalidData(
                "El estado debe ser 'Nuevo' o 'Usado'".to_string(),
            ));
        }

        parts[index] = part_to_update.clone();
        self.repository.save_all(parts).await?;
        Ok(part_to_update)
    }

    async fn delete(&self, id: u32) -> Result<(), PartsError> {
        let mut parts = self.repository.get_all().await?;
        let initial_len = parts.len();
        parts.retain(|part| part.id != id);

        if parts.len() == initial_len {
            return Err(PartsError::NotFound);
        }

        self.repository.save_all(parts).await?;
        Ok(())
    }
}

pub type DynPartsService = Arc<dyn PartsServiceTrait + Send + Sync>;
