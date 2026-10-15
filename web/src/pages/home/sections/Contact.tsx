import { ArrowLink, Container, Heading, Label } from '@components/ui'

import { paths } from '@/paths'

const API_URL = import.meta.env.VITE_API_URL ?? 'http://localhost:3000'

function Contact() {
  return (
    <section id="contact" className="pb-20 md:pb-28">
      <Container>
        <div className="grid items-end gap-12 border-t border-line pt-16 lg:grid-cols-[1.4fr_1fr]">
          <div className="flex flex-col gap-8">
            <Heading size="title" className="max-w-xl">
              Still taking roll on paper?
            </Heading>
            <div className="flex flex-wrap gap-x-8 gap-y-3">
              <ArrowLink href={paths.login} size="lead">
                Sign in
              </ArrowLink>
              <ArrowLink href={`${API_URL}/docs`} size="lead">
                Read the API docs
              </ArrowLink>
              <ArrowLink href="https://github.com/Abraham-ch/attendance-service" size="lead">
                View the source
              </ArrowLink>
            </div>
          </div>
          <div className="flex flex-col gap-2">
            <Label>Open source, MIT licensed</Label>
            <Label>OpenAPI reference served at /docs</Label>
            <Label>Runs on Rust, Axum and PostgreSQL</Label>
          </div>
        </div>
      </Container>
    </section>
  )
}

export default Contact
