<script lang="ts">
    // Floating Taskbar Component
    // Windows 12 feature: Floating taskbar with rounded corners
    // Enabled on Windows 11
    
    import { onMount } from 'svelte';
    import TaskbarItems from './TaskbarItems.svelte';
    import StartButton from './StartButton.svelte';
    import SystemTray from './SystemTray.svelte';
    
    export let position = 'bottom';
    export let roundedCorners = true;
    export let translucent = true;
    
    let isHovered = false;
    
    onMount(() => {
        console.log('Floating taskbar initialized with rounded corners');
    });
    
    function handleMouseEnter() {
        isHovered = true;
    }
    
    function handleMouseLeave() {
        isHovered = false;
    }
</script>

<style>
    .floating-taskbar {
        position: fixed;
        bottom: 20px;
        left: 50%;
        transform: translateX(-50%);
        width: 80%;
        max-width: 800px;
        min-width: 400px;
        background: rgba(30, 30, 30, 0.8);
        backdrop-filter: blur(20px);
        border-radius: 16px;
        padding: 8px 16px;
        display: flex;
        align-items: center;
        justify-content: space-between;
        box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
        border: 1px solid rgba(255, 255, 255, 0.1);
        transition: all 0.3s ease;
        z-index: 1000;
    }
    
    .floating-taskbar:hover {
        background: rgba(40, 40, 40, 0.9);
        box-shadow: 0 12px 48px rgba(0, 0, 0, 0.4);
    }
    
    .taskbar-content {
        display: flex;
        align-items: center;
        gap: 12px;
        width: 100%;
    }
    
    .start-section {
        display: flex;
        align-items: center;
        gap: 8px;
    }
    
    .center-section {
        display: flex;
        align-items: center;
        gap: 8px;
        flex: 1;
        justify-content: center;
    }
    
    .end-section {
        display: flex;
        align-items: center;
        gap: 8px;
    }
</style>

<div class="floating-taskbar" 
     on:mouseenter={handleMouseEnter}
     on:mouseleave={handleMouseLeave}
     class:hovered={isHovered}>
    <div class="taskbar-content">
        <div class="start-section">
            <StartButton />
        </div>
        <div class="center-section">
            <TaskbarItems />
        </div>
        <div class="end-section">
            <SystemTray />
        </div>
    </div>
</div>
