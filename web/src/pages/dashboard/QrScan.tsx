import Button from '../../components/atoms/Button'
import Label from '../../components/atoms/Label'
import ScanFrame from '../../components/atoms/ScanFrame'
import Text from '../../components/atoms/Text'
import PageHeader from '../../components/molecules/PageHeader'
import RollPanel from '../../components/organisms/RollPanel'
import { currentPeriod, roll } from '../../data/sample'

function QrScan() {
  return (
    <>
      <PageHeader kicker={`Period ${currentPeriod}`} title="QR scan">
        <Button>Start camera</Button>
      </PageHeader>
      <div className="grid items-start gap-10 lg:grid-cols-[24rem_1fr]">
        <div className="flex flex-col gap-4">
          <ScanFrame />
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
