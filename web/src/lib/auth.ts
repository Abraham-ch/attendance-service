import type { AuthResponse, LoginData } from '@/types/auth'
import type { User } from '@/types/user'

import { api } from './axios'
import { DEMO_TOKEN } from './demo'

export const login = async (data: LoginData): Promise<AuthResponse> => {
  const response = await api.post<AuthResponse>('/auth', data)

  localStorage.setItem('token', response.data.token)
  forgetCurrentUser()
  return response.data
}

// Uses the invite token from the email, the API answers like a login
export const register = async (token: string, password: string): Promise<AuthResponse> => {
  const response = await api.post<AuthResponse>('/auth/register', { token, password })

  localStorage.setItem('token', response.data.token)
  forgetCurrentUser()
  return response.data
}

// Signs in to demo mode (lib/demo.ts), answers the demo account
export function startDemo(): Promise<User> {
  localStorage.setItem('token', DEMO_TOKEN)
  forgetCurrentUser()
  return getCurrentUser()
}

export function logout() {
  localStorage.removeItem('token')
  forgetCurrentUser()
}

// One GET /user/get-me per token: AuthProvider (run twice by StrictMode in dev) and every
// roll loader ask for it, they all share this promise. A failed request isn't kept.
let currentUser: Promise<User> | null = null

export const getCurrentUser = (): Promise<User> =>
  (currentUser ??= api
    .get<User>('/user/get-me')
    .then((res) => res.data)
    .catch((error) => {
      currentUser = null
      throw error
    }))

// Next getCurrentUser() asks the API again, for when the token or the user changed
export function forgetCurrentUser() {
  currentUser = null
}
