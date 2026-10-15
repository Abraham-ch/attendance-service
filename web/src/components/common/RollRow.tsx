import type { Status } from '@components/ui'

import StatusBadge from './StatusBadge'

type Props = { name: string; time: string; status: Status }

function RollRow({ name, time, status }: Props) {
  return (
    <li className="grid grid-cols-[1fr_auto_6.5rem] items-center gap-4 border-b border-line px-5 py-3.5 last:border-b-0">
      <span className="font-medium">{name}</span>
      <span className="font-monospace text-xs text-dim">{time}</span>
      <StatusBadge status={status} />
    </li>
  )
}

export default RollRow
