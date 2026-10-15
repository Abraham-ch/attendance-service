export type Role = 'SuperAdmin' | 'Admin' | 'User'

export type User = {
  id: string
  first_name: string
  last_name: string
  email: string
  avatar: string | null
  role: Role
  created_at: string
  updated_at: string
}
