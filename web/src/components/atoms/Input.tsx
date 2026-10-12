import type { ComponentProps } from 'react'

function Input({ className = '', ...props }: ComponentProps<'input'>) {
  return (
    <input
      className={`w-full border border-line-lit bg-paper px-4 py-3 text-ink transition-colors placeholder:text-dim focus:border-ink focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-brass ${className}`}
      {...props}
    />
  )
}

export default Input
