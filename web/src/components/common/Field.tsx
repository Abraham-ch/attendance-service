import { Input, Label, Select } from '@components/ui'
import type { ComponentProps, ReactNode } from 'react'

type Common = { label: string; error?: string; compact?: boolean; className?: string }

function Wrapper({
  label,
  error,
  compact,
  className = '',
  children
}: Common & { children: ReactNode }) {
  return (
    <label className={`flex flex-col ${compact ? 'gap-1.5' : 'gap-2'} ${className}`}>
      <Label>{label}</Label>
      {children}
      {error && <span className="text-sm text-absent">{error}</span>}
    </label>
  )
}

type Props = ComponentProps<typeof Input> & Common

function Field({ label, error, compact, className, ...input }: Props) {
  return (
    <Wrapper label={label} error={error} compact={compact} className={className}>
      <Input compact={compact} aria-invalid={error ? true : undefined} {...input} />
    </Wrapper>
  )
}

type SelectProps = ComponentProps<typeof Select> & Common

export function SelectField({ label, error, compact, className, ...select }: SelectProps) {
  return (
    <Wrapper label={label} error={error} compact={compact} className={className}>
      <Select compact={compact} aria-invalid={error ? true : undefined} {...select} />
    </Wrapper>
  )
}

export default Field
