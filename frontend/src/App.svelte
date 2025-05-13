<script>
  import { invoke } from '@tauri-apps/api/tauri';
  import { onMount } from 'svelte';

  // Agent status enum
  const AgentStatus = {
    Active: 'Active',
    Inactive: 'Inactive',
    Suspended: 'Suspended'
  };

  // State variables
  let agents = [];
  let newAgent = { 
    name: '', 
    description: '', 
    capabilities: [],
    status: AgentStatus.Active 
  };
  let editingAgent = null;
  let error = null;

  // Agent management functions
  async function createAgent() {
    try {
      await invoke('create_agent', { agent: newAgent });
      newAgent = { 
        name: '', 
        description: '', 
        capabilities: [],
        status: AgentStatus.Active 
      };
      error = null;
      loadAgents();
    } catch (errorMsg) {
      error = errorMsg;
      console.error('Failed to create agent:', errorMsg);
    }
  }

  async function loadAgents() {
    try {
      agents = await invoke('list_agents');
    } catch (errorMsg) {
      error = errorMsg;
      console.error('Failed to load agents:', errorMsg);
    }
  }

  async function deleteAgent(id) {
    try {
      await invoke('delete_agent', { id });
      loadAgents();
    } catch (errorMsg) {
      error = errorMsg;
      console.error('Failed to delete agent:', errorMsg);
    }
  }

  async function updateAgent() {
    if (!editingAgent) return;

    try {
      await invoke('update_agent', { 
        id: editingAgent.id, 
        updated_agent: editingAgent 
      });
      editingAgent = null;
      loadAgents();
    } catch (errorMsg) {
      error = errorMsg;
      console.error('Failed to update agent:', errorMsg);
    }
  }

  async function changeAgentStatus(id, status) {
    try {
      await invoke('change_agent_status', { id, status });
      loadAgents();
    } catch (errorMsg) {
      error = errorMsg;
      console.error('Failed to change agent status:', errorMsg);
    }
  }

  // Lifecycle
  onMount(loadAgents);

  // Utility functions
  function startEditing(agent) {
    editingAgent = { ...agent };
  }

  function addCapability() {
    if (editingAgent) {
      editingAgent.capabilities = [...editingAgent.capabilities, ''];
    }
  }

  function removeCapability(index) {
    if (editingAgent) {
      editingAgent.capabilities = editingAgent.capabilities.filter((_, i) => i !== index);
    }
  }
</script>

