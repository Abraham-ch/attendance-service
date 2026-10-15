import { getCurrentUser } from '@lib/auth'
import { createContext, useContext, useEffect, useState, type ReactNode } from 'react'

import type { User } from '@/types/user'

type AuthContextType = {
  loading: boolean
  user: User | null
  setUser: (user: User | null) => void
}

const AuthContext = createContext<AuthContextType | null>(null)

const AuthProvider = ({ children }: { children: ReactNode }) => {
  const [user, setUser] = useState<User | null>(null)
  const [loading, setLoading] = useState(() => localStorage.getItem('token') !== null)

  useEffect(() => {
    const token = localStorage.getItem('token')
    if (!token) return

    getCurrentUser()
      .then((res) => setUser(res))
      .catch(() => {
        localStorage.removeItem('token')
        setUser(null)
      })
      .finally(() => setLoading(false))
  }, [])

  return <AuthContext.Provider value={{ user, setUser, loading }}>{children}</AuthContext.Provider>
}

const useAuth = () => {
  const context = useContext(AuthContext)

  if (!context) {
    throw new Error('useAuth must be used within AuthProvider')
  }

  return context
}

export { AuthProvider, AuthContext, useAuth }
