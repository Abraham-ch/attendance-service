import { Chart, PageHeader, RollPanel, StatCard } from '@components/common'
import { Label, Text, type Status } from '@components/ui'
import { periodState, type loadOverview } from '@lib/api'
import { statusBars } from '@lib/charts'
import { today, useLiveRoll } from '@lib/roll'
import type { ReactNode } from 'react'
import { useLoaderData } from 'react-router'

const statuses: Status[] = ['Present', 'Late', 'Absent', 'Excused']
const DAYS = 14 // school days in the by-day chart

const dayLabel = (date: string) =>
  new Date(`${date}T00:00:00Z`).toLocaleDateString([], {
    month: 'short',
    day: 'numeric',
    timeZone: 'UTC' // the API stores UTC dates
  })

// Staff see today's roll for the whole school, a student their own record for the period
function Overview() {
  const { admin, period, rows, live } = useLiveRoll()
  const { periods, history } = useLoaderData<typeof loadOverview>()
  const shown = admin ? rows.filter((row) => row.date === today()) : rows

  // days of the current period that have records, live scans included
  const days = [...new Set(rows.map((row) => row.date))].toSorted().slice(-DAYS)
  const byDay = statusBars(
    days.map((date) => ({ key: date, label: dayLabel(date) })),
    rows.map((row) => ({ key: row.date, status: row.status }))
  )

  // the current period from the live rows, the others as loaded
  const byPeriod = statusBars(
    periods
      .filter((p) => periodState(p) !== 'Upcoming')
      .toSorted((a, b) => a.start_date.localeCompare(b.start_date))
      .map((p) => ({ key: p.id, label: p.name })),
    [
      ...history
        .filter((record) => record.period_id !== period?.id)
        .map((record) => ({ key: record.period_id, status: record.status })),
      ...rows.map((row) => ({ key: period?.id ?? '', status: row.status }))
    ]
  )

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
      <div className={`grid gap-4 ${admin ? 'xl:grid-cols-2' : ''}`}>
        {admin && (
          <ChartPanel title="Attendance by day" empty={days.length === 0}>
            <Chart option={byDay} label="Attendance by day, stacked by status" />
          </ChartPanel>
        )}
        <ChartPanel
          title={admin ? 'Attendance by period' : 'My attendance by period'}
          empty={history.length === 0 && rows.length === 0}
        >
          <Chart option={byPeriod} label="Attendance by period, stacked by status" />
        </ChartPanel>
      </div>
      <RollPanel
        title={admin ? 'Latest records' : 'My latest records'}
        rows={shown.slice(0, 10)}
        live={live}
      />
    </>
  )
}

function ChartPanel({
  title,
  empty,
  children
}: {
  title: string
  empty: boolean
  children: ReactNode
}) {
  return (
    <section className="min-w-0 border border-line bg-surface">
      <div className="border-b border-line px-5 py-3">
        <Label>{title}</Label>
      </div>
      <div className="p-5">
        {empty ? <Text className="text-sm">No records yet.</Text> : children}
      </div>
    </section>
  )
}

export default Overview
