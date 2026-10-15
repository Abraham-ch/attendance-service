import type { ComponentProps } from 'react'

type Props = ComponentProps<'select'> & { compact?: boolean }

function Select({ compact, className = '', ...props }: Props) {
  return (
    <select
      className={`w-full rounded-ui border border-line-lit bg-paper text-ink transition-colors focus:border-ink focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-brass ${compact ? 'px-3 py-2 text-sm' : 'px-4 py-3'} ${className}`}
      {...props}
    />
  )
}

export default Select
