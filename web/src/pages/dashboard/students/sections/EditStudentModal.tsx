import { Field, Modal } from '@components/common'
import { Button, Text } from '@components/ui'
import { zodResolver } from '@hookform/resolvers/zod'
import { apiError, updateStudent } from '@lib/api'
import { toast } from '@lib/toast'
import { useState } from 'react'
import { useForm } from 'react-hook-form'

import { StudentUpdateSchema } from '@/schemas/input'
import type { Student } from '@/types/records'

type Props = { student: Student; onClose: () => void; onSaved: () => void }

// PATCH /student/{id} only changes phone and address (UpdateStudent in the API)
function EditStudentModal({ student, onClose, onSaved }: Props) {
  const [error, setError] = useState('')
  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting }
  } = useForm({
    resolver: zodResolver(StudentUpdateSchema),
    defaultValues: { phone: student.phone ?? '', address: student.address ?? '' }
  })

  const onSubmit = handleSubmit(async (changes) => {
    try {
      await updateStudent(student.id, changes)
      toast(`${student.first_name} ${student.last_name} updated`)
      onSaved()
    } catch (err) {
      setError(apiError(err))
      toast(`Couldn't update the student: ${apiError(err)}`, 'error')
    }
  })

  return (
    <Modal title={`Edit ${student.first_name} ${student.last_name}`} onClose={onClose}>
      <form className="flex flex-col gap-4" onSubmit={onSubmit} noValidate>
        <Text className="text-sm">
          DNI <strong>{student.dni}</strong>. Name, DNI and email can't be changed here.
        </Text>
        <Field
          compact
          label="Phone"
          type="tel"
          error={errors.phone?.message}
          {...register('phone')}
        />
        <Field compact label="Address" error={errors.address?.message} {...register('address')} />
        {error && (
          <p role="alert" className="text-sm text-absent">
            {error}
          </p>
        )}
        <div className="flex justify-end gap-3">
          <Button variant="ghost" onClick={onClose}>
            Cancel
          </Button>
          <Button type="submit" disabled={isSubmitting}>
            {isSubmitting ? 'Saving…' : 'Save changes'}
          </Button>
        </div>
      </form>
    </Modal>
  )
}

export default EditStudentModal
