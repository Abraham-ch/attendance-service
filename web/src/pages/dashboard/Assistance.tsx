import { type Column, DataTable, PageHeader, StatusBadge } from '@components/common'
import { Mono } from '@components/ui'

import { currentPeriod, roll } from '@/data/sample'

const columns: Column<(typeof roll)[number]>[] = [
  { header: 'Student', cell: (row) => row.name },
  { header: 'Time', cell: (row) => <Mono>{row.time}</Mono> },
  { header: 'Status', cell: (row) => <StatusBadge status={row.status} /> },
  { header: 'Notes', cell: (row) => row.notes || '—' }
]

function Assistance() {
  return (
    <>
      <PageHeader kicker={`Period ${currentPeriod}`} title="Assistance" />
      <DataTable columns={columns} rows={roll} rowKey={(row) => row.name} />
    </>
  )
}

export default Assistance
