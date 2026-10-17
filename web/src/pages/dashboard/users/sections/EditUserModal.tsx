import { AvatarPicker, Field, Modal, SelectField } from '@components/common'
import { Button } from '@components/ui'
import { zodResolver } from '@hookform/resolvers/zod'
import { apiError, updateUser, uploadAvatar } from '@lib/api'
import { useAuth } from '@lib/context/AuthContext'
import { assignableRoles } from '@lib/roles'
import { toast } from '@lib/toast'
import { useState } from 'react'
import { useForm } from 'react-hook-form'

import { UserUpdateSchema } from '@/schemas/input'
import type { User } from '@/types/user'

type Props = { user: User; onClose: () => void; onSaved: () => void }

// PATCH /user/{id}. Only opened for users the caller may manage (canManage), the API
// checks the same rule again
function EditUserModal({ user, onClose, onSaved }: Props) {
  const { user: me } = useAuth()
  const roles = assignableRoles(me)
  const name = `${user.first_name} ${user.last_name}`
  const [error, setError] = useState('')
  const [avatar, setAvatar] = useState<Blob | null>(null)
  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting }
  } = useForm({
    resolver: zodResolver(UserUpdateSchema),
    defaultValues: {
      first_name: user.first_name,
      last_name: user.last_name,
      email: user.email,
      role: user.role as 'Admin' | 'User', // canManage never opens this for the SuperAdmin
      password: ''
    }
  })

  const onSubmit = handleSubmit(async (changes) => {
    try {
      const url = avatar ? await uploadAvatar(avatar) : undefined
      await updateUser(user.id, { ...changes, avatar: url })
      toast(`${name} updated`)
      onSaved()
    } catch (err) {
      setError(apiError(err))
      toast(`Couldn't update the user: ${apiError(err)}`, 'error')
    }
  })

  return (
    <Modal title={`Edit ${name}`} onClose={onClose}>
      <form className="grid gap-4 sm:grid-cols-2" onSubmit={onSubmit} noValidate>
        <AvatarPicker
          name={name}
          current={user.avatar}
          label="Avatar"
          onChange={setAvatar}
          onError={setError}
          className="sm:col-span-2"
        />
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
          label="New password (optional)"
          type="password"
          autoComplete="new-password"
          placeholder="Leave empty to keep it"
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
            {isSubmitting ? 'Saving…' : 'Save changes'}
          </Button>
        </div>
      </form>
    </Modal>
  )
}

export default EditUserModal
