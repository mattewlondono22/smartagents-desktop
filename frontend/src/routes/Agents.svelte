<script lang="ts">
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/tauri';

    interface Agent {
        id: string;
        name: string;
        description?: string;
        capabilities: string[];
    }

    let agents: Agent[] = [];
    let newAgent: Partial<Agent> = { capabilities: [] };

    async function fetchAgents() {
        try {
            agents = await invoke('list_agents');
        } catch (error) {
            console.error('Failed to fetch agents:', error);
        }
    }

    async function createAgent() {
        try {
            await invoke('create_agent', { agent: newAgent });
            await fetchAgents();
            newAgent = { capabilities: [] };
        } catch (error) {
            console.error('Failed to create agent:', error);
        }
    }

    async function deleteAgent(id: string) {
        try {
            await invoke('delete_agent', { id });
            await fetchAgents();
        } catch (error) {
            console.error('Failed to delete agent:', error);
        }
    }

    onMount(fetchAgents);
</script>

<div class="agents-container">
    <h1>Agent Management</h1>

    <div class="create-agent-form">
        <input 
            type="text" 
            placeholder="Agent ID" 
            bind:value={newAgent.id}
        />
        <input 
            type="text" 
            placeholder="Agent Name" 
            bind:value={newAgent.name}
        />
        <textarea 
            placeholder="Description (optional)" 
            bind:value={newAgent.description}
        ></textarea>
        <input 
            type="text" 
            placeholder="Capabilities (comma-separated)" 
            on:change={(e) => {
                newAgent.capabilities = (e.target as HTMLInputElement).value
                    .split(',')
                    .map(cap => cap.trim())
                    .filter(cap => cap);
            }}
        />
        <button on:click={createAgent}>Create Agent</button>
    </div>

    <div class="agents-list">
        {#each agents as agent}
            <div class="agent-card">
                <h3>{agent.name}</h3>
                <p>ID: {agent.id}</p>
                {#if agent.description}
                    <p>Description: {agent.description}</p>
                {/if}
                <p>Capabilities: {agent.capabilities.join(', ')}</p>
                <button on:click={() => deleteAgent(agent.id)}>Delete</button>
            </div>
        {/each}
    </div>
</div>

<style>
    .agents-container {
        max-width: 800px;
        margin: 0 auto;
        padding: 20px;
    }

    .create-agent-form {
        display: flex;
        flex-direction: column;
        gap: 10px;
        margin-bottom: 20px;
    }

    .agents-list {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
        gap: 15px;
    }

    .agent-card {
        border: 1px solid #ddd;
        padding: 15px;
        border-radius: 8px;
    }
</style>
