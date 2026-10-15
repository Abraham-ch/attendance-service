import type { ReactNode } from 'react'

type Tone = 'dim' | 'brass' | 'ink'

const tones: Record<Tone, string> = {
  dim: 'text-dim',
  brass: 'text-brass-lo',
  ink: 'text-ink'
}

type Props = { children: ReactNode; tone?: Tone; className?: string }

function Label({ children, tone = 'dim', className = '' }: Props) {
  return (
    <span className={`font-monospace text-label uppercase ${tones[tone]} ${className}`}>
      {children}
    </span>
  )
}

export default Label
