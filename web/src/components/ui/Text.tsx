import type { ReactNode } from 'react'

function Text({ children, className = '' }: { children: ReactNode; className?: string }) {
  return (
    <p
      className={`leading-relaxed text-mute [&_strong]:font-medium [&_strong]:text-ink ${className}`}
    >
      {children}
    </p>
  )
}

export default Text
