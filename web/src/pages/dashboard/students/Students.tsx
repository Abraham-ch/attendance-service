import { type Column, DataTable, PageHeader, Pagination, SearchForm } from '@components/common'
import { Button, IconButton, Mono } from '@components/ui'
import { useIsAdmin } from '@lib/roles'
import { useState } from 'react'
import { useLoaderData, useRevalidator } from 'react-router'

import type { Page, Student } from '@/types/records'

import { CreateStudentModal, DeleteStudentModal, EditStudentModal, IdCardModal } from './sections'

const columns: Column<Student>[] = [
  { header: 'Name', cell: (row) => `${row.first_name} ${row.last_name}` },
  { header: 'DNI', cell: (row) => <Mono>{row.dni}</Mono> },
  { header: 'Email', cell: (row) => row.email ?? '—' },
  { header: 'Gender', cell: (row) => row.gender },
  { header: 'Phone', cell: (row) => <Mono>{row.phone ?? '—'}</Mono> }
]

type Dialog = { kind: 'create' } | { kind: 'edit' | 'delete' | 'card'; student: Student } | null

function Students() {
  const { items, total, page, per_page } = useLoaderData<Page<Student>>()
  const admin = useIsAdmin()
  const revalidator = useRevalidator()
  const [dialog, setDialog] = useState<Dialog>(null)

  const close = () => setDialog(null)
  const saved = () => {
    setDialog(null)
    revalidator.revalidate() // rerun the loader so the table shows the change
  }

  const options: Column<Student> = {
    header: 'Options',
    cell: (row) => {
      const name = `${row.first_name} ${row.last_name}`
      return (
        <div className="-my-2 flex gap-1">
          <IconButton
            icon="pdf"
            label={`Student card for ${name}`}
            onClick={() => setDialog({ kind: 'card', student: row })}
          />
          <IconButton
            icon="edit"
            label={`Edit ${name}`}
            onClick={() => setDialog({ kind: 'edit', student: row })}
          />
          <IconButton
            icon="delete"
            label={`Delete ${name}`}
            onClick={() => setDialog({ kind: 'delete', student: row })}
          />
        </div>
      )
    }
  }

  return (
    <>
      <PageHeader kicker="Records" title="Students">
        {admin && <Button onClick={() => setDialog({ kind: 'create' })}>New student</Button>}
      </PageHeader>
      <SearchForm placeholder="Search by name, DNI or email" />
      <DataTable
        columns={admin ? [...columns, options] : columns}
        rows={items}
        rowKey={(row) => row.id}
        footer={<Pagination page={page} perPage={per_page} total={total} />}
      />

      {dialog?.kind === 'create' && <CreateStudentModal onClose={close} onSaved={saved} />}
      {dialog?.kind === 'edit' && (
        <EditStudentModal student={dialog.student} onClose={close} onSaved={saved} />
      )}
      {dialog?.kind === 'card' && <IdCardModal student={dialog.student} onClose={close} />}
      {dialog?.kind === 'delete' && (
        <DeleteStudentModal student={dialog.student} onClose={close} onSaved={saved} />
      )}
    </>
  )
}

export default Students
