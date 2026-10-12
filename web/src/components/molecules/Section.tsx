import type { ComponentProps, ReactNode } from 'react'

import Container from '../atoms/Container'
import SectionHeader from './SectionHeader'

type Props = ComponentProps<typeof SectionHeader> & { id: string; children: ReactNode }

function Section({ id, children, ...header }: Props) {
  return (
    <section id={id} className="py-20 md:py-28">
      <Container>
        <SectionHeader {...header} />
        <div className="mt-12">{children}</div>
      </Container>
    </section>
  )
}

export default Section
