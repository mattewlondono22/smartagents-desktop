use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum AgentStatus {
    Active,
    Inactive,
    Suspended,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub description: String,
    pub capabilities: Vec<String>,
    pub status: AgentStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Default)]
pub struct AgentRegistry {
    agents: Mutex<HashMap<String, Agent>>,
}

impl AgentRegistry {
    pub fn new() -> Self {
        AgentRegistry {
            agents: Mutex::new(HashMap::new()),
        }
    }

    pub fn create_agent(&self, mut agent: Agent) -> Result<(), String> {
        let mut agents = self.agents.lock().map_err(|_| "Failed to lock agents")?;
        
        // Generate a unique ID if not provided
        if agent.id.is_empty() {
            agent.id = Uuid::new_v4().to_string();
        }

        // Set timestamps
        let now = Utc::now();
        agent.created_at = now;
        agent.updated_at = now;

        // Set default status if not specified
        if agent.status == AgentStatus::Inactive {
            agent.status = AgentStatus::Active;
        }

        // Check for duplicate names
        if agents.values().any(|a| a.name == agent.name) {
            return Err(format!("An agent with the name '{}' already exists", agent.name));
        }

        agents.insert(agent.id.clone(), agent);
        Ok(())
    }

    pub fn get_agent(&self, id: &str) -> Result<Agent, String> {
        let agents = self.agents.lock().map_err(|_| "Failed to lock agents")?;
        agents.get(id).cloned().ok_or_else(|| format!("Agent with ID {} not found", id))
    }

    pub fn list_agents(&self) -> Result<Vec<Agent>, String> {
        let agents = self.agents.lock().map_err(|_| "Failed to lock agents")?;
        Ok(agents.values().cloned().collect())
    }

    pub fn update_agent(&self, id: &str, mut updated_agent: Agent) -> Result<(), String> {
        let mut agents = self.agents.lock().map_err(|_| "Failed to lock agents")?;
        
        if let Some(agent) = agents.get_mut(id) {
            // Preserve the original created_at timestamp
            updated_agent.created_at = agent.created_at;
            // Update the updated_at timestamp
            updated_agent.updated_at = Utc::now();
            
            *agent = updated_agent;
            Ok(())
        } else {
            Err(format!("Agent with ID {} not found", id))
        }
    }

    pub fn delete_agent(&self, id: &str) -> Result<(), String> {
        let mut agents = self.agents.lock().map_err(|_| "Failed to lock agents")?;
        
        if agents.remove(id).is_some() {
            Ok(())
        } else {
            Err(format!("Agent with ID {} not found", id))
        }
    }

    pub fn change_agent_status(&self, id: &str, status: AgentStatus) -> Result<(), String> {
        let mut agents = self.agents.lock().map_err(|_| "Failed to lock agents")?;
        
        if let Some(agent) = agents.get_mut(id) {
            agent.status = status;
            agent.updated_at = Utc::now();
            Ok(())
        } else {
            Err(format!("Agent with ID {} not found", id))
        }
    }
}
