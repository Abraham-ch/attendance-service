import { useLayoutEffect, useEffect, useRef, useState } from 'react'
import { useLoaderData } from 'react-router'

import type { Assistance, NewAssistance } from '@/types/records'

import type { loadRoll } from './api'
import { getToken } from './utils'

function socketUrl() {
  const url = new URL('/assistance', import.meta.env.VITE_API_URL)
  url.protocol = url.protocol === 'https:' ? 'wss:' : 'ws:'
  // browsers can't set headers on a WebSocket, the auth middleware also reads ?token=
  url.searchParams.set('token', getToken() ?? '')
  return url
}

// ponytail: no reconnect, a dropped socket shows as "Offline" until the page reloads.
// Add a retry with backoff if connections drop in practice.
export function useAssistanceSocket(onAssistance: (assistance: Assistance) => void) {
  const socket = useRef<WebSocket | null>(null)
  const [connected, setConnected] = useState(false)

  const onAssistanceRef = useRef(onAssistance)
  useLayoutEffect(() => {
    onAssistanceRef.current = onAssistance
  }, [onAssistance])

  useEffect(() => {
    const ws = new WebSocket(socketUrl())
    socket.current = ws
    ws.onopen = () => setConnected(true)
    ws.onclose = () => setConnected(false)
    ws.onmessage = (event) => onAssistanceRef.current(JSON.parse(event.data))
    return () => ws.close()
  }, [])

  function send(scan: NewAssistance) {
    if (socket.current?.readyState !== WebSocket.OPEN) return false
    socket.current.send(JSON.stringify(scan))
    return true
  }

  return { connected, send }
}

export const today = () => new Date().toISOString().slice(0, 10) // the API stores UTC dates

// Roll of the current period: loader data plus every record the socket broadcasts
export function useLiveRoll() {
  const { period, students, assistances } = useLoaderData<typeof loadRoll>()
  const [records, setRecords] = useState(assistances)

  const { connected, send } = useAssistanceSocket((assistance) => {
    if (assistance.period_id === period?.id) setRecords((prev) => [assistance, ...prev])
  })

  const names = new Map(students.map((s) => [s.id, `${s.first_name} ${s.last_name}`]))
  const rows = records
    .toSorted((a, b) => b.created_at.localeCompare(a.created_at))
    .map((record) => ({
      id: record.id,
      studentId: record.student_id,
      date: record.date,
      name: names.get(record.student_id) ?? 'Unknown student',
      time: new Date(record.created_at).toLocaleTimeString([], {
        hour: '2-digit',
        minute: '2-digit'
      }),
      status: record.status,
      notes: record.notes ?? ''
    }))

  return { period, students, rows, connected, send }
}
