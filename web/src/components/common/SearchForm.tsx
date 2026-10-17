import { Input } from '@components/ui'
import { Form, useSearchParams, useSubmit } from 'react-router'

// GET form: typing puts ?search= in the URL and drops ?page, so results restart at page 1.
function SearchForm({ placeholder }: { placeholder: string }) {
  const [params] = useSearchParams()
  const submit = useSubmit()

  return (
    <Form role="search" onChange={(event) => submit(event.currentTarget, { replace: true })}>
      <Input
        type="search"
        compact
        name="search"
        aria-label={placeholder}
        placeholder={placeholder}
        defaultValue={params.get('search') ?? ''}
        className="max-w-md"
      />
    </Form>
  )
}

export default SearchForm
