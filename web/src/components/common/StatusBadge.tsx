import { Label, type Status, StatusDot } from '@components/ui'

function StatusBadge({ status }: { status: Status }) {
  return (
    <span className="inline-flex items-center gap-2">
      <StatusDot status={status} />
      <Label tone="ink">{status}</Label>
    </span>
  )
}

export default StatusBadge
