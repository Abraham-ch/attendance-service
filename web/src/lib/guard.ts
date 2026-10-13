import { redirect } from 'react-router'

export function requireAuth() {
  const token = localStorage.getItem('token')

  if (!token) {
    throw redirect('/auth/login')
  }

  return null
}

export function guestOnly() {
  const token = localStorage.getItem('token')

  if (token) {
    throw redirect('/dashboard')
  }

  return null
}
