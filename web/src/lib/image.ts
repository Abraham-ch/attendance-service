const AVATAR_PX = 256

// Center-crops the picked photo to a 256px square JPEG before upload: a phone photo is
// several MB, the API caps bodies at 1 MB and an avatar never shows bigger than this
export async function avatarBlob(file: File): Promise<Blob> {
  const bitmap = await createImageBitmap(file)
  const side = Math.min(bitmap.width, bitmap.height)
  const canvas = document.createElement('canvas')
  canvas.width = AVATAR_PX
  canvas.height = AVATAR_PX
  canvas
    .getContext('2d')!
    .drawImage(
      bitmap,
      (bitmap.width - side) / 2,
      (bitmap.height - side) / 2,
      side,
      side,
      0,
      0,
      AVATAR_PX,
      AVATAR_PX
    )
  bitmap.close()

  return new Promise((resolve, reject) =>
    canvas.toBlob(
      (blob) => (blob ? resolve(blob) : reject(new Error("Couldn't read that image"))),
      'image/jpeg',
      0.85
    )
  )
}
