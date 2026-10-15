import { Mark } from '@components/ui'
import { Link } from 'react-router'

import { paths } from '@/paths'

function Brand() {
  return (
    <Link to={paths.home} className="inline-flex items-center gap-2.5 font-semibold tracking-tight">
      <Mark />
      Attendance Service
    </Link>
  )
}

export default Brand
