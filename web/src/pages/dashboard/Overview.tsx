import { PageHeader, RollPanel, StatCard } from '@components/common'
import type { Status } from '@components/ui'
import { today, useLiveRoll } from '@lib/roll'

const statuses: Status[] = ['Present', 'Late', 'Absent', 'Excused']

// Staff see today's roll for the whole school, a student their own record for the period
function Overview() {
  const { admin, period, rows, live } = useLiveRoll()
  const shown = admin ? rows.filter((row) => row.date === today()) : rows

  return (
    <>
      <PageHeader
        kicker={period ? `Period ${period.name}` : 'No current period'}
        title={admin ? 'Today' : 'My attendance'}
      />
      <div className="grid gap-4 sm:grid-cols-2 xl:grid-cols-4">
        {statuses.map((status) => (
          <StatCard
            key={status}
            status={status}
            value={shown.filter((row) => row.status === status).length}
          />
        ))}
      </div>
      <RollPanel
        title={admin ? 'Latest records' : 'My latest records'}
        rows={shown.slice(0, 10)}
        live={live}
      />
    </>
  )
}

export default Overview
