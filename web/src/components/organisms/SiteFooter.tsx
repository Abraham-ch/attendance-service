import Container from '../atoms/Container'
import Label from '../atoms/Label'

function SiteFooter() {
  return (
    <footer className="border-t border-line py-6">
      <Container className="flex flex-wrap justify-between gap-x-10 gap-y-2">
        <Label>Attendance Service — roll call in real time</Label>
        <Label>React · Tailwind CSS v4</Label>
      </Container>
    </footer>
  )
}

export default SiteFooter
