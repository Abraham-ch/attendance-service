import type { Status } from '@components/ui'

import type { Assistance, Period, Representative, Student } from '@/types/records'
import type { User } from '@/types/user'

// The made-up school behind demo mode (lib/demo.ts). Dates are relative to today so the
// current period is always running, the fixed seed keeps the charts the same on every visit.

let seed = 20261018
const random = () => (seed = (seed * 16807) % 2147483647) / 2147483647
const pick = <T>(items: T[]) => items[Math.floor(random() * items.length)]
const between = (min: number, max: number) => min + Math.floor(random() * (max - min))

const DAY = 86_400_000
const now = Date.now()
export const isoDate = (time: number) => new Date(time).toISOString().slice(0, 10)
const created = new Date(now - 400 * DAY).toISOString()
// local time on that day, so the roll shows school hours wherever the demo is opened
const at = (date: string, minutes: number) =>
  new Date(new Date(`${date}T00:00`).getTime() + minutes * 60_000).toISOString()
const ascii = (text: string) => text.normalize('NFD').replace(/\p{M}/gu, '').toLowerCase()

const firstNames = [
  'Carlos',
  'María',
  'Diego',
  'Lucía',
  'Andrés',
  'Valeria',
  'Mateo',
  'Camila',
  'Sebastián',
  'Daniela',
  'Joaquín',
  'Sofía',
  'Gabriel',
  'Isabella',
  'Nicolás',
  'Fernanda',
  'Adrián',
  'Ximena',
  'Rodrigo',
  'Paula',
  'Emilio',
  'Renata',
  'Tomás',
  'Antonella'
]
const lastNames = [
  'Ramírez',
  'Gonzales',
  'Torres',
  'Vargas',
  'Castillo',
  'Flores',
  'Rojas',
  'Mendoza',
  'Quispe',
  'Huamán',
  'Chávez',
  'Díaz',
  'Herrera',
  'Silva',
  'Morales',
  'Paredes'
]
const streets = [
  'Av. Arequipa',
  'Jr. de la Unión',
  'Av. Brasil',
  'Calle Los Olivos',
  'Av. La Marina'
]

const students: Student[] = firstNames.map((first_name, i) => {
  const last_name = pick(lastNames)
  return {
    id: crypto.randomUUID(),
    dni: between(70_000_000, 79_999_999),
    first_name,
    last_name,
    email: `${ascii(first_name)}.${ascii(last_name)}@demo.school`,
    gender: i % 2 ? 'Female' : 'Male', // the names alternate
    phone: between(900_000_000, 999_999_999),
    address: `${pick(streets)} ${between(100, 2500)}`,
    avatar: null,
    created_at: created,
    updated_at: created
  }
})

const period = (name: string, from: number, to: number): Period => ({
  id: crypto.randomUUID(),
  name,
  start_date: new Date(now + from * DAY).toISOString(),
  end_date: new Date(now + to * DAY).toISOString()
})

export const current = period('Term 3', -75, 60)
const periods = [
  period('Term 4', 75, 200),
  current,
  period('Term 2', -215, -90),
  period('Term 1', -355, -230)
]

function status(sick: number): Status {
  const roll = random()
  if (roll < 0.8 - sick) return 'Present'
  if (roll < 0.9 - sick) return 'Late'
  if (roll < 0.97) return 'Absent'
  return 'Excused'
}

const record = (student: Student, period: Period, date: string, status: Status) => {
  const minutes = { Present: between(450, 480), Late: between(485, 520), Absent: 540, Excused: 540 }
  const time = at(date, minutes[status])
  return {
    id: crypto.randomUUID(),
    student_id: student.id,
    period_id: period.id,
    date,
    status,
    notes:
      status === 'Excused'
        ? pick(['Medical appointment', 'Family trip', 'Sports tournament'])
        : null,
    created_at: time,
    updated_at: time
  }
}

// every school day of the past periods and of this one up to yesterday
const assistances: Assistance[] = []
const yesterday = Date.parse(isoDate(now)) - DAY
for (const p of periods.slice(1)) {
  for (
    let day = Date.parse(isoDate(Date.parse(p.start_date)));
    day <= Math.min(Date.parse(p.end_date), yesterday);
    day += DAY
  ) {
    if ([0, 6].includes(new Date(day).getUTCDay())) continue
    const sick = random() < 0.1 ? 0.15 : 0 // a few bad days, so the by-day chart isn't flat
    for (const student of students) assistances.push(record(student, p, isoDate(day), status(sick)))
  }
}
// part of the class is already in today, the live feed brings the rest (lib/demo.ts)
for (const student of students.slice(0, 9)) {
  assistances.push(record(student, current, isoDate(now), random() < 0.85 ? 'Present' : 'Late'))
}

const me: User = {
  id: crypto.randomUUID(),
  first_name: 'Demo',
  last_name: 'Admin',
  email: 'demo@demo.school',
  avatar: null,
  student_id: null,
  role: 'SuperAdmin',
  created_at: created,
  updated_at: created
}

const user = (
  first_name: string,
  last_name: string,
  role: User['role'],
  student_id: string | null = null
): User => ({
  ...me,
  id: crypto.randomUUID(),
  first_name,
  last_name,
  email: `${ascii(first_name)}.${ascii(last_name)}@demo.school`,
  role,
  student_id
})

const users = [
  me,
  user('Rosa', 'Medina', 'Admin'),
  user('Jorge', 'Paz', 'Admin'),
  ...students.slice(0, 5).map((s) => user(s.first_name, s.last_name, 'User', s.id))
]

const representatives: Representative[] = [
  'Elena',
  'Ricardo',
  'Patricia',
  'Luis',
  'Carmen',
  'Miguel'
].map((first_name) => ({
  id: crypto.randomUUID(),
  first_name,
  last_name: pick(lastNames),
  phone:
    random() < 0.5
      ? [between(900_000_000, 999_999_999)]
      : [between(900_000_000, 999_999_999), between(900_000_000, 999_999_999)],
  created_at: created,
  updated_at: created
}))

// mutable: demo-mode changes live here until the page reloads
export const db = { me, users, students, representatives, periods, assistances }
