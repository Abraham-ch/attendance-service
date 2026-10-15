import { Heading, IconButton } from '@components/ui'
import { useEffect, useRef, type ReactNode } from 'react'

type Props = { title: string; onClose: () => void; children: ReactNode }

// Native <dialog>: focus trap, Esc to close and the backdrop come from the browser.
// Open while mounted, so render it conditionally: {open && <Modal …/>}
function Modal({ title, onClose, children }: Props) {
  const dialog = useRef<HTMLDialogElement>(null)

  useEffect(() => {
    dialog.current?.showModal()
  }, [])

  return (
    <dialog
      ref={dialog}
      onClose={onClose}
      aria-label={title}
      className="m-auto w-[min(32rem,calc(100vw-2rem))] rounded-xl border border-line bg-paper p-0 text-ink backdrop:bg-ink/40"
    >
      <header className="flex items-center justify-between gap-4 border-b border-line px-6 py-4">
        <Heading as="h2" size="item">
          {title}
        </Heading>
        <IconButton icon="close" label="Close" onClick={onClose} />
      </header>
      <div className="px-6 py-5">{children}</div>
    </dialog>
  )
}

export default Modal
