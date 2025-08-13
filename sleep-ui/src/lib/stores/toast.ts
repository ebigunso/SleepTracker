import { writable } from 'svelte/store';

export type ToastType = 'info' | 'success' | 'error';
export type Toast = {
  id: string;
  type: ToastType;
  message: string;
  timeout?: number;
};

function createToasts() {
  const { subscribe, update } = writable<Toast[]>([]);

  function pushToast(t: Omit<Toast, 'id'>) {
    const id = crypto.randomUUID?.() ?? Math.random().toString(36).slice(2);
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
