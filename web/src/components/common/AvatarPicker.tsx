import { Avatar, Label } from '@components/ui'
import { apiError } from '@lib/api'
import { avatarBlob } from '@lib/image'
import { useEffect, useState } from 'react'

type Props = {
  name: string // for the initials shown until a picture is picked
  current?: string | null // the saved picture, when editing
  label?: string
  onChange: (avatar: Blob) => void
  onError: (message: string) => void
  className?: string
}

// Optional picture field: crops the photo to the 256px JPEG the API takes and previews it.
// The form uploads it on submit, so a picture changed twice isn't stored twice.
function AvatarPicker({
  name,
  current,
  label = 'Avatar (optional)',
  onChange,
  onError,
  className = ''
}: Props) {
  const [preview, setPreview] = useState('')

  // the object URL holds the image in memory until it's revoked
  useEffect(() => () => URL.revokeObjectURL(preview), [preview])

  async function pick(file: File | undefined) {
    if (!file) return
    try {
      const blob = await avatarBlob(file)
      onChange(blob)
      setPreview(URL.createObjectURL(blob))
    } catch (err) {
      onError(apiError(err))
    }
  }

  return (
    <div className={`flex items-center gap-4 ${className}`}>
      <Avatar
        key={preview || current}
        src={preview || current}
        name={name || '?'}
        className="size-14 text-base"
      />
      <label className="flex min-w-0 cursor-pointer flex-col gap-1.5">
        <Label>{label}</Label>
        <input
          type="file"
          accept="image/*"
          onChange={(event) => pick(event.target.files?.[0])}
          className="text-sm text-mute file:mr-3 file:cursor-pointer file:rounded-ui file:border file:border-line-lit file:bg-transparent file:px-3 file:py-1.5 file:text-ink hover:file:border-ink"
        />
      </label>
    </div>
  )
}

export default AvatarPicker
