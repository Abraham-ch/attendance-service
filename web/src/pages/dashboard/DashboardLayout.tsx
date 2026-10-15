import { Label } from '@components/ui'
import { useAuth } from '@lib/context/AuthContext'
import { Outlet } from 'react-router'

import { Sidebar } from './sections'

function DashboardLayout() {
  const { user } = useAuth()
  if (!user) return null

  return (
    <div className="min-h-svh md:flex">
      <Sidebar {...user} />
      <main className="flex min-w-0 flex-1 flex-col gap-8 px-edge py-10">
        <Label tone="brass">Sample data · not connected to the API yet</Label>
        <Outlet />
      </main>
    </div>
  )
}

export default DashboardLayout
