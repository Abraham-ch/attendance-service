import type { ComponentProps } from 'react'

// compact: dense forms like modals and filters, the default size is for roomy pages like login
type Props = ComponentProps<'input'> & { compact?: boolean }

function Input({ compact, className = '', ...props }: Props) {
  return (
    <input
      className={`w-full rounded-ui border border-line-lit bg-paper text-ink transition-colors placeholder:text-dim focus:border-ink focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-brass ${compact ? 'px-3 py-2 text-sm' : 'px-4 py-3'} ${className}`}
      {...props}
    />
  )
}

export default Input
