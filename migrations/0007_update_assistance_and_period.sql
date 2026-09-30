CREATE TRIGGER assistances_updated_at
BEFORE UPDATE ON assistances
FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

CREATE TABLE periods (
    id UUID PRIMARY KEY,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL
);

ALTER TABLE assistances
    ADD COLUMN period_id UUID REFERENCES periods(id);
