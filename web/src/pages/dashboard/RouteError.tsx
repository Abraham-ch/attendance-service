import { PageHeader } from '@components/common'
import { Text } from '@components/ui'
import { apiError } from '@lib/api'
import { useRouteError } from 'react-router'

// Shown inside the dashboard layout when a page's loader fails, e.g. the API is down
function RouteError() {
  const error = useRouteError()
  const message = apiError(error)

  return (
    <>
      <PageHeader kicker="Error" title="Couldn't load this page" />
      <Text>{message}</Text>
    </>
  )
}

export default RouteError
