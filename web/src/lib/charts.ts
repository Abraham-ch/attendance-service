import type { ChartOption } from '@components/common'
import type { Status } from '@components/ui'

const statuses: Status[] = ['Present', 'Late', 'Absent', 'Excused']

// Same hues as StatusDot, stepped so neighbours in a stack stay apart with color blindness
// too (checked with the dataviz palette validator against bg-surface)
const statusColors: Record<Status, string> = {
  Present: '#2b7a4b',
  Late: '#d6962a',
  Absent: '#9c3426',
  Excused: '#4778b8'
}

// the @theme colors in index.css, so the chart chrome follows the rest of the UI
const token = (name: string) =>
  getComputedStyle(document.documentElement).getPropertyValue(`--color-${name}`).trim()

type Category = { key: string; label: string }

// One stacked bar per category (a day, a period), one segment per status
export function statusBars(categories: Category[], records: { key: string; status: Status }[]) {
  const counts = new Map(
    categories.map((c) => [c.key, { Present: 0, Late: 0, Absent: 0, Excused: 0 }])
  )
  for (const record of records) {
    const count = counts.get(record.key)
    if (count) count[record.status]++
  }
  const count = (category: Category, status: Status) => counts.get(category.key)![status]
  // only the top segment of each stack gets the rounded end
  const top = categories.map((c) => statuses.findLast((status) => count(c, status) > 0))

  const surface = token('surface')
  const axisText = { color: token('dim'), fontFamily: 'Inter, sans-serif' }

  return {
    aria: { enabled: true },
    textStyle: { fontFamily: 'Inter, sans-serif' },
    grid: { left: 0, right: 0, top: 36, bottom: 0, containLabel: true },
    legend: {
      top: 0,
      left: 0,
      icon: 'roundRect',
      itemWidth: 10,
      itemHeight: 10,
      textStyle: { color: token('mute') }
    },
    tooltip: {
      trigger: 'axis',
      axisPointer: { type: 'shadow', shadowStyle: { color: token('line') } },
      valueFormatter: (value: unknown) => String(value ?? 0)
    },
    xAxis: {
      type: 'category',
      data: categories.map((c) => c.label),
      axisTick: { show: false },
      axisLine: { lineStyle: { color: token('line-lit') } },
      axisLabel: axisText
    },
    yAxis: {
      type: 'value',
      minInterval: 1,
      splitLine: { lineStyle: { color: token('line') } },
      axisLabel: axisText
    },
    series: statuses.map((status) => ({
      name: status,
      type: 'bar',
      stack: 'total',
      barMaxWidth: 24,
      // the surface-colored border is the 2px gap between touching segments and bars
      itemStyle: { color: statusColors[status], borderColor: surface, borderWidth: 1 },
      data: categories.map((c, i) => ({
        value: count(c, status) || null, // a 0 would still draw its border as a line
        itemStyle: top[i] === status ? { borderRadius: [4, 4, 0, 0] } : undefined
      }))
    }))
  } satisfies ChartOption
}
