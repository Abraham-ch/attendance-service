import type { ReactNode } from 'react'

function Mono({ children }: { children: ReactNode }) {
  return <span className="font-monospace text-xs text-dim">{children}</span>
}

export default Mono
