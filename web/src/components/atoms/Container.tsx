import type { ReactNode } from 'react'

function Container({ children, className = '' }: { children: ReactNode; className?: string }) {
  return <div className={`mx-auto w-full max-w-page px-edge ${className}`}>{children}</div>
}

export default Container
