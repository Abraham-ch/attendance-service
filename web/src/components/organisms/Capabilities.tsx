import FeatureRow from '../molecules/FeatureRow'
import Section from '../molecules/Section'

function Capabilities() {
  return (
    <Section id="capability" index="01" kicker="Capability" title="What it takes care of">
      <ul className="border-t border-line">
        <FeatureRow index="01" title="Live roll call">
          A scan sends the student and the period over a WebSocket. The record is saved and{' '}
          <strong>pushed to every connected screen</strong>, so the front desk and the classroom
          read the same list.
        </FeatureRow>
        <FeatureRow index="02" title="Students & representatives">
          Each student carries the people responsible for them: mother, father, legal guardian and
          more, with <strong>one marked as primary</strong> and as many phone numbers as they need.
        </FeatureRow>
        <FeatureRow index="03" title="Periods">
          A period is a name and two dates. <strong>Every attendance record belongs to one</strong>,
          so a term can be read back on its own.
        </FeatureRow>
        <FeatureRow index="04" title="Protected and documented">
          Routes sit behind a <strong>bearer token</strong>, passwords are stored hashed, and the
          OpenAPI reference is served by the API itself.
        </FeatureRow>
      </ul>
    </Section>
  )
}

export default Capabilities
