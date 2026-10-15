import type { ComponentProps } from 'react'

const icons = {
  edit: 'M4 20h4L19 9l-4-4L4 16v4zM14 6l4 4',
  delete: 'M4 7h16M10 11v6M14 11v6M6 7l1 13h10l1-13M9 7V4h6v3',
  close: 'M6 6l12 12M18 6L6 18'
}

type Props = ComponentProps<'button'> & { icon: keyof typeof icons; label: string }

function IconButton({ icon, label, className = '', ...props }: Props) {
  return (
    <button
      type="button"
      aria-label={label}
      title={label}
      className={`cursor-pointer rounded-ui p-2 text-dim transition-colors hover:text-ink focus-visible:outline-2 focus-visible:outline-brass ${icon === 'delete' ? 'hover:text-absent' : ''} ${className}`}
      {...props}
    >
      <svg
        aria-hidden
        viewBox="0 0 24 24"
        className="size-4"
        fill="none"
        stroke="currentColor"
        strokeWidth={1.8}
        strokeLinecap="round"
        strokeLinejoin="round"
      >
        <path d={icons[icon]} />
      </svg>
    </button>
  )
}

export default IconButton
