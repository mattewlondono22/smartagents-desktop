use std::path::{Path, PathBuf};
use std::fs;
use serde::{Deserialize, Serialize};
use crate::agent::Agent;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileEmbedding {
    pub agent_id: String,
    pub file_name: String,
    pub file_path: PathBuf,
    pub embedding_vector: Vec<f32>,
}

pub struct FileEmbeddingService {
    base_storage_path: PathBuf,
    embeddings: Vec<FileEmbedding>,
}

impl FileEmbeddingService {
    pub fn new(base_path: &Path) -> Self {
        fs::create_dir_all(base_path).expect("Failed to create base storage directory");
        
        FileEmbeddingService {
            base_storage_path: base_path.to_path_buf(),
            embeddings: Vec::new(),
        }
    }

    pub fn upload_and_embed_file(
        &mut self, 
        agent_id: &str, 
        file_name: &str, 
        file_content: Vec<u8>
    ) -> Result<FileEmbedding, String> {
        // Create agent-specific directory
        let agent_dir = self.base_storage_path.join(agent_id);
        fs::create_dir_all(&agent_dir).map_err(|e| format!("Failed to create agent directory: {}", e))?;

        // Save file
        let file_path = agent_dir.join(file_name);
        fs::write(&file_path, &file_content)
            .map_err(|e| format!("Failed to save file: {}", e))?;

        // Simulate embedding generation (replace with actual embedding logic)
        let embedding_vector = self.generate_embedding(&file_content);

        let embedding = FileEmbedding {
            agent_id: agent_id.to_string(),
            file_name: file_name.to_string(),
            file_path,
            embedding_vector,
        };

        self.embeddings.push(embedding.clone());
        Ok(embedding)
    }

    fn generate_embedding(&self, file_content: &[u8]) -> Vec<f32> {
        // Placeholder: In a real implementation, use an ML model like BERT or sentence transformers
        // For now, we'll generate a simple random vector
        use rand::Rng;
        let mut rng = rand::thread_rng();
        (0..128).map(|_| rng.gen()).collect()
    }

    pub fn list_embeddings_for_agent(&self, agent_id: &str) -> Vec<&FileEmbedding> {
        self.embeddings
            .iter()
            .filter(|e| e.agent_id == agent_id)
            .collect()
    }
}
