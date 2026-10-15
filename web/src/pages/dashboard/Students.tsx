import { type Column, DataTable, PageHeader } from '@components/common'
import { Button, Mono } from '@components/ui'

import { students } from '@/data/sample'

const columns: Column<(typeof students)[number]>[] = [
  { header: 'Name', cell: (row) => row.name },
  { header: 'DNI', cell: (row) => <Mono>{row.dni}</Mono> },
  { header: 'Email', cell: (row) => row.email },
  { header: 'Gender', cell: (row) => row.gender },
  { header: 'Phone', cell: (row) => <Mono>{row.phone}</Mono> }
]

function Students() {
  return (
    <>
      <PageHeader kicker="Records" title="Students">
        <Button>New student</Button>
      </PageHeader>
      <DataTable columns={columns} rows={students} rowKey={(row) => row.dni} />
    </>
  )
}

export default Students
