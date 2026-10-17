import { type Column, DataTable, PageHeader, Pagination, SearchForm } from '@components/common'
import { Avatar, Button, IconButton, Label } from '@components/ui'
import { useAuth } from '@lib/context/AuthContext'
import { canManage } from '@lib/roles'
import { useState } from 'react'
import { useLoaderData, useRevalidator } from 'react-router'

import type { Page } from '@/types/records'
import type { User } from '@/types/user'

import { CreateUserModal, EditUserModal } from './sections'

const columns: Column<User>[] = [
  {
    header: 'Name',
    cell: (row) => {
      const name = `${row.first_name} ${row.last_name}`
      return (
        <span className="flex items-center gap-3">
          <Avatar src={row.avatar} name={name} className="size-8 text-xs" />
          {name}
        </span>
      )
    }
  },
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
  const { user: me } = useAuth()
  const [creating, setCreating] = useState(false)
  const [editing, setEditing] = useState<User | null>(null)

  const options: Column<User> = {
    header: 'Options',
    cell: (row) =>
      canManage(me, row) && (
        <IconButton
          icon="edit"
          label={`Edit ${row.first_name} ${row.last_name}`}
          onClick={() => setEditing(row)}
          className="-my-2"
        />
      )
  }

  return (
    <>
      <PageHeader kicker="Admin" title="Users">
        <Button onClick={() => setCreating(true)}>New user</Button>
      </PageHeader>
      <SearchForm placeholder="Search by name or email" />
      <DataTable
        columns={[...columns, options]}
        rows={items}
        rowKey={(row) => row.id}
        footer={<Pagination page={page} perPage={per_page} total={total} />}
      />

      {editing && (
        <EditUserModal
          user={editing}
          onClose={() => setEditing(null)}
          onSaved={() => {
            setEditing(null)
            revalidator.revalidate()
          }}
        />
      )}
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
