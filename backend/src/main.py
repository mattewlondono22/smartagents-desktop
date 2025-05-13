import os
from typing import List, Optional
from fastapi import FastAPI, File, UploadFile, HTTPException
from pydantic import BaseModel
import chromadb
from sentence_transformers import SentenceTransformer
import numpy as np

app = FastAPI(title="Windsurf Agent Backend")

# Initialize embedding model and vector database
embedding_model = SentenceTransformer('all-MiniLM-L6-v2')
chroma_client = chromadb.PersistentClient(path="./chroma_storage")

class Agent(BaseModel):
    id: str
    name: str
    description: Optional[str] = None
    capabilities: List[str] = []

class FileEmbedding(BaseModel):
    agent_id: str
    file_name: str
    embedding: List[float]

@app.post("/agents/", response_model=Agent)
async def create_agent(agent: Agent):
    """Create a new agent"""
    # In a real implementation, you'd save to a database
    return agent

@app.get("/agents/", response_model=List[Agent])
async def list_agents():
    """List all agents"""
    # Placeholder: return mock agents
    return [
        Agent(
            id="data_analyst", 
            name="Data Analyst", 
            description="Specializes in data analysis and visualization",
            capabilities=["data_processing", "visualization", "statistical_analysis"]
        )
    ]

@app.post("/upload/")
async def upload_and_embed_file(
    agent_id: str, 
    file: UploadFile = File(...)
):
    """Upload and embed a file for a specific agent"""
    try:
        # Read file content
        content = await file.read()
        text_content = content.decode('utf-8')

        # Generate embedding
        embedding = embedding_model.encode(text_content).tolist()

        # Create or get collection for the agent
        collection = chroma_client.get_or_create_collection(name=f"agent_{agent_id}")

        # Add embedding to the collection
        collection.add(
            embeddings=[embedding],
            documents=[text_content],
            ids=[file.filename]
        )

        return {
            "file_name": file.filename,
            "agent_id": agent_id,
            "status": "Embedded successfully"
        }
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

@app.get("/search/")
async def semantic_search(
    agent_id: str, 
    query: str, 
    top_k: int = 5
):
    """Perform semantic search across an agent's documents"""
    try:
        # Get agent's collection
        collection = chroma_client.get_collection(name=f"agent_{agent_id}")

        # Generate query embedding
        query_embedding = embedding_model.encode(query).tolist()

        # Perform semantic search
        results = collection.query(
            query_embeddings=[query_embedding],
            n_results=top_k
        )

        return {
            "query": query,
            "results": [
                {
                    "document": doc,
                    "metadata": metadata
                } 
                for doc, metadata in zip(results['documents'][0], results['metadatas'][0])
            ]
        }
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000)
