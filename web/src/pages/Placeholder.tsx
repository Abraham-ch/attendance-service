import { useLocation } from 'react-router'

import ArrowLink from '../components/atoms/ArrowLink'
import Container from '../components/atoms/Container'
import Heading from '../components/atoms/Heading'
import Label from '../components/atoms/Label'

// ponytail: stands in for every page that is not built yet, replace per route in route.ts
function Placeholder() {
  const { pathname } = useLocation()

  return (
    <main className="flex min-h-svh items-center">
      <Container className="flex flex-col items-start gap-4">
        <Label>Not built yet</Label>
        <Heading as="h1" size="title">
          {pathname}
        </Heading>
        <ArrowLink href="/">Back to home</ArrowLink>
      </Container>
    </main>
  )
}

export default Placeholder
