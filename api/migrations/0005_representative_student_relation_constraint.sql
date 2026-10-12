ALTER TABLE student_representatives
ADD CONSTRAINT fk_students
FOREIGN KEY (student_id)
REFERENCES students(id)
ON DELETE CASCADE;

ALTER TABLE student_representatives
ADD CONSTRAINT fk_representatives
FOREIGN KEY (representative_id)
REFERENCES representatives(id)
ON DELETE CASCADE;
