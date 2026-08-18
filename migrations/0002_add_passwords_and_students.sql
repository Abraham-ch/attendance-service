CREATE TYPE assistance_status AS ENUM ('present', 'absent', 'late', 'excused');

ALTER TABLE users
ADD password VARCHAR(2000) NOT NULL;

CREATE TABLE students (
  id UUID PRIMARY KEY,
  dni BIGINT NOT NULL UNIQUE,
  first_name VARCHAR(50) NOT NULL,
  last_name VARCHAR(64) NOT NULL,
  gender VARCHAR(10) NOT NULL,
  phone BIGINT,
  address TEXT,

  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE representatives (
  id UUID PRIMARY KEY,
  first_name VARCHAR(50) NOT NULL,
  last_name VARCHAR(64) NOT NULL,
  phone JSONB NOT NULL,

  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE student_representatives (
    student_id UUID NOT NULL REFERENCES students(id),
    representative_id UUID NOT NULL REFERENCES representatives(id),
    relationship TEXT NOT NULL, -- 'mother', 'father', 'guardian'
    is_primary BOOLEAN NOT NULL DEFAULT false,

    PRIMARY KEY (student_id, representative_id)
);

CREATE TABLE assistances (
  id UUID PRIMARY KEY,
  student_id UUID NOT NULL REFERENCES students(id),
  date DATE NOT NULL,
  status assistance_status NOT NULL, -- 'present', 'absent', 'late', etc.
  notes TEXT,

  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER students_updated_at
BEFORE UPDATE ON students
FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

CREATE TRIGGER representatives_updated_at
BEFORE UPDATE ON representatives
FOR EACH ROW
EXECUTE FUNCTION update_updated_at();