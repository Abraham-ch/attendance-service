import { useSyncExternalStore } from 'react'

export type Toast = { id: number; message: string; tone: 'success' | 'error' }

// Module-level store: toast() works from anywhere (handlers, lib code) without a provider,
// <Toaster /> subscribes and renders the list
let toasts: Toast[] = []
const listeners = new Set<() => void>()
let nextId = 0

const emit = () => listeners.forEach((listener) => listener())

export function dismiss(id: number) {
  toasts = toasts.filter((toast) => toast.id !== id)
  emit()
}

export function toast(message: string, tone: Toast['tone'] = 'success') {
  const id = nextId++
  toasts = [...toasts, { id, message, tone }]
  emit()
  setTimeout(() => dismiss(id), tone === 'error' ? 6000 : 4000) // errors stay longer to be read
}

export function useToasts() {
  return useSyncExternalStore(
    (listener) => {
      listeners.add(listener)
      return () => listeners.delete(listener)
    },
    () => toasts
  )
}
