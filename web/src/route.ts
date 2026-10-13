import { createBrowserRouter } from 'react-router'

import DashboardLayout from './components/templates/DashboardLayout'
import { guestOnly, requireAuth } from './lib/guard'
import Assistance from './pages/dashboard/Assistance'
import Overview from './pages/dashboard/Overview'
import Periods from './pages/dashboard/Periods'
import QrScan from './pages/dashboard/QrScan'
import Representatives from './pages/dashboard/Representatives'
import Students from './pages/dashboard/Students'
import Users from './pages/dashboard/Users'
import Landing from './pages/Landing'
import Login from './pages/Login'
import Placeholder from './pages/Placeholder'
import { paths } from './paths'

// ponytail: no auth guard yet. Add a loader on the dashboard route that checks the token
// once the login form really signs in.
export const router = createBrowserRouter([
  { path: paths.home, Component: Landing },
  { path: paths.login, Component: Login, loader: guestOnly },
  {
    path: paths.dashboard,
    Component: DashboardLayout,
    loader: requireAuth,
    children: [
      { index: true, Component: Overview },
      { path: paths.qrScan, Component: QrScan },
      { path: paths.assistance, Component: Assistance },
      { path: paths.students, Component: Students },
      { path: paths.representatives, Component: Representatives },
      { path: paths.periods, Component: Periods },
      { path: paths.users, Component: Users }
    ]
  },
  { path: '*', Component: Placeholder }
])
