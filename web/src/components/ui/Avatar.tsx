import { useState } from 'react'

type Props = { src?: string | null; name: string; className?: string }

// Falls back to initials when there is no picture or it fails to load
function Avatar({ src, name, className = 'size-10 text-sm' }: Props) {
  const [broken, setBroken] = useState(false)
  const initials = name
    .split(' ')
    .map((word) => word[0])
    .slice(0, 2)
    .join('')
    .toUpperCase()

  if (src && !broken) {
    return (
      <img
        src={src}
        alt=""
        onError={() => setBroken(true)}
        className={`shrink-0 rounded-full object-cover ${className}`}
      />
    )
  }
  return (
    <span
      aria-hidden
      className={`grid shrink-0 place-items-center rounded-full border border-line bg-surface font-semibold text-mute ${className}`}
    >
      {initials}
    </span>
  )
}

export default Avatar
