import { useEffect, useLayoutEffect, useRef } from 'react'

import { decodeFromCanvas } from '@/lib/utils'

const corner = 'absolute size-8 border-brass'

type Props = { onDetect: (text: string) => void }

function ScanFrame({ onDetect }: Props) {
  const videoRef = useRef<HTMLVideoElement>(null)
  const canvasRef = useRef<HTMLCanvasElement>(null)

  const onDetectRef = useRef(onDetect)
  useLayoutEffect(() => {
    onDetectRef.current = onDetect
  }, [onDetect])

  useEffect(() => {
    let stream: MediaStream | undefined
    let raf
    let last = 0
    let cancelled = false

    async function start() {
      stream = await navigator.mediaDevices.getUserMedia({
        video: { facingMode: 'environment' }, // rear camera on phones
        audio: false
      })
      if (cancelled) {
        stream.getTracks().forEach((t) => t.stop())
        return
      }
      const video = videoRef.current!
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

    start().catch((err) => console.error('Camera error:', err))

    return () => {
      cancelled = true
      stream?.getTracks().forEach((t) => t.stop())
    }
  }, [])

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
