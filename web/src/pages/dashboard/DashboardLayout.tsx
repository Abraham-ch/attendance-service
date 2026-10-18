import { Toaster } from '@components/common'
import { logout } from '@lib/auth'
import { useAuth } from '@lib/context/AuthContext'
import { isDemo } from '@lib/demo'
import { Link, Outlet } from 'react-router'

import { paths } from '@/paths'

import { Sidebar } from './sections'

function DashboardLayout() {
  const { user, setUser } = useAuth()
  if (!user) return null

  return (
    <div className="dashboard min-h-svh">
      {isDemo() && (
        <div className="flex flex-wrap items-center justify-center gap-x-4 gap-y-1 bg-ink px-edge py-2 text-center text-sm text-paper">
          <span>Demo mode: made-up school, changes reset when you reload.</span>
          <Link
            to={paths.home}
            onClick={() => {
              logout()
              setUser(null)
            }}
            className="font-ui text-label text-paper uppercase underline underline-offset-4 hover:text-brass focus-visible:outline-2 focus-visible:outline-brass"
          >
            Exit demo
          </Link>
        </div>
      )}
      <div className="md:flex">
        <Sidebar {...user} />
        <main className="flex min-w-0 flex-1 flex-col gap-8 px-edge py-10">
          <Outlet />
        </main>
      </div>
      <Toaster />
    </div>
  )
}

export default DashboardLayout
