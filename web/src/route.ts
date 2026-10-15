import {
  getInvite,
  getPeriods,
  getRepresentatives,
  getStudents,
  getUsers,
  loadRoll
} from '@lib/api'
import { guestOnly, requireAuth } from '@lib/guard'
import { createBrowserRouter, type LoaderFunctionArgs } from 'react-router'

import Assistance from '@/pages/dashboard/Assistance'
import DashboardLayout from '@/pages/dashboard/DashboardLayout'
import Overview from '@/pages/dashboard/Overview'
import Periods from '@/pages/dashboard/Periods'
import QrScan from '@/pages/dashboard/QrScan'
import Representatives from '@/pages/dashboard/Representatives'
import RouteError from '@/pages/dashboard/RouteError'
import Students from '@/pages/dashboard/students/Students'
import Users from '@/pages/dashboard/users/Users'
import Home from '@/pages/home/Home'
import Login from '@/pages/login/Login'
import Placeholder from '@/pages/placeholder/Placeholder'
import Register, { InvalidInvite } from '@/pages/register/Register'

import { paths } from './paths'

export const router = createBrowserRouter([
  { path: paths.home, Component: Home },
  { path: paths.login, Component: Login, loader: guestOnly },
  {
    path: paths.register,
    Component: Register,
    ErrorBoundary: InvalidInvite,
    loader: ({ request }: LoaderFunctionArgs) => {
      guestOnly()
      return getInvite(new URL(request.url).searchParams.get('token') ?? '')
    }
  },
  {
    path: paths.dashboard,
    Component: DashboardLayout,
    loader: requireAuth,
    children: [
      { index: true, Component: Overview, loader: loadRoll },
      { path: paths.qrScan, Component: QrScan, loader: loadRoll },
      { path: paths.assistance, Component: Assistance, loader: loadRoll },
      {
        path: paths.students,
        Component: Students,
        // ?page, ?per_page and ?search go straight to GET /student
        loader: ({ request }: LoaderFunctionArgs) =>
          getStudents(Object.fromEntries(new URL(request.url).searchParams))
      },
      { path: paths.representatives, Component: Representatives, loader: getRepresentatives },
      { path: paths.periods, Component: Periods, loader: getPeriods },
      {
        path: paths.users,
        Component: Users,
        loader: ({ request }: LoaderFunctionArgs) =>
          getUsers(Object.fromEntries(new URL(request.url).searchParams))
      }
    ].map((route) => ({ ...route, ErrorBoundary: RouteError }))
  },
  { path: '*', Component: Placeholder }
])
