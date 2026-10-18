import { Label } from '@components/ui'
import { startDemo } from '@lib/auth'
import { useAuth } from '@lib/context/AuthContext'
import { useEffect } from 'react'
import { useNavigate } from 'react-router'

import { paths } from '@/paths'

// /demo, the link to share: signs in to demo mode and opens the dashboard
function Demo() {
  const { setUser } = useAuth()
  const navigate = useNavigate()

  useEffect(() => {
    startDemo().then((user) => {
      setUser(user)
      navigate(paths.dashboard, { replace: true })
    })
  }, [setUser, navigate])

  return (
    <main className="grid min-h-svh place-items-center">
      <Label>Opening the demo…</Label>
    </main>
  )
}

export default Demo
