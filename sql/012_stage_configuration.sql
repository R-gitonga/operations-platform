ALTER TABLE production_stages
ADD COLUMN expected_duration_hours INTEGER;

ALTER TABLE production_stages
ADD COLUMN attention_enabled BOOLEAN
NOT NULL DEFAULT TRUE;