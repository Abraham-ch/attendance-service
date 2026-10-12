CREATE TYPE status AS ENUM ('active', 'pending', 'inactive');

ALTER TABLE users
  ADD student_id UUID REFERENCES students(id) ON DELETE SET NULL,
  ADD status status NOT NULL DEFAULT 'pending';

ALTER TABLE students
  ADD email VARCHAR(64) NULL;

CREATE TABLE invite_tokens (
  id UUID PRIMARY KEY,
  student_id UUID REFERENCES students(id) ON DELETE CASCADE NOT NULL,
  token VARCHAR(64) NOT NULL UNIQUE,
  expires_at TIMESTAMPTZ NOT NULL,
  used_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
