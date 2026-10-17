-- A student gets one attendance record per day, enforced by the database so two scans
-- in the same instant (two staff phones, a double tap) can't both be saved.
--
-- The constraint can't be created while duplicates exist, so older double scans are
-- removed first, keeping each student's earliest record of the day (the real arrival time).
-- Preview what this deletes before running it:
--   SELECT student_id, date, COUNT(*) FROM assistances
--   GROUP BY student_id, date HAVING COUNT(*) > 1;
DELETE FROM assistances a
USING assistances b
WHERE a.student_id = b.student_id
  AND a.date = b.date
  AND (a.created_at, a.id) > (b.created_at, b.id);

ALTER TABLE assistances
ADD CONSTRAINT assistances_student_date_key UNIQUE (student_id, date);