<main>
  <h1>SmartAgents</h1>
  
  {#if error}
    <div class="error-banner">
      <p>{error}</p>
      <button on:click={() => error = null}>Dismiss</button>
    </div>
  {/if}

  <section class="create-agent">
    <h2>Create New Agent</h2>
    <form on:submit|preventDefault={createAgent}>
      <input 
        type="text" 
        bind:value={newAgent.name} 
        placeholder="Agent Name" 
        required 
      />
      <textarea 
        bind:value={newAgent.description} 
        placeholder="Agent Description" 
        required
      ></textarea>
      <div class="capabilities">
        <h3>Capabilities</h3>
        {#each newAgent.capabilities as capability, index}
          <div class="capability-input">
            <input 
              type="text" 
              bind:value={newAgent.capabilities[index]} 
              placeholder="Capability"
            />
            <button type="button" on:click={() => newAgent.capabilities.splice(index, 1)}>
              Remove
            </button>
          </div>
        {/each}
        <button type="button" on:click={() => newAgent.capabilities = [...newAgent.capabilities, '']}>
          Add Capability
        </button>
      </div>
      <button type="submit">Create Agent</button>
    </form>
  </section>

  <section class="agent-list">
    <h2>Existing Agents</h2>
    {#if agents.length === 0}
      <p>No agents created yet.</p>
    {:else}
      <ul>
        {#each agents as agent}
          <li class={`agent-card ${agent.status.toLowerCase()}`}>
            {#if editingAgent && editingAgent.id === agent.id}
              <!-- Editing mode -->
              <form on:submit|preventDefault={updateAgent}>
                <input 
                  type="text" 
                  bind:value={editingAgent.name} 
                  placeholder="Agent Name" 
                  required 
                />
                <textarea 
                  bind:value={editingAgent.description} 
                  placeholder="Agent Description" 
                  required
                ></textarea>
                
                <div class="capabilities">
                  <h3>Capabilities</h3>
                  {#each editingAgent.capabilities as capability, index}
                    <div class="capability-input">
                      <input 
                        type="text" 
                        bind:value={editingAgent.capabilities[index]} 
                        placeholder="Capability"
                      />
                      <button type="button" on:click={() => removeCapability(index)}>
                        Remove
                      </button>
                    </div>
                  {/each}
                  <button type="button" on:click={addCapability}>
                    Add Capability
                  </button>
                </div>

                <div class="status-select">
                  <label for="agent-status">Agent Status:</label>
                  <select 
                    id="agent-status"
                    bind:value={editingAgent.status}
                    aria-label="Select agent status"
                  >
                    {#each Object.values(AgentStatus) as status}
                      <option value={status}>{status}</option>
                    {/each}
                  </select>
                </div>

                <div class="edit-actions">
                  <button type="submit">Save Changes</button>
                  <button type="button" on:click={() => editingAgent = null}>Cancel</button>
                </div>
              </form>
            {:else}
              <!-- View mode -->
              <div class="agent-header">
                <h3>{agent.name}</h3>
                <span class={`status ${agent.status.toLowerCase()}`}>{agent.status}</span>
              </div>
              
              <p>{agent.description || 'No description'}</p>
              
              {#if agent.capabilities && agent.capabilities.length > 0}
                <div class="capabilities">
                  <h4>Capabilities:</h4>
                  <ul>
                    {#each agent.capabilities as capability}
                      <li>{capability}</li>
                    {/each}
                  </ul>
                </div>
              {/if}
              
              <div class="agent-actions">
                <button on:click={() => startEditing(agent)}>Edit</button>
                <button on:click={() => deleteAgent(agent.id)}>Delete</button>
                
                <div class="status-actions">
                  {#each Object.values(AgentStatus).filter(s => s !== agent.status) as status}
                    <button 
                      class={`status-change ${status.toLowerCase()}`}
                      on:click={() => changeAgentStatus(agent.id, status)}
                    >
                      Set {status}
                    </button>
                  {/each}
                </div>
              </div>
            {/if}
          </li>
        {/each}
      </ul>
    {/if}
  </section>
</main>

<style>
  main {
    max-width: 800px;
    margin: 0 auto;
    padding: 20px;
    font-family: Arial, sans-serif;
  }

  .error-banner {
    background-color: #ffdddd;
    color: #ff0000;
    padding: 10px;
    margin-bottom: 20px;
    border-radius: 5px;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  form {
    display: flex;
    flex-direction: column;
    gap: 10px;
    margin-bottom: 20px;
  }

  input, textarea, select {
    padding: 10px;
    margin-bottom: 10px;
  }

  button {
    padding: 10px;
    margin: 5px 0;
    border: none;
    border-radius: 5px;
    cursor: pointer;
    transition: background-color 0.3s;
  }

  button:hover {
    opacity: 0.8;
  }

  .create-agent button[type='submit'] {
    background-color: #4CAF50;
    color: white;
  }

  .capabilities {
    margin-bottom: 15px;
  }

  .capability-input {
    display: flex;
    gap: 10px;
    margin-bottom: 5px;
  }

  .agent-list ul {
    list-style-type: none;
    padding: 0;
  }

  .agent-card {
    background-color: #f4f4f4;
    margin-bottom: 15px;
    padding: 20px;
    border-radius: 5px;
    position: relative;
  }

  .agent-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 10px;
  }

  .status {
    padding: 5px 10px;
    border-radius: 3px;
    font-size: 0.8em;
    text-transform: uppercase;
  }

  .status.active {
    background-color: #4CAF50;
    color: white;
  }

  .status.inactive {
    background-color: #FFC107;
    color: black;
  }

  .status.suspended {
    background-color: #F44336;
    color: white;
  }

  .agent-actions {
    display: flex;
    justify-content: space-between;
    margin-top: 15px;
  }

  .agent-actions button {
    margin-right: 10px;
  }

  .status-actions {
    display: flex;
    gap: 10px;
  }

  .status-change {
    font-size: 0.8em;
    padding: 5px 10px;
  }

  .status-change.active {
    background-color: #4CAF50;
    color: white;
  }

  .status-change.inactive {
    background-color: #FFC107;
    color: black;
  }

  .status-change.suspended {
    background-color: #F44336;
    color: white;
  }
</style>
