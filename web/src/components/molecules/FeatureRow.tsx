import type { ReactNode } from 'react'

import Heading from '../atoms/Heading'
import Label from '../atoms/Label'
import Text from '../atoms/Text'

type Props = { index: string; title: string; children: ReactNode; tag?: string }

function FeatureRow({ index, title, children, tag }: Props) {
  return (
    <li
      className={`grid gap-x-10 gap-y-2 border-b border-line py-7 ${tag ? 'md:grid-cols-[3rem_13rem_1fr_16rem]' : 'md:grid-cols-[3rem_18rem_1fr]'}`}
    >
      <Label>{index}</Label>
      <Heading as="h3" size="item">
        {title}
      </Heading>
      <Text>{children}</Text>
      {tag && <Label>{tag}</Label>}
    </li>
  )
}

export default FeatureRow
