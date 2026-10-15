import { PageHeader, RollPanel, StatCard } from '@components/common'
import type { Status } from '@components/ui'
import { today, useLiveRoll } from '@lib/roll'

const statuses: Status[] = ['Present', 'Late', 'Absent', 'Excused']

function Overview() {
  const { period, rows, connected } = useLiveRoll()
  const todayRows = rows.filter((row) => row.date === today())

  return (
    <>
      <PageHeader kicker={period ? `Period ${period.name}` : 'No current period'} title="Today" />
      <div className="grid gap-4 sm:grid-cols-2 xl:grid-cols-4">
        {statuses.map((status) => (
          <StatCard
            key={status}
            status={status}
            value={todayRows.filter((row) => row.status === status).length}
          />
        ))}
      </div>
      <RollPanel title="Latest records" rows={todayRows.slice(0, 10)} live={connected} />
    </>
  )
}

export default Overview
