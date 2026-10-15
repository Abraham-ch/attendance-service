import { Heading, Label } from '@components/ui'
import type { ReactNode } from 'react'

type Props = { kicker: string; title: string; children?: ReactNode }

function PageHeader({ kicker, title, children }: Props) {
  return (
    <header className="flex flex-wrap items-end justify-between gap-4 border-b border-line pb-6">
      <div className="flex flex-col gap-2">
        <Label>{kicker}</Label>
        <Heading as="h1" size="title">
          {title}
        </Heading>
      </div>
      {children}
    </header>
  )
}

export default PageHeader
