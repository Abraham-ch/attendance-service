import ArrowLink from '../atoms/ArrowLink'
import Heading from '../atoms/Heading'
import Label from '../atoms/Label'
import type { Status } from '../atoms/StatusDot'
import Text from '../atoms/Text'
import RollRow from '../molecules/RollRow'
import Section from '../molecules/Section'

const roll: { name: string; time: string; status: Status }[] = [
  { name: 'Carlos Ramírez', time: '07:52', status: 'Present' },
  { name: 'María Gonzales', time: '07:58', status: 'Present' },
  { name: 'Diego Torres', time: '08:14', status: 'Late' },
  { name: 'Lucía Vargas', time: '—', status: 'Excused' },
  { name: 'Andrés Castillo', time: '—', status: 'Absent' }
]

function LiveRoll() {
  return (
    <Section id="live" index="02" kicker="Live" title="One scan, every screen" aside="WebSocket">
      <div className="grid items-center gap-12 lg:grid-cols-[1.2fr_1fr]">
        <div className="border border-line bg-surface">
          <div className="flex items-center justify-between gap-4 border-b border-line px-5 py-3">
            <Label>Period 2026-I · sample roll</Label>
            <span className="inline-flex items-center gap-2">
              <span
                aria-hidden
                className="size-1.5 rounded-full bg-present motion-safe:animate-pulse"
              />
              <Label tone="ink">Live</Label>
            </span>
          </div>
          <ul>
            {roll.map((row) => (
              <RollRow key={row.name} {...row} />
            ))}
          </ul>
        </div>
        <div className="flex flex-col items-start gap-4">
          <Label tone="brass">Attendance feed</Label>
          <Heading as="h3" size="item">
            Nobody presses refresh
          </Heading>
          <Text>
            The client keeps one socket open on <strong>/assistance</strong>. It sends a student id
            and a period id, and receives every new record as JSON the moment it is stored.
          </Text>
          <Label className="border-t border-line pt-4">JSON in · JSON out · broadcast</Label>
          <ArrowLink href="#api">See the endpoints</ArrowLink>
        </div>
      </div>
    </Section>
  )
}

export default LiveRoll
