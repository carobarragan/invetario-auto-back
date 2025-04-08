use crate::models::parts::{Part, PartsError};
use async_trait::async_trait;
use std::path::Path;
use std::sync::Arc;
use tokio::fs::{self, File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[async_trait]
pub trait PartsRepositoryTrait {
    async fn get_all(&self) -> Result<Vec<Part>, PartsError>;
    async fn get_by_id(&self, id: u32) -> Result<Part, PartsError>;
    async fn save_all(&self, parts: Vec<Part>) -> Result<(), PartsError>;
}

pub struct PartsRepository;

impl Default for PartsRepository {
    fn default() -> Self {
        PartsRepository {}
    }
}

#[async_trait]
impl PartsRepositoryTrait for PartsRepository {
    async fn get_all(&self) -> Result<Vec<Part>, PartsError> {
        let path = "data/parts.json";

        // Asegurarse de que el directorio exista
        if let Some(parent) = Path::new(path).parent() {
            fs::create_dir_all(parent).await.map_err(PartsError::Io)?;
        }

        // Abrir o crear el archivo
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .await
            .map_err(PartsError::Io)?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .await
            .map_err(PartsError::Io)?;

        // Si el archivo está vacío, escribir []
        if contents.trim().is_empty() {
            file.write_all(b"[]").await.map_err(PartsError::Io)?;
            return Ok(Vec::new());
        }

        let parts: Vec<Part> = serde_json::from_str(&contents).map_err(PartsError::Json)?;
        Ok(parts)
    }

    async fn get_by_id(&self, id: u32) -> Result<Part, PartsError> {
        let parts = self.get_all().await?;
        parts
            .into_iter()
            .find(|part| part.id == id)
            .ok_or(PartsError::NotFound)
    }

    async fn save_all(&self, parts: Vec<Part>) -> Result<(), PartsError> {
        let path = "data/parts.json";

        // Asegurarse de que el directorio exista
        if let Some(parent) = Path::new(path).parent() {
            fs::create_dir_all(parent).await.map_err(PartsError::Io)?;
        }

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)
            .await
            .map_err(PartsError::Io)?;

        let contents = serde_json::to_string_pretty(&parts).map_err(PartsError::Json)?;

        file.write_all(contents.as_bytes())
            .await
            .map_err(PartsError::Io)?;

        Ok(())
    }
}

pub type DynPartsRepository = Arc<dyn PartsRepositoryTrait + Send + Sync>;
