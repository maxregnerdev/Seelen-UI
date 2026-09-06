<script lang="ts">
    // Adaptive Layout Component
    // Windows 12 feature: Adaptive, context-aware desktop layout
    // Enabled on Windows 11
    
    import { onMount, onDestroy } from 'svelte';
    import { fade, slide } from 'svelte/transition';
    
    export let children: any;
    
    let layoutMode = 'auto';
    let screenWidth = 0;
    let screenHeight = 0;
    let isMultiMonitor = false;
    let currentContext = 'desktop';
    
    // Layout modes
    type LayoutMode = 'auto' | 'compact' | 'expanded' | 'touch';
    
    // Context types
    type ContextType = 'desktop' | 'tablet' | 'laptop' | 'touch';
    
    onMount(() => {
        console.log('Adaptive layout initialized for context-aware desktop');
        
        // Initial detection
        detectScreenSize();
        detectContext();
        
        // Set up resize listener
        const resizeObserver = new ResizeObserver(() => {
            detectScreenSize();
            detectContext();
        });
        
        resizeObserver.observe(document.body);
        
        // Set up window resize listener
        window.addEventListener('resize', handleResize);
        
        return () => {
            resizeObserver.disconnect();
            window.removeEventListener('resize', handleResize);
        };
    });
    
    function handleResize() {
        detectScreenSize();
        detectContext();
    }
    
    function detectScreenSize() {
        screenWidth = window.innerWidth;
        screenHeight = window.innerHeight;
        
        // Determine layout mode based on screen size
        if (screenWidth < 1024) {
            layoutMode = 'compact';
        } else if (screenWidth >= 1920 && screenHeight >= 1080) {
            layoutMode = 'expanded';
        } else {
            layoutMode = 'auto';
        }
        
        console.log(`Screen: ${screenWidth}x${screenHeight}, Layout: ${layoutMode}`);
    }
    
    function detectContext() {
        // Detect touch support
        const hasTouch = 'ontouchstart' in window || navigator.maxTouchPoints > 0;
        
        // Detect screen size
        if (screenWidth < 768) {
            currentContext = hasTouch ? 'touch' : 'tablet';
        } else if (screenWidth < 1200) {
            currentContext = 'laptop';
        } else {
            currentContext = 'desktop';
        }
        
        // Check for multi-monitor setup
        isMultiMonitor = screenWidth > 2560 || screenHeight > 1440;
        
        console.log(`Context: ${currentContext}, Multi-monitor: ${isMultiMonitor}`);
    }
    
    function setLayoutMode(mode: LayoutMode) {
        layoutMode = mode;
        console.log('Layout mode set to:', mode);
    }
    
    function getLayoutClasses() {
        return [
            `layout-${layoutMode}`,
            `context-${currentContext}`,
            isMultiMonitor ? 'multi-monitor' : 'single-monitor'
        ].join(' ');
    }
</script>

<style>
    .adaptive-layout {
        display: contents;
    }
    
    /* Layout modes */
    .layout-compact {
        --item-spacing: 4px;
        --item-size: 32px;
        --padding: 4px;
    }
    
    .layout-expanded {
        --item-spacing: 12px;
        --item-size: 48px;
        --padding: 16px;
    }
    
    .layout-auto {
        --item-spacing: 8px;
        --item-size: 40px;
        --padding: 8px;
    }
    
    .layout-touch {
        --item-spacing: 16px;
        --item-size: 64px;
        --padding: 24px;
    }
    
    /* Context types */
    .context-desktop {
        --font-size: 12px;
        --icon-size: 20px;
    }
    
    .context-laptop {
        --font-size: 11px;
        --icon-size: 18px;
    }
    
    .context-tablet {
        --font-size: 14px;
        --icon-size: 24px;
    }
    
    .context-touch {
        --font-size: 16px;
        --icon-size: 32px;
    }
    
    /* Multi-monitor */
    .multi-monitor {
        --max-width: 1200px;
    }
    
    .single-monitor {
        --max-width: 100%;
    }
    
    .layout-info {
        position: fixed;
        bottom: 10px;
        right: 10px;
        font-size: 10px;
        color: rgba(255, 255, 255, 0.5);
        background: rgba(0, 0, 0, 0.5);
        padding: 4px 8px;
        border-radius: 4px;
        z-index: 9999;
    }
</style>

<div class="adaptive-layout {getLayoutClasses()}">
    {#if children}
        {@html children}
    {/if}
    
    <!-- Debug info (remove in production) -->
    {#if false}
        <div class="layout-info">
            Layout: {layoutMode} | Context: {currentContext} | Multi: {isMultiMonitor}
        </div>
    {/if}
</div>
