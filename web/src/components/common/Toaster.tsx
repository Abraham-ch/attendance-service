import { IconButton } from '@components/ui'
import { dismiss, useToasts } from '@lib/toast'
import { useEffect, useRef } from 'react'

// A manual popover lives in the browser's top layer, so toasts show above an open <dialog>.
// Re-showing it on every new toast moves it above whatever opened since.
function Toaster() {
  const toasts = useToasts()
  const region = useRef<HTMLDivElement>(null)

  useEffect(() => {
    const el = region.current
    if (!el) return
    if (el.matches(':popover-open')) el.hidePopover()
    if (toasts.length > 0) el.showPopover()
  }, [toasts])

  return (
    <div
      ref={region}
      popover="manual"
      className="inset-auto right-4 bottom-4 m-0 flex w-[min(22rem,calc(100vw-2rem))] flex-col gap-2 overflow-visible border-0 bg-transparent p-0"
    >
      {toasts.map((toast) => (
        <div
          key={toast.id}
          role={toast.tone === 'error' ? 'alert' : 'status'}
          className={`flex items-start gap-3 rounded-ui border border-line border-l-4 bg-paper py-3 pr-2 pl-4 font-ui text-sm text-ink shadow-lg ${toast.tone === 'error' ? 'border-l-absent' : 'border-l-present'}`}
        >
          <span className="flex-1 pt-1.5">{toast.message}</span>
          <IconButton icon="close" label="Dismiss" onClick={() => dismiss(toast.id)} />
        </div>
      ))}
    </div>
  )
}

export default Toaster
