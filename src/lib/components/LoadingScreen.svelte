<script lang="ts">
  import { appState } from '../stores/appState.svelte';
</script>

<div class="loading-container" class:hidden={appState.isWebViewReady}>
  <div class="loading-content">
    <div class="logo-wrapper">
      <div class="logo-pulse"></div>
      <img src="/logo.png" alt="Waplus" class="wa-logo" />
    </div>

    <h2 class="title">Waplus</h2>
    <p class="subtitle">Restoring WhatsApp session...</p>

    <div class="progress-bar">
      <div class="progress-fill"></div>
    </div>

    <div class="encryption-notice">
      <svg viewBox="0 0 24 24" width="13" height="13" fill="currentColor">
        <path d="M18 8h-1V6c0-2.76-2.24-5-5-5S7 3.24 7 6v2H6c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V10c0-1.1-.9-2-2-2zm-6 9c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2zm3.1-9H8.9V6c0-1.71 1.39-3.1 3.1-3.1 1.71 0 3.1 1.39 3.1 3.1v2z"/>
      </svg>
      <span>Encrypted Persistent Session Storage</span>
    </div>
  </div>
</div>

<style>
  .loading-container {
    position: absolute;
    top: var(--titlebar-height);
    left: 0;
    right: 0;
    bottom: 0;
    background-color: var(--bg-surface);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 50;
    transition: opacity 0.3s ease, visibility 0.3s ease;
  }

  .loading-container.hidden {
    opacity: 0;
    visibility: hidden;
    pointer-events: none;
  }

  .loading-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    max-width: 320px;
    padding: 32px 24px;
  }

  .logo-wrapper {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 80px;
    height: 80px;
    margin-bottom: 20px;
  }

  .logo-pulse {
    position: absolute;
    width: 100%;
    height: 100%;
    border-radius: 50%;
    background-color: var(--wa-green);
    opacity: 0.15;
    animation: ripple 2s infinite cubic-bezier(0.25, 0.46, 0.45, 0.94);
  }

  .wa-logo {
    width: 52px;
    height: 52px;
    border-radius: 12px;
    position: relative;
    z-index: 1;
    object-fit: cover;
    box-shadow: 0 4px 14px rgba(0, 0, 0, 0.25);
  }

  @keyframes ripple {
    0% { transform: scale(0.8); opacity: 0.3; }
    100% { transform: scale(1.4); opacity: 0; }
  }

  .title {
    font-size: 18px;
    font-weight: 700;
    color: var(--text-primary);
    margin-bottom: 6px;
    letter-spacing: -0.3px;
  }

  .subtitle {
    font-size: 13px;
    color: var(--text-secondary);
    margin-bottom: 24px;
  }

  .progress-bar {
    width: 220px;
    height: 3px;
    background-color: var(--border-color);
    border-radius: 3px;
    overflow: hidden;
    margin-bottom: 24px;
    position: relative;
  }

  .progress-fill {
    position: absolute;
    height: 100%;
    width: 40%;
    background-color: var(--wa-green);
    border-radius: 3px;
    animation: indeterminate 1.8s infinite ease-in-out;
  }

  @keyframes indeterminate {
    0% { left: -40%; }
    50% { left: 40%; width: 50%; }
    100% { left: 100%; width: 40%; }
  }

  .encryption-notice {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 11px;
    color: var(--text-muted);
  }
</style>
