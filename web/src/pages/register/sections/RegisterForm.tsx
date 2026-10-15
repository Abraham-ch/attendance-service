import { Field } from '@components/common'
import { Button, Heading, Label, Text } from '@components/ui'
import { zodResolver } from '@hookform/resolvers/zod'
import { apiError } from '@lib/api'
import { register as registerUser } from '@lib/auth'
import { useAuth } from '@lib/context/AuthContext'
import { useState } from 'react'
import { useForm } from 'react-hook-form'
import { useLoaderData, useNavigate, useSearchParams } from 'react-router'

import { paths } from '@/paths'
import { RegisterSchema } from '@/schemas/input'
import type { InviteInfo } from '@/types/records'

function RegisterForm() {
  const invite = useLoaderData<InviteInfo>()
  const [params] = useSearchParams()
  const navigate = useNavigate()
  const { setUser } = useAuth()
  const [error, setError] = useState('')
  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting }
  } = useForm({ resolver: zodResolver(RegisterSchema) })

  const onSubmit = handleSubmit(async ({ password }) => {
    try {
      const { user } = await registerUser(params.get('token') ?? '', password)
      setUser(user)
      navigate(paths.dashboard, { replace: true })
    } catch (err) {
      setError(apiError(err))
    }
  })

  return (
    <form className="flex w-full max-w-sm flex-col gap-6" onSubmit={onSubmit} noValidate>
      <div className="flex flex-col gap-2">
        <Label tone="brass">Activate account</Label>
        <Heading as="h1" size="title">
          Hi {invite.first_name}
        </Heading>
        <Text>
          Choose a password for <strong>{invite.email}</strong>. You'll use it with this email to
          log in.
        </Text>
      </div>
      <Field
        label="Password"
        type="password"
        autoComplete="new-password"
        error={errors.password?.message}
        {...register('password')}
      />
      <Field
        label="Repeat password"
        type="password"
        autoComplete="new-password"
        error={errors.confirm?.message}
        {...register('confirm')}
      />
      {error && (
        <p role="alert" className="text-sm text-absent">
          {error}
        </p>
      )}
      <Button type="submit" disabled={isSubmitting}>
        {isSubmitting ? 'Activating…' : 'Activate account'}
      </Button>
    </form>
  )
}

export default RegisterForm
