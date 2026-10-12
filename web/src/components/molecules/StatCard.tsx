import type { Status } from '../atoms/StatusDot'
import StatusBadge from './StatusBadge'

function StatCard({ status, value }: { status: Status; value: number }) {
  return (
    <article className="flex flex-col gap-6 border border-line bg-surface p-6">
      <StatusBadge status={status} />
      <span className="text-5xl font-extrabold tracking-tight tabular-nums">{value}</span>
    </article>
  )
}

export default StatCard
