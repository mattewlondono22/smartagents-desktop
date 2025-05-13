<script lang="ts">
    import { AppShell, AppBar } from '@skeletonlabs/skeleton';
    import { navigating } from '$app/stores';
    import { page } from '$app/stores';
    import { goto } from '$app/navigation';

    const routes = [
        { name: 'Agents', path: '/agents' },
        { name: 'File Upload', path: '/upload' },
        { name: 'Plugins', path: '/plugins' }
    ];

    function handleNavigation(path: string) {
        goto(path);
    }
</script>

<AppShell>
    <svelte:fragment slot="header">
        <AppBar>
            <svelte:fragment slot="lead">
                <strong class="text-xl">Windsurf Agent Studio</strong>
            </svelte:fragment>
            <svelte:fragment slot="trail">
                {#each routes as route}
                    <button 
                        class="btn {$page.url.pathname === route.path ? 'variant-filled' : 'variant-ghost'}"
                        on:click={() => handleNavigation(route.path)}
                    >
                        {route.name}
                    </button>
                {/each}
            </svelte:fragment>
        </AppBar>
    </svelte:fragment>

    <main class="p-4">
        <slot />
    </main>
</AppShell>

<style>
    main {
        max-width: 1200px;
        margin: 0 auto;
    }
</style>
