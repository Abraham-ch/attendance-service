import { isAxiosError } from 'axios'

import type {
  Assistance,
  DailyCode,
  InviteInfo,
  NewStudent,
  NewUser,
  Page,
  PageQuery,
  Period,
  Representative,
  Student,
  UpdateStudent,
  UpdateUser
} from '@/types/records'
import type { User } from '@/types/user'

import { forgetCurrentUser, getCurrentUser } from './auth'
import { api } from './axios'
import type { Spot } from './geo'
import { isAdmin } from './roles'

// The API's own message when it sent one (AppError), else axios' generic one
export function apiError(error: unknown) {
  if (isAxiosError(error)) return error.response?.data?.message ?? error.message
  if (error instanceof Error) return error.message
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

export const updateUser = (id: string, changes: UpdateUser) =>
  api.patch<User>(`/user/${id}`, changes).then((res) => res.data)

export const getInvite = (token: string) =>
  api.get<InviteInfo>('/auth/invite', { params: { token } }).then((res) => res.data)

export const getPeriods = () =>
  api
    .get<Period[]>('/period')
    .then((res) => res.data.sort((a, b) => b.start_date.localeCompare(a.start_date)))

export const getPeriodAssistances = (periodId: string) =>
  api.get<Assistance[]>(`/period/${periodId}/assistance`).then((res) => res.data)

// The code only works near `spot`, where the admin showing it is
export const getDailyCode = (spot: Spot) =>
  api.get<DailyCode>('/assistance/daily-code', { params: spot }).then((res) => res.data)

// The API takes the student from the token, the body carries what the camera read and
// where the phone is
export const checkIn = (code: string, spot: Spot) =>
  api.post<Assistance>('/assistance/check-in', { code, ...spot }).then((res) => res.data)

// The raw image as the body, both answer the public URL of the stored picture
const postAvatar = (path: string, image: Blob) =>
  api
    .post<{ url: string }>(path, image, { headers: { 'Content-Type': image.type } })
    .then((res) => res.data.url)

// admins, for the user they're creating (sent as NewUser.avatar)
export const uploadAvatar = (image: Blob) => postAvatar('/user/avatar', image)

// the logged-in user's own picture, saved on their account by the API
export const uploadMyAvatar = (image: Blob) =>
  postAvatar('/user/me/avatar', image).finally(forgetCurrentUser)

export const getMyAssistances = () =>
  api.get<Assistance[]>('/assistance/me').then((res) => res.data)

export function periodState(period: Period, now = Date.now()) {
  if (now < Date.parse(period.start_date)) return 'Upcoming'
  if (now > Date.parse(period.end_date)) return 'Closed'
  return 'Current'
}

type RollStudent = Pick<Student, 'id' | 'first_name' | 'last_name'>

// Loader for the pages that show the attendance roll of the current period.
// Staff get the whole roll, a student only their own records.
export async function loadRoll(): Promise<{
  me: User
  period: Period | null
  students: RollStudent[]
  assistances: Assistance[]
}> {
  const [me, periods] = await Promise.all([getCurrentUser(), getPeriods()])
  const period = periods.find((p) => periodState(p) === 'Current') ?? null

  if (!isAdmin(me)) {
    const mine = await getMyAssistances()
    return {
      me,
      period,
      students: me.student_id ? [{ ...me, id: me.student_id }] : [],
      assistances: mine.filter((a) => a.period_id === period?.id)
    }
  }

  // the student name with each record once schools outgrow that
  const [{ items: students }, assistances] = await Promise.all([
    getStudents({ per_page: 1000 }),
    period ? getPeriodAssistances(period.id) : []
  ])
  return { me, period, students, assistances }
}
