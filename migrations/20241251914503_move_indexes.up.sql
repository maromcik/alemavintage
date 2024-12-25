BEGIN;
LOCK TABLE "Brand" IN EXCLUSIVE MODE;
SELECT setval('"Brand_id_seq"', COALESCE((SELECT MAX(id)+1 FROM "Brand"), 1), false);
COMMIT;

BEGIN;
LOCK TABLE "User" IN EXCLUSIVE MODE;
SELECT setval('"User_id_seq"', COALESCE((SELECT MAX(id)+1 FROM "User"), 1), false);
COMMIT;

BEGIN;
LOCK TABLE "Model" IN EXCLUSIVE MODE;
SELECT setval('"Model_id_seq"', COALESCE((SELECT MAX(id)+1 FROM "Model"), 1), false);
COMMIT;

BEGIN;
LOCK TABLE "Bike" IN EXCLUSIVE MODE;
SELECT setval('"Bike_id_seq"', COALESCE((SELECT MAX(id)+1 FROM "Bike"), 1), false);
COMMIT;