import { AxiosError, type AxiosAdapter, type AxiosResponse } from 'axios'

import { current, db, isoDate } from '@/data/demo'
import type { Assistance, PageQuery } from '@/types/records'

import { getToken } from './utils'

// Demo mode: the whole dashboard on made-up data (data/demo.ts), no API needed. axios.ts
// swaps its adapter for this one while the stored token is the demo one, roll.ts fakes the
// socket. Changes live in memory, a reload starts over.

export const DEMO_TOKEN = 'demo'

export const isDemo = () => getToken() === DEMO_TOKEN

type Person = { id: string; updated_at: string; first_name: string; last_name: string }
type Reply = [status: number, data: unknown]

// GET /student and /user: same search, order and page bounds as PageQuery in the API
function page<T extends Person>(items: T[], params: PageQuery, text: (item: T) => string) {
  const term = String(params.search ?? '')
    .trim()
    .toLowerCase()
  const found = items
    .filter((item) => text(item).toLowerCase().includes(term))
    .toSorted(
      (a, b) => a.last_name.localeCompare(b.last_name) || a.first_name.localeCompare(b.first_name)
    )
  const number = Math.max(1, Number(params.page) || 1)
  const per_page = Math.min(1000, Math.max(1, Number(params.per_page) || 10))
  return {
    items: found.slice((number - 1) * per_page, number * per_page),
    total: found.length,
    page: number,
    per_page
  }
}

function update<T extends Person>(items: T[], id: string, changes: Partial<T>): Reply {
  const item = items.find((i) => i.id === id)
  if (!item) return [404, { message: 'Not found' }]
  Object.assign(item, changes, { updated_at: new Date().toISOString() })
  return [200, item]
}

const stamps = () => {
  const now = new Date().toISOString()
  return { id: crypto.randomUUID(), created_at: now, updated_at: now }
}

function route(method: string, path: string, params: PageQuery, body: any): Reply {
  const [, resource, id, sub] = path.split('/')
  const { password: _, ...changes } = body ?? {} // demo users have no password

  switch (`${method} ${path}`) {
    case 'GET /user/get-me':
      return [200, db.me]
    case 'GET /user':
      return [200, page(db.users, params, (u) => `${u.first_name} ${u.last_name} ${u.email}`)]
    case 'POST /user': {
      const user = { avatar: null, student_id: null, ...changes, ...stamps() }
      db.users.push(user)
      return [201, user]
    }
    case 'POST /user/avatar':
    case 'POST /user/me/avatar': {
      const url = URL.createObjectURL(body)
      if (id === 'me') db.me.avatar = url
      return [200, { url }]
    }
    case 'GET /student':
      return [
        200,
        page(db.students, params, (s) => `${s.first_name} ${s.last_name} ${s.email} ${s.dni}`)
      ]
    case 'POST /student': {
      const student = {
        email: null,
        phone: null,
        address: null,
        avatar: null,
        ...body,
        ...stamps()
      }
      db.students.push(student)
      return [201, student]
    }
    case 'GET /representative':
      return [200, db.representatives]
    case 'GET /period':
      return [200, db.periods]
    case 'GET /assistance/me':
      return [200, []] // the demo account is staff, not a student
    case 'GET /assistance/daily-code':
      return [200, { code: `demo-${isoDate(Date.now())}`, date: isoDate(Date.now()) }]
  }

  if (method === 'GET' && resource === 'period' && sub === 'assistance') {
    return [200, db.assistances.filter((a) => a.period_id === id)]
  }
  if (method === 'PATCH' && resource === 'student') return update(db.students, id, body)
  if (method === 'PATCH' && resource === 'user') return update(db.users, id, changes)
  if (method === 'DELETE' && resource === 'student') {
    db.students = db.students.filter((s) => s.id !== id)
    db.assistances = db.assistances.filter((a) => a.student_id !== id)
    return [204, null]
  }
  return [404, { message: 'Not available in the demo' }]
}

export const demoAdapter: AxiosAdapter = async (config) => {
  await new Promise((resolve) => setTimeout(resolve, 150)) // like a fast network
  const body = typeof config.data === 'string' ? JSON.parse(config.data) : config.data
  const [status, data] = route(config.method!.toUpperCase(), config.url!, config.params ?? {}, body)
  // copies, the pages must not change the demo data without a request
  const response: AxiosResponse = {
    data: structuredClone(data),
    status,
    statusText: '',
    headers: {},
    config
  }
  if (status >= 400) {
    throw new AxiosError(
      (data as { message: string }).message,
      AxiosError.ERR_BAD_REQUEST,
      config,
      null,
      response
    )
  }
  return response
}

// What the socket would broadcast when a student is scanned: the given one, or a random
// student not on today's roll yet. null once the whole class is in.
export function demoScan(studentId?: string): Assistance | null {
  const date = isoDate(Date.now())
  const done = new Set(db.assistances.filter((a) => a.date === date).map((a) => a.student_id))
  const left = db.students.filter((s) => !done.has(s.id))
  const student = studentId
    ? left.find((s) => s.id === studentId)
    : left[Math.floor(Math.random() * left.length)]
  if (!student) return null

  const assistance: Assistance = {
    ...stamps(),
    student_id: student.id,
    period_id: current.id,
    date,
    status: Math.random() < 0.85 ? 'Present' : 'Late',
    notes: null
  }
  db.assistances.push(assistance)
  return structuredClone(assistance)
}
