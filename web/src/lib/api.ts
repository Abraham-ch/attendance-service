import { isAxiosError } from 'axios'

import type {
  Assistance,
  InviteInfo,
  NewStudent,
  NewUser,
  Page,
  PageQuery,
  Period,
  Representative,
  Student,
  UpdateStudent
} from '@/types/records'
import type { User } from '@/types/user'

import { api } from './axios'

// The API's own message when it sent one (AppError), else axios' generic one
export function apiError(error: unknown) {
  if (isAxiosError(error)) return error.response?.data?.message ?? error.message
  return String(error)
}

export const getStudents = (params: PageQuery = {}) =>
  api.get<Page<Student>>('/student', { params }).then((res) => res.data)

export const createStudent = (student: NewStudent) => api.post('/student', student)

export const updateStudent = (id: string, student: UpdateStudent) =>
  api.patch(`/student/${id}`, student)

export const deleteStudent = (id: string) => api.delete(`/student/${id}`)

export const getRepresentatives = () =>
  api.get<Representative[]>('/representative').then((res) => res.data)

export const getUsers = (params: PageQuery = {}) =>
  api.get<Page<User>>('/user', { params }).then((res) => res.data)

export const createUser = (user: NewUser) => api.post<User>('/user', user)

export const getInvite = (token: string) =>
  api.get<InviteInfo>('/auth/invite', { params: { token } }).then((res) => res.data)

export const getPeriods = () =>
  api
    .get<Period[]>('/period')
    .then((res) => res.data.sort((a, b) => b.start_date.localeCompare(a.start_date)))

export const getPeriodAssistances = (periodId: string) =>
  api.get<Assistance[]>(`/period/${periodId}/assistance`).then((res) => res.data)

export function periodState(period: Period, now = Date.now()) {
  if (now < Date.parse(period.start_date)) return 'Upcoming'
  if (now > Date.parse(period.end_date)) return 'Closed'
  return 'Current'
}

// Loader for the pages that show the attendance roll of the current period
export async function loadRoll() {
  // ponytail: loads up to 1000 students to put names on the roll, have the API return
  // the student name with each record once schools outgrow that
  const [periods, { items: students }] = await Promise.all([
    getPeriods(),
    getStudents({ per_page: 1000 })
  ])
  const period = periods.find((p) => periodState(p) === 'Current') ?? null
  const assistances = period ? await getPeriodAssistances(period.id) : []
  return { period, students, assistances }
}
