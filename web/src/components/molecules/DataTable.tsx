import type { ReactNode } from 'react'

import Label from '../atoms/Label'

export type Column<T> = { header: string; cell: (row: T) => ReactNode }

type Props<T> = { columns: Column<T>[]; rows: T[]; rowKey: (row: T) => string | number }

function DataTable<T>({ columns, rows, rowKey }: Props<T>) {
  return (
    <div className="overflow-x-auto border border-line bg-surface">
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
  )
}

export default DataTable
