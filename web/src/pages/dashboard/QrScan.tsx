import { PageHeader, RollPanel } from '@components/common'
import { Button, Label, ScanFrame, Text } from '@components/ui'

import { currentPeriod, roll } from '@/data/sample'

function QrScan() {
  return (
    <>
      <PageHeader kicker={`Period ${currentPeriod}`} title="QR scan">
        <Button>Start camera</Button>
      </PageHeader>
      <div className="grid items-start gap-10 lg:grid-cols-[24rem_1fr]">
        <div className="flex flex-col gap-4">
          <ScanFrame onDetect={(code) => console.log('Scanned:', code)} />
          <Label>Camera preview</Label>
          <Text className="text-sm">
            Point the camera at a student code. Each scan is sent with the current period and shows
            up in the list as soon as it is saved.
          </Text>
        </div>
        <RollPanel title="Last scans" rows={roll.filter((entry) => entry.time !== '—')} />
      </div>
    </>
  )
}

export default QrScan
