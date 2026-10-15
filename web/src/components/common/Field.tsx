import { Input, Label } from '@components/ui'
import type { ComponentProps } from 'react'

type Props = ComponentProps<typeof Input> & { label: string }

function Field({ label, ...input }: Props) {
  return (
    <label className="flex flex-col gap-2">
      <Label>{label}</Label>
      <Input {...input} />
    </label>
  )
}

export default Field
