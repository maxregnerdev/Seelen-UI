<script lang="ts">
    // Search Bar Component
    // Windows 12 feature: AI-powered search with NPU indicator
    // Enabled on Windows 11
    
    import { onMount } from 'svelte';
    import { fade } from 'svelte/transition';
    
    export let placeholder = 'Search with AI...';
    export let showNpuIndicator = true;
    
    let searchQuery = '';
    let isFocused = false;
    let searchResults: string[] = [];
    let isSearching = false;
    let npuActive = false;
    
    // Sample search suggestions
    const suggestions = [
        'Apps',
        'Files',
        'Settings',
        'Web',
        'Calculator',
        'Calendar',
        'Weather'
    ];
    
    onMount(() => {
        console.log('AI-powered search bar initialized with NPU indicator');
    });
    
    function handleInput(event: Event) {
        const target = event.target as HTMLInputElement;
        searchQuery = target.value;
        
        if (searchQuery.length > 0) {
            performSearch();
        } else {
            searchResults = [];
        }
    }
    
    function handleFocus() {
        isFocused = true;
        if (searchQuery.length === 0) {
            // Show suggestions when focused
            searchResults = suggestions;
        }
    }
    
    function handleBlur() {
        // Small delay to allow click on results
        setTimeout(() => {
            isFocused = false;
        }, 200);
    }
    
    async function performSearch() {
        if (searchQuery.length === 0) {
            searchResults = [];
            return;
        }
        
        isSearching = true;
        npuActive = true;
        
        // Simulate AI-powered search with NPU
        await new Promise(resolve => setTimeout(resolve, 300));
        
        // Generate search results based on query
        searchResults = [
            `Search results for: ${searchQuery}`,
            `AI-enhanced: ${searchQuery}`,
            `NPU-optimized: ${searchQuery}`,
            ...suggestions.filter(s => 
                s.toLowerCase().includes(searchQuery.toLowerCase())
            )
        ];
        
        isSearching = false;
        
        // NPU indicator stays active for a bit longer
        setTimeout(() => {
            npuActive = false;
        }, 1000);
    }
    
    function handleKeyDown(event: KeyboardEvent) {
        if (event.key === 'Enter' && searchQuery.length > 0) {
            console.log('Search submitted:', searchQuery);
            // In a real implementation, this would execute the search
        } else if (event.key === 'Escape') {
            searchQuery = '';
            searchResults = [];
            isFocused = false;
        }
    }
    
    function selectResult(result: string) {
        console.log('Selected result:', result);
        searchQuery = result;
        searchResults = [];
        isFocused = false;
    }
</script>

<style>
    .search-container {
        position: relative;
        width: 300px;
        min-width: 200px;
    }
    
    .search-bar {
        width: 100%;
        display: flex;
        align-items: center;
        gap: 8px;
        padding: 8px 12px;
        border-radius: 20px;
        background: rgba(255, 255, 255, 0.1);
        border: 1px solid rgba(255, 255, 255, 0.2);
        cursor: text;
        transition: all 0.2s ease;
    }
    
    .search-bar:focus-within {
        background: rgba(255, 255, 255, 0.15);
        border-color: rgba(0, 120, 215, 0.5);
        box-shadow: 0 0 0 2px rgba(0, 120, 215, 0.2);
    }
    
    .search-icon {
        width: 18px;
        height: 18px;
        display: flex;
        align-items: center;
        justify-content: center;
        color: rgba(255, 255, 255, 0.7);
        font-size: 12px;
        pointer-events: none;
    }
    
    .search-input {
        flex: 1;
        border: none;
        background: transparent;
        color: white;
        font-size: 14px;
        outline: none;
    }
    
    .search-input::placeholder {
        color: rgba(255, 255, 255, 0.5);
    }
    
    .npu-indicator {
        width: 16px;
        height: 16px;
        display: flex;
        align-items: center;
        justify-content: center;
        border-radius: 50%;
        background: rgba(0, 120, 215, 0.2);
        color: #0078d7;
        font-size: 10px;
        opacity: 0;
        transition: opacity 0.2s ease;
    }
    
    .npu-indicator.active {
        opacity: 1;
        animation: pulse 1s ease-in-out infinite;
    }
    
    @keyframes pulse {
        0%, 100% { opacity: 1; }
        50% { opacity: 0.5; }
    }
    
    .search-results {
        position: absolute;
        top: calc(100% + 8px);
        left: 0;
        right: 0;
        background: rgba(40, 40, 40, 0.98);
        backdrop-filter: blur(10px);
        border-radius: 12px;
        box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5);
        border: 1px solid rgba(255, 255, 255, 0.1);
        padding: 8px;
        max-height: 300px;
        overflow-y: auto;
        opacity: 0;
        visibility: hidden;
        transition: all 0.2s ease;
        z-index: 1001;
    }
    
    .search-results.visible {
        opacity: 1;
        visibility: visible;
    }
    
    .result-item {
        padding: 8px 12px;
        border-radius: 6px;
        cursor: pointer;
        transition: background 0.2s ease;
        font-size: 13px;
        color: rgba(255, 255, 255, 0.9);
        display: flex;
        align-items: center;
        gap: 8px;
    }
    
    .result-item:hover {
        background: rgba(255, 255, 255, 0.1);
    }
    
    .result-icon {
        width: 16px;
        height: 16px;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 10px;
        color: rgba(255, 255, 255, 0.6);
    }
    
    .loading {
        display: flex;
        align-items: center;
        justify-content: center;
        padding: 16px;
        color: rgba(255, 255, 255, 0.7);
    }
    
    .spinner {
        width: 16px;
        height: 16px;
        border: 2px solid rgba(255, 255, 255, 0.3);
        border-top-color: #0078d7;
        border-radius: 50%;
        animation: spin 0.8s linear infinite;
    }
    
    @keyframes spin {
        to { transform: rotate(360deg); }
    }
</style>

<div class="search-container">
    <div class="search-bar">
        <div class="search-icon">
            {#if isSearching}
                <div class="spinner"></div>
            {:else}
                🔍
            {/if}
        </div>
        <input 
            type="text"
            class="search-input"
            bind:value={searchQuery}
            on:input={handleInput}
            on:focus={handleFocus}
            on:blur={handleBlur}
            on:keydown={handleKeyDown}
            placeholder={placeholder}
        />
        {#if showNpuIndicator}
            <div class="npu-indicator {npuActive ? 'active' : ''}" title="NPU Accelerated">
                ⚡
            </div>
        {/if}
    </div>
    
    {#if isFocused && (searchResults.length > 0 || isSearching)}
        <div class="search-results {isFocused ? 'visible' : ''}" transition:fade={{ duration: 200 }}>
            {#if isSearching}
                <div class="loading">
                    <div class="spinner"></div>
                    <span style="margin-left: 8px;">Searching with AI...</span>
                </div>
            {:else}
                {#each searchResults as result}
                    <div class="result-item" role="option" tabindex="0" on:click={() => selectResult(result)} on:keydown={(e) => { if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); selectResult(result); } }}>
                        <span class="result-icon">🔍</span>
                        <span>{result}</span>
                    </div>
                {/each}
            {/if}
        </div>
    {/if}
</div>
