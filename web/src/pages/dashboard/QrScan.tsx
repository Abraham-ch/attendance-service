import { PageHeader, RollPanel } from '@components/common'
import { Button, Label, Mono, QrCode, ScanFrame, Text } from '@components/ui'
import { apiError, checkIn, getDailyCode } from '@lib/api'
import { getPosition, type Spot } from '@lib/geo'
import { today, useLiveRoll } from '@lib/roll'
import { useEffect, useRef, useState, type ReactNode } from 'react'
import { useRevalidator } from 'react-router'

import type { DailyCode } from '@/types/records'

const RETRY_MS = 5000 // the camera reads the same code several times per second

type Roll = ReturnType<typeof useLiveRoll>
type Tab = 'scan' | 'show'

// Staff scan the students' own QR codes or show the daily QR to the room.
// Students scan that daily QR to check themselves in, or show their own QR to staff.
function QrScan() {
  const roll = useLiveRoll()
  const { admin, period, rows, live } = roll
  const [tab, setTab] = useState<Tab>('scan')
  const tabs: Record<Tab, string> = admin
    ? { scan: 'Scan students', show: 'Daily QR' }
    : { scan: 'Scan daily QR', show: 'My QR' }

  return (
    <>
      <PageHeader kicker={period ? `Period ${period.name}` : 'No current period'} title="QR scan" />
      <div role="tablist" aria-label="QR mode" className="flex gap-1 border-b border-line">
        {(Object.keys(tabs) as Tab[]).map((key) => (
          <button
            key={key}
            type="button"
            role="tab"
            aria-selected={tab === key}
            onClick={() => setTab(key)}
            className={`-mb-px cursor-pointer border-b-2 px-4 py-2.5 text-sm transition-colors focus-visible:outline-2 focus-visible:outline-brass ${tab === key ? 'border-brass font-medium text-ink' : 'border-transparent text-mute hover:text-ink'}`}
          >
            {tabs[key]}
          </button>
        ))}
      </div>
      <div className="grid items-start gap-10 lg:grid-cols-[24rem_1fr]">
        <div role="tabpanel" aria-label={tabs[tab]} className="flex flex-col gap-4">
          {admin && tab === 'scan' && <StudentScanner roll={roll} />}
          {admin && tab === 'show' && <DailyQr />}
          {!admin && tab === 'scan' && <CheckInScanner roll={roll} />}
          {!admin && tab === 'show' && <MyQr roll={roll} />}
        </div>
        <RollPanel
          title={admin ? "Today's scans" : 'My records today'}
          rows={rows.filter((row) => row.date === today())}
          live={live}
        />
      </div>
    </>
  )
}

type ScannerProps = {
  disabled: boolean
  scanning: boolean
  setScanning: (on: boolean) => void
  message: string
  setMessage: (message: string) => void
  onDetect: (code: string) => void
  hint: ReactNode
}

function Scanner(props: ScannerProps) {
  const { disabled, scanning, setScanning, message, setMessage, onDetect, hint } = props
  return (
    <>
      <ScanFrame
        active={scanning}
        onDetect={onDetect}
        onError={(error) => {
          setScanning(false)
          setMessage(error)
        }}
      />
      <Button
        variant={scanning ? 'ghost' : 'primary'}
        disabled={disabled}
        onClick={() => setScanning(!scanning)}
        className="max-w-sm"
      >
        {scanning ? 'Stop camera' : 'Start camera'}
      </Button>
      <p aria-live="polite">
        <Label tone={message ? 'ink' : 'dim'}>
          {message || (scanning ? 'Camera on' : 'Camera off')}
        </Label>
      </p>
      <Text className="text-sm">{hint}</Text>
    </>
  )
}

const noPeriod =
  'There is no period running today. Create one in Periods to start taking attendance.'

