import { Modal } from '@components/common'
import { Button, Text } from '@components/ui'
import { apiError, deleteStudent } from '@lib/api'
import { toast } from '@lib/toast'
import { useState } from 'react'

import type { Student } from '@/types/records'

type Props = { student: Student; onClose: () => void; onSaved: () => void }

function DeleteStudentModal({ student, onClose, onSaved }: Props) {
  const [error, setError] = useState('')
  const [deleting, setDeleting] = useState(false)

  async function onDelete() {
    setDeleting(true)
    try {
      await deleteStudent(student.id)
      toast(`${student.first_name} ${student.last_name} deleted`)
      onSaved()
    } catch (err) {
      setError(apiError(err)) // 409 when the student already has attendance or representatives
      toast(`Couldn't delete the student: ${apiError(err)}`, 'error')
      setDeleting(false)
    }
  }

  return (
    <Modal title="Delete student" onClose={onClose}>
      <div className="flex flex-col gap-4">
        <Text>
          Delete{' '}
          <strong>
            {student.first_name} {student.last_name}
          </strong>{' '}
          (DNI {student.dni})? This can't be undone.
        </Text>
        {error && (
          <p role="alert" className="text-sm text-absent">
            {error}
          </p>
        )}
        <div className="flex justify-end gap-3">
          <Button variant="ghost" onClick={onClose}>
            Cancel
          </Button>
          <Button className="bg-absent! hover:opacity-90" disabled={deleting} onClick={onDelete}>
            {deleting ? 'Deleting…' : 'Delete'}
          </Button>
        </div>
      </div>
    </Modal>
  )
}

export default DeleteStudentModal
