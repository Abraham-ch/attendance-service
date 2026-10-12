import Container from '../atoms/Container'
import Label from '../atoms/Label'
import Brand from '../molecules/Brand'

function SiteHeader() {
  return (
    <header className="absolute inset-x-0 top-0 z-10 py-10">
      <Container className="flex items-center justify-between gap-6">
        <Brand />
        <Label className="hidden sm:inline">Attendance · Schools &amp; academies</Label>
      </Container>
    </header>
  )
}

export default SiteHeader