// Staff: each student's QR holds their id, sent over the socket with the current period
function StudentScanner({ roll }: { roll: Roll }) {
  const { period, students, rows, send } = roll
  const [scanning, setScanning] = useState(false)
  const [message, setMessage] = useState('')
  const sentAt = useRef(new Map<string, number>())

  function onDetect(code: string) {
    if (!period) return
    const student = students.find((s) => s.id === code)
    if (!student) return setMessage('Unknown code, not a student of this school')

    const name = `${student.first_name} ${student.last_name}`
    if (rows.some((row) => row.date === today() && row.studentId === student.id)) {
      return setMessage(`${name} is already on today's roll`)
    }
    if (Date.now() - (sentAt.current.get(student.id) ?? 0) < RETRY_MS) return

    if (!send({ student_id: student.id, period_id: period.id })) {
      return setMessage('Not connected to the server, the scan was not saved')
    }
    sentAt.current.set(student.id, Date.now())
    setMessage(`Sent ${name}`)
  }

  return (
    <Scanner
      disabled={!period}
      {...{ scanning, setScanning, message, setMessage, onDetect }}
      hint={
        period
          ? 'Point the camera at a student code. Each scan is sent with the current period and shows up in the list as soon as it is saved.'
          : noPeriod
      }
    />
  )
}

// Staff: the day's QR, shown on a screen or projector for students to scan
function DailyQr() {
  const [daily, setDaily] = useState<DailyCode | null>(null)
  const [error, setError] = useState('')

  useEffect(() => {
    getPosition()
      .then(getDailyCode)
      .then(setDaily)
      .catch((err) => setError(apiError(err)))
  }, [])

  if (error) return <Text>{error}</Text>
  if (!daily) return <Label>Getting your location…</Label>
  return (
    <>
      <QrCode
        value={daily.code}
        label="Today's attendance QR"
        className="aspect-square w-full max-w-sm border border-line"
      />
      <Label>
        Valid for <Mono>{daily.date}</Mono> (UTC)
      </Label>
      <Text className="text-sm">
        Students open <strong>QR scan</strong> on their phone and scan this code to mark themselves
        present. It only works for phones near this spot, and stops working at midnight UTC.
      </Text>
    </>
  )
}

// Student: scan the daily QR, the API checks the code and puts this account's student on the roll
function CheckInScanner({ roll }: { roll: Roll }) {
  const { period, rows } = roll
  const revalidator = useRevalidator()
  const [scanning, setScanning] = useState(false)
  const [message, setMessage] = useState('')
  const busy = useRef(false)
  // asked for when the camera starts, so it's usually ready by the time a code is read
  const position = useRef<Promise<Spot> | null>(null)
  const checkedIn = rows.some((row) => row.date === today())

  function toggle(on: boolean) {
    if (on) {
      position.current = getPosition()
      position.current.catch(() => {}) // reported by onDetect
    }
    setScanning(on)
  }

  async function onDetect(code: string) {
    if (busy.current || !position.current) return
    busy.current = true
    try {
      await checkIn(code, await position.current)
      setMessage("You're on today's roll")
      revalidator.revalidate()
    } catch (err) {
      setMessage(apiError(err))
    } finally {
      setScanning(false) // one try per press, a wrong code doesn't hammer the API
      busy.current = false
    }
  }

  return (
    <Scanner
      disabled={!period || checkedIn}
      setScanning={toggle}
      {...{ scanning, message, setMessage, onDetect }}
      hint={
        !period
          ? 'There is no period running today.'
          : checkedIn
            ? "You're already on today's roll."
            : 'Point the camera at the QR your teacher shows to mark yourself present. Your location is checked to make sure you are in class.'
      }
    />
  )
}

// Student: their own QR, for staff to scan on the Scan students tab
function MyQr({ roll }: { roll: Roll }) {
  const { me } = roll
  if (!me.student_id) {
    return <Text>This account isn't linked to a student, so it has no QR of its own.</Text>
  }
  return (
    <>
      <QrCode
        value={me.student_id}
        label={`QR of ${me.first_name} ${me.last_name}`}
        className="aspect-square w-full max-w-sm border border-line"
      />
      <Label tone="ink">
        {me.first_name} {me.last_name}
      </Label>
      <Text className="text-sm">
        Show this code to your teacher to be marked present. It's the same one printed on your
        student card.
      </Text>
    </>
  )
}

export default QrScan
