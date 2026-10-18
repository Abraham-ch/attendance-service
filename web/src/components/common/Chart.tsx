import { BarChart } from 'echarts/charts'
import { AriaComponent, GridComponent, LegendComponent, TooltipComponent } from 'echarts/components'
import * as echarts from 'echarts/core'
import { SVGRenderer } from 'echarts/renderers'
import { useEffect, useRef } from 'react'

// only what the charts use, the full echarts build is ~1MB
echarts.use([
  BarChart,
  AriaComponent,
  GridComponent,
  LegendComponent,
  TooltipComponent,
  SVGRenderer
])

export type ChartOption = echarts.EChartsCoreOption

type Props = { option: ChartOption; label: string; className?: string }

function Chart({ option, label, className = 'h-72' }: Props) {
  const el = useRef<HTMLDivElement>(null)
  const chart = useRef<echarts.ECharts | null>(null)

  useEffect(() => {
    const instance = echarts.init(el.current, null, { renderer: 'svg' })
    chart.current = instance
    const observer = new ResizeObserver(() => instance.resize())
    observer.observe(el.current!)
    return () => {
      observer.disconnect()
      instance.dispose()
    }
  }, [])

  useEffect(() => {
    chart.current?.setOption(option, true)
  }, [option])

  return <div ref={el} role="img" aria-label={label} className={className} />
}

export default Chart
