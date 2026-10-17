import { qrPath } from '@lib/qr'

type Props = { value: string; label: string; className?: string }

// Always black on white, whatever the theme: scanners need the contrast
function QrCode({ value, label, className = '' }: Props) {
  const { size, d } = qrPath(value)
  return (
    <svg
      role="img"
      aria-label={label}
      viewBox={`0 0 ${size} ${size}`}
      shapeRendering="crispEdges"
      className={className}
    >
      <rect width={size} height={size} fill="#fff" />
      <path d={d} fill="#000" />
    </svg>
  )
}

export default QrCode
