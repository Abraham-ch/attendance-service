import { Link } from 'react-router'

import { currentUser } from '../../data/sample'
import { paths } from '../../paths'
import Label from '../atoms/Label'
import Brand from '../molecules/Brand'
import NavItem from '../molecules/NavItem'

const groups: { title: string; items: { to: string; label: string; end?: boolean }[] }[] = [
  {
    title: 'Today',
    items: [
      { to: paths.dashboard, label: 'Overview', end: true },
      { to: paths.qrScan, label: 'QR scan' },
      { to: paths.assistance, label: 'Assistance' }
    ]
  },
  {
    title: 'Records',
    items: [
      { to: paths.students, label: 'Students' },
      { to: paths.representatives, label: 'Representatives' },
      { to: paths.periods, label: 'Periods' }
    ]
  },
  { title: 'Admin', items: [{ to: paths.users, label: 'Users' }] }
]

function Sidebar() {
  return (
    <aside className="flex shrink-0 flex-col gap-6 border-b border-line p-6 md:sticky md:top-0 md:h-svh md:w-64 md:gap-10 md:border-r md:border-b-0">
      <Brand />
      <nav className="flex gap-6 overflow-x-auto md:flex-col md:gap-8 md:overflow-visible">
        {groups.map((group) => (
          <div key={group.title} className="flex gap-1 md:flex-col">
            <Label className="hidden px-3 pb-2 md:block">{group.title}</Label>
            {group.items.map((item) => (
              <NavItem key={item.to} to={item.to} end={item.end}>
                {item.label}
              </NavItem>
            ))}
          </div>
        ))}
      </nav>
      <div className="mt-auto hidden flex-col gap-1 border-t border-line pt-5 md:flex">
        <span className="font-medium">{currentUser.name}</span>
        <Label>{currentUser.role}</Label>
        <Link
          to={paths.login}
          className="mt-3 font-monospace text-label text-brass-lo uppercase transition-colors hover:text-ink"
        >
          Sign out
        </Link>
      </div>
    </aside>
  )
}

export default Sidebar
