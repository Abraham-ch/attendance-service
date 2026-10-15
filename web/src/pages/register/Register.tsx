import { Brand } from '@components/common'
import { ArrowLink, Backdrop, Heading, Label, Text } from '@components/ui'
import type { ReactNode } from 'react'

import { paths } from '@/paths'

import { RegisterForm } from './sections'

// Same split layout as Login, the student lands here from the invite email
function Layout({ children }: { children: ReactNode }) {
  return (
    <main className="grid min-h-svh lg:grid-cols-2">
      <section className="relative isolate hidden flex-col justify-between border-r border-line p-edge lg:flex">
        <Backdrop />
        <Brand />
        <Heading size="title">
          Welcome to <span className="block text-brass">the roll</span>
        </Heading>
        <Label>Roll call in real time</Label>
      </section>
      <section className="flex items-center justify-center p-edge">{children}</section>
    </main>
  )
}

function Register() {
  return (
    <Layout>
      <RegisterForm />
    </Layout>
  )
}

// The loader answers 404 for unknown, used or expired tokens
export function InvalidInvite() {
  return (
    <Layout>
      <div className="flex w-full max-w-sm flex-col gap-4">
        <Label tone="brass">Activate account</Label>
        <Heading as="h1" size="title">
          This link doesn't work
        </Heading>
        <Text>
          It was already used or it expired (links last 3 days). If you already activated your
          account, log in. If not, ask your school to send you a new invitation.
        </Text>
        <ArrowLink href={paths.login}>Go to log in</ArrowLink>
      </div>
    </Layout>
  )
}

export default Register
