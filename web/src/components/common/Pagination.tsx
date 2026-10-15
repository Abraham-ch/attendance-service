import { Label } from '@components/ui'
import { Link, useSearchParams } from 'react-router'

type Props = { page: number; perPage: number; total: number }

const MAX_PER_PAGE = 100

const linkClass =
  'rounded-ui border border-line-lit px-3 py-1.5 font-ui text-label uppercase transition-colors hover:border-ink'

// Page lives in the URL (?page=2&per_page=10) so the page's loader refetches and links can be shared.
// Meant for DataTable's footer.
function Pagination({ page, perPage, total }: Props) {
  const [params, setParams] = useSearchParams()
  const pages = Math.max(1, Math.ceil(total / perPage))
  const first = total === 0 ? 0 : (page - 1) * perPage + 1
  const last = Math.min(page * perPage, total)

  const to = (target: number) => {
    const next = new URLSearchParams(params)
    next.set('page', String(target))
    return `?${next}`
  }

  // "1–[10] of 40": the end of the range is editable, Enter shows rows 1 to N
  function setPerPage(input: HTMLInputElement) {
    const size = Math.round(Number(input.value))
    if (!Number.isFinite(size) || size < 1) {
      input.value = String(last) // nonsense typed, put the range back
      return
    }
    const next = new URLSearchParams(params)
    next.set('per_page', String(Math.min(size, MAX_PER_PAGE)))
    next.delete('page')
    setParams(next)
  }

  return (
    <nav aria-label="Pagination" className="flex flex-wrap items-center justify-between gap-3">
      <span className="flex items-center gap-1.5 font-ui text-sm text-mute tabular-nums">
        {first}–
        <input
          key={`${page}-${perPage}-${total}`} // reset the typed value after every navigation
          type="number"
          min={1}
          max={MAX_PER_PAGE}
          defaultValue={last}
          aria-label="Rows per page, press Enter to apply"
          title="Rows per page, press Enter to apply"
          className="w-12 rounded-ui border border-line-lit bg-paper px-1.5 py-0.5 text-center text-ink [appearance:textfield] focus:border-ink [&::-webkit-inner-spin-button]:appearance-none"
          onKeyDown={(event) => {
            if (event.key === 'Enter') setPerPage(event.currentTarget)
          }}
          onBlur={(event) => {
            event.currentTarget.value = String(last) // only Enter applies
          }}
        />
        of {total}
      </span>
      <div className="flex items-center gap-3">
        {page > 1 ? (
          <Link className={linkClass} to={to(page - 1)}>
            Previous
          </Link>
        ) : (
          <span aria-disabled className={`${linkClass} opacity-40`}>
            Previous
          </span>
        )}
        <Label tone="ink">
          Page {page} of {pages}
        </Label>
        {page < pages ? (
          <Link className={linkClass} to={to(page + 1)}>
            Next
          </Link>
        ) : (
          <span aria-disabled className={`${linkClass} opacity-40`}>
            Next
          </span>
        )}
      </div>
    </nav>
  )
}

export default Pagination
