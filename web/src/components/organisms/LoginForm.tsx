import { useNavigate } from 'react-router'

import { paths } from '../../paths'
import ArrowLink from '../atoms/ArrowLink'
import Button from '../atoms/Button'
import Heading from '../atoms/Heading'
import Label from '../atoms/Label'
import Field from '../molecules/Field'

function LoginForm() {
  const navigate = useNavigate()

  return (
    <form
      className="flex w-full max-w-sm flex-col gap-6"
      // ponytail: design only, goes straight to the dashboard. Call POST /auth and keep the token here.
      onSubmit={(event) => {
        event.preventDefault()
        navigate(paths.dashboard)
      }}
    >
      <div className="flex flex-col gap-2">
        <Label tone="brass">Sign in</Label>
        <Heading as="h1" size="title">
          Welcome back
        </Heading>
      </div>
      <Field
        label="Email"
        type="email"
        name="email"
        autoComplete="email"
        placeholder="you@school.edu"
        required
      />
      <Field
        label="Password"
        type="password"
        name="password"
        autoComplete="current-password"
        minLength={10}
        maxLength={20}
        required
      />
      <Button type="submit">Sign in</Button>
      <ArrowLink href={paths.home}>Back to home</ArrowLink>
    </form>
  )
}

export default LoginForm
