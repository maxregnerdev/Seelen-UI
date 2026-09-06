<script lang="ts">
    // Windows 12 UI Container
    // Enables Windows 12 features on Windows 11
    
    import { onMount } from 'svelte';
    import FloatingTaskbar from './components/FloatingTaskbar.svelte';
    import SystemWidgets from './components/SystemWidgets.svelte';
    import SearchBar from './components/SearchBar.svelte';
    import AdaptiveLayout from './components/AdaptiveLayout.svelte';
    
    export let visible = true;
    
    // Check if Windows 12 features are enabled (also on Windows 11)
    let isWindows12FeaturesEnabled = true; // Will be set based on system detection
    
    onMount(() => {
        // In a real implementation, this would check the system
        // For now, we assume Windows 12 features are enabled on Windows 11
        console.log('Windows 12 UI features initialized (enabled on Windows 11)');
    });
</script>

{#if visible && isWindows12FeaturesEnabled}
    <AdaptiveLayout>
        <FloatingTaskbar />
        <SystemWidgets />
        <SearchBar />
    </AdaptiveLayout>
{/if}
