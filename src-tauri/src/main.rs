// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod agent;
mod file_embedding;

use agent::{Agent, AgentRegistry, AgentStatus};
use file_embedding::FileEmbeddingService;
use std::sync::Arc;
use std::path::Path;
use tauri::State;
use std::sync::Mutex;

#[tauri::command]
fn create_agent(registry: State<Arc<AgentRegistry>>, agent: Agent) -> Result<(), String> {
    registry.create_agent(agent)
}

#[tauri::command]
fn get_agent(registry: State<Arc<AgentRegistry>>, id: String) -> Result<Agent, String> {
    registry.get_agent(&id)
}

#[tauri::command]
fn list_agents(registry: State<Arc<AgentRegistry>>) -> Result<Vec<Agent>, String> {
    registry.list_agents()
}

#[tauri::command]
fn update_agent(registry: State<Arc<AgentRegistry>>, id: String, updated_agent: Agent) -> Result<(), String> {
    registry.update_agent(&id, updated_agent)
}

#[tauri::command]
fn delete_agent(registry: State<Arc<AgentRegistry>>, id: String) -> Result<(), String> {
    registry.delete_agent(&id)
}

#[tauri::command]
fn change_agent_status(registry: State<Arc<AgentRegistry>>, id: String, status: AgentStatus) -> Result<(), String> {
    registry.change_agent_status(&id, status)
}

fn main() {
    tauri::Builder::default()
        .manage(Arc::new(AgentRegistry::new()))
        .invoke_handler(tauri::generate_handler![
            create_agent,
            get_agent,
            list_agents,
            update_agent,
            delete_agent,
            change_agent_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn list_agents(registry: State<Arc<AgentRegistry>>) -> Result<Vec<Agent>, String> {
    registry.list_agents()
}

#[tauri::command]
fn update_agent(registry: State<Arc<AgentRegistry>>, id: String, updated_agent: Agent) -> Result<(), String> {
    registry.update_agent(&id, updated_agent)
}

#[tauri::command]
fn delete_agent(registry: State<Arc<AgentRegistry>>, id: String) -> Result<(), String> {
    registry.delete_agent(&id)
}

#[tauri::command]
fn upload_and_embed_file(
    embedding_service: State<Arc<Mutex<FileEmbeddingService>>>, 
    agent_id: String, 
    file_name: String, 
    file_content: Vec<u8>
) -> Result<(), String> {
    let mut service = embedding_service.lock().map_err(|_| "Failed to lock embedding service")?;
    service.upload_and_embed_file(&agent_id, &file_name, file_content)?;
    Ok(())
}

#[tauri::command]
fn list_agent_embeddings(
    embedding_service: State<Arc<Mutex<FileEmbeddingService>>>, 
    agent_id: String
) -> Result<Vec<String>, String> {
    let service = embedding_service.lock().map_err(|_| "Failed to lock embedding service")?;
    Ok(service
        .list_embeddings_for_agent(&agent_id)
        .into_iter()
        .map(|e| e.file_name.clone())
        .collect())
}

fn main() {
    let agent_registry = Arc::new(AgentRegistry::new());
    let embedding_service = Arc::new(Mutex::new(
        FileEmbeddingService::new(Path::new("./embeddings"))
    ));

    tauri::Builder::default()
        .manage(agent_registry)
        .manage(embedding_service)
        .invoke_handler(tauri::generate_handler![
            create_agent,
            get_agent,
            list_agents,
            update_agent,
            delete_agent,
            upload_and_embed_file,
            list_agent_embeddings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
