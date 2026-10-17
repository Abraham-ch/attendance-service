-- Where the daily check-in QR was shown, one row each time an admin generates it.
-- The QR itself carries the location for the check-in (see utils::daily_code), this keeps
-- the record of where and by whom class was taken.
CREATE TABLE daily_code_locations (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    date DATE NOT NULL,
    lat DOUBLE PRECISION NOT NULL CHECK (lat BETWEEN -90 AND 90),
    lng DOUBLE PRECISION NOT NULL CHECK (lng BETWEEN -180 AND 180),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX daily_code_locations_date_idx ON daily_code_locations (date);

-- Same as every other table (0011): not readable through Supabase's public API
ALTER TABLE daily_code_locations ENABLE ROW LEVEL SECURITY;
