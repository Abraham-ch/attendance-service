import type { Status } from '@components/ui'

// Demo roll for the public landing page, the dashboard reads the API

export const roll: { name: string; time: string; status: Status; notes: string }[] = [
  { name: 'Carlos Ramírez', time: '07:52', status: 'Present', notes: '' },
  { name: 'María Gonzales', time: '07:58', status: 'Present', notes: '' },
  { name: 'Diego Torres', time: '08:14', status: 'Late', notes: '' },
  { name: 'Lucía Vargas', time: '—', status: 'Excused', notes: 'Medical appointment' },
  { name: 'Andrés Castillo', time: '—', status: 'Absent', notes: '' }
]
