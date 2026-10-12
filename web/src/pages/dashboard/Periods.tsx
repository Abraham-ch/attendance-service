import Button from '../../components/atoms/Button'
import Label from '../../components/atoms/Label'
import Mono from '../../components/atoms/Mono'
import DataTable, { type Column } from '../../components/molecules/DataTable'
import PageHeader from '../../components/molecules/PageHeader'
import { periods } from '../../data/sample'

const columns: Column<(typeof periods)[number]>[] = [
  { header: 'Name', cell: (row) => row.name },
  { header: 'Starts', cell: (row) => <Mono>{row.start}</Mono> },
  { header: 'Ends', cell: (row) => <Mono>{row.end}</Mono> },
  {
    header: 'State',
    cell: (row) => <Label tone={row.state === 'Current' ? 'brass' : 'dim'}>{row.state}</Label>
  }
]

function Periods() {
  return (
    <>
      <PageHeader kicker="Records" title="Periods">
        <Button>New period</Button>
      </PageHeader>
      <DataTable columns={columns} rows={periods} rowKey={(row) => row.name} />
    </>
  )
}

export default Periods
