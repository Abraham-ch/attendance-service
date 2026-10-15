import { convert_js_image_to_luma, decode_barcode } from 'rxing-wasm'

export function getToken(): string | null {
  return localStorage.getItem('token')
}

export function decodeFromCanvas(canvas: HTMLCanvasElement): string | null {
  const ctx = canvas.getContext('2d', { willReadFrequently: true })
  if (!ctx) return null

  const { width, height } = canvas
  const img = ctx.getImageData(0, 0, width, height)
  const luma = convert_js_image_to_luma(
    new Uint8Array(img.data.buffer, img.data.byteOffset, img.data.byteLength)
  )

  try {
    return decode_barcode(luma, width, height).text()
  } catch {
    return null // no se encontró código en este frame
  }
}
