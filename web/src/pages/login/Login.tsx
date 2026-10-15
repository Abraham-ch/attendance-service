import { Brand } from '@components/common'
import { Backdrop, Heading, Label } from '@components/ui'

import { LoginForm } from './sections'

function Login() {
  return (
    <main className="grid min-h-svh lg:grid-cols-2">
      <section className="relative isolate hidden flex-col justify-between border-r border-line p-edge lg:flex">
        <Backdrop />
        <Brand />
        <Heading size="title">
          Back to <span className="block text-brass">the roll</span>
        </Heading>
        <Label>Roll call in real time</Label>
      </section>
      <section className="flex items-center justify-center p-edge">
        <LoginForm />
      </section>
    </main>
  )
}

export default Login
