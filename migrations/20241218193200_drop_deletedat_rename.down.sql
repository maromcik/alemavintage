ALTER TABLE "Bike" RENAME COLUMN HIDDEN TO INCOMPLETE;
ALTER TABLE "Bike" ADD COLUMN deleted_at timestamptz DEFAULT now();