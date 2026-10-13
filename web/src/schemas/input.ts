import * as z from 'zod'

const passwordSchema = z
  .string()
  .min(8, { message: 'Password must be at least 8 characters long.' })
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
  .refine((password) => /[!@#$%^&*?]/.test(password), {
    message: 'Password must contain at least one special character (!@#$%^&*?).'
  })

export const LoginSchema = z.object({
  email: z.email(),
  password: passwordSchema
})
