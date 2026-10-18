import {
  type Column,
  DataTable,
  PageHeader,
  Pagination,
  SearchForm,
  StatusBadge
} from '@components/common'
import { Mono } from '@components/ui'
import { useLiveRoll } from '@lib/roll'
import { useSearchParams } from 'react-router'

type Row = ReturnType<typeof useLiveRoll>['rows'][number]

const columns: Column<Row>[] = [
  { header: 'Student', cell: (row) => row.name },
  { header: 'Date', cell: (row) => <Mono>{row.date}</Mono> },
  { header: 'Time', cell: (row) => <Mono>{row.time}</Mono> },
  { header: 'Status', cell: (row) => <StatusBadge status={row.status} /> },
  { header: 'Notes', cell: (row) => row.notes || '—' }
]

// The roll is already loaded whole for the live feed, so ?search, ?page and ?per_page
// (same URL as Students and Users) slice it here instead of in the API.
// ponytail: a paginated GET /period/:id/assistance once a period holds too many records
function Assistance() {
  const { period, rows } = useLiveRoll()
  const [params] = useSearchParams()

  const search = (params.get('search') ?? '').trim().toLowerCase()
  const found = rows.filter((row) => `${row.name} ${row.date}`.toLowerCase().includes(search))
  const perPage = Math.min(100, Math.max(1, Number(params.get('per_page')) || 10))
  const pages = Math.max(1, Math.ceil(found.length / perPage))
  const page = Math.min(pages, Math.max(1, Number(params.get('page')) || 1))

  return (
    <>
      <PageHeader
        kicker={period ? `Period ${period.name}` : 'No current period'}
        title="Assistance"
      />
      <SearchForm placeholder="Search by student or date" />
      <DataTable
        columns={columns}
        rows={found.slice((page - 1) * perPage, page * perPage)}
        rowKey={(row) => row.id}
        footer={<Pagination page={page} perPage={perPage} total={found.length} />}
      />
    </>
  )
}

export default Assistance
