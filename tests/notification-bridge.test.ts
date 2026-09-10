/// <reference types="bun-types" />
import { describe, it, expect, mock } from 'bun:test';
import { readFileSync } from 'fs';
import { resolve } from 'path';

interface InvokeCall {
  cmd: string;
  args: {
    options?: {
      title?: string;
      body?: string;
      silent?: boolean;
    };
  };
}

interface MockNotificationInstance {
  title: string;
  body: string;
  tag: string;
  silent: boolean;
  onclick: ((ev: unknown) => void) | null;
  onclose: ((ev: unknown) => void) | null;
  onerror: ((ev: unknown) => void) | null;
  onshow: ((ev: unknown) => void) | null;
  addEventListener: (type: string, listener: (ev: unknown) => void) => void;
  removeEventListener: (type: string, listener: (ev: unknown) => void) => void;
  dispatchEvent: (event: { type: string }) => boolean;
  close: () => void;
}

interface MockNotificationConstructor {
  new (title: string, options?: { body?: string; tag?: string; silent?: boolean }): MockNotificationInstance;
  permission: string;
  requestPermission: (callback?: (permission: string) => void) => Promise<string>;
  maxActions: number;
}

interface MockWindow {
  __TAURI_INTERNALS__: {
    invoke: (cmd: string, args: unknown) => Promise<unknown>;
  };
  Notification: MockNotificationConstructor;
  ServiceWorkerRegistration: {
    new (): {
      showNotification: (title: string, options?: { body?: string }) => Promise<void>;
    };
    prototype: {
      showNotification?: (title: string, options?: { body?: string }) => Promise<void>;
    };
  };
}

interface MockNavigator {
  permissions: {
    query: (parameters: { name: string }) => Promise<{ state: string; name: string }>;
  };
}

describe('WhatsApp Native Notification Bridge E2E Contract', () => {
  const libRsPath = resolve(import.meta.dir, '../src-tauri/src/lib.rs');
  const libRsContent = readFileSync(libRsPath, 'utf8');
  const scriptMatch = libRsContent.match(/const NOTIFICATION_SCRIPT: &str = r#"\s*([\s\S]*?)\s*"#;/);

  if (!scriptMatch) {
    throw new Error('Failed to extract NOTIFICATION_SCRIPT from src-tauri/src/lib.rs');
  }

  const notificationScript = scriptMatch[1];

  function setupMockEnvironment() {
    const invokeCalls: InvokeCall[] = [];
    const { promise: invokeSettled, resolve: markInvokeSettled } = Promise.withResolvers<void>();

    const mockInvoke = mock(async (cmd: string, args: unknown) => {
      invokeCalls.push({ cmd, args: args as InvokeCall['args'] });
      markInvokeSettled();
      return null;
    });

    const mockWindow: MockWindow = {
      __TAURI_INTERNALS__: {
        invoke: mockInvoke,
      },
      Notification: undefined as unknown as MockNotificationConstructor,
      ServiceWorkerRegistration: function (this: { showNotification: (title: string, options?: { body?: string }) => Promise<void> }) {} as unknown as MockWindow['ServiceWorkerRegistration'],
    };
    mockWindow.ServiceWorkerRegistration.prototype = {};

    const mockNavigator: MockNavigator = {
      permissions: {
        query: mock(async (parameters: { name: string }) => ({
          state: 'prompt',
          name: parameters.name,
        })),
      },
    };

    const runner = new Function('window', 'navigator', 'console', notificationScript);
    runner(mockWindow, mockNavigator, console);

    return {
      window: mockWindow,
      navigator: mockNavigator,
      invokeCalls,
      invokeSettled,
    };
  }

  it('script extracts cleanly and is valid JavaScript', () => {
    expect(notificationScript).toBeDefined();
    expect(notificationScript.length).toBeGreaterThan(100);
  });

  it('auto-resolves Notification.permission to "granted"', () => {
    const { window } = setupMockEnvironment();
    expect(window.Notification).toBeDefined();
    expect(window.Notification.permission).toBe('granted');
  });

  it('Notification.requestPermission() resolves to "granted"', async () => {
    const { window } = setupMockEnvironment();
    let callbackResult = '';
    const promise = window.Notification.requestPermission((res: string) => {
      callbackResult = res;
    });
    const result = await promise;
    expect(result).toBe('granted');
    expect(callbackResult).toBe('granted');
  });

  it('intercepts navigator.permissions.query({ name: "notifications" }) and returns granted state', async () => {
    const { navigator } = setupMockEnvironment();
    const queryRes = await navigator.permissions.query({ name: 'notifications' });
    expect(queryRes.state).toBe('granted');
    expect(queryRes.name).toBe('notifications');
  });

  it('forwards new Notification(...) to Tauri plugin:notification|notify IPC', async () => {
    const { window, invokeCalls, invokeSettled } = setupMockEnvironment();

    const notif = new window.Notification('WhatsApp - Budi', {
      body: 'Halo bro, ada waktu ngobrol?',
      tag: 'chat-42',
      silent: false,
    });

    expect(notif.title).toBe('WhatsApp - Budi');
    expect(notif.body).toBe('Halo bro, ada waktu ngobrol?');
    expect(notif.tag).toBe('chat-42');

    await invokeSettled;

    expect(invokeCalls.length).toBe(1);
    expect(invokeCalls[0].cmd).toBe('plugin:notification|notify');
    expect(invokeCalls[0].args).toEqual({
      options: {
        title: 'WhatsApp - Budi',
        body: 'Halo bro, ada waktu ngobrol?',
        silent: false,
      },
    });
  });

  it('supports DOM EventTarget methods (addEventListener, removeEventListener, dispatchEvent, close)', async () => {
    const { window, invokeSettled } = setupMockEnvironment();

    const notif = new window.Notification('Test Event', { body: 'Testing events' });
    let showFired = false;
    let closeFired = false;

    notif.addEventListener('show', () => {
      showFired = true;
    });
    notif.onclose = () => {
      closeFired = true;
    };

    await invokeSettled;
    // Tick microtasks for notification onshow / show event listeners
    await Promise.resolve();
    expect(showFired).toBe(true);

    notif.close();
    expect(closeFired).toBe(true);
  });

  it('ServiceWorkerRegistration.prototype.showNotification forwards to native notification', async () => {
    const { window, invokeCalls, invokeSettled } = setupMockEnvironment();

    const swReg = new window.ServiceWorkerRegistration();
    await swReg.showNotification('ServiceWorker WhatsApp', {
      body: 'Pesan masuk dari service worker',
    });

    await invokeSettled;

    expect(invokeCalls.length).toBe(1);
    expect(invokeCalls[0].cmd).toBe('plugin:notification|notify');
    expect(invokeCalls[0].args.options?.title).toBe('ServiceWorker WhatsApp');
    expect(invokeCalls[0].args.options?.body).toBe('Pesan masuk dari service worker');
  });
});
