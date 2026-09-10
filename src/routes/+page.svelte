<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { check, type Update, type DownloadEvent } from '@tauri-apps/plugin-updater';
  import { openUrl } from '@tauri-apps/plugin-opener';
  import { appState, type AppTheme } from '$lib/stores/appState.svelte';

  const appWindow = getCurrentWindow();
  let isClearing = $state(false);
  let clearSuccess = $state(false);
  let appVersion = $state('0.1.2');

  // Update states
  let updateStatus = $state<'idle' | 'checking' | 'up-to-date' | 'available' | 'downloading' | 'ready' | 'error'>('idle');
  let availableUpdate = $state<Update | null>(null);
  let updateProgress = $state(0);
  let updateStatusText = $state('');
  let errorMessage = $state('');

  onMount(async () => {
    try {
      const ver = await invoke<string>('get_app_version');
      if (ver) appVersion = ver;
    } catch (_) {}
  });

  async function handleCheckForUpdates() {
    try {
      updateStatus = 'checking';
      errorMessage = '';
      availableUpdate = null;
      const update = await check();
      if (update) {
        availableUpdate = update;
        updateStatus = 'available';
      } else {
        updateStatus = 'up-to-date';
      }
    } catch (error: any) {
      updateStatus = 'error';
      errorMessage = error?.message || String(error);
    }
  }

  async function handleInstallUpdate() {
    if (!availableUpdate) return;
    try {
      updateStatus = 'downloading';
      let downloaded = 0;
      let contentLength = 0;

      await availableUpdate.downloadAndInstall((event: DownloadEvent) => {
        if (event.event === 'Started') {
          contentLength = event.data.contentLength || 0;
          updateStatusText = 'Starting download...';
        } else if (event.event === 'Progress') {
          downloaded += event.data.chunkLength;
          if (contentLength > 0) {
            updateProgress = Math.round((downloaded / contentLength) * 100);
            updateStatusText = `${(downloaded / (1024 * 1024)).toFixed(1)} MB / ${(contentLength / (1024 * 1024)).toFixed(1)} MB (${updateProgress}%)`;
          } else {
            updateStatusText = `${(downloaded / (1024 * 1024)).toFixed(1)} MB downloaded`;
          }
        } else if (event.event === 'Finished') {
          updateStatus = 'ready';
          updateStatusText = 'Update installed. Restarting...';
        }
      });
    } catch (error: any) {
      updateStatus = 'error';
      errorMessage = error?.message || String(error);
    }
  }

  function openGitHub() {
    openUrl('https://github.com/luqman-v1');
  }
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
      <section class="section update-section">
        <div class="section-title-row">
          <h4 class="section-title">Software Update</h4>
          <span class="version-badge">v{appVersion}</span>
        </div>

        {#if (updateStatus === 'available' || updateStatus === 'downloading' || updateStatus === 'ready') && availableUpdate}
          <div class="update-card">
            <div class="update-card-info">
              <div class="update-pill">Update Available</div>
              <span class="update-ver-label">v{availableUpdate.version}</span>
            </div>
            {#if availableUpdate.body}
              <div class="update-changelog">{availableUpdate.body}</div>
            {/if}
            <button
              class="btn-update"
              disabled={updateStatus === 'downloading'}
              onclick={handleInstallUpdate}
            >
              {#if updateStatus === 'downloading'}
                <span>{updateStatusText || 'Downloading...'}</span>
              {:else}
                <svg viewBox="0 0 24 24" width="14" height="14" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
                  <polyline points="7 10 12 15 17 10"></polyline>
                  <line x1="12" y1="15" x2="12" y2="3"></line>
                </svg>
                <span>Download & Install Update</span>
              {/if}
            </button>
            {#if updateStatus === 'downloading' && updateProgress > 0}
              <div class="progress-track">
                <div class="progress-fill" style="width: {updateProgress}%"></div>
              </div>
            {/if}
          </div>
        {:else}
          <div class="update-panel">
            <div class="update-status-msg">
              {#if updateStatus === 'checking'}
                <span class="spinner"></span>
                <span class="msg-text">Checking for latest version...</span>
              {:else if updateStatus === 'up-to-date'}
                <span class="text-success status-icon">✓</span>
                <span class="msg-text text-success">You're on the latest version</span>
              {:else if updateStatus === 'ready'}
                <span class="text-success status-icon">✓</span>
                <span class="msg-text text-success">Update complete. Restarting...</span>
              {:else if updateStatus === 'error'}
                <span class="text-danger status-icon">!</span>
                <span class="msg-text text-danger" title={errorMessage}>
                  {errorMessage.length > 35 ? errorMessage.slice(0, 35) + '...' : errorMessage}
                </span>
              {:else}
                <span class="msg-text text-muted">Auto-check enabled on startup</span>
              {/if}
            </div>

            <button
              class="btn-check"
              disabled={updateStatus === 'checking'}
              onclick={handleCheckForUpdates}
            >
              {#if updateStatus === 'checking'}
                <span>Checking...</span>
              {:else}
                <svg viewBox="0 0 24 24" width="13" height="13" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M21.5 2v6h-6M21.34 15.57a10 10 0 1 1-.57-8.38l5.67-5.67"></path>
                </svg>
                <span>Check Updates</span>
              {/if}
            </button>
          </div>
        {/if}
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
        <span>Waplus v{appVersion}</span>
        <span class="divider">•</span>
        <button class="github-credit" onclick={openGitHub}>
          <svg viewBox="0 0 24 24" width="13" height="13" fill="currentColor">
            <path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0 0 24 12c0-6.63-5.37-12-12-12z"/>
          </svg>
          <span>@luqman-v1</span>
        </button>
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
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    font-size: 11px;
    color: var(--text-muted);
    padding-top: 4px;
  }

  .divider {
    opacity: 0.5;
  }

  .github-credit {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    background: none;
    border: none;
    padding: 2px 4px;
    border-radius: 4px;
    color: var(--text-secondary);
    font-size: 11px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .github-credit:hover {
    color: var(--text-primary);
    background-color: var(--bg-surface-hover);
  }
  .section-title-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .version-badge {
    font-size: 11px;
    font-weight: 600;
    color: var(--text-secondary);
    background-color: var(--bg-surface-hover);
    padding: 2px 8px;
    border-radius: 12px;
    border: 1px solid var(--border-color);
  }

  .update-panel {
    background-color: var(--bg-surface-hover);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-sm);
    padding: 10px 14px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 12px;
  }

  .update-status-msg {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 12px;
    min-width: 0;
  }

  .status-icon {
    font-weight: 700;
  }

  .msg-text {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .spinner {
    width: 12px;
    height: 12px;
    border: 2px solid var(--border-color);
    border-top-color: var(--wa-green);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .btn-check {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 6px 12px;
    border-radius: var(--radius-sm);
    background-color: var(--bg-surface);
    border: 1px solid var(--border-color);
    color: var(--text-primary);
    font-size: 12px;
    font-weight: 500;
    white-space: nowrap;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .btn-check:hover:not(:disabled) {
    background-color: var(--bg-surface-active);
    border-color: var(--wa-green);
    color: var(--wa-green);
  }

  .btn-check:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .update-card {
    background: linear-gradient(135deg, rgba(37, 211, 102, 0.08), rgba(18, 140, 126, 0.04));
    border: 1px solid var(--wa-green);
    border-radius: var(--radius-sm);
    padding: 12px 14px;
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .update-card-info {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .update-pill {
    background-color: var(--wa-green);
    color: #ffffff;
    font-size: 10px;
    font-weight: 700;
    text-transform: uppercase;
    padding: 2px 6px;
    border-radius: 4px;
    letter-spacing: 0.5px;
  }

  .update-ver-label {
    font-size: 13px;
    font-weight: 600;
    color: var(--text-primary);
  }

  .update-changelog {
    font-size: 12px;
    color: var(--text-secondary);
    max-height: 80px;
    overflow-y: auto;
    white-space: pre-wrap;
    background: var(--bg-surface);
    padding: 8px 10px;
    border-radius: var(--radius-sm);
    border: 1px solid var(--border-color);
  }

  .btn-update {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 9px 14px;
    border-radius: var(--radius-sm);
    background-color: var(--wa-green);
    border: none;
    color: #ffffff;
    font-size: 12px;
    font-weight: 600;
    cursor: pointer;
    transition: opacity 0.15s ease;
  }

  .btn-update:hover:not(:disabled) {
    opacity: 0.9;
  }

  .btn-update:disabled {
    opacity: 0.7;
    cursor: wait;
  }

  .progress-track {
    width: 100%;
    height: 4px;
    background: var(--border-color);
    border-radius: 2px;
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    background: var(--wa-green);
    transition: width 0.2s ease;
  }
</style>
