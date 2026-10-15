import { type Column, DataTable, PageHeader, StatusBadge } from '@components/common'
import { Mono } from '@components/ui'
import { useLiveRoll } from '@lib/roll'

type Row = ReturnType<typeof useLiveRoll>['rows'][number]

const columns: Column<Row>[] = [
  { header: 'Student', cell: (row) => row.name },
  { header: 'Date', cell: (row) => <Mono>{row.date}</Mono> },
  { header: 'Time', cell: (row) => <Mono>{row.time}</Mono> },
  { header: 'Status', cell: (row) => <StatusBadge status={row.status} /> },
  { header: 'Notes', cell: (row) => row.notes || '—' }
]

function Assistance() {
  const { period, rows } = useLiveRoll()

  return (
    <>
      <PageHeader
        kicker={period ? `Period ${period.name}` : 'No current period'}
        title="Assistance"
      />
      <DataTable columns={columns} rows={rows} rowKey={(row) => row.id} />
    </>
  )
}

export default Assistance
