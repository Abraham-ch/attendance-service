import Button from '../../components/atoms/Button'
import Label from '../../components/atoms/Label'
import Mono from '../../components/atoms/Mono'
import DataTable, { type Column } from '../../components/molecules/DataTable'
import PageHeader from '../../components/molecules/PageHeader'
import { representatives } from '../../data/sample'

const columns: Column<(typeof representatives)[number]>[] = [
  { header: 'Name', cell: (row) => row.name },
  { header: 'Student', cell: (row) => row.student },
  { header: 'Relationship', cell: (row) => row.relationship },
  { header: 'Phones', cell: (row) => <Mono>{row.phones}</Mono> },
  { header: 'Contact', cell: (row) => (row.primary ? <Label tone="brass">Primary</Label> : '—') }
]

function Representatives() {
  return (
    <>
      <PageHeader kicker="Records" title="Representatives">
        <Button>New representative</Button>
      </PageHeader>
      <DataTable columns={columns} rows={representatives} rowKey={(row) => row.name} />
    </>
  )
}

export default Representatives
