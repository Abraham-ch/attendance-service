import type { ReactNode } from 'react'

import type { Status } from '../atoms/StatusDot'
import Text from '../atoms/Text'
import StatusBadge from './StatusBadge'

function StatusCard({ status, children }: { status: Status; children: ReactNode }) {
  return (
    <article className="flex flex-col gap-4 border border-line bg-surface p-6 transition-colors hover:border-line-lit">
      <StatusBadge status={status} />
      <Text>{children}</Text>
    </article>
  )
}

export default StatusCard
