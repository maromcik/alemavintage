CREATE INDEX IF NOT EXISTS "Model_brand_id_idx" ON "Model" (brand_id);
CREATE INDEX IF NOT EXISTS "Bike_model_id_idx" ON "Bike" (model_id);
CREATE INDEX IF NOT EXISTS "BikeImage_bike_id_idx" ON "BikeImage" (bike_id);