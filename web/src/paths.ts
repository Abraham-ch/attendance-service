// lives outside route.ts so components can link to routes without importing the pages
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
