import { Modal } from '@components/common'
import { Avatar, Button, QrCode } from '@components/ui'

import type { Student } from '@/types/records'

type Props = { student: Student; onClose: () => void }

// Printable student card, credit-card sized (CR80, 85.6 x 54 mm). Sizes are in cqw so the
// card looks the same in the modal and printed (index.css sets the print width).
// "Save as PDF" in the print dialog gives the file.
function IdCardModal({ student, onClose }: Props) {
  const name = `${student.first_name} ${student.last_name}`

  return (
    <Modal title="Student card" onClose={onClose}>
      <div className="flex flex-col gap-5">
        <article className="id-card @container aspect-[85.6/54] w-full overflow-hidden rounded-[3cqw] border border-line-lit bg-white text-[#14161a]">
          <div className="flex h-full gap-[4cqw] p-[5cqw]">
            <div className="flex min-w-0 flex-1 flex-col">
              <span className="text-[3.2cqw] font-semibold tracking-[0.18em] text-[#8f5408] uppercase">
                Attendance Service
              </span>
              <Avatar
                src={student.avatar}
                name={name}
                className="mt-[4cqw] size-[19cqw] text-[6cqw]"
              />
              <div className="mt-auto flex flex-col gap-[1cqw]">
                <span className="truncate text-[6cqw] leading-tight font-extrabold">
                  {student.first_name}
                </span>
                <span className="truncate text-[4.6cqw] leading-tight">{student.last_name}</span>
                <span className="font-monospace text-[3.4cqw] text-[#4a5058]">
                  DNI {student.dni}
                </span>
              </div>
            </div>
            <QrCode
              value={student.id}
              label={`QR of ${name}`}
              className="aspect-square h-full shrink-0"
            />
          </div>
        </article>
        <div className="flex justify-end gap-2">
          <Button variant="ghost" onClick={onClose}>
            Close
          </Button>
          <Button onClick={() => window.print()}>Print or save as PDF</Button>
        </div>
      </div>
    </Modal>
  )
}

export default IdCardModal
