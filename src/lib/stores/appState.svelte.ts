export type ConnectionStatus = 'connected' | 'connecting' | 'offline';
export type AppTheme = 'dark' | 'light' | 'system';

export class AppState {
  connectionStatus = $state<ConnectionStatus>('connecting');
  isSettingsOpen = $state<boolean>(false);
  isWebViewReady = $state<boolean>(false);
  theme = $state<AppTheme>('system');
  isMaximized = $state<boolean>(false);

  setConnectionStatus(status: ConnectionStatus) {
    this.connectionStatus = status;
  }

  toggleSettings() {
    this.isSettingsOpen = !this.isSettingsOpen;
  }

  closeSettings() {
    this.isSettingsOpen = false;
  }

  setWebViewReady(ready: boolean) {
    this.isWebViewReady = ready;
  }

  setTheme(theme: AppTheme) {
    this.theme = theme;
  }

  setMaximized(maximized: boolean) {
    this.isMaximized = maximized;
  }
}

export const appState = new AppState();
