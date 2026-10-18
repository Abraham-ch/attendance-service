import { Brand } from '@components/common'
import { ArrowLink, Container, Label } from '@components/ui'

import { paths } from '@/paths'

function SiteHeader() {
  return (
    <header className="absolute inset-x-0 top-0 z-10 py-10">
      <Container className="flex items-center justify-between gap-6">
        <Brand />
        <Label className="hidden lg:inline">Attendance · Schools &amp; academies</Label>
        <nav className="flex items-center gap-6">
          <ArrowLink href={paths.demo}>Try the demo</ArrowLink>
          <ArrowLink href={paths.dashboard}>Dashboard</ArrowLink>
          <ArrowLink href={paths.login}>Sign in</ArrowLink>
        </nav>
      </Container>
    </header>
  )
}

export default SiteHeader
