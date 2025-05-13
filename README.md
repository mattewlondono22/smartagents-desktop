# SmartAgents Desktop

## Overview
SmartAgents is a cross-platform desktop application built with Tauri and Svelte, bringing intelligent agent capabilities to your desktop.

## Features
- Agent CRUD operations
- File upload and embedding pipeline
- Plugin registry
- Onboarding wizard

## Prerequisites
- Rust (latest stable version)
- Node.js (v16 or later)
- npm or yarn

## Setup
1. Clone the repository
2. Install dependencies:
   ```bash
   cd frontend
   npm install
   cd ../src-tauri
   cargo build
   ```

## Development
Run the application in development mode:
```bash
cargo tauri dev
```

## Build for Production
Create a production build:
```bash
cargo tauri build
```

## Technology Stack
- Tauri (Rust desktop framework)
- Svelte (Frontend UI)
- Python backend services (optional)
   ```bash
   rustup update
   ```

3. Install frontend dependencies:
   ```bash
   cd frontend
   npm install
   ```

## Development
Run the application in development mode:
```bash
cargo tauri dev
```

## Building for Production
Create a production build:
```bash
cargo tauri build
```

## Features
- Agent CRUD operations
- File upload and embedding
- Plugin registry
- Onboarding wizard

## Tech Stack
- Tauri (Rust desktop framework)
- Svelte (Frontend UI)
- Optional Python backend services
