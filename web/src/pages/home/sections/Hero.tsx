import { ArrowLink, Backdrop, Container, Heading, Label, Mark, Text } from '@components/ui'

import { paths } from '@/paths'

function Hero() {
  return (
    <section className="relative isolate flex min-h-svh flex-col justify-end pt-40">
      <Backdrop />
      <Container>
        <div className="grid items-end gap-10 pb-10 lg:grid-cols-[1.5fr_1fr]">
          <Heading as="h1" size="display">
            Roll call <span className="block text-brass">in real time</span>
          </Heading>
          <div className="flex max-w-md flex-col items-start gap-6">
            <Text className="text-lg">
              Every scan becomes an <strong>attendance record</strong> the moment it happens.
              Students, representatives and periods live in <strong>one API</strong>, and every open
              screen sees the roll update without a refresh.
            </Text>
            <ArrowLink href={paths.demo} size="lead">
              Try the live demo
            </ArrowLink>
          </div>
        </div>
        <div className="flex flex-wrap items-center justify-between gap-x-10 gap-y-2 border-t border-line py-5">
          <Label>Rust · Axum · PostgreSQL</Label>
          <Label>WebSocket feed, no polling</Label>
          <span className="inline-flex items-center gap-2">
            <Mark />
            <Label tone="ink">MIT licensed</Label>
          </span>
        </div>
      </Container>
    </section>
  )
}

export default Hero
