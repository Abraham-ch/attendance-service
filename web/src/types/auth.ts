import type z from 'zod'

import type { LoginSchema } from '../schemas/input'
import type { User } from './user'

export type LoginData = z.infer<typeof LoginSchema>
export type AuthResponse = { user: User; token: string }
