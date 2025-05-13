<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import { open } from '@tauri-apps/api/dialog';
    import { readBinaryFile } from '@tauri-apps/api/fs';
    import { onMount } from 'svelte';

    let files: File[] = [];
    let embeddingStatus: { [key: string]: string } = {};
    let selectedAgent: string = '';
    let agents: Array<{id: string, name: string}> = [];

    async function selectFiles() {
        try {
            const selected = await open({
                multiple: true,
                filters: [{
                    name: 'Documents',
                    extensions: ['txt', 'pdf', 'docx', 'md']
                }]
            });

            if (selected) {
                const selectedFiles = Array.isArray(selected) ? selected : [selected];
                files = await Promise.all(
                    selectedFiles.map(async (path) => {
                        const content = await readBinaryFile(path);
                        return new File([content], path.split('/').pop() || 'unknown');
                    })
                );
            }
        } catch (error) {
            console.error('Error selecting files:', error);
        }
    }

    async function uploadAndEmbed() {
        if (!selectedAgent) {
            alert('Please select an agent');
            return;
        }

        for (const file of files) {
            try {
                embeddingStatus[file.name] = 'Processing...';
                const fileContent = await file.arrayBuffer();
                
                const result = await invoke('upload_and_embed_file', {
                    agentId: selectedAgent,
                    fileName: file.name,
                    fileContent: Array.from(new Uint8Array(fileContent))
                });

                embeddingStatus[file.name] = 'Embedded successfully';
            } catch (error) {
                console.error(`Error processing ${file.name}:`, error);
                embeddingStatus[file.name] = 'Embedding failed';
            }
        }
    }

    async function fetchAgents() {
        try {
            agents = await invoke('list_agents');
        } catch (error) {
            console.error('Failed to fetch agents:', error);
        }
    }

    onMount(fetchAgents);
</script>

<div class="file-upload-container">
    <h1>File Upload & Embedding</h1>

    <div class="agent-selection">
        <label for="agent-select">Select Agent:</label>
        <select 
            id="agent-select" 
            bind:value={selectedAgent}
        >
            <option value="">Choose an Agent</option>
            {#each agents as agent}
                <option value={agent.id}>{agent.name}</option>
            {/each}
        </select>
    </div>

    <div class="file-selection">
        <button on:click={selectFiles}>Select Files</button>
        {#if files.length > 0}
            <div class="file-list">
                <h3>Selected Files:</h3>
                {#each files as file}
                    <div class="file-item">
                        {file.name} 
                        {#if embeddingStatus[file.name]}
                            <span class="status">
                                {embeddingStatus[file.name]}
                            </span>
                        {/if}
                    </div>
                {/each}
            </div>
        {/if}
    </div>

    {#if files.length > 0 && selectedAgent}
        <button 
            class="upload-button" 
            on:click={uploadAndEmbed}
        >
            Upload and Embed
        </button>
    {/if}
</div>

<style>
    .file-upload-container {
        max-width: 600px;
        margin: 0 auto;
        padding: 20px;
        display: flex;
        flex-direction: column;
        gap: 20px;
    }

    .agent-selection, .file-selection {
        display: flex;
        flex-direction: column;
        gap: 10px;
    }

    .file-list {
        border: 1px solid #ddd;
        padding: 10px;
        max-height: 200px;
        overflow-y: auto;
    }

    .file-item {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 5px 0;
    }

    .status {
        font-size: 0.8em;
        color: #666;
    }

    button {
        padding: 10px;
        background-color: #4CAF50;
        color: white;
        border: none;
        cursor: pointer;
        transition: background-color 0.3s;
    }

    button:hover {
        background-color: #45a049;
    }
</style>
