import { guestOnly, requireAuth } from '@lib/guard'
import { createBrowserRouter } from 'react-router'

import Assistance from '@/pages/dashboard/Assistance'
import DashboardLayout from '@/pages/dashboard/DashboardLayout'
import Overview from '@/pages/dashboard/Overview'
import Periods from '@/pages/dashboard/Periods'
import QrScan from '@/pages/dashboard/QrScan'
import Representatives from '@/pages/dashboard/Representatives'
import Students from '@/pages/dashboard/Students'
import Users from '@/pages/dashboard/Users'
import Home from '@/pages/home/Home'
import Login from '@/pages/login/Login'
import Placeholder from '@/pages/placeholder/Placeholder'

import { paths } from './paths'

// ponytail: no auth guard yet. Add a loader on the dashboard route that checks the token
// once the login form really signs in.
export const router = createBrowserRouter([
  { path: paths.home, Component: Home },
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
