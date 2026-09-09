<script lang="ts">
    // System Tray Component
    // Windows 12 feature: System icons with notifications
    // Enabled on Windows 11
    
    import { onMount } from 'svelte';
    
    export let showClock = true;
    
    interface TrayIcon {
        id: string;
        icon: string;
        label: string;
        hasNotification: boolean;
        notificationCount: number;
        tooltip: string;
    }
    
    let trayIcons: TrayIcon[] = [
        { id: 'network', icon: 'wifi', label: 'Network', hasNotification: false, notificationCount: 0, tooltip: 'Wi-Fi connected' },
        { id: 'volume', icon: 'volume', label: 'Volume', hasNotification: false, notificationCount: 0, tooltip: 'Volume: 50%' },
        { id: 'battery', icon: 'battery', label: 'Battery', hasNotification: true, notificationCount: 1, tooltip: 'Battery: 75%' },
        { id: 'bluetooth', icon: 'bluetooth', label: 'Bluetooth', hasNotification: false, notificationCount: 0, tooltip: 'Bluetooth enabled' },
        { id: 'updates', icon: 'update', label: 'Updates', hasNotification: true, notificationCount: 3, tooltip: '3 updates available' },
    ];
    
    let currentTime = '';
    let currentDate = '';
    let showTrayMenu = false;
    let selectedIcon: string | null = null;
    
    onMount(() => {
        console.log('System tray initialized with notifications');
        
        // Update time every second
        updateTime();
        const interval = setInterval(updateTime, 1000);
        
        return () => clearInterval(interval);
    });
    
    function updateTime() {
        const now = new Date();
        currentTime = now.toLocaleTimeString('en-US', { hour: '2-digit', minute: '2-digit' });
        currentDate = now.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
    }
    
    function handleIconClick(iconId: string) {
        selectedIcon = iconId;
        showTrayMenu = !showTrayMenu;
        console.log('Tray icon clicked:', iconId);
    }
    
    function handleNotificationClick(iconId: string) {
        // Clear notification
        const icon = trayIcons.find(i => i.id === iconId);
        if (icon) {
            icon.hasNotification = false;
            icon.notificationCount = 0;
        }
        console.log('Notification cleared for:', iconId);
    }
    
    function getIconColor(icon: TrayIcon): string {
        if (icon.hasNotification) {
            return '#ff0000';
        }
        return '#ffffff';
    }
</script>

<style>
    .system-tray {
        display: flex;
        align-items: center;
        gap: 8px;
        padding: 4px 8px;
    }
    
    .clock {
        display: flex;
        flex-direction: column;
        align-items: center;
        padding: 4px 8px;
        border-radius: 4px;
        cursor: pointer;
        transition: background 0.2s ease;
    }
    
    .clock:hover {
        background: rgba(255, 255, 255, 0.1);
    }
    
    .time {
        font-size: 12px;
        font-weight: 500;
        color: white;
        line-height: 1;
    }
    
    .date {
        font-size: 10px;
        color: rgba(255, 255, 255, 0.7);
        margin-top: 2px;
    }
    
    .tray-icon {
        position: relative;
        display: flex;
        flex-direction: column;
        align-items: center;
        padding: 4px 6px;
        border-radius: 4px;
        cursor: pointer;
        transition: all 0.2s ease;
        min-width: 32px;
    }
    
    .tray-icon:hover {
        background: rgba(255, 255, 255, 0.1);
    }
    
    .icon {
        width: 18px;
        height: 18px;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 12px;
    }
    
    .icon-label {
        font-size: 8px;
        color: rgba(255, 255, 255, 0.7);
        margin-top: 2px;
        white-space: nowrap;
    }
    
    .notification-badge {
        position: absolute;
        top: 0;
        right: 0;
        background: #ff0000;
        color: white;
        font-size: 6px;
        font-weight: bold;
        border-radius: 50%;
        width: 14px;
        height: 14px;
        display: flex;
        align-items: center;
        justify-content: center;
        transform: translate(50%, -50%);
    }
    
    .tray-menu {
        position: absolute;
        bottom: 40px;
        right: 0;
        width: 200px;
        background: rgba(40, 40, 40, 0.98);
        backdrop-filter: blur(10px);
        border-radius: 8px;
        box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5);
        border: 1px solid rgba(255, 255, 255, 0.1);
        padding: 8px;
        opacity: 0;
        visibility: hidden;
        transition: all 0.2s ease;
        z-index: 1001;
    }
    
    .tray-menu.visible {
        opacity: 1;
        visibility: visible;
    }
    
    .menu-item {
        padding: 8px 12px;
        border-radius: 4px;
        cursor: pointer;
        transition: background 0.2s ease;
        font-size: 12px;
        color: rgba(255, 255, 255, 0.9);
    }
    
    .menu-item:hover {
        background: rgba(255, 255, 255, 0.1);
    }
</style>

<div class="system-tray">
    {#if showClock}
        <div class="clock" role="button" tabindex="0" on:click={() => console.log('Clock clicked')} on:keydown={(e) => { if (e.key === 'Enter' || e.key === ' ') console.log('Clock clicked'); }}>
            <span class="time">{currentTime}</span>
            <span class="date">{currentDate}</span>
        </div>
    {/if}
    
    {#each trayIcons as icon}
        <div 
            class="tray-icon"
            role="button"
            tabindex="0"
            on:click={() => handleIconClick(icon.id)}
            on:keydown={(e) => { if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); handleIconClick(icon.id); } }}
        >
            <div class="icon" style="color: {getIconColor(icon)}">
                {icon.icon}
            </div>
            <span class="icon-label">{icon.label}</span>
            {#if icon.hasNotification && icon.notificationCount > 0}
                <div class="notification-badge">
                    {icon.notificationCount > 99 ? '99+' : icon.notificationCount}
                </div>
            {/if}
        </div>
    {/each}
    
    {#if showTrayMenu && selectedIcon}
        <div class="tray-menu {showTrayMenu ? 'visible' : ''}" role="menu">
            <div class="menu-item" role="menuitem" tabindex="0" on:click={() => { if (selectedIcon) handleNotificationClick(selectedIcon); }} on:keydown={(e) => { if ((e.key === 'Enter' || e.key === ' ') && selectedIcon) { e.preventDefault(); handleNotificationClick(selectedIcon); } }}>
                Clear notifications
            </div>
            <div class="menu-item" role="menuitem" tabindex="0" on:click={() => showTrayMenu = false} on:keydown={(e) => { if (e.key === 'Enter' || e.key === ' ') showTrayMenu = false; }}>
                Close
            </div>
        </div>
    {/if}
</div>
