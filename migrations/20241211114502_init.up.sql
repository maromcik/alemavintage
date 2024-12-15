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
    name        text NOT NULL,
    description text NOT NULL
);

CREATE TABLE IF NOT EXISTS "Bike"
(
    id           bigserial PRIMARY KEY,
    ---------------------------------------------
    brand_id     bigserial   NOT NULL,
    model_id     bigserial   NOT NULL,
    name         text        NOT NULL,
    thumbnail    text        NOT NULL,
    description  text        NOT NULL,
    view_count   bigint      NOT NULL DEFAULT 0,
    like_count   bigint      NOT NULL DEFAULT 0,
    created_at   timestamptz NOT NULL DEFAULT now(),
    edited_at    timestamptz NOT NULL DEFAULT now(),
    deleted_at   timestamptz,

    FOREIGN KEY (brand_id) REFERENCES "Brand" (id) ON DELETE CASCADE,
    FOREIGN KEY (model_id) REFERENCES "Model" (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "BikeImage"
(
    id       bigserial PRIMARY KEY,
    ---------------------------------------------
    bike_id  bigserial NOT NULL,
    path     text      NOT NULL,

    FOREIGN KEY (bike_id) REFERENCES "Bike" (id) ON DELETE CASCADE
);

-- FKs on M-to-N
-- CREATE INDEX IF NOT EXISTS "Bookmark_user_id_idx" ON "audiobooks".public."Bookmark" (user_id);
-- CREATE INDEX IF NOT EXISTS "Bookmark_audiobook_id_idx" ON "audiobooks".public."Bookmark" (audiobook_id);
--
-- CREATE INDEX IF NOT EXISTS "Active_Audiobook_user_id_idx" ON "audiobooks".public."Active_Audiobook" (user_id);
-- CREATE INDEX IF NOT EXISTS "Active_Audiobook_audiobook_id_idx" ON "audiobooks".public."Active_Audiobook" (audiobook_id);
--
-- CREATE INDEX IF NOT EXISTS "Audiobook_Author_author_id_idx" ON "audiobooks".public."Audiobook_Author" (author_id);
-- CREATE INDEX IF NOT EXISTS "Audiobook_Author_audiobook_id_idx" ON "audiobooks".public."Audiobook_Author" (audiobook_id);

