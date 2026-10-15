import type { AuthResponse, LoginData } from '@/types/auth'
import type { User } from '@/types/user'

import { api } from './axios'

export const login = async (data: LoginData): Promise<AuthResponse> => {
  const response = await api.post<AuthResponse>('/auth', data)

  localStorage.setItem('token', response.data.token)
  return response.data
}

export function logout() {
  localStorage.removeItem('token')
}

export const getCurrentUser = async (): Promise<User> => {
  const response = await api.get<User>('/user/get-me')

  return response.data
}
