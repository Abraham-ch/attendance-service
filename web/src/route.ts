import { createBrowserRouter } from 'react-router'

import Landing from './pages/Landing'
import Placeholder from './pages/Placeholder'

export const paths = {
  home: '/',
  login: '/auth/login',
  dashboard: '/dashboard',
  users: '/dashboard/user',
  students: '/dashboard/student',
  representatives: '/dashboard/representative',
  periods: '/dashboard/period',
  assistance: '/dashboard/assistance',
  qrScan: '/dashboard/qr-scan'
} as const

// ponytail: flat list, no layout or auth guard yet. Nest the dashboard routes under a
// layout route with a loader that checks the token once the dashboard shell exists.
export const router = createBrowserRouter([
  { path: paths.home, Component: Landing },
  { path: paths.login, Component: Placeholder },
  { path: paths.dashboard, Component: Placeholder },
  { path: paths.users, Component: Placeholder },
  { path: paths.students, Component: Placeholder },
  { path: paths.representatives, Component: Placeholder },
  { path: paths.periods, Component: Placeholder },
  { path: paths.assistance, Component: Placeholder },
  { path: paths.qrScan, Component: Placeholder },
  { path: '*', Component: Placeholder }
])
