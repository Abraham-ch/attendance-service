import { useEffect, useLayoutEffect, useRef } from 'react'

import { decodeFromCanvas } from '@/lib/utils'

const corner = 'absolute size-8 border-brass'

type Props = {
  active: boolean
  onDetect: (text: string) => void
  onError: (message: string) => void
}

// Browsers only expose the camera on HTTPS or localhost, so on http://<lan-ip> there is
// no permission prompt at all: navigator.mediaDevices is just undefined
function cameraError(err: unknown) {
  if (!window.isSecureContext)
    return 'The camera needs HTTPS, open the app over https:// (or localhost)'
  const name = err instanceof DOMException ? err.name : ''
  if (name === 'NotAllowedError')
    return 'Camera access was denied, allow it in the browser site settings'
  if (name === 'NotFoundError' || name === 'OverconstrainedError')
    return 'No camera found on this device'
  if (name === 'NotReadableError') return 'The camera is in use by another app'
  return `Could not start the camera: ${String(err)}`
}

function ScanFrame({ active, onDetect, onError }: Props) {
  const videoRef = useRef<HTMLVideoElement>(null)
  const canvasRef = useRef<HTMLCanvasElement>(null)

  const onDetectRef = useRef(onDetect)
  const onErrorRef = useRef(onError)
  useLayoutEffect(() => {
    onDetectRef.current = onDetect
    onErrorRef.current = onError
  }, [onDetect, onError])

  useEffect(() => {
    const video = videoRef.current
    if (!active || !video) return
    let stream: MediaStream | undefined
    let raf = 0
    let last = 0
    let cancelled = false

    const start = async () => {
      stream = await navigator.mediaDevices.getUserMedia({
        video: { facingMode: 'environment' }, // rear camera on phones
        audio: false
      })
      if (cancelled) {
        stream.getTracks().forEach((t) => t.stop())
        return
      }
      video.srcObject = stream
      await video.play()
      raf = requestAnimationFrame(tick)
    }

    function tick(now: number) {
      raf = requestAnimationFrame(tick)
      if (now - last < 150) return // decode ~6 times per second, not every frame
      last = now

      const video = videoRef.current
      const canvas = canvasRef.current
      if (!video || !canvas || video.readyState < 2) return

      // Center square crop, matching what the frame shows
      const size = Math.min(video.videoWidth, video.videoHeight)
      if (canvas.width !== size) {
        canvas.width = size
        canvas.height = size
      }
      const ctx = canvas.getContext('2d', { willReadFrequently: true })!
      ctx.drawImage(
        video,
        (video.videoWidth - size) / 2,
        (video.videoHeight - size) / 2,
        size,
        size,
        0,
        0,
        size,
        size
      )

      const text = decodeFromCanvas(canvas)
      if (text) {
        onDetectRef.current?.(text)
      }
    }

    start().catch((err) => {
      if (!cancelled) onErrorRef.current(cameraError(err))
    })

    return () => {
      cancelled = true
      cancelAnimationFrame(raf)
      stream?.getTracks().forEach((t) => t.stop())
      video.srcObject = null // don't freeze on the last frame
    }
  }, [active])

  return (
    <div
      aria-hidden
      className="relative aspect-square w-full max-w-sm border border-line bg-surface"
    >
      <video
        ref={videoRef}
        muted
        playsInline
        className="absolute inset-0 h-full w-full object-cover"
      />
      <canvas ref={canvasRef} className="hidden" />
      <span className={`${corner} top-4 left-4 border-t-2 border-l-2`} />
      <span className={`${corner} top-4 right-4 border-t-2 border-r-2`} />
      <span className={`${corner} bottom-4 left-4 border-b-2 border-l-2`} />
      <span className={`${corner} right-4 bottom-4 border-r-2 border-b-2`} />
    </div>
  )
}

export default ScanFrame
