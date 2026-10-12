const corner = 'absolute size-8 border-brass'

function ScanFrame() {
  return (
    <div
      aria-hidden
      className="relative aspect-square w-full max-w-sm border border-line bg-surface"
    >
      <span className={`${corner} top-4 left-4 border-t-2 border-l-2`} />
      <span className={`${corner} top-4 right-4 border-t-2 border-r-2`} />
      <span className={`${corner} bottom-4 left-4 border-b-2 border-l-2`} />
      <span className={`${corner} right-4 bottom-4 border-r-2 border-b-2`} />
      <span className="absolute inset-x-8 top-1/2 h-px bg-brass motion-safe:animate-scan" />
    </div>
  )
}

export default ScanFrame
