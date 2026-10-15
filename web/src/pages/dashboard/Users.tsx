import { type Column, DataTable, PageHeader } from '@components/common'
import { Button, Label } from '@components/ui'

import { users } from '@/data/sample'

const columns: Column<(typeof users)[number]>[] = [
  { header: 'Name', cell: (row) => row.name },
  { header: 'Email', cell: (row) => row.email },
  {
    header: 'Role',
    cell: (row) => <Label tone={row.role === 'User' ? 'dim' : 'brass'}>{row.role}</Label>
  }
]

function Users() {
  return (
    <>
      <PageHeader kicker="Admin" title="Users">
        <Button>New user</Button>
      </PageHeader>
      <DataTable columns={columns} rows={users} rowKey={(row) => row.email} />
    </>
  )
}

export default Users
