import { Field, Modal, SelectField } from '@components/common'
import { Button } from '@components/ui'
import { zodResolver } from '@hookform/resolvers/zod'
import { apiError, createStudent } from '@lib/api'
import { toast } from '@lib/toast'
import { useState } from 'react'
import { useForm } from 'react-hook-form'

import { StudentSchema } from '@/schemas/input'

type Props = { onClose: () => void; onSaved: () => void }

function CreateStudentModal({ onClose, onSaved }: Props) {
  const [error, setError] = useState('')
  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting }
  } = useForm({ resolver: zodResolver(StudentSchema), defaultValues: { gender: 'Male' } })

  const onSubmit = handleSubmit(async (student) => {
    try {
      await createStudent(student)
      toast(
        student.email
          ? `${student.first_name} ${student.last_name} added, invite sent to ${student.email}`
          : `${student.first_name} ${student.last_name} added`
      )
      onSaved()
    } catch (err) {
      setError(apiError(err))
      toast(`Couldn't create the student: ${apiError(err)}`, 'error')
    }
  })

  return (
    <Modal title="New student" onClose={onClose}>
      <form className="grid gap-4 sm:grid-cols-2" onSubmit={onSubmit} noValidate>
        <Field
          compact
          label="First name"
          autoComplete="off"
          error={errors.first_name?.message}
          {...register('first_name')}
        />
        <Field
          compact
          label="Last name"
          autoComplete="off"
          error={errors.last_name?.message}
          {...register('last_name')}
        />
        <Field
          compact
          label="DNI"
          inputMode="numeric"
          error={errors.dni?.message}
          {...register('dni')}
        />
        <SelectField compact label="Gender" {...register('gender')}>
          <option value="Male">Male</option>
          <option value="Female">Female</option>
        </SelectField>
        <Field
          compact
          label="Email (optional, emails an invite)"
          type="email"
          className="sm:col-span-2"
          error={errors.email?.message}
          {...register('email')}
        />
        <Field
          compact
          label="Phone (optional)"
          type="tel"
          error={errors.phone?.message}
          {...register('phone')}
        />
        <Field
          compact
          label="Address (optional)"
          error={errors.address?.message}
          {...register('address')}
        />
        {error && (
          <p role="alert" className="text-sm text-absent sm:col-span-2">
            {error}
          </p>
        )}
        <div className="flex justify-end gap-3 sm:col-span-2">
          <Button variant="ghost" onClick={onClose}>
            Cancel
          </Button>
          <Button type="submit" disabled={isSubmitting}>
            {isSubmitting ? 'Saving…' : 'Create student'}
          </Button>
        </div>
      </form>
    </Modal>
  )
}

export default CreateStudentModal
