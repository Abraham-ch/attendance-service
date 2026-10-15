import { PageHeader, RollPanel } from '@components/common'
import { Button, Label, ScanFrame, Text } from '@components/ui'
import { today, useLiveRoll } from '@lib/roll'
import { useRef, useState } from 'react'

const RETRY_MS = 5000 // the camera reads the same code several times per second

function QrScan() {
  const { period, students, rows, connected, send } = useLiveRoll()
  const [scanning, setScanning] = useState(false)
  const [message, setMessage] = useState('')
  const sentAt = useRef(new Map<string, number>())

  const todayRows = rows.filter((row) => row.date === today())

  function onDetect(code: string) {
    if (!period) return
    const student = students.find((s) => s.id === code)
    if (!student) return setMessage('Unknown code, not a student of this school')

    const name = `${student.first_name} ${student.last_name}`
    if (todayRows.some((row) => row.studentId === student.id)) {
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
    <>
      <PageHeader kicker={period ? `Period ${period.name}` : 'No current period'} title="QR scan">
        <Button
          variant={scanning ? 'ghost' : 'primary'}
          disabled={!period}
          onClick={() => setScanning((on) => !on)}
        >
          {scanning ? 'Stop camera' : 'Start camera'}
        </Button>
      </PageHeader>
      <div className="grid items-start gap-10 lg:grid-cols-[24rem_1fr]">
        <div className="flex flex-col gap-4">
          <ScanFrame active={scanning} onDetect={onDetect} />
          <p aria-live="polite">
            <Label tone={message ? 'ink' : 'dim'}>
              {message || (scanning ? 'Camera on' : 'Camera off')}
            </Label>
          </p>
          <Text className="text-sm">
            {period
              ? 'Point the camera at a student code. Each scan is sent with the current period and shows up in the list as soon as it is saved.'
              : 'There is no period running today. Create one in Periods to start taking attendance.'}
          </Text>
        </div>
        <RollPanel title="Today's scans" rows={todayRows} live={connected} />
      </div>
    </>
  )
}

export default QrScan
