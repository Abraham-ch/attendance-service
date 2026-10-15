import { Section, StatusCard } from '@components/common'

function Statuses() {
  return (
    <Section id="statuses" index="03" kicker="Records" title="Four ways a day can go">
      <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
        <StatusCard status="Present">The student was scanned in for the day.</StatusCard>
        <StatusCard status="Late">They arrived, after the roll had already started.</StatusCard>
        <StatusCard status="Absent">No scan for that date in the period.</StatusCard>
        <StatusCard status="Excused">
          Justified with a note. A record can be updated later, on a different day than the one it
          belongs to.
        </StatusCard>
      </div>
    </Section>
  )
}

export default Statuses
