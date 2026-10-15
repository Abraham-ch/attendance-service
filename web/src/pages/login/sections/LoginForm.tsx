import { Field } from '@components/common'
import { ArrowLink, Button, Heading, Label } from '@components/ui'
import { zodResolver } from '@hookform/resolvers/zod'
import { login } from '@lib/auth'
import { useAuth } from '@lib/context/AuthContext'
import { useForm, type SubmitHandler } from 'react-hook-form'
import { useNavigate } from 'react-router'

import { paths } from '@/paths'
import { LoginSchema } from '@/schemas/input'
import type { LoginData } from '@/types/auth'

function LoginForm() {
  const navigate = useNavigate()
  const { setUser } = useAuth()
  const { register, handleSubmit } = useForm<LoginData>({ resolver: zodResolver(LoginSchema) })

  const onSubmit: SubmitHandler<LoginData> = async (data) => {
    try {
      const { user } = await login(data)
      setUser(user)
      navigate(paths.dashboard)
    } catch (err) {
      console.log('Login failed', err)
    }
  }
  return (
    <form className="flex w-full max-w-sm flex-col gap-6" onSubmit={handleSubmit(onSubmit)}>
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
        required
        {...register('email')}
      />
      <Field
        label="Password"
        type="password"
        placeholder="********"
        autoComplete="current-password"
        minLength={10}
        maxLength={20}
        required
        {...register('password')}
      />
      <Button type="submit">Log in</Button>
      <ArrowLink href={paths.home}>Back to home</ArrowLink>
    </form>
  )
}

export default LoginForm
