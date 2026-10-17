import { Label, type Status } from '@components/ui'

import RollRow from './RollRow'

type Props = {
  title: string
  rows: { id?: string; name: string; time: string; status: Status }[]
  live?: boolean // leave out on pages without a live feed
}

function RollPanel({ title, rows, live }: Props) {
  return (
    <div className="border border-line bg-surface">
      <div className="flex items-center justify-between gap-4 border-b border-line px-5 py-3">
        <Label>{title}</Label>
        {live !== undefined && (
          <span className="inline-flex items-center gap-2">
            <span
              aria-hidden
              className={`size-1.5 rounded-full ${live ? 'bg-present motion-safe:animate-pulse' : 'bg-absent'}`}
            />
            <Label tone="ink">{live ? 'Live' : 'Offline'}</Label>
          </span>
        )}
      </div>
      <ul>
        {rows.map((row) => (
          <RollRow key={row.id ?? row.name} name={row.name} time={row.time} status={row.status} />
        ))}
      </ul>
    </div>
  )
}

export default RollPanel
