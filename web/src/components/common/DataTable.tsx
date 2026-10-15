import { Label } from '@components/ui'
import type { ReactNode } from 'react'

export type Column<T> = { header: string; cell: (row: T) => ReactNode }

type Props<T> = {
  columns: Column<T>[]
  rows: T[]
  rowKey: (row: T) => string | number
  footer?: ReactNode // e.g. <Pagination />, sits inside the table's frame
}

function DataTable<T>({ columns, rows, rowKey, footer }: Props<T>) {
  return (
    <div className="rounded-ui border border-line bg-surface">
      <div className="overflow-x-auto">
        <table className="w-full min-w-xl text-left text-sm">
          <thead>
            <tr className="border-b border-line">
              {columns.map((column) => (
                <th key={column.header} scope="col" className="px-5 py-3 font-normal">
                  <Label>{column.header}</Label>
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            {rows.map((row) => (
              <tr
                key={rowKey(row)}
                className="border-b border-line transition-colors last:border-b-0 hover:bg-paper/60"
              >
                {columns.map((column) => (
                  <td
                    key={column.header}
                    className="px-5 py-3.5 text-mute first:font-medium first:text-ink"
                  >
                    {column.cell(row)}
                  </td>
                ))}
              </tr>
            ))}
          </tbody>
        </table>
      </div>
      {footer && <div className="border-t border-line px-5 py-3">{footer}</div>}
    </div>
  )
}

export default DataTable
