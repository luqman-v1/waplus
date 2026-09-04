<script lang="ts">
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { appState } from '../stores/appState.svelte';
  import { onMount } from 'svelte';

  const appWindow = getCurrentWindow();

  async function handleMinimize() {
    await appWindow.minimize();
  }

  async function handleToggleMaximize() {
    await appWindow.toggleMaximize();
    appState.setMaximized(await appWindow.isMaximized());
  }

  async function handleClose() {
    // Hide to system tray for instant warm resume
    await appWindow.hide();
  }

  function handleToggleSettings() {
    appState.toggleSettings();
  }

  onMount(() => {
    let unlisten: (() => void) | null = null;
    (async () => {
      appState.setMaximized(await appWindow.isMaximized());
      unlisten = await appWindow.onResized(async () => {
        appState.setMaximized(await appWindow.isMaximized());
      });
    })();

    return () => {
      if (unlisten) unlisten();
    };
  });
</script>

<header class="titlebar" data-tauri-drag-region>
  <div class="titlebar-left" data-tauri-drag-region>
    <div class="brand">
      <img src="/logo.png" alt="Waplus" class="brand-img" />
      <span class="brand-name">Waplus</span>
    </div>

    <div class="status-indicator status-{appState.connectionStatus}">
      <span class="status-dot"></span>
      <span class="status-text">
        {#if appState.connectionStatus === 'connected'}
          Connected
        {:else if appState.connectionStatus === 'connecting'}
          Connecting
        {:else}
          Offline
        {/if}
      </span>
    </div>
  </div>

  <div class="titlebar-center" data-tauri-drag-region></div>

  <div class="titlebar-right">
    <button
      class="icon-btn"
      title="Settings"
      aria-label="Open Settings"
      onclick={handleToggleSettings}
    >
      <svg viewBox="0 0 24 24" width="15" height="15" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <circle cx="12" cy="12" r="3"></circle>
        <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path>
      </svg>
    </button>

    <div class="window-controls">
      <button class="win-btn win-min" title="Minimize" aria-label="Minimize Window" onclick={handleMinimize}>
        <svg viewBox="0 0 12 12" width="10" height="10" fill="currentColor">
          <rect y="5.5" width="12" height="1"></rect>
        </svg>
      </button>

      <button class="win-btn win-max" title={appState.isMaximized ? "Restore" : "Maximize"} aria-label="Maximize Window" onclick={handleToggleMaximize}>
        {#if appState.isMaximized}
          <svg viewBox="0 0 12 12" width="10" height="10" fill="none" stroke="currentColor" stroke-width="1">
            <rect x="2.5" y="0.5" width="9" height="9"></rect>
            <rect x="0.5" y="2.5" width="9" height="9" fill="var(--bg-titlebar)"></rect>
          </svg>
        {:else}
          <svg viewBox="0 0 12 12" width="10" height="10" fill="none" stroke="currentColor" stroke-width="1">
            <rect x="1" y="1" width="10" height="10"></rect>
          </svg>
        {/if}
      </button>

      <button class="win-btn win-close" title="Close to Tray" aria-label="Close to Tray" onclick={handleClose}>
        <svg viewBox="0 0 12 12" width="10" height="10" fill="none" stroke="currentColor" stroke-width="1.2" stroke-linecap="round">
          <line x1="1" y1="1" x2="11" y2="11"></line>
          <line x1="11" y1="1" x2="1" y2="11"></line>
        </svg>
      </button>
    </div>
  </div>
</header>

<style>
  .titlebar {
    height: var(--titlebar-height);
    background-color: var(--bg-titlebar);
    border-bottom: 1px solid var(--border-color);
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 0 0 12px;
    z-index: 100;
    position: relative;
    user-select: none;
  }

  .titlebar-left {
    display: flex;
    align-items: center;
    gap: 14px;
    height: 100%;
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .brand-img {
    width: 18px;
    height: 18px;
    border-radius: 4px;
    object-fit: cover;
  }

  .brand-name {
    font-weight: 700;
    font-size: 13px;
    letter-spacing: -0.2px;
    color: var(--text-primary);
  }

  .status-indicator {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 11px;
    font-weight: 500;
    padding: 2px 8px;
    border-radius: 12px;
    background-color: var(--bg-surface-hover);
    color: var(--text-secondary);
    border: 1px solid var(--border-color);
  }

  .status-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
  }

  .status-connected .status-dot {
    background-color: var(--wa-green);
    box-shadow: 0 0 6px var(--wa-green);
  }

  .status-connecting .status-dot {
    background-color: #f59e0b;
    animation: pulse 1.5s infinite ease-in-out;
  }

  .status-offline .status-dot {
    background-color: var(--danger);
  }

  @keyframes pulse {
    0%, 100% { opacity: 1; transform: scale(1); }
    50% { opacity: 0.4; transform: scale(0.85); }
  }

  .titlebar-center {
    flex: 1;
    height: 100%;
  }

  .titlebar-right {
    display: flex;
    align-items: center;
    height: 100%;
  }

  .icon-btn {
    width: 36px;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-secondary);
    transition: background-color 0.15s ease, color 0.15s ease;
  }

  .icon-btn:hover {
    background-color: var(--bg-surface-hover);
    color: var(--text-primary);
  }

  .window-controls {
    display: flex;
    align-items: center;
    height: 100%;
  }

  .win-btn {
    width: 44px;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-secondary);
    transition: background-color 0.15s ease, color 0.15s ease;
  }

  .win-btn:hover {
    background-color: var(--bg-surface-hover);
    color: var(--text-primary);
  }

  .win-close:hover {
    background-color: var(--danger);
    color: #ffffff;
  }
</style>
