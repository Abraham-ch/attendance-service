import { Field } from '@components/common'
import { ArrowLink, Button, Heading, Label } from '@components/ui'
import { zodResolver } from '@hookform/resolvers/zod'
import { apiError } from '@lib/api'
import { login } from '@lib/auth'
import { useAuth } from '@lib/context/AuthContext'
import { isAxiosError } from 'axios'
import { useState } from 'react'
import { useForm, type SubmitHandler } from 'react-hook-form'
import { useNavigate } from 'react-router'

import { paths } from '@/paths'
import { LoginSchema } from '@/schemas/input'
import type { LoginData } from '@/types/auth'

function LoginForm() {
  const navigate = useNavigate()
  const { setUser } = useAuth()
  const [error, setError] = useState('')
  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting }
  } = useForm<LoginData>({ resolver: zodResolver(LoginSchema) })

  const onSubmit: SubmitHandler<LoginData> = async (data) => {
    setError('')
    try {
      const { user } = await login(data)
      setUser(user)
      navigate(paths.dashboard)
    } catch (err) {
      // The API answers 401 for both an unknown email and a wrong password, on purpose:
      // telling them apart would let anyone check which emails have an account
      setError(
        isAxiosError(err) && err.response?.status === 401
          ? 'Invalid email or password.'
          : apiError(err)
      )
    }
  }
  return (
    <form
      className="flex w-full max-w-sm flex-col gap-6"
      onSubmit={handleSubmit(onSubmit)}
      noValidate
    >
      <div className="flex flex-col gap-2">
        <Label tone="brass">Log in</Label>
        <Heading as="h1" size="title">
          Welcome back
        </Heading>
      </div>
      <Field
        label="Email"
        type="email"
        autoComplete="email"
        placeholder="you@school.edu"
        error={errors.email?.message}
        {...register('email')}
      />
      <Field
        label="Password"
        type="password"
        placeholder="********"
        autoComplete="current-password"
        error={errors.password?.message}
        {...register('password')}
      />
      {error && (
        <p role="alert" className="text-sm text-absent">
          {error}
        </p>
      )}
      <Button type="submit" disabled={isSubmitting}>
        {isSubmitting ? 'Logging in…' : 'Log in'}
      </Button>
      <ArrowLink href={paths.home}>Back to home</ArrowLink>
    </form>
  )
}

export default LoginForm
