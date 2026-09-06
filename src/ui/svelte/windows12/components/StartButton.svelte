<script lang="ts">
    // Start Button Component
    // Windows 12 feature: Modern Start button with animations
    // Enabled on Windows 11
    
    import { onMount } from 'svelte';
    import { fade, scale } from 'svelte/transition';
    
    export let size = 32;
    export let showLabel = false;
    
    let isHovered = false;
    let isClicked = false;
    let isAnimating = false;
    
    onMount(() => {
        console.log('Modern Start button initialized with animations');
    });
    
    function handleClick() {
        isClicked = true;
        isAnimating = true;
        
        // In a real implementation, this would open the Start menu
        console.log('Start button clicked');
        
        // Reset animation after a short delay
        setTimeout(() => {
            isClicked = false;
        }, 100);
        
        setTimeout(() => {
            isAnimating = false;
        }, 500);
    }
    
    function handleMouseEnter() {
        isHovered = true;
    }
    
    function handleMouseLeave() {
        isHovered = false;
    }
</script>

<style>
    .start-button {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        gap: 8px;
        padding: 6px 12px;
        border-radius: 8px;
        background: linear-gradient(135deg, #0078d7 0%, #0063b1 100%);
        border: none;
        cursor: pointer;
        transition: all 0.2s ease;
        box-shadow: 0 2px 8px rgba(0, 120, 215, 0.3);
        position: relative;
        overflow: hidden;
    }
    
    .start-button:hover {
        background: linear-gradient(135deg, #0088ff 0%, #0078d7 100%);
        box-shadow: 0 4px 16px rgba(0, 120, 215, 0.4);
        transform: translateY(-1px);
    }
    
    .start-button:active {
        transform: translateY(0);
        box-shadow: 0 1px 4px rgba(0, 120, 215, 0.3);
    }
    
    .start-button.clicked {
        animation: clickPulse 0.3s ease;
    }
    
    .start-button.animating {
        animation: glowPulse 0.5s ease;
    }
    
    @keyframes clickPulse {
        0% { transform: scale(0.95); }
        50% { transform: scale(1.05); }
        100% { transform: scale(1); }
    }
    
    @keyframes glowPulse {
        0% { box-shadow: 0 0 0 0 rgba(0, 120, 215, 0.4); }
        70% { box-shadow: 0 0 0 10px rgba(0, 120, 215, 0); }
        100% { box-shadow: 0 0 0 0 rgba(0, 120, 215, 0); }
    }
    
    .start-icon {
        width: {size}px;
        height: {size}px;
        display: flex;
        align-items: center;
        justify-content: center;
        color: white;
        font-size: {size * 0.6}px;
        font-weight: bold;
    }
    
    .start-label {
        font-size: 12px;
        color: white;
        font-weight: 500;
    }
    
    .windows-logo {
        width: 100%;
        height: 100%;
        display: flex;
        align-items: center;
        justify-content: center;
    }
</style>

<button 
    class="start-button {isHovered ? 'hovered' : ''} {isClicked ? 'clicked' : ''} {isAnimating ? 'animating' : ''}"
    on:click={handleClick}
    on:mouseenter={handleMouseEnter}
    on:mouseleave={handleMouseLeave}
    aria-label="Start menu"
>
    <div class="start-icon">
        <svg class="windows-logo" viewBox="0 0 24 24" fill="currentColor">
            <path d="M0 3.5L5 0V4.5L0 7.5M0 12.5L5 9.5V14L0 17M0 20.5L5 17.5V22L0 24.5M24 3.5L19 0V4.5L24 7.5M24 12.5L19 9.5V14L24 17M24 20.5L19 17.5V22L24 24.5M12 3.5L7 0V4.5L12 7.5M12 12.5L7 9.5V14L12 17M12 20.5L7 17.5V22L12 24.5"/>
        </svg>
    </div>
    {#if showLabel}
        <span class="start-label">Start</span>
    {/if}
</button>
