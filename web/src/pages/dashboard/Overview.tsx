import { PageHeader, RollPanel, StatCard } from '@components/common'
import type { Status } from '@components/ui'

import { currentPeriod, roll } from '@/data/sample'

const statuses: Status[] = ['Present', 'Late', 'Absent', 'Excused']

function Overview() {
  return (
    <>
      <PageHeader kicker={`Period ${currentPeriod}`} title="Today" />
      <div className="grid gap-4 sm:grid-cols-2 xl:grid-cols-4">
        {statuses.map((status) => (
          <StatCard
            key={status}
            status={status}
            value={roll.filter((entry) => entry.status === status).length}
          />
        ))}
      </div>
      <RollPanel title="Latest records" rows={roll} />
    </>
  )
}

export default Overview
