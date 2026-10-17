import type { Status } from '@components/ui'
import type z from 'zod'

import type {
  StudentSchema,
  StudentUpdateSchema,
  UserSchema,
  UserUpdateSchema
} from '@/schemas/input'

// mirrors api/src/schema/*.rs

export type Page<T> = { items: T[]; total: number; page: number; per_page: number }

export type PageQuery = { page?: string | number; per_page?: string | number; search?: string }

export type Student = {
  id: string
  dni: number
  first_name: string
  last_name: string
  email: string | null
  gender: 'Male' | 'Female'
  phone: number | null
  address: string | null
  avatar: string | null // from the student's own account, read-only
  created_at: string
  updated_at: string
}

export type NewStudent = z.output<typeof StudentSchema>

export type UpdateStudent = z.output<typeof StudentUpdateSchema>

export type NewUser = z.output<typeof UserSchema> & { avatar?: string }

export type UpdateUser = z.output<typeof UserUpdateSchema> & { avatar?: string }

export type InviteInfo = { first_name: string; last_name: string; email: string }

export type Representative = {
  id: string
  first_name: string
  last_name: string
  phone: number[]
  created_at: string
  updated_at: string
}

export type Period = {
  id: string
  name: string
  start_date: string
  end_date: string
}

export type Assistance = {
  id: string
  student_id: string
  period_id: string
  date: string
  status: Status
  notes: string | null
  created_at: string
  updated_at: string
}

export type DailyCode = { code: string; date: string }

export type NewAssistance = Pick<Assistance, 'student_id' | 'period_id'>
