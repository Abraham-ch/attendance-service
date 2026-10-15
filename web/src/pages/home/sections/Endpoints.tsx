import { FeatureRow, Section } from '@components/common'

function Endpoints() {
  return (
    <Section id="api" index="04" kicker="API" title="The whole surface" aside="REST + WebSocket">
      <ul className="border-t border-line">
        <FeatureRow index="01" title="/student" tag="GET · POST · PATCH · DELETE">
          Create, list, read, update and remove students, identified by their DNI.
        </FeatureRow>
        <FeatureRow index="02" title="/representative" tag="GET · POST · PATCH · DELETE">
          A representative is created together with their relationship to a student.
        </FeatureRow>
        <FeatureRow index="03" title="/period" tag="GET · POST · DELETE">
          Named date ranges that attendance records are grouped under.
        </FeatureRow>
        <FeatureRow index="04" title="/assistance" tag="WebSocket">
          The live channel: send a scan, receive every record as it is saved.
        </FeatureRow>
        <FeatureRow index="05" title="/auth · /user" tag="POST · GET · PATCH · DELETE">
          Sign in for a token, and manage the accounts that use the service.
        </FeatureRow>
      </ul>
    </Section>
  )
}

export default Endpoints
