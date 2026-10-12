import Label from '../atoms/Label'
import type { Status } from '../atoms/StatusDot'
import RollRow from '../molecules/RollRow'

type Props = { title: string; rows: { name: string; time: string; status: Status }[] }

function RollPanel({ title, rows }: Props) {
  return (
    <div className="border border-line bg-surface">
      <div className="flex items-center justify-between gap-4 border-b border-line px-5 py-3">
        <Label>{title}</Label>
        <span className="inline-flex items-center gap-2">
          <span
            aria-hidden
            className="size-1.5 rounded-full bg-present motion-safe:animate-pulse"
          />
          <Label tone="ink">Live</Label>
        </span>
      </div>
      <ul>
        {rows.map((row) => (
          <RollRow key={row.name} name={row.name} time={row.time} status={row.status} />
        ))}
      </ul>
    </div>
  )
}

export default RollPanel
