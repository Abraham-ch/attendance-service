import { Brand, NavItem } from '@components/common'
import { Avatar, Button, Label } from '@components/ui'
import { apiError, uploadMyAvatar } from '@lib/api'
import { logout } from '@lib/auth'
import { useAuth } from '@lib/context/AuthContext'
import { avatarBlob } from '@lib/image'
import { isAdmin } from '@lib/roles'
import { toast } from '@lib/toast'
import { useNavigate } from 'react-router'

import { paths } from '@/paths'
import type { User } from '@/types/user'

type Group = {
  title: string
  adminOnly?: boolean
  items: { to: string; label: string; end?: boolean }[]
}

const groups: Group[] = [
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
    adminOnly: true,
    items: [
      { to: paths.students, label: 'Students' },
      { to: paths.representatives, label: 'Representatives' },
      { to: paths.periods, label: 'Periods' }
    ]
  },
  { title: 'Admin', adminOnly: true, items: [{ to: paths.users, label: 'Users' }] }
]

function Sidebar(user: User) {
  const navigate = useNavigate()
  const { setUser } = useAuth()

  // anyone can change their own picture here, not only at register or through an admin
  async function changeAvatar(file: File | undefined) {
    if (!file) return
    try {
      const avatar = await uploadMyAvatar(await avatarBlob(file))
      setUser({ ...user, avatar })
      toast('Picture updated')
    } catch (err) {
      toast(`Couldn't change your picture: ${apiError(err)}`, 'error')
    }
  }

  const handleLogout = () => {
    logout()
    navigate(paths.login, { replace: true })
  }

  return (
    <aside className="flex shrink-0 flex-col gap-6 border-b border-line p-6 md:sticky md:top-0 md:h-svh md:w-64 md:gap-10 md:border-r md:border-b-0">
      <Brand />
      <nav className="flex gap-6 overflow-x-auto md:flex-col md:gap-8 md:overflow-visible">
        {groups
          .filter((group) => !group.adminOnly || isAdmin(user))
          .map((group) => (
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
        <div className="flex items-center gap-3">
          <label
            title="Change your picture"
            className="cursor-pointer rounded-full focus-within:outline-2 focus-within:outline-offset-2 focus-within:outline-brass"
          >
            <Avatar
              key={user.avatar}
              src={user.avatar}
              name={`${user.first_name} ${user.last_name}`}
            />
            <input
              type="file"
              accept="image/*"
              aria-label="Change your picture"
              className="sr-only"
              onChange={(event) => changeAvatar(event.target.files?.[0])}
            />
          </label>
          <div className="flex min-w-0 flex-col gap-1">
            <span className="truncate font-medium">
              {user.first_name} {user.last_name}
            </span>
            <Label>{user.role}</Label>
          </div>
        </div>
        <Button
          variant="ghost"
          onClick={handleLogout}
          className="mt-3 font-ui text-label text-brass-lo uppercase transition-colors hover:text-ink"
        >
          Sign out
        </Button>
      </div>
    </aside>
  )
}

export default Sidebar
