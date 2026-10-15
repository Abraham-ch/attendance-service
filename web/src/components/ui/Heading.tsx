import type { ReactNode } from 'react'

type Size = 'display' | 'title' | 'item'

const sizes: Record<Size, string> = {
  display: 'text-display uppercase',
  title: 'text-title uppercase',
  item: 'text-xl font-semibold tracking-tight'
}

type Props = { children: ReactNode; size: Size; as?: 'h1' | 'h2' | 'h3'; className?: string }

function Heading({ children, size, as: Tag = 'h2', className = '' }: Props) {
  return <Tag className={`text-balance ${sizes[size]} ${className}`}>{children}</Tag>
}

export default Heading
