import Heading from '../atoms/Heading'
import Label from '../atoms/Label'

type Props = { index: string; kicker: string; title: string; aside?: string }

function SectionHeader({ index, kicker, title, aside }: Props) {
  return (
    <header className="flex flex-wrap items-baseline gap-x-10 gap-y-3 border-t border-line pt-6">
      <Label>
        {index} — {kicker}
      </Label>
      <Heading size="title">{title}</Heading>
      {aside && <Label className="ml-auto">{aside}</Label>}
    </header>
  )
}

export default SectionHeader
