import { Outlet } from 'react-router'

import Label from '../atoms/Label'
import Sidebar from '../organisms/Sidebar'

function DashboardLayout() {
  return (
    <div className="min-h-svh md:flex">
      <Sidebar />
      <main className="flex min-w-0 flex-1 flex-col gap-8 px-edge py-10">
        <Label tone="brass">Sample data · not connected to the API yet</Label>
        <Outlet />
      </main>
    </div>
  )
}

export default DashboardLayout
