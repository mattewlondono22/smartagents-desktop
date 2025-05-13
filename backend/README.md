# Windsurf Backend Microservices

## Overview
Optional Python/FastAPI microservices for the Windsurf Agent Studio. 
Provides extensible backend services with placeholders for:
- Agent management
- Tool/plugin registry
- File embedding pipeline
- Onboarding wizard configuration

## Features
- Lightweight FastAPI microservice
- Placeholder endpoints for core Agent Studio features
- Easy to extend and customize

## Prerequisites
- Python 3.10+
- pip

## Local Development Setup
1. Create a virtual environment
```bash
python3 -m venv venv
source venv/bin/activate
```

2. Install dependencies
```bash
pip install -r requirements.txt
```

3. Run the application
```bash
uvicorn app:app --reload
```

## Key Endpoints
- `GET /agents`: List agents
- `POST /agents`: Create an agent
- `GET /tools`: List available tools
- `POST /tools`: Register a tool
- `POST /file/embed`: Placeholder for file embedding
- `GET /onboarding/steps`: Retrieve onboarding wizard steps

## Extensibility
This backend is designed as a flexible, optional microservice. 
Easily extend the functionality by adding new endpoints or 
implementing more complex logic in the existing routes.

## Integration
The backend can be called via HTTP from the Tauri Rust 
application or used as a standalone service.
