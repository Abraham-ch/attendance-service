import { RollPanel, Section } from '@components/common'
import { ArrowLink, Heading, Label, Text } from '@components/ui'

import { roll } from '@/data/sample'

function LiveRoll() {
  return (
    <Section id="live" index="02" kicker="Live" title="One scan, every screen" aside="WebSocket">
      <div className="grid items-center gap-12 lg:grid-cols-[1.2fr_1fr]">
        <RollPanel title="Period 2026-I · sample roll" rows={roll} live />
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
