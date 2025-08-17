import { writable } from 'svelte/store';

export type ToastType = 'info' | 'success' | 'error';
export type Toast = {
  id: string;
  type: ToastType;
  message: string;
  timeout?: number;
};

/**
 * Secure random ID generator as a fallback for crypto.randomUUID.
 * Generates 16 random bytes and returns a 32-char hex string.
 */
function secureRandomId(): string {
  const c = (globalThis as any).crypto as Crypto | undefined;
  if (c?.getRandomValues) {
    const array = new Uint8Array(16);
    c.getRandomValues(array);
    return Array.from(array, (b) => b.toString(16).padStart(2, '0')).join('');
  }
  // Last-resort non-cryptographic fallback; extremely unlikely on modern runtimes.
  // This path is retained only to avoid hard failure if no Web Crypto is available.
  return `${Date.now().toString(36)}-${Math.random().toString(36).slice(2)}`;
}

function createToasts() {
  const { subscribe, update } = writable<Toast[]>([]);

  function pushToast(t: Omit<Toast, 'id'>) {
    const id =
      typeof crypto !== 'undefined' &&
      typeof crypto.randomUUID === 'function'
        ? crypto.randomUUID()
        : secureRandomId();
    const toast: Toast = { id, ...t };
    update((list) => [...list, toast]);
    if (t.timeout && t.timeout > 0) {
      setTimeout(() => dismissToast(id), t.timeout);
    }
    return id;
  }

  function dismissToast(id: string) {
    update((list) => list.filter((t) => t.id !== id));
  }

  return {
    subscribe,
    pushToast,
    dismissToast
  };
}

export const toasts = createToasts();
export const pushToast = toasts.pushToast;
export const dismissToast = toasts.dismissToast;
