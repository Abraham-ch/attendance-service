import { type Column, DataTable, PageHeader, Pagination, SearchForm } from '@components/common'
import { Button, Label } from '@components/ui'
import { useState } from 'react'
import { useLoaderData, useRevalidator } from 'react-router'

import type { Page } from '@/types/records'
import type { User } from '@/types/user'

import { CreateUserModal } from './sections'

const columns: Column<User>[] = [
  { header: 'Name', cell: (row) => `${row.first_name} ${row.last_name}` },
  { header: 'Email', cell: (row) => row.email },
  {
    header: 'Role',
    cell: (row) => <Label tone={row.role === 'User' ? 'dim' : 'brass'}>{row.role}</Label>
  }
]

// The route itself is admin-only in the API (GET /user), so no role check is needed here
function Users() {
  const { items, total, page, per_page } = useLoaderData<Page<User>>()
  const revalidator = useRevalidator()
  const [creating, setCreating] = useState(false)

  return (
    <>
      <PageHeader kicker="Admin" title="Users">
        <Button onClick={() => setCreating(true)}>New user</Button>
      </PageHeader>
      <SearchForm placeholder="Search by name or email" />
      <DataTable
        columns={columns}
        rows={items}
        rowKey={(row) => row.id}
        footer={<Pagination page={page} perPage={per_page} total={total} />}
      />

      {creating && (
        <CreateUserModal
          onClose={() => setCreating(false)}
          onSaved={() => {
            setCreating(false)
            revalidator.revalidate()
          }}
        />
      )}
    </>
  )
}

export default Users
