import { decodeFromCanvas } from './utils'

const video = document.querySelector('video')!
const canvas = document.getElementById('myCanvas') as HTMLCanvasElement
canvas.width = 640
canvas.height = 640

function scan() {
  const ctx = canvas.getContext('2d', { willReadFrequently: true })!
  ctx.drawImage(video, 0, 0, canvas.width, canvas.height) // primero dibujar el frame
  const text = decodeFromCanvas(canvas)
  if (text) {
    console.log('Código:', text)
    return // detener al encontrar uno
  }
  requestAnimationFrame(scan)
}
video.addEventListener('playing', () => requestAnimationFrame(scan))
