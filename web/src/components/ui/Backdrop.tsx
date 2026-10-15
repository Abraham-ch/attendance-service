function Backdrop() {
  return (
    <div
      aria-hidden
      className="pointer-events-none absolute inset-0 -z-10 overflow-hidden mask-b-from-60%"
    >
      <div className="absolute inset-0 bg-blueprint" />
      <div className="absolute inset-[-10%] bg-glow motion-safe:animate-drift" />
    </div>
  )
}

export default Backdrop
