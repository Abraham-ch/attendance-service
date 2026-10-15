import { type Column, DataTable, PageHeader } from '@components/common'
import { Button, Mono } from '@components/ui'
import { useLoaderData } from 'react-router'

import type { Representative } from '@/types/records'

// ponytail: GET /representative doesn't return the linked student or relationship yet,
// add those columns back once the API joins student_representatives
const columns: Column<Representative>[] = [
  { header: 'Name', cell: (row) => `${row.first_name} ${row.last_name}` },
  { header: 'Phones', cell: (row) => <Mono>{row.phone.join(' · ')}</Mono> }
]

function Representatives() {
  const representatives = useLoaderData<Representative[]>()

  return (
    <>
      <PageHeader kicker="Records" title="Representatives">
        <Button>New representative</Button>
      </PageHeader>
      <DataTable columns={columns} rows={representatives} rowKey={(row) => row.id} />
    </>
  )
}

export default Representatives
