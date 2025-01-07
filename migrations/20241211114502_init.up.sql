CREATE TABLE IF NOT EXISTS "User"
(
    id            bigserial PRIMARY KEY,
    ---------------------------------------------
    email         text UNIQUE NOT NULL,
    name          text        NOT NULL,
    surname       text        NOT NULL,
    password_hash text        NOT NULL,
    password_salt text        NOT NULL,
    admin         bool        NOT NULL DEFAULT false
);


CREATE TABLE IF NOT EXISTS "Brand"
(
    id          bigserial PRIMARY KEY,
    ---------------------------------------------
    name        text NOT NULL,
    description text NOT NULL
);

CREATE TABLE IF NOT EXISTS "Model"
(
    id          bigserial PRIMARY KEY,
    ---------------------------------------------
    brand_id    bigint NOT NULL,
    name        text   NOT NULL,
    description text   NOT NULL,

    FOREIGN KEY (brand_id) REFERENCES "Brand" (id) ON DELETE CASCADE

);

CREATE TABLE IF NOT EXISTS "Image"
(
    id             bigserial PRIMARY KEY,
    ---------------------------------------------
    path           text NOT NULL,
    width          int  NOT NULL,
    height         int  NOT NULL,
    thumbnail_path text NOT NULL
);

CREATE TABLE IF NOT EXISTS "Bike"
(
    id              bigserial PRIMARY KEY,
    ---------------------------------------------
    model_id        bigint      NOT NULL,
    name            text        NOT NULL,
    preview         bigint,
    description     text        NOT NULL,
    view_count      bigint      NOT NULL DEFAULT 0,
    like_count      bigint      NOT NULL DEFAULT 0,
    created_at      timestamptz NOT NULL DEFAULT now(),
    edited_at       timestamptz NOT NULL DEFAULT now(),
    hidden          bool        NOT NULL DEFAULT true,
    year            int         NOT NULL,
    price           int         NOT NULL,
    frame           text        NOT NULL,
    seat_tube_sizes text        NOT NULL,
    top_tube_size   int         NOT NULL,
    height          int         NOT NULL,
    headset         text        NOT NULL,
    crankset        text        NOT NULL,
    bottom_bracket  text        NOT NULL,
    front_derail    text        NOT NULL,
    rear_derail     text        NOT NULL,
    brakes          text        NOT NULL,
    shifters        text        NOT NULL,
    brake_levers    text        NOT NULL,
    saddle          text        NOT NULL,
    seat_post       text        NOT NULL,
    hubs            text        NOT NULL,
    rims            text        NOT NULL,
    handlebar       text        NOT NULL,
    stem            text        NOT NULL,
    status          text,

    FOREIGN KEY (model_id) REFERENCES "Model" (id) ON DELETE CASCADE,
    FOREIGN KEY (preview) REFERENCES "Image" (id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS "BikeImage"
(
    image_id bigint NOT NULL,
    bike_id  bigint NOT NULL,

    PRIMARY KEY (bike_id, image_id),
    FOREIGN KEY (bike_id) REFERENCES "Bike" (id) ON DELETE CASCADE,
    FOREIGN KEY (image_id) REFERENCES "Image" (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "OtherImageType"
(
    id   bigserial PRIMARY KEY,
    name text NOT NULL

);

CREATE TABLE IF NOT EXISTS "OtherImage"
(
    image_id   bigint NOT NULL,
    image_type bigint NOT NULL,

    PRIMARY KEY (image_id),
    FOREIGN KEY (image_id) REFERENCES "Image" (id) ON DELETE CASCADE,
    FOREIGN KEY (image_type) REFERENCES "OtherImageType" (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "Tag"
(
    id   bigserial PRIMARY KEY,
    name text NOT NULL
);

CREATE TABLE IF NOT EXISTS "BikeTag"
(
    bike_id bigint,
    tag_id  bigint,
    PRIMARY KEY (bike_id, tag_id),
    FOREIGN KEY (bike_id) REFERENCES "Bike" (id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES "Tag" (id) ON DELETE CASCADE

);

CREATE INDEX IF NOT EXISTS "Model_brand_id_idx" ON "Model" (brand_id);
CREATE INDEX IF NOT EXISTS "Bike_model_id_idx" ON "Bike" (model_id);
CREATE INDEX IF NOT EXISTS "BikeImage_bike_id_idx" ON "BikeImage" (bike_id);
CREATE INDEX IF NOT EXISTS "OtherImage_image_type_idx" ON "OtherImage" (image_type);