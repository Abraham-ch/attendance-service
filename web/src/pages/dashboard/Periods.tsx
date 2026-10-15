import { type Column, DataTable, PageHeader } from '@components/common'
import { Button, Label, Mono } from '@components/ui'
import { periodState } from '@lib/api'
import { useLoaderData } from 'react-router'

import type { Period } from '@/types/records'

const columns: Column<Period>[] = [
  { header: 'Name', cell: (row) => row.name },
  { header: 'Starts', cell: (row) => <Mono>{row.start_date.slice(0, 10)}</Mono> },
  { header: 'Ends', cell: (row) => <Mono>{row.end_date.slice(0, 10)}</Mono> },
  {
    header: 'State',
    cell: (row) => {
      const state = periodState(row)
      return <Label tone={state === 'Current' ? 'brass' : 'dim'}>{state}</Label>
    }
  }
]

function Periods() {
  const periods = useLoaderData<Period[]>()

  return (
    <>
      <PageHeader kicker="Records" title="Periods">
        <Button>New period</Button>
      </PageHeader>
      <DataTable columns={columns} rows={periods} rowKey={(row) => row.id} />
    </>
  )
}

export default Periods
