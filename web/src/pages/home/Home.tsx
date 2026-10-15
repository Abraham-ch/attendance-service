import {
  Capabilities,
  Contact,
  Endpoints,
  Hero,
  LiveRoll,
  SiteFooter,
  SiteHeader,
  Statuses
} from './sections'

function Home() {
  return (
    <>
      <SiteHeader />
      <main>
        <Hero />
        <Capabilities />
        <LiveRoll />
        <Statuses />
        <Endpoints />
        <Contact />
      </main>
      <SiteFooter />
    </>
  )
}

export default Home
