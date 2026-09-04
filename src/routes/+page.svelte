<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { appState, type AppTheme } from '$lib/stores/appState.svelte';

  const appWindow = getCurrentWindow();
  let isClearing = $state(false);
  let clearSuccess = $state(false);

  function setTheme(newTheme: AppTheme) {
    appState.setTheme(newTheme);
    if (newTheme === 'dark') {
      document.body.classList.add('theme-dark');
      document.body.classList.remove('theme-light');
    } else if (newTheme === 'light') {
      document.body.classList.add('theme-light');
      document.body.classList.remove('theme-dark');
    } else {
      document.body.classList.remove('theme-dark', 'theme-light');
    }
  }

  async function handleClearSession() {
    const confirmed = confirm('Are you sure you want to clear your WhatsApp session and local data? You will need to scan the QR code again.');
    if (!confirmed) return;

    try {
      isClearing = true;
      await invoke('clear_session');
      clearSuccess = true;
      setTimeout(async () => {
        clearSuccess = false;
        await appWindow.close();
      }, 1000);
    } catch (error) {
      console.error('Failed to clear session:', error);
      alert('Failed to clear session: ' + error);
    } finally {
      isClearing = false;
    }
  }

  async function handleClose() {
    await appWindow.close();
  }
</script>

<div class="settings-container">
  <div class="settings-card">
    <div class="settings-header">
      <div class="brand">
        <img src="/logo.png" alt="Waplus" class="brand-img" />
        <h3 class="title">Waplus Settings</h3>
      </div>
    </div>

    <div class="settings-body">
      <section class="section">
        <h4 class="section-title">Appearance</h4>
        <div class="theme-options">
          <button
            class="theme-btn"
            class:active={appState.theme === 'system'}
            onclick={() => setTheme('system')}
          >
            <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2">
              <rect x="2" y="3" width="20" height="14" rx="2" ry="2"></rect>
              <line x1="8" y1="21" x2="16" y2="21"></line>
              <line x1="12" y1="17" x2="12" y2="21"></line>
            </svg>
            <span>System</span>
          </button>

          <button
            class="theme-btn"
            class:active={appState.theme === 'dark'}
            onclick={() => setTheme('dark')}
          >
            <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path>
            </svg>
            <span>Dark</span>
          </button>

          <button
            class="theme-btn"
            class:active={appState.theme === 'light'}
            onclick={() => setTheme('light')}
          >
            <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="12" cy="12" r="5"></circle>
              <line x1="12" y1="12" x2="12" y2="3"></line>
              <line x1="12" y1="21" x2="12" y2="23"></line>
              <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line>
              <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line>
              <line x1="1" y1="12" x2="3" y2="12"></line>
              <line x1="21" y1="12" x2="23" y2="12"></line>
              <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line>
              <line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line>
            </svg>
            <span>Light</span>
          </button>
        </div>
      </section>

      <section class="section">
        <h4 class="section-title">Application Behavior</h4>
        <div class="info-box">
          <div class="info-row">
            <span class="info-label">Minimize to Tray</span>
            <span class="info-value text-success">Enabled (Default)</span>
          </div>
          <div class="info-row">
            <span class="info-label">Persistent Session Storage</span>
            <span class="info-value">Isolated Directory</span>
          </div>
        </div>
      </section>

      <section class="section danger-section">
        <h4 class="section-title text-danger">Session Management</h4>
        <p class="section-desc">Clear all WhatsApp Web session tokens and local cache from disk, then restart WebView.</p>
        <button
          class="btn-danger"
          disabled={isClearing}
          onclick={handleClearSession}
        >
          {#if isClearing}
            <span>Clearing Session...</span>
          {:else if clearSuccess}
            <span>Session Cleared Successfully</span>
          {:else}
            <svg viewBox="0 0 24 24" width="14" height="14" fill="none" stroke="currentColor" stroke-width="2">
              <polyline points="3 6 5 6 21 6"></polyline>
              <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path>
            </svg>
            <span>Clear WhatsApp Session</span>
          {/if}
        </button>
      </section>

      <div class="app-info">
        <span>Waplus v0.1.0 (Tauri v2 + Svelte 5 + Rust)</span>
      </div>
    </div>
  </div>
</div>

<style>
  .settings-container {
    width: 100vw;
    height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 16px;
    background-color: var(--bg-app);
  }

  .settings-card {
    width: 100%;
    height: 100%;
    background-color: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-md);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .settings-header {
    display: flex;
    align-items: center;
    padding: 14px 18px;
    border-bottom: 1px solid var(--border-color);
    background-color: var(--bg-titlebar);
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .brand-img {
    width: 22px;
    height: 22px;
    border-radius: 5px;
    object-fit: cover;
  }

  .title {
    font-size: 14px;
    font-weight: 700;
    color: var(--text-primary);
  }

  .settings-body {
    padding: 18px;
    display: flex;
    flex-direction: column;
    gap: 16px;
    overflow-y: auto;
  }

  .section {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .section-title {
    font-size: 11px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    color: var(--text-secondary);
  }

  .section-desc {
    font-size: 12px;
    color: var(--text-muted);
    line-height: 1.4;
  }

  .theme-options {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 8px;
  }

  .theme-btn {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 6px;
    padding: 10px 8px;
    border: 1px solid var(--border-color);
    border-radius: var(--radius-sm);
    background-color: var(--bg-surface);
    color: var(--text-secondary);
    font-size: 12px;
    font-weight: 500;
    transition: all 0.15s ease;
  }

  .theme-btn:hover {
    background-color: var(--bg-surface-hover);
    color: var(--text-primary);
  }

  .theme-btn.active {
    background-color: var(--bg-surface-active);
    color: var(--wa-green);
    border-color: var(--wa-green);
  }

  .info-box {
    background-color: var(--bg-surface-hover);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-sm);
    padding: 10px 14px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .info-row {
    display: flex;
    justify-content: space-between;
    font-size: 12px;
  }

  .info-label {
    color: var(--text-secondary);
  }

  .info-value {
    color: var(--text-primary);
    font-weight: 500;
  }

  .text-success {
    color: var(--wa-green);
  }

  .text-danger {
    color: var(--danger);
  }

  .danger-section {
    border-top: 1px solid var(--border-color);
    padding-top: 12px;
  }

  .btn-danger {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 9px 14px;
    border-radius: var(--radius-sm);
    background-color: transparent;
    border: 1px solid var(--danger);
    color: var(--danger);
    font-size: 12px;
    font-weight: 600;
    transition: all 0.15s ease;
  }

  .btn-danger:hover:not(:disabled) {
    background-color: var(--danger);
    color: #ffffff;
  }

  .btn-danger:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .app-info {
    text-align: center;
    font-size: 11px;
    color: var(--text-muted);
    padding-top: 4px;
  }
</style>
