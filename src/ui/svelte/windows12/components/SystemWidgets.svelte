<script lang="ts">
    // System Widgets Component
    // Windows 12 feature: Floating widgets (weather, calendar, clock, system)
    // Enabled on Windows 11
    
    import { onMount } from 'svelte';
    import { fade } from 'svelte/transition';
    
    export let position = 'right';
    export let widgets = ['weather', 'calendar', 'clock', 'system'];
    
    interface Widget {
        id: string;
        title: string;
        content: string;
        icon: string;
        isVisible: boolean;
    }
    
    let systemWidgets: Widget[] = [
        {
            id: 'weather',
            title: 'Weather',
            content: '72°F • Sunny',
            icon: '☀️',
            isVisible: widgets.includes('weather')
        },
        {
            id: 'calendar',
            title: 'Calendar',
            content: 'Sep 6, 2026',
            icon: '📅',
            isVisible: widgets.includes('calendar')
        },
        {
            id: 'clock',
            title: 'Clock',
            content: '',
            icon: '⏰',
            isVisible: widgets.includes('clock')
        },
        {
            id: 'system',
            title: 'System',
            content: 'CPU: 25% • RAM: 4.2GB',
            icon: '⚙️',
            isVisible: widgets.includes('system')
        }
    ];
    
    let currentTime = '';
    let isExpanded = false;
    
    onMount(() => {
        console.log('System widgets initialized (weather, calendar, clock, system)');
        
        // Update time every second
        updateTime();
        const interval = setInterval(updateTime, 1000);
        
        return () => clearInterval(interval);
    });
    
    function updateTime() {
        const now = new Date();
        currentTime = now.toLocaleTimeString('en-US', { hour: '2-digit', minute: '2-digit' });
        
        // Update clock widget
        const clockWidget = systemWidgets.find(w => w.id === 'clock');
        if (clockWidget) {
            clockWidget.content = currentTime;
            systemWidgets = [...systemWidgets];
        }
    }
    
    function toggleExpand() {
        isExpanded = !isExpanded;
    }
    
    function toggleWidget(widgetId: string) {
        const widget = systemWidgets.find(w => w.id === widgetId);
        if (widget) {
            widget.isVisible = !widget.isVisible;
            systemWidgets = [...systemWidgets];
        }
    }
</script>

<style>
    .widgets-container {
        position: fixed;
        top: 20px;
        right: 20px;
        display: flex;
        flex-direction: column;
        gap: 12px;
        z-index: 999;
    }
    
    .widgets-container.left {
        right: auto;
        left: 20px;
    }
    
    .widget {
        background: rgba(30, 30, 30, 0.8);
        backdrop-filter: blur(20px);
        border-radius: 12px;
        padding: 12px;
        box-shadow: 0 4px 16px rgba(0, 0, 0, 0.3);
        border: 1px solid rgba(255, 255, 255, 0.1);
        transition: all 0.2s ease;
        min-width: 180px;
    }
    
    .widget:hover {
        background: rgba(40, 40, 40, 0.9);
        box-shadow: 0 8px 24px rgba(0, 0, 0, 0.4);
        transform: translateY(-2px);
    }
    
    .widget-header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        margin-bottom: 8px;
    }
    
    .widget-title {
        display: flex;
        align-items: center;
        gap: 6px;
        font-size: 12px;
        font-weight: 500;
        color: white;
    }
    
    .widget-icon {
        font-size: 14px;
    }
    
    .widget-content {
        font-size: 13px;
        color: rgba(255, 255, 255, 0.9);
    }
    
    .widget-actions {
        display: flex;
        gap: 4px;
    }
    
    .widget-action {
        width: 20px;
        height: 20px;
        display: flex;
        align-items: center;
        justify-content: center;
        border-radius: 4px;
        cursor: pointer;
        transition: background 0.2s ease;
        font-size: 10px;
    }
    
    .widget-action:hover {
        background: rgba(255, 255, 255, 0.1);
    }
    
    .expand-button {
        position: fixed;
        top: 20px;
        right: 20px;
        width: 32px;
        height: 32px;
        display: flex;
        align-items: center;
        justify-content: center;
        background: rgba(30, 30, 30, 0.8);
        backdrop-filter: blur(20px);
        border-radius: 8px;
        border: 1px solid rgba(255, 255, 255, 0.1);
        cursor: pointer;
        transition: all 0.2s ease;
        z-index: 1000;
    }
    
    .expand-button:hover {
        background: rgba(40, 40, 40, 0.9);
        transform: scale(1.1);
    }
    
    .widgets-expanded {
        display: flex;
        flex-direction: column;
        gap: 12px;
    }
    
    .widgets-collapsed {
        display: none;
    }
    
    .widget.minimized {
        width: 40px;
        height: 40px;
        padding: 8px;
        display: flex;
        align-items: center;
        justify-content: center;
    }
    
    .widget.minimized .widget-header {
        display: none;
    }
    
    .widget.minimized .widget-content {
        display: none;
    }
    
    .widget.minimized .widget-icon {
        font-size: 18px;
    }
</style>

<div class="widgets-container {position}">
    {#if !isExpanded}
        <div class="expand-button" on:click={toggleExpand} title="Expand widgets">
            ⚙️
        </div>
    {/if}
    
    {#if isExpanded}
        <div class="widgets-expanded">
            {#each systemWidgets as widget, index}
                {#if widget.isVisible}
                    <div 
                        class="widget"
                        transition:fade={{ duration: 200, delay: index * 50 }}
                    >
                        <div class="widget-header">
                            <span class="widget-title">
                                <span class="widget-icon">{widget.icon}</span>
                                {widget.title}
                            </span>
                            <div class="widget-actions">
                                <span 
                                    class="widget-action"
                                    on:click={() => toggleWidget(widget.id)}
                                    title="Hide widget"
                                >
                                    ×
                                </span>
                            </div>
                        </div>
                        <div class="widget-content">{widget.content}</div>
                    </div>
                {/if}
            {/each}
        </div>
    {:else}
        <div class="widgets-collapsed">
            {#each systemWidgets as widget, index}
                {#if widget.isVisible}
                    <div 
                        class="widget minimized"
                        transition:fade={{ duration: 200, delay: index * 50 }}
                        title="{widget.title}: {widget.content}"
                    >
                        <span class="widget-icon">{widget.icon}</span>
                    </div>
                {/if}
            {/each}
        </div>
    {/if}
</div>
