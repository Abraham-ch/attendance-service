import type { ReactNode } from 'react'
import { Link } from 'react-router'

type Size = 'label' | 'lead'

const sizes: Record<Size, string> = {
  label: 'font-monospace text-label uppercase',
  lead: 'text-xl font-semibold tracking-tight'
}

type Props = { children: ReactNode; href: string; size?: Size }

function ArrowLink({ children, href, size = 'label' }: Props) {
  const className = `group inline-flex items-baseline gap-2 text-brass-lo transition-colors hover:text-ink focus-visible:outline-2 focus-visible:outline-offset-4 focus-visible:outline-brass ${sizes[size]}`
  const content = (
    <>
      {children}
      <span aria-hidden className="transition-transform group-hover:translate-x-1">
        →
      </span>
    </>
  )

  // app routes navigate client-side, hashes and external urls stay plain anchors
  return href.startsWith('/') ? (
    <Link to={href} className={className}>
      {content}
    </Link>
  ) : (
    <a href={href} className={className}>
      {content}
    </a>
  )
}

export default ArrowLink
