export type Status = 'Present' | 'Late' | 'Absent' | 'Excused'

const colors: Record<Status, string> = {
  Present: 'bg-present',
  Late: 'bg-brass',
  Absent: 'bg-absent',
  Excused: 'bg-steel'
}

function StatusDot({ status }: { status: Status }) {
  return <span aria-hidden className={`inline-block size-1.5 rounded-full ${colors[status]}`} />
}

export default StatusDot
