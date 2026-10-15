import type { Status } from '@components/ui'

// ponytail: static sample data so the pages can be designed, replace with API calls per page

export const currentUser = { name: 'Abraham Chafloque', role: 'SuperAdmin' }

export const currentPeriod = '2026-II'

export const roll: { name: string; time: string; status: Status; notes: string }[] = [
  { name: 'Carlos Ramírez', time: '07:52', status: 'Present', notes: '' },
  { name: 'María Gonzales', time: '07:58', status: 'Present', notes: '' },
  { name: 'Diego Torres', time: '08:14', status: 'Late', notes: '' },
  { name: 'Lucía Vargas', time: '—', status: 'Excused', notes: 'Medical appointment' },
  { name: 'Andrés Castillo', time: '—', status: 'Absent', notes: '' }
]

export const students = [
  {
    dni: 74283156,
    name: 'Carlos Ramírez',
    email: 'carlos.ramirez@example.com',
    gender: 'Male',
    phone: '987 654 321'
  },
  {
    dni: 68192437,
    name: 'María Gonzales',
    email: 'maria.gonzales@example.com',
    gender: 'Female',
    phone: '986 543 217'
  },
  {
    dni: 71938542,
    name: 'Diego Torres',
    email: 'diego.torres@example.com',
    gender: 'Male',
    phone: '975 321 864'
  },
  {
    dni: 65381729,
    name: 'Lucía Vargas',
    email: 'lucia.vargas@example.com',
    gender: 'Female',
    phone: '964 218 753'
  },
  {
    dni: 78452631,
    name: 'Andrés Castillo',
    email: 'andres.castillo@example.com',
    gender: 'Male',
    phone: '953 647 821'
  }
]

export const representatives = [
  {
    name: 'Rosa Ramírez',
    phones: '987 654 321 · 912 345 678',
    student: 'Carlos Ramírez',
    relationship: 'Mother',
    primary: true
  },
  {
    name: 'Luis Gonzales',
    phones: '986 111 222',
    student: 'María Gonzales',
    relationship: 'Father',
    primary: true
  },
  {
    name: 'Elena Torres',
    phones: '975 333 444',
    student: 'Diego Torres',
    relationship: 'GrandMother',
    primary: false
  },
  {
    name: 'Jorge Vargas',
    phones: '964 555 666',
    student: 'Lucía Vargas',
    relationship: 'LegalGuardian',
    primary: true
  }
]

export const periods = [
  { name: '2027-I', start: '2027-03-08', end: '2027-07-16', state: 'Upcoming' },
  { name: '2026-II', start: '2026-08-10', end: '2026-12-18', state: 'Current' },
  { name: '2026-I', start: '2026-03-09', end: '2026-07-17', state: 'Closed' },
  { name: '2025-II', start: '2025-08-11', end: '2025-12-19', state: 'Closed' }
]

export const users = [
  { name: 'Abraham Chafloque', email: 'abraham.chafloque@example.com', role: 'SuperAdmin' },
  { name: 'Alice Johnson', email: 'alice.johnson@example.com', role: 'Admin' },
  { name: 'Diana Prince', email: 'diana.prince@example.com', role: 'Admin' },
  { name: 'John Doe', email: 'john.doe@example.com', role: 'User' },
  { name: 'Bob Smith', email: 'bob.smith@example.com', role: 'User' }
]
