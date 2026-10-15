import type { ReactNode } from 'react'
import { NavLink } from 'react-router'

type Props = { to: string; children: ReactNode; end?: boolean }

function NavItem({ to, children, end }: Props) {
  return (
    <NavLink
      to={to}
      end={end}
      className={({ isActive }) =>
        `border-l-2 px-3 py-2 text-sm whitespace-nowrap transition-colors focus-visible:outline-2 focus-visible:outline-brass ${isActive ? 'border-brass bg-surface font-medium text-ink' : 'border-transparent text-mute hover:text-ink'}`
      }
    >
      {children}
    </NavLink>
  )
}

export default NavItem
