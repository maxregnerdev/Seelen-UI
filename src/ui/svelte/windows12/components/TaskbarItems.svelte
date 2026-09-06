<script lang="ts">
    // Taskbar Items Component
    // Windows 12 feature: App buttons with badges and thumbnails
    // Enabled on Windows 11
    
    import { onMount } from 'svelte';
    
    export let maxItems = 10;
    
    // Sample app data
    interface AppItem {
        id: string;
        name: string;
        icon: string;
        isRunning: boolean;
        hasBadge: boolean;
        badgeCount: number;
        thumbnail?: string;
        isPinned: boolean;
    }
    
    let appItems: AppItem[] = [
        { id: 'explorer', name: 'File Explorer', icon: 'folder', isRunning: true, hasBadge: false, badgeCount: 0, isPinned: true },
        { id: 'edge', name: 'Microsoft Edge', icon: 'globe', isRunning: true, hasBadge: true, badgeCount: 3, isPinned: true },
        { id: 'spotify', name: 'Spotify', icon: 'music', isRunning: true, hasBadge: false, badgeCount: 0, isPinned: true },
        { id: 'vscode', name: 'VS Code', icon: 'code', isRunning: true, hasBadge: true, badgeCount: 1, isPinned: false },
        { id: 'discord', name: 'Discord', icon: 'chat', isRunning: true, hasBadge: true, badgeCount: 5, isPinned: false },
    ];
    
    let selectedIndex = -1;
    
    onMount(() => {
        console.log('Taskbar items initialized with badges and thumbnails');
    });
    
    function handleClick(index: number) {
        selectedIndex = index;
        // In a real implementation, this would focus the app
        console.log('App clicked:', appItems[index].name);
    }
    
    function handleContextMenu(event: MouseEvent, index: number) {
        event.preventDefault();
        // In a real implementation, this would show a context menu
        console.log('Context menu for:', appItems[index].name);
    }
</script>

<style>
    .taskbar-items {
        display: flex;
        align-items: center;
        gap: 4px;
        overflow-x: auto;
        padding: 4px 0;
        scrollbar-width: none;
    }
    
    .taskbar-items::-webkit-scrollbar {
        display: none;
    }
    
    .app-item {
        position: relative;
        display: flex;
        flex-direction: column;
        align-items: center;
        padding: 6px 10px;
        border-radius: 8px;
        cursor: pointer;
        transition: all 0.2s ease;
        min-width: 48px;
    }
    
    .app-item:hover {
        background: rgba(255, 255, 255, 0.1);
    }
    
    .app-item.selected {
        background: rgba(0, 120, 215, 0.2);
    }
    
    .app-item.running {
        background: rgba(0, 120, 215, 0.1);
    }
    
    .app-icon {
        width: 24px;
        height: 24px;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 14px;
        color: white;
    }
    
    .app-name {
        font-size: 10px;
        color: rgba(255, 255, 255, 0.7);
        margin-top: 2px;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
        max-width: 60px;
    }
    
    .app-badge {
        position: absolute;
        top: 0;
        right: 0;
        background: #ff0000;
        color: white;
        font-size: 8px;
        font-weight: bold;
        border-radius: 50%;
        width: 16px;
        height: 16px;
        display: flex;
        align-items: center;
        justify-content: center;
        transform: translate(50%, -50%);
    }
    
    .thumbnail {
        position: absolute;
        top: -40px;
        left: 50%;
        transform: translateX(-50%);
        width: 120px;
        height: 80px;
        background: rgba(0, 0, 0, 0.8);
        border-radius: 4px;
        border: 1px solid rgba(255, 255, 255, 0.2);
        opacity: 0;
        transition: opacity 0.2s ease;
        pointer-events: none;
    }
    
    .app-item:hover .thumbnail {
        opacity: 1;
    }
</style>

<div class="taskbar-items">
    {#each appItems.slice(0, maxItems) as item, index}
        <div 
            class="app-item {item.isRunning ? 'running' : ''} {selectedIndex === index ? 'selected' : ''}"
            on:click={() => handleClick(index)}
            on:contextmenu={(e) => handleContextMenu(e, index)}
        >
            {#if item.thumbnail}
                <div class="thumbnail"></div>
            {/if}
            <div class="app-icon">{item.icon}</div>
            <div class="app-name">{item.name}</div>
            {#if item.hasBadge && item.badgeCount > 0}
                <div class="app-badge">{item.badgeCount > 99 ? '99+' : item.badgeCount}</div>
            {/if}
        </div>
    {/each}
</div>
