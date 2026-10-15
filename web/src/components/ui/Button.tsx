import type { ComponentProps } from 'react'

type Variant = 'primary' | 'ghost'

const variants: Record<Variant, string> = {
    primary: 'bg-ink text-paper hover:bg-brass-lo',
  ghost: 'border border-line-lit text-ink hover:border-ink'
}

type Props = ComponentProps<'button'> & { variant?: Variant }

function Button({ variant = 'primary', className = '', type = 'button', ...props }: Props) {
  return (
    <button
      type={type}
      className={`cursor-pointer rounded-ui px-5 py-3 font-ui text-label uppercase transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-brass ${variants[variant]} ${className}`}
      {...props}
    />
  )
}

export default Button
