import * as z from 'zod'

// Same rules as valid_password + length(min = 10, max = 20) in the API
const passwordSchema = z
  .string()
  .min(10, { message: 'Password must be at least 10 characters long.' })
  .max(20, { message: 'Password cannot exceed 20 characters.' })
  .refine((password) => /[A-Z]/.test(password), {
    message: 'Password must contain at least one uppercase letter (A-Z).'
  })
  .refine((password) => /[a-z]/.test(password), {
    message: 'Password must contain at least one lowercase letter (a-z).'
  })
  .refine((password) => /[0-9]/.test(password), {
    message: 'Password must contain at least one number (0-9).'
  })
  .refine((password) => /[^a-zA-Z0-9]/.test(password), {
    message: 'Password must contain at least one symbol, like ! or ?.'
  })

// Login only checks presence: the strength rules belong to setting a password, and
// listing them here would just hint at what a valid password looks like
export const LoginSchema = z.object({
  email: z.email({ message: 'Enter a valid email address.' }),
  password: z.string().min(1, { message: 'Enter your password.' })
})

// Inputs give '' for an empty optional field, the API wants it left out
const blank = (value: unknown) =>
  typeof value === 'string' && value.trim() === '' ? undefined : value

// Same limits as NewStudent / UpdateStudent in api/src/schema/student.rs
export const StudentSchema = z.object({
  dni: z.coerce.number({ message: 'DNI must be a number.' }).int().positive(),
  first_name: z.string().trim().min(3).max(20),
  last_name: z.string().trim().min(3).max(20),
  email: z.preprocess(blank, z.email().optional()),
  gender: z.enum(['Male', 'Female']),
  phone: z.preprocess(
    blank,
    z.coerce.number({ message: 'Phone must be a number.' }).int().positive().optional()
  ),
  address: z.preprocess(blank, z.string().trim().min(3).max(100).optional())
})

export const StudentUpdateSchema = StudentSchema.pick({ phone: true, address: true })

export const RegisterSchema = z
  .object({ password: passwordSchema, confirm: z.string() })
  .refine((data) => data.password === data.confirm, {
    message: "Passwords don't match.",
    path: ['confirm']
  })

// NewUser in api/src/schema/user.rs
export const UserSchema = z.object({
  first_name: z.string().trim().min(3).max(20),
  last_name: z.string().trim().min(3).max(20),
  email: z.email(),
  password: passwordSchema,
  role: z.enum(['Admin', 'User'])
})
