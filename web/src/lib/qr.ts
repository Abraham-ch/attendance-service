import { BarcodeFormat, encode_barcode } from 'rxing-wasm'

// rxing draws the code as text, two characters per module: 'X ' dark, '  ' light,
// quiet zone included. Turned into one SVG path of 1x1 squares.
export function qrPath(text: string) {
  const rows = encode_barcode(text, 1, 1, BarcodeFormat.QrCode)
    .split('\n')
    .filter((row) => row.length > 0)
  let d = ''
  rows.forEach((row, y) => {
    for (let x = 0; x * 2 < row.length; x++) {
      if (row[x * 2] === 'X') d += `M${x} ${y}h1v1h-1z`
    }
  })
  return { size: rows.length, d }
}
