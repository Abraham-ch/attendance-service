import Capabilities from '../components/organisms/Capabilities'
import Contact from '../components/organisms/Contact'
import Endpoints from '../components/organisms/Endpoints'
import Hero from '../components/organisms/Hero'
import LiveRoll from '../components/organisms/LiveRoll'
import SiteFooter from '../components/organisms/SiteFooter'
import SiteHeader from '../components/organisms/SiteHeader'
import Statuses from '../components/organisms/Statuses'

function Landing() {
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

export default Landing
