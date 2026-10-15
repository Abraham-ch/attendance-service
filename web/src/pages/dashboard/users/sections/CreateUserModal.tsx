import { Field, Modal, SelectField } from '@components/common'
import { Button } from '@components/ui'
import { zodResolver } from '@hookform/resolvers/zod'
import { apiError, createUser } from '@lib/api'
import { useAuth } from '@lib/context/AuthContext'
import { assignableRoles } from '@lib/roles'
import { toast } from '@lib/toast'
import { useState } from 'react'
import { useForm } from 'react-hook-form'

import { UserSchema } from '@/schemas/input'

type Props = { onClose: () => void; onSaved: () => void }

function CreateUserModal({ onClose, onSaved }: Props) {
  const { user } = useAuth()
  const roles = assignableRoles(user)
  const [error, setError] = useState('')
  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting }
  } = useForm({ resolver: zodResolver(UserSchema), defaultValues: { role: 'User' } })

  const onSubmit = handleSubmit(async (newUser) => {
    try {
      await createUser(newUser)
      toast(`${newUser.first_name} ${newUser.last_name} added as ${newUser.role}`)
      onSaved()
    } catch (err) {
      setError(apiError(err))
      toast(`Couldn't create the user: ${apiError(err)}`, 'error')
    }
  })

  return (
    <Modal title="New user" onClose={onClose}>
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
          label="Email"
          type="email"
          autoComplete="off"
          className="sm:col-span-2"
          error={errors.email?.message}
          {...register('email')}
        />
        <Field
          compact
          label="Password"
          type="password"
          autoComplete="new-password"
          error={errors.password?.message}
          {...register('password')}
        />
        <SelectField compact label="Role" error={errors.role?.message} {...register('role')}>
          {roles.map((role) => (
            <option key={role} value={role}>
              {role}
            </option>
          ))}
        </SelectField>
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
            {isSubmitting ? 'Saving…' : 'Create user'}
          </Button>
        </div>
      </form>
    </Modal>
  )
}

export default CreateUserModal
