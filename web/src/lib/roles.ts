import type { User } from '@/types/user'

import { useAuth } from './context/AuthContext'

// Every role check in the UI goes through here. The API enforces the same rule
// (middlewares::user::admin_only), this only decides what to show.
const ADMIN_ROLES: User['role'][] = ['SuperAdmin', 'Admin']

export const isAdmin = (user: User | null) => user !== null && ADMIN_ROLES.includes(user.role)

// Mirrors Role::can_create in the API: admins add users, only the SuperAdmin adds admins
export function assignableRoles(user: User | null): User['role'][] {
  if (user?.role === 'SuperAdmin') return ['User', 'Admin']
  if (isAdmin(user)) return ['User']
  return []
}

export function useIsAdmin() {
  const { user } = useAuth()
  return isAdmin(user)
}
