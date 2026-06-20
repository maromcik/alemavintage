ALTER TABLE public."Bike" ADD COLUMN IF NOT EXISTS internal_id varchar(99) UNIQUE NOT NULL;
