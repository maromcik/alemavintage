--
-- PostgreSQL database dump
--

-- Dumped from database version 15.7 (Debian 15.7-1.pgdg120+1)
-- Dumped by pg_dump version 16.3

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

ALTER TABLE IF EXISTS ONLY public."Model" DROP CONSTRAINT IF EXISTS "Model_brand_id_fkey";
ALTER TABLE IF EXISTS ONLY public."Bike" DROP CONSTRAINT IF EXISTS "Bike_model_id_fkey";
ALTER TABLE IF EXISTS ONLY public."BikeTag" DROP CONSTRAINT IF EXISTS "BikeTag_tag_id_fkey";
ALTER TABLE IF EXISTS ONLY public."BikeTag" DROP CONSTRAINT IF EXISTS "BikeTag_bike_id_fkey";
ALTER TABLE IF EXISTS ONLY public."BikeImage" DROP CONSTRAINT IF EXISTS "BikeImage_bike_id_fkey";
DROP INDEX IF EXISTS public."Model_brand_id_idx";
DROP INDEX IF EXISTS public."Bike_model_id_idx";
DROP INDEX IF EXISTS public."BikeImage_bike_id_idx";
ALTER TABLE IF EXISTS ONLY public._sqlx_migrations DROP CONSTRAINT IF EXISTS _sqlx_migrations_pkey;
ALTER TABLE IF EXISTS ONLY public."User" DROP CONSTRAINT IF EXISTS "User_pkey";
ALTER TABLE IF EXISTS ONLY public."User" DROP CONSTRAINT IF EXISTS "User_email_key";
ALTER TABLE IF EXISTS ONLY public."Tag" DROP CONSTRAINT IF EXISTS "Tag_pkey";
ALTER TABLE IF EXISTS ONLY public."Model" DROP CONSTRAINT IF EXISTS "Model_pkey";
ALTER TABLE IF EXISTS ONLY public."Brand" DROP CONSTRAINT IF EXISTS "Brand_pkey";
ALTER TABLE IF EXISTS ONLY public."Bike" DROP CONSTRAINT IF EXISTS "Bike_pkey";
ALTER TABLE IF EXISTS ONLY public."BikeTag" DROP CONSTRAINT IF EXISTS "BikeTag_pkey";
ALTER TABLE IF EXISTS ONLY public."BikeImage" DROP CONSTRAINT IF EXISTS "BikeImage_pkey";
ALTER TABLE IF EXISTS public."User" ALTER COLUMN id DROP DEFAULT;
ALTER TABLE IF EXISTS public."Tag" ALTER COLUMN id DROP DEFAULT;
ALTER TABLE IF EXISTS public."Model" ALTER COLUMN brand_id DROP DEFAULT;
ALTER TABLE IF EXISTS public."Model" ALTER COLUMN id DROP DEFAULT;
ALTER TABLE IF EXISTS public."Brand" ALTER COLUMN id DROP DEFAULT;
ALTER TABLE IF EXISTS public."BikeTag" ALTER COLUMN tag_id DROP DEFAULT;
ALTER TABLE IF EXISTS public."BikeTag" ALTER COLUMN bike_id DROP DEFAULT;
ALTER TABLE IF EXISTS public."BikeImage" ALTER COLUMN bike_id DROP DEFAULT;
ALTER TABLE IF EXISTS public."BikeImage" ALTER COLUMN id DROP DEFAULT;
ALTER TABLE IF EXISTS public."Bike" ALTER COLUMN model_id DROP DEFAULT;
ALTER TABLE IF EXISTS public."Bike" ALTER COLUMN id DROP DEFAULT;
DROP TABLE IF EXISTS public._sqlx_migrations;
DROP SEQUENCE IF EXISTS public."User_id_seq";
DROP TABLE IF EXISTS public."User";
DROP SEQUENCE IF EXISTS public."Tag_id_seq";
DROP TABLE IF EXISTS public."Tag";
DROP SEQUENCE IF EXISTS public."Model_id_seq";
DROP SEQUENCE IF EXISTS public."Model_brand_id_seq";
DROP TABLE IF EXISTS public."Model";
DROP SEQUENCE IF EXISTS public."Brand_id_seq";
DROP TABLE IF EXISTS public."Brand";
DROP SEQUENCE IF EXISTS public."Bike_model_id_seq";
DROP SEQUENCE IF EXISTS public."Bike_id_seq";
DROP SEQUENCE IF EXISTS public."BikeTag_tag_id_seq";
DROP SEQUENCE IF EXISTS public."BikeTag_bike_id_seq";
DROP TABLE IF EXISTS public."BikeTag";
DROP SEQUENCE IF EXISTS public."BikeImage_id_seq";
DROP SEQUENCE IF EXISTS public."BikeImage_bike_id_seq";
DROP TABLE IF EXISTS public."BikeImage";
DROP TABLE IF EXISTS public."Bike";
SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: Bike; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public."Bike" (
    id bigint NOT NULL,
    model_id bigint NOT NULL,
    name text NOT NULL,
    thumbnail text NOT NULL,
    description text NOT NULL,
    view_count bigint DEFAULT 0 NOT NULL,
    like_count bigint DEFAULT 0 NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    edited_at timestamp with time zone DEFAULT now() NOT NULL,
    hidden boolean DEFAULT true NOT NULL,
    year integer NOT NULL,
    price integer NOT NULL,
    frame text NOT NULL,
    seat_tube_sizes text NOT NULL,
    top_tube_size integer NOT NULL,
    height integer NOT NULL,
    headset text NOT NULL,
    crankset text NOT NULL,
    bottom_bracket text NOT NULL,
    front_derail text NOT NULL,
    rear_derail text NOT NULL,
    brakes text NOT NULL,
    shifters text NOT NULL,
    brake_levers text NOT NULL,
    saddle text NOT NULL,
    seat_post text NOT NULL,
    hubs text NOT NULL,
    rims text NOT NULL,
    handlebar text NOT NULL,
    stem text NOT NULL,
    status text
);


ALTER TABLE public."Bike" OWNER TO postgres;

--
-- Name: BikeImage; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public."BikeImage" (
    id bigint NOT NULL,
    bike_id bigint NOT NULL,
    path text NOT NULL,
    width integer NOT NULL,
    height integer NOT NULL,
    thumbnail_path text NOT NULL
);


ALTER TABLE public."BikeImage" OWNER TO postgres;

--
-- Name: BikeImage_bike_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public."BikeImage_bike_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public."BikeImage_bike_id_seq" OWNER TO postgres;

--
-- Name: BikeImage_bike_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public."BikeImage_bike_id_seq" OWNED BY public."BikeImage".bike_id;


--
-- Name: BikeImage_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public."BikeImage_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public."BikeImage_id_seq" OWNER TO postgres;

--
-- Name: BikeImage_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public."BikeImage_id_seq" OWNED BY public."BikeImage".id;


--
-- Name: BikeTag; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public."BikeTag" (
    bike_id bigint NOT NULL,
    tag_id bigint NOT NULL
);


ALTER TABLE public."BikeTag" OWNER TO postgres;

--
-- Name: BikeTag_bike_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public."BikeTag_bike_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public."BikeTag_bike_id_seq" OWNER TO postgres;

--
-- Name: BikeTag_bike_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public."BikeTag_bike_id_seq" OWNED BY public."BikeTag".bike_id;


--
-- Name: BikeTag_tag_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public."BikeTag_tag_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public."BikeTag_tag_id_seq" OWNER TO postgres;

--
-- Name: BikeTag_tag_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public."BikeTag_tag_id_seq" OWNED BY public."BikeTag".tag_id;


--
-- Name: Bike_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public."Bike_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public."Bike_id_seq" OWNER TO postgres;

--
-- Name: Bike_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public."Bike_id_seq" OWNED BY public."Bike".id;


--
-- Name: Bike_model_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public."Bike_model_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public."Bike_model_id_seq" OWNER TO postgres;

--
-- Name: Bike_model_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public."Bike_model_id_seq" OWNED BY public."Bike".model_id;


--
-- Name: Brand; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public."Brand" (
    id bigint NOT NULL,
    name text NOT NULL,
    description text NOT NULL
);


ALTER TABLE public."Brand" OWNER TO postgres;

--
-- Name: Brand_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public."Brand_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public."Brand_id_seq" OWNER TO postgres;

--
-- Name: Brand_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public."Brand_id_seq" OWNED BY public."Brand".id;


--
-- Name: Model; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public."Model" (
    id bigint NOT NULL,
    brand_id bigint NOT NULL,
    name text NOT NULL,
    description text NOT NULL
);


ALTER TABLE public."Model" OWNER TO postgres;

--
-- Name: Model_brand_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public."Model_brand_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public."Model_brand_id_seq" OWNER TO postgres;

--
-- Name: Model_brand_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public."Model_brand_id_seq" OWNED BY public."Model".brand_id;


--
-- Name: Model_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public."Model_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public."Model_id_seq" OWNER TO postgres;

--
-- Name: Model_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public."Model_id_seq" OWNED BY public."Model".id;


--
-- Name: Tag; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public."Tag" (
    id bigint NOT NULL,
    tag text NOT NULL
);


ALTER TABLE public."Tag" OWNER TO postgres;

--
-- Name: Tag_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public."Tag_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public."Tag_id_seq" OWNER TO postgres;

--
-- Name: Tag_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public."Tag_id_seq" OWNED BY public."Tag".id;


--
-- Name: User; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public."User" (
    id bigint NOT NULL,
    email text NOT NULL,
    name text NOT NULL,
    surname text NOT NULL,
    password_hash text NOT NULL,
    password_salt text NOT NULL,
    admin boolean DEFAULT false NOT NULL
);


ALTER TABLE public."User" OWNER TO postgres;

--
-- Name: User_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public."User_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public."User_id_seq" OWNER TO postgres;

--
-- Name: User_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public."User_id_seq" OWNED BY public."User".id;


--
-- Name: _sqlx_migrations; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public._sqlx_migrations (
    version bigint NOT NULL,
    description text NOT NULL,
    installed_on timestamp with time zone DEFAULT now() NOT NULL,
    success boolean NOT NULL,
    checksum bytea NOT NULL,
    execution_time bigint NOT NULL
);


ALTER TABLE public._sqlx_migrations OWNER TO postgres;

--
-- Name: Bike id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Bike" ALTER COLUMN id SET DEFAULT nextval('public."Bike_id_seq"'::regclass);


--
-- Name: Bike model_id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Bike" ALTER COLUMN model_id SET DEFAULT nextval('public."Bike_model_id_seq"'::regclass);


--
-- Name: BikeImage id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."BikeImage" ALTER COLUMN id SET DEFAULT nextval('public."BikeImage_id_seq"'::regclass);


--
-- Name: BikeImage bike_id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."BikeImage" ALTER COLUMN bike_id SET DEFAULT nextval('public."BikeImage_bike_id_seq"'::regclass);


--
-- Name: BikeTag bike_id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."BikeTag" ALTER COLUMN bike_id SET DEFAULT nextval('public."BikeTag_bike_id_seq"'::regclass);


--
-- Name: BikeTag tag_id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."BikeTag" ALTER COLUMN tag_id SET DEFAULT nextval('public."BikeTag_tag_id_seq"'::regclass);


--
-- Name: Brand id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Brand" ALTER COLUMN id SET DEFAULT nextval('public."Brand_id_seq"'::regclass);


--
-- Name: Model id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Model" ALTER COLUMN id SET DEFAULT nextval('public."Model_id_seq"'::regclass);


--
-- Name: Model brand_id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Model" ALTER COLUMN brand_id SET DEFAULT nextval('public."Model_brand_id_seq"'::regclass);


--
-- Name: Tag id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Tag" ALTER COLUMN id SET DEFAULT nextval('public."Tag_id_seq"'::regclass);


--
-- Name: User id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."User" ALTER COLUMN id SET DEFAULT nextval('public."User_id_seq"'::regclass);


--
-- Data for Name: Bike; Type: TABLE DATA; Schema: public; Owner: postgres
--

INSERT INTO public."Bike" VALUES (3, 4, 'Gentleman', '/media/bf4d238c-60b5-42d7-80a4-ea808f0518c1.jpg', '## Restoration
- sand blasting
- chroming
- painting', 2, 0, '2024-12-19 23:53:02.061463+00', '2025-01-04 13:20:37.985019+00', false, 1976, 200000, 'Columbus SL Steel Tubing', 'C-C: 57 cm, C-T: 58 cm', 56, 82, 'Campagnolo Nuovo Record', 'Campagnolo Nuovo Record', 'Campagnolo Nuovo Record', 'Campagnolo Nuovo Record', 'Campagnolo Nuovo Record', 'Campagnolo Nuovo Record', 'Campagnolo Nuovo Record', 'Campagnolo Nuovo Record', 'Profil', 'Selcof', 'Campagnolo Nuovo Record', 'Mavic', '3ttt', '3ttt', '<p class="text-green-500">OK</p>');
INSERT INTO public."Bike" VALUES (1, 12, 'Beautiful Colnago Master Piu', '/media/481dad43-e294-41cc-a4ff-0fcfbe5c2bca.jpg', '## Restoration
- sand blasting
- chroming
- painting', 2, 0, '2024-12-19 23:53:02.061463+00', '2025-01-04 13:24:18.022438+00', false, 1982, 200000, 'Columbus SL Steel Tubing', 'C-C: 57 cm, C-T: 58 cm', 56, 82, 'Campagnolo Nuovo Record', 'Campagnolo Nuovo Record', 'Campagnolo Nuovo Record', 'Campagnolo Nuovo Record', 'Campagnolo Nuovo Record', 'Campagnolo Nuovo Record', 'Campagnolo Nuovo Record', 'Campagnolo Nuovo Record', 'Profil', 'Selcof', 'Campagnolo Nuovo Record', 'Mavic', '3ttt', '3ttt', '<p class="text-green-500">OK</p>');
INSERT INTO public."Bike" VALUES (2, 6, 'Silver Colnago Super', '/media/d6ef3d4a-3b72-47ae-9b2d-95ebb25e3295.jpg', '## eBay
You can also buy this bike via [eBay](https://www.ebay.com/itm/226156316287)

## Restoration
- sand blasting
- chroming
- painting', 0, 0, '2024-12-19 23:53:02.061463+00', '2025-01-02 10:14:01.775843+00', false, 1978, 275000, 'Columbus SL Steel Tubing', 'C-C: 57 cm, C-T: 58 cm', 56, 82, 'Campagnolo Nuovo Record', 'Campagnolo Nuovo Record', 'Campagnolo Nuovo Record', 'Campagnolo Nuovo Record', 'Campagnolo Nuovo Record', 'Campagnolo Nuovo Record', 'Campagnolo Nuovo Record', 'Campagnolo Nuovo Record', 'Profil', 'Selcof', 'Campagnolo Nuovo Record', 'Mavic', '3ttt', '3ttt', '<p class="text-green-500">OK</p>');
INSERT INTO public."Bike" VALUES (7, 6, 'Green Colnago Super', '/media/d6b93a39-833d-40ec-aecb-0af10146fb2d.jpg', '## Restoration
- sand blasting
- chroming
- painting', 0, 0, '2024-12-30 00:50:34.143712+00', '2025-01-02 09:59:14.187003+00', false, 1978, 150000, 'Columbus SL Steel Tubing', 'C-C: 57 cm, C-T: 58 cm', 56, 82, 'Campagnolo Nuovo Record', 'Campagnolo Nuovo Record', 'Campagnolo Nuovo Record', 'Campagnolo Nuovo Record', 'Campagnolo Nuovo Record', 'Campagnolo Nuovo Record', 'Campagnolo Nuovo Record', 'Campagnolo Nuovo Record', 'Profil', 'Selcof', 'Campagnolo Nuovo Record', 'Mavic', '3ttt', '3ttt', '<p class="text-green-500">OK</p>');
INSERT INTO public."Bike" VALUES (4, 6, 'Yellow Colnago', '/media/296255dd-a878-41e0-a249-c6778f421175.jpg', '## eBay
You can also buy this bike via [eBay](https://www.ebay.com/itm/226156264969). 

## Restoration
- sand blasting
- chroming
- painting
', 0, 0, '2024-12-19 23:53:02.061463+00', '2025-01-02 10:14:17.358017+00', false, 1972, 145000, 'Columbus SL Steel Tubing', 'C-C: 57 cm, C-T: 58 cm', 56, 82, 'Campagnolo Nuovo Record', 'Campagnolo Nuovo Record', 'Campagnolo Nuovo Record', 'Campagnolo Nuovo Record', 'Campagnolo Nuovo Record', 'Campagnolo Nuovo Record', 'Campagnolo Nuovo Record', 'Campagnolo Nuovo Record', 'Profil', 'Selcof', 'Campagnolo Nuovo Record', 'Mavic', '3ttt', '3ttt', '<p class="text-green-500">OK</p>');


--
-- Data for Name: BikeImage; Type: TABLE DATA; Schema: public; Owner: postgres
--

INSERT INTO public."BikeImage" VALUES (110, 1, '/media/9ed29047-a102-42c4-8e7f-5838ec835cd2.jpg', 1500, 2000, '/media/20f5264a-6d40-43f6-a1bd-d759018a7453.jpg');
INSERT INTO public."BikeImage" VALUES (111, 1, '/media/c0cfeaa8-23b2-42e3-a192-d7068c782e04.jpg', 2000, 1500, '/media/04538992-1c13-4a80-82bd-1aca24e8676e.jpg');
INSERT INTO public."BikeImage" VALUES (112, 1, '/media/c69f9ea7-f283-4ce6-9803-06d591d0a715.jpg', 2000, 1500, '/media/474422e8-05be-489a-a557-af9feb05374b.jpg');
INSERT INTO public."BikeImage" VALUES (113, 1, '/media/258c62c5-9803-42af-b8a5-4be2ba7b5f9d.jpg', 1500, 2000, '/media/3babf7a3-14b3-4184-9682-bcfd6a5ea4f0.jpg');
INSERT INTO public."BikeImage" VALUES (114, 1, '/media/29fffc47-d52c-434e-930c-e6481b6cb4ef.jpg', 1500, 2000, '/media/df797fc1-4682-49d1-a294-86f8101ab1b4.jpg');
INSERT INTO public."BikeImage" VALUES (115, 1, '/media/036fbd7c-9b22-42f6-b7b4-cc3045219a0c.jpg', 2000, 1500, '/media/dc89b4f2-0962-433b-9dc8-b2f38a1a8ff6.jpg');
INSERT INTO public."BikeImage" VALUES (116, 1, '/media/d17b7d0d-f272-468a-aec9-db63e42b3bf3.jpg', 1500, 2000, '/media/1884cb95-55bc-4c69-9dc0-140df5d9b46f.jpg');
INSERT INTO public."BikeImage" VALUES (117, 1, '/media/c79fbe92-d068-4bbe-b4b2-9070ee844f98.jpg', 1500, 2000, '/media/445f629f-7297-42f6-95d8-1a8e6703ab70.jpg');
INSERT INTO public."BikeImage" VALUES (118, 1, '/media/2aaace91-05a4-4cea-8fea-759e95577cfc.jpg', 1500, 2000, '/media/5c7faeeb-0c8a-4cd9-9300-f90268de1428.jpg');
INSERT INTO public."BikeImage" VALUES (119, 1, '/media/4eae5b38-8520-4566-92d4-b535faad77ad.jpg', 1500, 2000, '/media/d428ee36-7962-4849-a229-443c32726e30.jpg');
INSERT INTO public."BikeImage" VALUES (120, 1, '/media/9cfb79d7-c657-47f7-bec6-ce5564ea6fa5.jpg', 1500, 2000, '/media/817379d3-369c-4c86-b576-fccbcf0d0cf7.jpg');
INSERT INTO public."BikeImage" VALUES (121, 1, '/media/063a91cb-2106-4acd-b084-d462a5855d81.jpg', 1500, 2000, '/media/09d0967e-5659-459c-b1b9-e31f2101c6ee.jpg');
INSERT INTO public."BikeImage" VALUES (122, 1, '/media/e2cc2a0e-7791-451f-84d3-491e033dc811.jpg', 1500, 2000, '/media/81606900-1ac9-4437-b758-9100ad5e3343.jpg');
INSERT INTO public."BikeImage" VALUES (123, 1, '/media/e2771a84-f959-4595-b188-8d3a3d87058d.jpg', 1500, 2000, '/media/dfa202ea-c1d4-43a9-a79a-7fcaa7fecf0b.jpg');
INSERT INTO public."BikeImage" VALUES (124, 1, '/media/7042bd10-ba66-4fdb-acb7-d8267cdd52bd.jpg', 2000, 1500, '/media/aec18932-8387-4956-8503-bfc0c7a020ba.jpg');
INSERT INTO public."BikeImage" VALUES (125, 1, '/media/b8794a59-20a4-47c7-b0d2-c205156210d5.jpg', 1500, 2000, '/media/9510a30d-0e85-47fa-9f4f-afa93a17a142.jpg');
INSERT INTO public."BikeImage" VALUES (126, 1, '/media/01e83e57-e558-49d5-850c-363b585b766a.jpg', 1500, 2000, '/media/82c67224-c911-4298-affb-b2e1567e8022.jpg');
INSERT INTO public."BikeImage" VALUES (127, 1, '/media/31fdff7a-13b8-4619-8688-d3bed087e946.jpg', 1500, 2000, '/media/3ee1f6d9-3e5d-4fcb-ba99-83c0fd2aa5b8.jpg');
INSERT INTO public."BikeImage" VALUES (128, 1, '/media/bf5b7109-bca8-4545-9493-64f383c52ef2.jpg', 1500, 2000, '/media/699f104d-b967-4445-90a9-e03183488b38.jpg');
INSERT INTO public."BikeImage" VALUES (129, 1, '/media/81ae971a-bba6-4f93-9be8-b792cfceaddd.jpg', 1500, 2000, '/media/0a515558-4183-45cb-bc79-e12d363f7698.jpg');
INSERT INTO public."BikeImage" VALUES (130, 1, '/media/85da20ad-4ee8-4792-8133-0de61d31fb3a.jpg', 2000, 1500, '/media/577c2a1a-4932-49a1-809e-e8063a2b6ffc.jpg');
INSERT INTO public."BikeImage" VALUES (131, 1, '/media/a908dc50-371d-4ed7-8c43-fc238010eebb.jpg', 1500, 2000, '/media/adaf92f3-cbe0-4b04-ac8a-e050e6c73135.jpg');
INSERT INTO public."BikeImage" VALUES (132, 1, '/media/f18d807b-e089-41e7-a999-3c5b5bb4fed4.jpg', 1500, 2000, '/media/a00b29a8-d258-4be0-b089-113948748d0e.jpg');
INSERT INTO public."BikeImage" VALUES (133, 1, '/media/202a55a6-4535-4bb4-8745-fb6d6e8b1042.jpg', 1500, 2000, '/media/9a9c90a6-4ccb-411e-8adf-46f38c863b0c.jpg');
INSERT INTO public."BikeImage" VALUES (134, 1, '/media/6db581c6-00a2-4502-a0cb-f401bbd9bf4f.jpg', 1500, 2000, '/media/29041900-76e2-49a0-b2b3-0c5d7ff00ef8.jpg');
INSERT INTO public."BikeImage" VALUES (258, 3, '/media/c5b49460-5190-444a-a9ef-72b2a47856b7.jpg', 2000, 1500, '/media/a58efb39-1f26-46df-be2c-3bfea8c8e9a6.jpg');
INSERT INTO public."BikeImage" VALUES (259, 3, '/media/7d1afe66-38ce-4697-b0fb-0635c087472c.jpg', 1500, 2000, '/media/6113960f-5403-4d72-baa4-a3b770963a75.jpg');
INSERT INTO public."BikeImage" VALUES (260, 3, '/media/f57eddf2-ab08-424a-a6d7-bbb32f335367.jpg', 1500, 2000, '/media/12385d26-0a76-41ac-81b5-93a39fdcc73a.jpg');
INSERT INTO public."BikeImage" VALUES (261, 3, '/media/0f807c9a-bc25-4cbe-9ed5-07a0f83e8ca5.jpg', 2000, 1500, '/media/6e94b3e3-7c1a-4e28-bfc6-cef77a5b5b5f.jpg');
INSERT INTO public."BikeImage" VALUES (262, 3, '/media/e9041b2c-4ac9-4991-a2b3-9bcb7c8e71f1.jpg', 1500, 2000, '/media/52f75e5b-af83-4241-89e9-8be633e9679f.jpg');
INSERT INTO public."BikeImage" VALUES (263, 3, '/media/675c8d6f-ca9d-4efc-8a2a-97b0045ec7ec.jpg', 1500, 2000, '/media/b41a975d-f6e7-4444-8254-e14707a02bdb.jpg');
INSERT INTO public."BikeImage" VALUES (264, 3, '/media/8c916b94-82f7-4de9-a310-a913652f113b.jpg', 1500, 2000, '/media/31630e20-a3be-4608-82ae-7379d2ec728f.jpg');
INSERT INTO public."BikeImage" VALUES (265, 3, '/media/5e791bd7-f254-451a-9407-1e66588b6ea2.jpg', 1500, 2000, '/media/090da445-d425-490b-8495-7ba4e22c9bdf.jpg');
INSERT INTO public."BikeImage" VALUES (266, 3, '/media/5060b69f-1f2c-41ed-ae86-68a592942daf.jpg', 2000, 1500, '/media/b71643ec-8a1c-4c00-b3a2-e4680eed5ad7.jpg');
INSERT INTO public."BikeImage" VALUES (267, 3, '/media/4f63942a-1fe6-41bc-8f71-f5e5897c5b29.jpg', 2000, 1500, '/media/99244085-b6ad-4ea5-91d8-a9f9857511d8.jpg');
INSERT INTO public."BikeImage" VALUES (268, 3, '/media/ca238737-7fa3-4fd7-9b16-14d495a30f24.jpg', 1500, 2000, '/media/cdd02484-931e-498e-b454-0096bbdddd50.jpg');
INSERT INTO public."BikeImage" VALUES (269, 3, '/media/b4b0f09e-d8dc-43bf-baba-441f13fb5861.jpg', 1500, 2000, '/media/ed59b8b2-02fd-4f47-84d6-4e5ecd7d556c.jpg');
INSERT INTO public."BikeImage" VALUES (270, 3, '/media/c754bd5d-1a0e-445a-8c60-c336b0bb196a.jpg', 1500, 2000, '/media/4fd2840b-036f-4c1f-9243-8bba54cb2905.jpg');
INSERT INTO public."BikeImage" VALUES (271, 3, '/media/dded7ff5-012c-4566-8d66-4aba8e82fa0c.jpg', 2000, 1500, '/media/ef91fb9c-a432-4aee-b84a-b400e34e5ec1.jpg');
INSERT INTO public."BikeImage" VALUES (272, 3, '/media/73b9a006-6399-40b7-8c46-93d21ca13d22.jpg', 1500, 2000, '/media/5abae851-d503-44ce-8f8d-65984e7db25b.jpg');
INSERT INTO public."BikeImage" VALUES (273, 3, '/media/f2eddb18-fe5f-40c9-818a-900db056798b.jpg', 1500, 2000, '/media/cbf8800f-8274-44e2-a903-c5d3ce0814a1.jpg');
INSERT INTO public."BikeImage" VALUES (274, 3, '/media/0d061317-d3e2-465a-ab00-d32d1b352faf.jpg', 1500, 2000, '/media/b62f7d1e-4565-4875-a441-d1a243fc4279.jpg');
INSERT INTO public."BikeImage" VALUES (275, 3, '/media/a4397974-6175-464a-9a7c-ff2b01f5a207.jpg', 1500, 2000, '/media/c1322cf6-9828-4fce-a27b-3e72c3b53d95.jpg');
INSERT INTO public."BikeImage" VALUES (67, 4, '/media/b43119b5-a74d-46d6-b4f4-381b9aeccc1d.jpg', 2000, 1500, '/media/fc8c0d32-9748-4a94-baa4-2e410adab82a.jpg');
INSERT INTO public."BikeImage" VALUES (68, 4, '/media/2aba81e6-d2e9-4fca-989c-0f25b61afa0a.jpg', 2000, 1500, '/media/030f8c63-8ae5-4aa1-8740-12419cd21c38.jpg');
INSERT INTO public."BikeImage" VALUES (69, 4, '/media/e7cc08e6-7ce3-40fc-8a08-c5ce12ccbf18.jpg', 2000, 1500, '/media/691f32fe-84f2-4ceb-8bb4-d119b3fb0453.jpg');
INSERT INTO public."BikeImage" VALUES (70, 4, '/media/8a3c5a2a-4cb7-4be2-8a43-a63a02c515ef.jpg', 1500, 2000, '/media/821efc61-83fd-4ccd-a71a-48bccdbbd50e.jpg');
INSERT INTO public."BikeImage" VALUES (71, 4, '/media/4564678d-0d9d-4459-a685-152f4d9cd6f4.jpg', 1500, 2000, '/media/213fcd31-66c8-4860-aa20-257a3edebcc6.jpg');
INSERT INTO public."BikeImage" VALUES (72, 4, '/media/3a70b033-b6d3-43db-933e-5c6c3438bb70.jpg', 1500, 2000, '/media/571ccf60-5895-47fa-a813-af7b71cb9c38.jpg');
INSERT INTO public."BikeImage" VALUES (73, 4, '/media/754ec2da-a9e7-4046-a07a-e5e7add6f03c.jpg', 2000, 1500, '/media/71577b0c-a409-487a-9e47-50bc743631bd.jpg');
INSERT INTO public."BikeImage" VALUES (74, 4, '/media/14673088-8ce5-4ee5-b000-f382f7b3cbe4.jpg', 1500, 2000, '/media/853459d5-03b4-41f2-9d9f-86332af9d6b0.jpg');
INSERT INTO public."BikeImage" VALUES (75, 4, '/media/431c726f-8ff3-444e-8494-b33f86da3928.jpg', 2000, 1500, '/media/4ac094ab-ccfa-4d0e-a641-36c687c348c2.jpg');
INSERT INTO public."BikeImage" VALUES (76, 4, '/media/2bdc32c3-f92f-4b22-9a03-2bf34690b712.jpg', 1500, 2000, '/media/40a2d05a-b5a3-4889-80b4-581097d9c260.jpg');
INSERT INTO public."BikeImage" VALUES (77, 4, '/media/9e65727f-743c-4cf0-9064-883ea7451d79.jpg', 1500, 2000, '/media/016de570-03d4-4090-a349-ef13ee6f6ca5.jpg');
INSERT INTO public."BikeImage" VALUES (78, 4, '/media/5afe5987-1348-4805-8d53-fbb1bdfddd5a.jpg', 2000, 1500, '/media/4fe30b84-b547-4b37-b475-2e81c8c47dd2.jpg');
INSERT INTO public."BikeImage" VALUES (79, 4, '/media/30963003-8b37-42c9-ac35-ccbe6c6a8aa8.jpg', 1500, 2000, '/media/7a88b024-ca30-43ee-962c-701510bd0828.jpg');
INSERT INTO public."BikeImage" VALUES (80, 4, '/media/ee48098e-5756-422a-ae36-f019054fa9a4.jpg', 2000, 1500, '/media/3e1c5a7b-5f88-48cc-a974-592aa28dad60.jpg');
INSERT INTO public."BikeImage" VALUES (81, 4, '/media/757bdfad-d833-4cbb-b56c-836a3f6f39d1.jpg', 2000, 1500, '/media/c5e59326-15e6-4cd4-bb51-897e23512fa9.jpg');
INSERT INTO public."BikeImage" VALUES (82, 4, '/media/32a1d16f-9dbc-4c53-8976-41803178abb9.jpg', 2000, 1500, '/media/bb5686d4-31fe-4b80-ae62-6abf23bf1283.jpg');
INSERT INTO public."BikeImage" VALUES (83, 4, '/media/e473d617-53fa-4fb3-b38f-7a9ef5720d50.jpg', 2000, 1500, '/media/88bb9496-a05b-4d0f-b97a-5bf46ed1f93a.jpg');
INSERT INTO public."BikeImage" VALUES (84, 4, '/media/518140fb-0f91-4900-ba7c-d63d194c24f1.jpg', 2000, 1500, '/media/e6a2dcb0-bd20-4ca0-a56f-bcac27af6d42.jpg');
INSERT INTO public."BikeImage" VALUES (85, 4, '/media/3af4a7f8-31a4-4c48-80c8-2dfd52a513b4.jpg', 2000, 1500, '/media/b36efc80-17b0-457c-ac18-e9b2e2edee3c.jpg');
INSERT INTO public."BikeImage" VALUES (86, 4, '/media/ac000991-04b2-40dc-8322-f1f1f255b06b.jpg', 1500, 2000, '/media/3c13c8ff-ca6f-44a5-9d50-770bf1172a54.jpg');
INSERT INTO public."BikeImage" VALUES (87, 4, '/media/aa8e3bb8-2d69-4bd6-a955-1ed3c3fac911.jpg', 2000, 1500, '/media/6a58d053-71e9-4ea4-afa4-b18ac7226ded.jpg');
INSERT INTO public."BikeImage" VALUES (88, 4, '/media/7cdffcff-8a28-42a0-be87-d5620d7a3669.jpg', 2000, 1500, '/media/1a004778-ea0b-4f0c-83a0-13a5183a38cd.jpg');
INSERT INTO public."BikeImage" VALUES (89, 4, '/media/6cc43f78-aee2-49d4-a339-9bc2e34e76b5.jpg', 2000, 1500, '/media/5a6cfb5c-24e3-4142-add8-4a6830e9c54d.jpg');
INSERT INTO public."BikeImage" VALUES (90, 2, '/media/920e180d-d4dc-4116-9a3c-36bb657a76fd.jpg', 2000, 1500, '/media/ee4aa1ab-df4e-4d48-b3e9-7f07ab4bbe76.jpg');
INSERT INTO public."BikeImage" VALUES (91, 2, '/media/0e703f7f-fca0-460e-a168-d7e2579d1176.jpg', 1500, 2000, '/media/a2e44706-280f-475e-b523-69c185ef0815.jpg');
INSERT INTO public."BikeImage" VALUES (92, 2, '/media/4d1b4fbb-095e-4a24-860f-43084edb1842.jpg', 2000, 1500, '/media/a4c5dbcc-4866-49d3-ae1a-b4e3d80de275.jpg');
INSERT INTO public."BikeImage" VALUES (93, 2, '/media/3a66e4a3-adc8-4215-99c2-05372abfd868.jpg', 2000, 1500, '/media/821ed998-b90e-4543-a8b0-13ed6307901f.jpg');
INSERT INTO public."BikeImage" VALUES (94, 2, '/media/80e69bb3-e64f-4d30-bc80-f1f5d4a0b5af.jpg', 2000, 1500, '/media/44d2b388-4d20-4b5f-bf0d-5b84fd19e955.jpg');
INSERT INTO public."BikeImage" VALUES (95, 2, '/media/56f376e6-4213-4e53-82e8-ceba30d0b407.jpg', 2000, 1500, '/media/3f3ae8f5-07da-454c-8635-0f4971574147.jpg');
INSERT INTO public."BikeImage" VALUES (96, 2, '/media/8df28806-59c9-4fff-aa54-960e0c7350a7.jpg', 1500, 2000, '/media/0c525c19-9e22-4770-a3ca-8300cc7e8e19.jpg');
INSERT INTO public."BikeImage" VALUES (97, 2, '/media/2a6f56cd-85f6-49ed-9d6e-bbed12c6d4a6.jpg', 1500, 2000, '/media/ae0208b5-da5e-43da-a5bf-3bb1e9e7c185.jpg');
INSERT INTO public."BikeImage" VALUES (98, 2, '/media/2577d0ef-c3b3-4271-af69-9180a8753015.jpg', 2000, 1500, '/media/9913d8f3-609f-4a1c-b8fe-cffb9a19ba71.jpg');
INSERT INTO public."BikeImage" VALUES (99, 2, '/media/56a89fcb-abed-4070-9e64-32ea80b2cec2.jpg', 1500, 2000, '/media/4370d156-5a79-4ac3-9736-00fafd3e6252.jpg');
INSERT INTO public."BikeImage" VALUES (100, 2, '/media/45b6dcfe-8e72-4ce1-9c1b-57b24acdeaf6.jpg', 1500, 2000, '/media/4668b4d7-ff37-413e-938f-ac72fcf6c0e8.jpg');
INSERT INTO public."BikeImage" VALUES (101, 2, '/media/e8e135bb-7855-4c0f-a519-8833bd1e723d.jpg', 2000, 1500, '/media/4ef9a279-6587-4a02-b1b9-70c7c4c99c31.jpg');
INSERT INTO public."BikeImage" VALUES (102, 2, '/media/fcbea110-8c31-4e8f-afb4-aae1eb79e327.jpg', 1500, 2000, '/media/8f159736-e0bd-4ba4-8a47-f294a306f9cf.jpg');
INSERT INTO public."BikeImage" VALUES (103, 2, '/media/0603d788-b996-4753-910c-39b0395c17b2.jpg', 1500, 2000, '/media/4bb01b4b-a71c-48db-bbd6-3170e48c0923.jpg');
INSERT INTO public."BikeImage" VALUES (104, 2, '/media/57f2f076-d855-46f1-bc3e-cce856a1a0c2.jpg', 1500, 2000, '/media/fbcf62e3-66f1-49f7-a774-98be6211fac7.jpg');
INSERT INTO public."BikeImage" VALUES (105, 2, '/media/340c5aaa-02e0-4b0b-a12f-3ba46251ccfe.jpg', 1500, 2000, '/media/13cfc482-86aa-49b3-966c-078d2a22c979.jpg');
INSERT INTO public."BikeImage" VALUES (106, 2, '/media/ddef78dd-50b7-4085-9ee0-360b96a64a3e.jpg', 2000, 1500, '/media/0a1b9bc3-7084-4f20-930d-d84cd6e5c0b6.jpg');
INSERT INTO public."BikeImage" VALUES (107, 2, '/media/95acbe8f-3bc4-42b3-a948-890059f00040.jpg', 2000, 1500, '/media/9f71c2d8-c18d-4482-82b1-faacc62ff1c8.jpg');
INSERT INTO public."BikeImage" VALUES (108, 2, '/media/28f48d62-26ea-4b0e-93a0-0ce5983cd06e.jpg', 2000, 1500, '/media/6017b4c8-5ee4-414d-9e91-83f98ad89cbb.jpg');
INSERT INTO public."BikeImage" VALUES (109, 2, '/media/2a96eb78-074d-470c-8cbc-757acd130c18.jpg', 1500, 2000, '/media/c05c1606-f2c5-4465-adde-b40728cf2d00.jpg');
INSERT INTO public."BikeImage" VALUES (276, 3, '/media/65d294f8-e421-46f3-abaf-115003a2f9d3.jpg', 1500, 2000, '/media/727939b0-a127-4344-9753-e34825aea816.jpg');
INSERT INTO public."BikeImage" VALUES (277, 3, '/media/d23dfe6a-4c9f-48e2-9f30-1bc05625bb46.jpg', 1500, 2000, '/media/921ef352-c1e3-4933-9ab7-59ff8e0c3c91.jpg');
INSERT INTO public."BikeImage" VALUES (278, 3, '/media/df3fad1b-cbb5-47ce-b8e4-66f7f85a2023.jpg', 1500, 2000, '/media/5d19d873-3f5e-43f4-ab2b-c0dd02b9d09c.jpg');
INSERT INTO public."BikeImage" VALUES (279, 3, '/media/96200c3d-3fd1-47ab-ad43-eb679d6ea74f.jpg', 1500, 2000, '/media/8a5ada70-8132-4f18-b362-6ca0681ca5ce.jpg');
INSERT INTO public."BikeImage" VALUES (280, 7, '/media/613c9df2-43b6-45fb-a3eb-d9f1604549fc.jpg', 2000, 1500, '/media/0353c41c-ed24-4109-ba85-f6f9240def2b.jpg');
INSERT INTO public."BikeImage" VALUES (281, 7, '/media/9a9be634-703d-4d06-b16b-143591733423.jpg', 1500, 2000, '/media/c96b8a1a-43cc-405a-a59a-e0b26737c4f8.jpg');
INSERT INTO public."BikeImage" VALUES (282, 7, '/media/c7eb9823-bd1e-4454-810c-102a92c4cda7.jpg', 2000, 1500, '/media/0cf740aa-d45f-46c4-9d90-59a4016c59b5.jpg');
INSERT INTO public."BikeImage" VALUES (283, 7, '/media/b6202484-ec43-498c-a27c-971ea4b39f89.jpg', 1500, 2000, '/media/c1709ae3-a24c-425c-a4f1-74bcf785251e.jpg');
INSERT INTO public."BikeImage" VALUES (284, 7, '/media/b21a1fcd-40eb-4968-8287-bc39e56ebe8c.jpg', 2000, 1500, '/media/5a769ce6-0d35-4f0b-9ca1-20b6ea4da691.jpg');
INSERT INTO public."BikeImage" VALUES (285, 7, '/media/07178b6b-cf2b-46e0-8caa-4487b658a411.jpg', 1500, 2000, '/media/f2792d0e-5dcc-47cf-b215-5b38cbc4ccb6.jpg');
INSERT INTO public."BikeImage" VALUES (286, 7, '/media/155fc0cf-f3f9-4e37-97d0-bda24132dc7e.jpg', 1500, 2000, '/media/67fd783b-9214-4a09-a3e3-a3b44d8a09b1.jpg');
INSERT INTO public."BikeImage" VALUES (287, 7, '/media/58544d41-fee8-4fec-b947-9fb86cc611ac.jpg', 1500, 2000, '/media/3d9ef10c-33b7-4471-99f9-d0d4f21aa581.jpg');
INSERT INTO public."BikeImage" VALUES (288, 7, '/media/604440ad-4cfe-4354-bf64-966ee768d4ca.jpg', 1500, 2000, '/media/8bb21657-98ef-48eb-b2dd-48e3a048e98d.jpg');
INSERT INTO public."BikeImage" VALUES (289, 7, '/media/848e61ca-4a7e-498b-aac4-435814adff25.jpg', 1500, 2000, '/media/a430f35c-a5c6-4e2b-9c19-78ed06462303.jpg');
INSERT INTO public."BikeImage" VALUES (290, 7, '/media/937d3e48-7961-4100-967d-f268ce1f87fd.jpg', 1500, 2000, '/media/6018522d-9953-4961-b499-1ed243593d95.jpg');
INSERT INTO public."BikeImage" VALUES (291, 7, '/media/29ebc7ad-38e6-4e80-903b-ec3e4724c629.jpg', 1500, 2000, '/media/f6f95993-e3ce-4e81-a998-e4d55167b8e9.jpg');
INSERT INTO public."BikeImage" VALUES (292, 7, '/media/dc61c517-0360-4b64-b70d-c0b5d86ddf57.jpg', 2000, 1500, '/media/9821c8bf-735b-42e7-a09b-c87dbf4f0ccb.jpg');
INSERT INTO public."BikeImage" VALUES (293, 7, '/media/37e9ca79-030a-47e8-812c-fc1847f06d36.jpg', 2000, 1500, '/media/525bd519-a985-4739-88cb-6d0ac176fdc3.jpg');
INSERT INTO public."BikeImage" VALUES (294, 7, '/media/594573fe-2f38-4ae9-9268-571bcb2d4a57.jpg', 2000, 1500, '/media/e18dea8c-feab-4a2e-97f7-aa98e4b9ed71.jpg');
INSERT INTO public."BikeImage" VALUES (295, 7, '/media/6769925c-2c1e-4e29-b65d-8851a4874e3c.jpg', 2000, 1500, '/media/8ff619fa-d9d2-4ec2-8c71-45525c86d387.jpg');
INSERT INTO public."BikeImage" VALUES (296, 7, '/media/b3f0e424-60e8-4899-a441-929842c6ac98.jpg', 1500, 2000, '/media/d2c81848-5886-44ae-95e4-f48b83abbc1f.jpg');
INSERT INTO public."BikeImage" VALUES (297, 7, '/media/ad686f3f-f9e3-44d7-8c9e-ec82e186ea10.jpg', 1500, 2000, '/media/cf8109df-d8ce-4678-89df-a0251e48c456.jpg');
INSERT INTO public."BikeImage" VALUES (298, 7, '/media/1ca022f0-bc3e-4823-81cf-8922133dc12f.jpg', 2000, 1500, '/media/136929b6-b51c-4be9-8f7d-36d4512683c5.jpg');
INSERT INTO public."BikeImage" VALUES (299, 7, '/media/ada845cc-f1a1-48d6-a125-e62c4fa63c34.jpg', 1500, 2000, '/media/f6e1cdbe-d470-49db-a344-3d0dafeb0e8c.jpg');
INSERT INTO public."BikeImage" VALUES (300, 7, '/media/630b33f4-cb0e-44a3-a9e1-00f2dfea5fc2.jpg', 1500, 2000, '/media/0f70fccb-4db4-4c68-9ed2-0f713214bc27.jpg');
INSERT INTO public."BikeImage" VALUES (301, 7, '/media/60198e35-9828-45af-ba1a-5a2cc62d95bf.jpg', 1500, 2000, '/media/58afbcc8-9c24-4a0a-8044-a485f1fdde6e.jpg');
INSERT INTO public."BikeImage" VALUES (302, 7, '/media/2b7c324e-6373-4c1e-a74a-46ea3359402b.jpg', 1500, 2000, '/media/b7c807f6-4b0c-4dfd-87cf-8b498bbe8ef7.jpg');
INSERT INTO public."BikeImage" VALUES (303, 7, '/media/62d42212-8715-41b0-a41d-4f2ce0aa414b.jpg', 1500, 2000, '/media/63814d62-ff3e-4dfb-9264-1de8b12e4fb4.jpg');


--
-- Data for Name: BikeTag; Type: TABLE DATA; Schema: public; Owner: postgres
--



--
-- Data for Name: Brand; Type: TABLE DATA; Schema: public; Owner: postgres
--

INSERT INTO public."Brand" VALUES (3, 'Peugeot', '
Peugeot, a cornerstone of French industry, began producing bicycles in 1882, leveraging its expertise in steel manufacturing. By 1886, Peugeot Cycles had introduced a range of models, marking the start of a rich history in cycling innovation and competition.

Throughout the 20th century, Peugeot became synonymous with competitive cycling, sponsoring professional teams and achieving significant victories in events like the Tour de France. The brand''s bicycles were known for their reliability and performance, with models such as the PX-10 becoming iconic among racers and enthusiasts.

In the 1970s and 1980s, Peugeot expanded its range to cater to the growing popularity of cycling, offering a variety of models for different disciplines. The company''s commitment to innovation continued, embracing new materials and technologies to enhance performance.

Today, Peugeot Cycles remains a key pillar in Peugeot''s mobility strategy, with a specific focus on the electric revolution, offering a diverse range of bicycles that blend tradition with modern technology.

For more information, visit [Peugeot Cycles Official History](https://cycles.peugeot.com/history).
');
INSERT INTO public."Brand" VALUES (4, 'Bianchi', 'Bianchi, established in 1885 by Edoardo Bianchi in Milan, Italy, is the world’s oldest bicycle manufacturer still in operation. Known for its iconic **celeste** green color, Bianchi has become a symbol of Italian cycling heritage. From its early days, Bianchi distinguished itself by introducing innovative designs, including the first use of equal-sized wheels and pneumatic tires in bicycles.

Bianchi has long been associated with competitive success. Legendary riders like Fausto Coppi and Marco Pantani achieved historic victories on Bianchi bicycles, further elevating the brand''s status. The company''s commitment to quality and performance is reflected in its range of steel, aluminum, and carbon fiber bicycles, catering to both professional and recreational cyclists.

Over the decades, Bianchi has embraced innovation while honoring its legacy, ensuring its place as a cornerstone of cycling culture.

Learn more at [Bianchi Official History](https://www.bianchi.com/our-story/).
');
INSERT INTO public."Brand" VALUES (2, 'Colnago', '
Colnago, founded in 1954 by Ernesto Colnago in Cambiago, Italy, is a name synonymous with innovation and prestige in the cycling world. Initially focusing on custom race bikes, the brand quickly gained prominence for its high-quality steel frames and attention to detail. Colnago''s partnership with professional teams and legendary cyclists, such as Eddy Merckx, cemented its reputation in the competitive arena.

Throughout its history, Colnago has been at the forefront of technological advancements. The brand pioneered innovations like the Precisa straight-bladed fork and was an early adopter of carbon fiber technology, collaborating with Ferrari in the 1980s. Iconic models like the Colnago Super and the Master series highlight its blend of performance and artistry. With its intricate paintwork and Italian craftsmanship, Colnago continues to be a revered name among cyclists and collectors.

For more about Colnago, visit [Colnago Official History](https://www.colnago.com/en-us/explore/history).

## Learn More

- [Colnago Official History](https://www.colnago.com/en-us/explore/history)
- [Colnago: History of the Mythic Brand](https://www.thecyclisthouse.com/en/blogs/news/colnago-history-of-the-mythic-brand)
- [How to Identify a Vintage Colnago Bike](https://cycling-obsession.com/how-to-identify-a-colnago-vintage-bike/)


## Vintage Models

[Colnago Past Models](https://www.colnago.com/en/collections/past-models/)');
INSERT INTO public."Brand" VALUES (1, 'Hetchins', 'Hetchins, established in the early 1920s by Hyman Hetchin in Tottenham, London, is renowned for its intricately designed bicycle frames, particularly the distinctive "curly" or "vibrant" stays and ornate lugwork. Originally, Hetchin''s shop sold household appliances and mass-produced bicycles. In 1934, Hetchin partnered with frame builder Jack Denny, who had been experimenting with innovative curved stay designs. Recognizing the potential, Hetchin patented the design that same year, marking the beginning of Hetchins'' production of bespoke bicycles.

The brand quickly gained prominence, with its frames contributing to Olympic and World Championship victories in 1936. Despite challenges during World War II, Hetchins maintained production and experienced a resurgence in the 1950s. After Hyman''s passing in 1961, his son Alf took over the business, continuing the tradition of quality and craftsmanship. In 1974, the shop relocated to Southend-On-Sea due to re-zoning in Tottenham. Alf managed the business until his retirement in 1986. Since 1993, Hetchins operations have been centered in Preston, UK, under the management of David Miller, with Paul Riley as the current frame builder.

Today, Hetchins frames are highly sought after by collectors and cycling enthusiasts, celebrated for their unique aesthetic and historical significance.

For more information, visit [Hetchins.org](https://www.hetchins.org/).');


--
-- Data for Name: Model; Type: TABLE DATA; Schema: public; Owner: postgres
--

INSERT INTO public."Model" VALUES (7, 1, 'Hellenic Spyder', 'Model z roku 1973, Hellenic Spyder, bol známy svojím jedinečným dizajnom rámu s charakteristickými krivkami a detailmi. Bol to ručne vyrobený bicykel, ktorý kombinoval estetiku s výkonom.');
INSERT INTO public."Model" VALUES (8, 4, 'Specialissima', 'Tento model bol vlajkovou loďou značky v 70. a 80. rokoch. Bol známy svojou ľahkou konštrukciou a ikonickou farbou „celeste“, čo z neho robilo obľúbenú voľbu medzi profesionálnymi cyklistami.');
INSERT INTO public."Model" VALUES (10, 3, 'PX-10', 'Jeden z najznámejších modelov značky, PX-10, bol populárny v 70. rokoch. Bol vybavený kvalitnými komponentmi a bol často používaný v profesionálnych pretekoch. ');
INSERT INTO public."Model" VALUES (11, 3, 'Carbolite 103', 'Model z 80. rokov, Carbolite 103, bol známy svojou odolnou konštrukciou a spoľahlivosťou, čo z neho robilo obľúbenú voľbu pre rekreačných cyklistov.');
INSERT INTO public."Model" VALUES (4, 2, 'Oval CX Gentleman', 'The Colnago Oval CX Gentleman, introduced in the early 1980s, is a distinctive model that blends aerodynamic innovation with urban practicality. Derived from the Colnago Oval CX—a bicycle renowned for its ovalized tubing designed to reduce air resistance—the Gentleman variant was tailored for city riding, offering both style and functionality.

## Design and Features

The Oval CX Gentleman shares the aerodynamic frame design of its racing counterpart, featuring oval-shaped tubes developed in collaboration with Columbus. These tubes were intended to enhance airflow and improve performance. However, the Gentleman model diverges by incorporating practical accessories suited for urban commuting:

- **Mudguards**: Provide protection against road spray, keeping the rider clean in various weather conditions.

- **Chain Guard**: Prevents clothing from getting caught in the chain, enhancing safety and convenience.

- **Rear Rack**: Offers additional storage capacity for carrying personal items or groceries.

These additions made the Oval CX Gentleman a versatile choice for city dwellers seeking a blend of performance and practicality.

## Components

The bicycle was equipped with high-quality components typical of Colnago''s craftsmanship during that era. While specific configurations could vary, many models featured:

- **Campagnolo Groupset**: Renowned for precision and reliability.

- **Comfort-Oriented Handlebars**: Designed to provide a more upright and relaxed riding position compared to the racing-oriented Oval CX.

- **Specialized Saddle**: Aimed at enhancing comfort during longer city rides.

## Rarity and Collectibility

Due to its unique positioning as a high-performance city bike, the Oval CX Gentleman was produced in limited numbers. Its rarity, combined with Colnago''s esteemed reputation, makes it a sought-after piece among vintage bicycle collectors and enthusiasts. Well-preserved models are particularly valued for their distinctive design and the craftsmanship emblematic of Colnago''s legacy.

For more detailed information and visual references, you can explore the following resources:

- [Colnago Oval CX Gentleman Sports Bike 1980s - Steel Vintage Bikes](https://steel-vintage.com/products/colnago-oval-cx-gentleman-sports-bike-1980s)

- [Colnago Oval CX Gentleman 1983 - Saarf London](https://saarf.london/2016/02/12/colnago-oval-cx-gentleman-1983/)

- [Colnago Oval CX Gentleman - Pedal Room](https://www.pedalroom.com/bike/colnago-oval-cx-gentleman-31534)

These sources provide additional insights into the model''s specifications and its place in cycling history.
');
INSERT INTO public."Model" VALUES (2, 2, 'Arabesque', 'The Colnago Arabesque, introduced in the early 1980s, is celebrated for its intricate craftsmanship and distinctive aesthetic, embodying the elegance and precision of Italian bicycle design. Originally produced to commemorate Colnago''s 30th anniversary in 1984, the Arabesque has become one of the most sought-after models among collectors and cycling enthusiasts.

## Frame Design and Construction

The Arabesque is renowned for its unique decorative lugs, which set it apart from other models of its era. Key features include:

- **Decorative Lugs**: The frame showcases ornate lugs with intricate arabesque patterns, particularly noticeable on the head tube and seat tube. These lugs were crafted by an Italian manufacturer known as ''Rauler'' and were also used in other Colnago models, such as the Colnago Regal and the 2016 Colnago Master Arabesque.

- **Crimped Tubing**: Early versions of the Arabesque featured crimped tubing similar to the Colnago Super Profil, with single crimps on each side of the top and down tubes. Later models adopted the crimping style of the Nuovo Mexico, with a single crimp on each side of the top tube and two offset crimps on each side of the down tube, totaling four crimps.

- **Materials**: The frame was constructed using high-quality Columbus steel tubing, known for its durability and ride quality. Some versions, like the Master Arabesque, utilized the Gilco Master profile tubes, characterized by their star-shaped cross-section for enhanced stiffness.

- **Finish**: Many Arabesque frames were chrome-plated, providing a lustrous and durable finish that accentuated the intricate lug work. Some models featured the distinctive Cromovelato paint finish, achieved by chrome-plating the entire frame and applying a tinted color coat, resulting in a reflective, glossy appearance.

## Special Editions

In 1984, to mark its 30th anniversary, Colnago released a limited edition Arabesque equipped with a special Campagnolo Super Record groupset featuring gold-plated components and Colnago club symbols. These models are particularly rare and highly valued by collectors.

## Legacy and Modern Revival

The Colnago Arabesque holds a special place in cycling history, symbolizing the blend of art and engineering that defines classic Italian bicycles. Its intricate design and limited production have made it a coveted piece among vintage cycling aficionados. Recognizing its enduring appeal, Colnago reissued the Arabesque in limited numbers, utilizing original lugs discovered in their factory, allowing a new generation of cyclists to experience this iconic model.

For more detailed information, you can explore the following resources:

- [Colnago Arabesque - Colnago Official Website](https://www.colnago.com/en-us/collections/past-models/arabesque)

- [How to Identify a Colnago Arabesque - Cycling Obsession](https://cycling-obsession.com/how-to-identify-a-colnago-arabesque/)

- [Colnago Arabesque - Ray Dobbins](https://raydobbins.com/arabesque/bike-arabesque_main.htm)

These sources provide additional insights into the model''s specifications and its place in cycling history.
');
INSERT INTO public."Model" VALUES (6, 2, 'Super', 'The **[Colnago Super](https://www.colnago.com/en/collections/past-models/super)**, introduced in 1968, is a landmark model in the history of competitive cycling, renowned for its lightweight construction and responsive handling. Designed by Ernesto Colnago, the Super was tailored to meet the demands of professional racers, contributing to numerous victories and solidifying Colnago''s reputation in the cycling world.

![image](https://cdn.shopify.com/s/files/1/0828/8980/2039/files/01-SUPERORIGINALE.jpg?v=1725469236&width=2000&height=1294&crop=center)

## Frame Design and Construction

The Super was constructed using Columbus SL steel tubing, known for its excellent balance of strength and weight. The frame featured a classic diamond geometry with several distinctive characteristics:

- **Reduced Weight**: Strategic thinning of the tubing and meticulous craftsmanship resulted in a frame that was significantly lighter than its contemporaries, enhancing climbing and acceleration performance.

- **Lowered Bottom Bracket**: This design choice improved stability and handling, allowing riders to maintain higher speeds through corners.

- **Shortened Chainstays**: Contributed to the bike''s agility and responsive ride quality, making it well-suited for the rigors of professional racing.

These design elements collectively provided a competitive edge, aligning with the performance needs of elite cyclists.

## Evolution and Variants

Over its production span, the Colnago Super underwent several refinements:

- **Super Version 1 (1968–1978)**: Featured Columbus SL tubing with circular top, down, and seat tubes, crimped chainstays, and cable routing above the bottom bracket.

- **Super Version 2 (1978–Late 1980s)**: Maintained the use of Columbus SL tubing with circular profiles but introduced embossed Colnago lettering on the seat stay caps and a machined chainstay bridge for enhanced rigidity.

- **Super Profil (1982–1984)**: Incorporated fluted tubing profiles and updated seat stay caps, reflecting Colnago''s commitment to innovation in frame design.

These iterations demonstrate Colnago''s dedication to continuous improvement and adaptation to evolving cycling technologies and rider preferences.

## Legacy and Impact

The Colnago Super holds a prestigious place in cycling history, representing a shift towards lighter and more responsive racing bicycles. Its success in professional competitions and widespread popularity among amateur cyclists contributed significantly to Colnago''s global reputation for excellence in bicycle manufacturing.

For more detailed information, you can explore the following resources:

- [Colnago Super Timeline - Velo-Retro](https://www.velo-retro.com/colnagotline.html)

- [How to Identify a Colnago Super - Cycling Obsession](https://cycling-obsession.com/how-to-identify-a-colnago-super/)

- [Colnago Super - Colnago Official Website](https://www.colnago.com/en-us/collections/past-models/super)

These sources provide additional insights into the model''s specifications and its place in cycling history.
');
INSERT INTO public."Model" VALUES (1, 2, 'Mexico', 'The **[Colnago Mexico](https://www.colnago.com/en/collections/past-models/mexico)**, introduced in the mid-1970s, is a celebrated model that commemorates Eddy Merckx''s 1972 world hour record set in Mexico City. Designed to be exceptionally lightweight and responsive, the Mexico became a favorite among professional cyclists and remains highly regarded by vintage cycling enthusiasts.

![image](https://cdn.shopify.com/s/files/1/0828/8980/2039/files/05-MEXICO.jpg?v=1726567407&width=2000&height=1294&crop=center)


## Frame Design and Construction

The Mexico was constructed using Columbus Record tubing, which is thinner and lighter than the tubing used for the Colnago Super. This choice of material contributed to the frame''s reduced weight and enhanced performance. The frame featured a classic diamond geometry with several distinctive characteristics:

- **Slim Tubing**: The use of Columbus Record tubing resulted in a frame that was significantly lighter than its contemporaries, enhancing climbing and acceleration performance.

- **Classic Lugged Construction**: The frame employed traditional lugged joints, showcasing Colnago''s commitment to craftsmanship and attention to detail.

- **Refined Aesthetics**: The Mexico often featured elegant paint schemes and chrome accents, reflecting the brand''s Italian heritage and dedication to artistry in bicycle manufacturing.

These design elements collectively provided a competitive edge, aligning with the performance needs of elite cyclists.

## Variants and Evolution

Over its production span, the Colnago Mexico saw several notable variants:

- **Mexico Oro**: This exclusive version was covered in a very thin layer of 18-carat gold and came equipped with a Campagnolo Super Record group set, Nisi Solidal superlight rims, and a Brooks saddle, exemplifying luxury and performance. :contentReference[oaicite:0]{index=0}

- **Nuovo Mexico**: Introduced around 1983, the Nuovo Mexico featured crimped tubing with a single crimp on either side of the top tube and two offset crimps on each side of the down tube, totaling four crimps. This design enhanced frame stiffness and was the first to feature Colnago’s iconic club-shaped down tube. :contentReference[oaicite:1]{index=1}

These iterations demonstrate Colnago''s dedication to continuous improvement and adaptation to evolving cycling technologies and rider preferences.

## Legacy and Collectibility

The Colnago Mexico holds a prestigious place in cycling history, representing a shift towards lighter and more responsive racing bicycles. Its success in professional competitions and widespread popularity among amateur cyclists contributed significantly to Colnago''s global reputation for excellence in bicycle manufacturing. Today, the Mexico is highly sought after by collectors and vintage cycling enthusiasts, appreciated for its historical significance and exceptional ride quality.

For more detailed information, you can explore the following resources:

- [Colnago Mexico - Colnago Official Website](https://www.colnago.com/en-us/collections/past-models/mexico)

- [How to Identify a Colnago Mexico - Cycling Obsession](https://cycling-obsession.com/how-to-identify-a-colnago-mexico/)

- [Colnago Mexico 1974 Vintage Bicycle - Steel Vintage Bikes](https://steel-vintage.com/products/colnago-mexico-1974-vintage-bicycle-detail)

These sources provide additional insights into the model''s specifications and its place in cycling history.

');
INSERT INTO public."Model" VALUES (5, 2, 'Master', 'The **[Colnago Master](https://www.colnago.com/en/products/master-bike)**, introduced in 1983, stands as a testament to Colnago''s dedication to innovation and craftsmanship in the realm of high-performance steel bicycles. Designed to meet the rigorous demands of professional racing, the Master series has evolved over the years, maintaining its status as an iconic model in cycling history.

![image](https://cdn.shopify.com/s/files/1/0828/8980/2039/files/Colnago_MASTER_-_Fondo_nero-laterale_bagliore_oro_1.jpg?v=1721219303&width=2000&height=1167&crop=center)

## Frame Design and Construction

The Master series is renowned for its distinctive star-shaped tubing, developed in collaboration with Gilberto Colombo of Columbus Tubing. This unique design, known as **Gilco Master tubing**, enhances the frame''s rigidity and responsiveness, providing superior ride quality. The main features of the frame include:

- **Star-Shaped Tubing**: The top tube, down tube, and seat tube all feature the signature star-shaped cross-section, which increases torsional stiffness while maintaining a lightweight profile.

- **Lugged Construction**: The frame employs traditional lugged joints, showcasing Colnago''s commitment to meticulous craftsmanship.

- **Chromed Accents**: Many Master frames are adorned with chromed lugs and chainstays, adding both durability and aesthetic appeal.

- **Precisa Fork**: Introduced in the late 1980s, the straight-bladed Precisa fork improved handling precision and became a hallmark of the Master series.

These design elements collectively contribute to the Master’s reputation for exceptional performance and timeless elegance.

## Evolution and Variants

The Colnago Master series has seen several iterations since its inception, each introducing refinements to meet the evolving needs of cyclists:

- **Master Più**: Featured minor updates, including internal cable routing for a cleaner aesthetic and improved aerodynamics.

- **Master Olympic**: Introduced in the early 1990s, this variant incorporated updated tubing and often showcased elaborate paint schemes, reflecting the vibrant cycling culture of the era.

- **Master Light**: Focused on weight reduction without compromising structural integrity, appealing to riders seeking enhanced climbing performance.

- **Master X-Light**: The latest evolution in the Master series, continuing the legacy with modern materials and construction techniques while preserving the classic design elements that define the Master lineage.

Each variant represents Colnago''s pursuit of perfection, blending tradition with technological advancement.

## Legacy and Modern Production

The Colnago Master holds a revered place in cycling history, symbolizing the pinnacle of steel frame design. Its combination of innovative engineering and artisanal craftsmanship has made it a favorite among professional racers and cycling enthusiasts alike. Notably, the Master is still in production today, built entirely in Italy using the iconic DT15V steel tubes with the characteristic star-shaped cross-section, allowing a new generation of riders to experience its timeless performance. :contentReference[oaicite:0]{index=0}

For more detailed information, you can explore the following resources:

- [Colnago Master - Colnago Official Website](https://www.colnago.com/en/products/master-bike)

- [How to Identify a Colnago Master - Cycling Obsession](https://cycling-obsession.com/how-to-identify-a-colnago-master/)

- [Colnago Master Olympic Competition Road Bike 1990s - Steel Vintage Bikes](https://steel-vintage.com/products/colnago-master-olympic-competition-road-bike-1990s-detail)

These sources provide additional insights into the model''s specifications and its place in cycling history.
');
INSERT INTO public."Model" VALUES (12, 2, 'Master Più', 'The **[Colnago Master Più](https://www.colnago.com/en/collections/past-models/master-piu)**, introduced in 1987, is a distinguished model in Colnago''s Master series, renowned for its exceptional craftsmanship and performance-oriented design. Building upon the success of the original Master, the Master Più incorporated refinements that appealed to both professional cyclists and enthusiasts.

![image](https://cdn.shopify.com/s/files/1/0828/8980/2039/files/06-MASTERPiu.jpg?v=1726567462&width=2000&height=1294&crop=center)

## Frame Design and Construction

The Master Più features a lugged construction utilizing proprietary Columbus Gilco S4 tubing, characterized by its star-shaped cross-section. This design enhances frame rigidity and responsiveness, contributing to superior ride quality. A notable advancement in the Master Più is the internal routing of the rear brake cable through the top tube, offering a sleeker appearance and improved aerodynamics compared to the externally routed cables of earlier models. :contentReference[oaicite:0]{index=0}

## Technical Specifications

While specific configurations of the Master Più varied, common specifications included:

- **Frame Material**: Columbus Gilco S4 steel tubing

- **Fork**: Steel, designed to complement the frame''s geometry

- **Bottom Bracket**: Italian-threaded

- **Seatpost Diameter**: 27.2mm

- **Rear Spacing**: Typically 130mm, accommodating modern wheelsets

The frame was often paired with high-quality components from manufacturers like Campagnolo, enhancing its performance credentials.

## Aesthetic Details

Colnago is celebrated for its attention to aesthetic details, and the Master Più is no exception. The frame often showcases intricate paint schemes, chrome accents, and the iconic Colnago clover logo, reflecting the brand''s Italian heritage and commitment to artistry in bicycle manufacturing.

## Legacy and Collectibility

The Master Più holds a revered place in cycling history, representing the pinnacle of steel frame development before the widespread adoption of aluminum and carbon fiber. Its combination of technical innovation and timeless design makes it a sought-after model among collectors and cycling aficionados.

For a visual overview of the Colnago Master Più, you might find this restoration video informative:



For more detailed information, you can explore the following resources:

- [Colnago Master Più - Steel Vintage Bikes](https://steel-vintage.com/products/colnago-master-piu-bike-detail)

- [Colnago Master Più - Speedbicycles](https://www.speedbicycles.ch/velo/111/colnago_master_piu_1989.html)

- [Colnago Master Più - Cycling Obsession](https://cycling-obsession.com/how-to-identify-a-colnago-master/)

These sources provide additional insights into the model''s specifications and its place in cycling history.
');
INSERT INTO public."Model" VALUES (3, 2, 'Oval CX', 'The Colnago Oval CX, introduced in the early 1980s, represents a significant advancement in aerodynamic bicycle design. Developed in collaboration with Columbus, this model features distinctive oval-shaped tubing aimed at reducing aerodynamic drag and enhancing frame stiffness.

## Frame Design and Construction

The Oval CX is distinguished by several innovative design elements:

- **Oval-Shaped Tubing**: The frame utilizes Columbus Oval CX tubing with an oval cross-section, increasing vertical stiffness and improving aerodynamic efficiency.

- **Specialized Lugs**: To accommodate the unique tubing, the frame incorporates custom lugs, contributing to its distinctive appearance.

- **Internal Cable Routing**: Both shift and rear brake cables are routed internally—the shift cables through the down tube and the rear brake cable through the top tube—resulting in a cleaner aesthetic and reduced air resistance.

- **Aerodynamic Fork**: The CX fork features a flat side profile designed to minimize frontal drag, enhancing the bike''s aerodynamic performance.

- **Reverse-Mounted Rear Brake**: The rear brake is mounted inside the rear triangle, a design choice aimed at further reducing aerodynamic drag.

These features collectively contribute to the Oval CX''s reputation as a pioneering aero road bike.

## Notable Features

The Oval CX includes several distinctive characteristics:

- **Shift Lever Placement**: Shift levers are mounted on top of the down tube, a unique positioning that complements the bike''s aerodynamic design.

- **Cromovelato Finish**: Some models feature a Cromovelato paint finish, achieved by chrome-plating the entire frame and applying a tinted color coat, resulting in a reflective, glossy appearance.

- **Absence of Chainstay Bridge**: The design omits a chainstay bridge, a choice that aligns with the bike''s streamlined aesthetic.

These elements highlight Colnago''s commitment to both form and function in the Oval CX.

## Legacy and Collectibility

Produced for a limited time, the Colnago Oval CX is now considered a collector''s item, valued for its innovative design and rarity. Its unique features and historical significance make it a sought-after model among vintage bicycle enthusiasts. :contentReference[oaicite:9]{index=9}

For more detailed information, you can explore the following resources:

- [How to Identify a Colnago Oval CX - Cycling Obsession](https://cycling-obsession.com/how-to-identify-a-colnago-oval-cx/)

- [Colnago Oval CX - Colnago Official Website](https://www.colnago.com/en-us/collections/past-models/oval-cx)

- [Colnago Oval CX 1982 - Speedbicycles](https://www.speedbicycles.ch/velo/335/colnago_oval_cx_1982.html)

These sources provide additional insights into the model''s specifications and its place in cycling history.
');
INSERT INTO public."Model" VALUES (9, 4, 'Campione', 'Model z 80. rokov, Campione, bol vybavený komponentmi Campagnolo Mirage a predstavoval spoľahlivý a výkonný bicykel pre nadšencov cestnej cyklistiky. ');


--
-- Data for Name: Tag; Type: TABLE DATA; Schema: public; Owner: postgres
--



--
-- Data for Name: User; Type: TABLE DATA; Schema: public; Owner: postgres
--

INSERT INTO public."User" VALUES (2, 'nabytkovavyroba@gmail.com', 'Alexander', 'Mariančík', '$pbkdf2-sha256$i=600000,l=32$UOB/uNKLd1iRmBTTmMqjFQ$OTyclcE7FXybX7hqnBP1hVGudyxME+dqsB6jaPeQgpU', 'UOB/uNKLd1iRmBTTmMqjFQ', false);
INSERT INTO public."User" VALUES (3, 'roman.mariancik@gmail.com', 'Roman', 'Mariančík', '$pbkdf2-sha256$i=600000,l=32$UOB/uNKLd1iRmBTTmMqjFQ$OTyclcE7FXybX7hqnBP1hVGudyxME+dqsB6jaPeQgpU', 'UOB/uNKLd1iRmBTTmMqjFQ', true);
INSERT INTO public."User" VALUES (4, 'monikarehakova333@gmail.com', 'Monika', 'Reháková', '$pbkdf2-sha256$i=600000,l=32$UOB/uNKLd1iRmBTTmMqjFQ$OTyclcE7FXybX7hqnBP1hVGudyxME+dqsB6jaPeQgpU', 'UOB/uNKLd1iRmBTTmMqjFQ', false);
INSERT INTO public."User" VALUES (1, 'a@a.com', 'Acko', 'Ackove', '$pbkdf2-sha256$i=600000,l=32$UOB/uNKLd1iRmBTTmMqjFQ$OTyclcE7FXybX7hqnBP1hVGudyxME+dqsB6jaPeQgpU', 'UOB/uNKLd1iRmBTTmMqjFQ', true);


--
-- Data for Name: _sqlx_migrations; Type: TABLE DATA; Schema: public; Owner: postgres
--

INSERT INTO public._sqlx_migrations VALUES (20241211114502, 'init', '2024-12-30 00:25:07.600209+00', true, '\x3262a1aebd2d80a25ec45913307613761b99c38fb7a153161711306c8c968a678d920eaeca2246e1ee6af0a4d9413194', 42322470);
INSERT INTO public._sqlx_migrations VALUES (20241251114502, 'seed', '2024-12-30 00:25:07.662551+00', true, '\x33f7ce305ac7a3ee09fb47942aaa2e36a1c3ac0c1b1a0cf246543fc43a3214ef2c749d159c6def3f265fdf1ffe210f6f', 21211600);
INSERT INTO public._sqlx_migrations VALUES (20250101114502, 'add bike status', '2024-12-30 00:25:07.675455+00', true, '\x5af6ec73f54ac9ce5ed4cb8e787be798f3b41a7c62dbd989c52f8b1f2a544476a53fcf5e4e373df02d1134599b165395', 13832204);


--
-- Name: BikeImage_bike_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public."BikeImage_bike_id_seq"', 1, false);


--
-- Name: BikeImage_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public."BikeImage_id_seq"', 5626, true);


--
-- Name: BikeTag_bike_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public."BikeTag_bike_id_seq"', 1, false);


--
-- Name: BikeTag_tag_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public."BikeTag_tag_id_seq"', 1, false);


--
-- Name: Bike_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public."Bike_id_seq"', 58, true);


--
-- Name: Bike_model_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public."Bike_model_id_seq"', 1, false);


--
-- Name: Brand_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public."Brand_id_seq"', 5, true);


--
-- Name: Model_brand_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public."Model_brand_id_seq"', 1, false);


--
-- Name: Model_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public."Model_id_seq"', 13, true);


--
-- Name: Tag_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public."Tag_id_seq"', 1, false);


--
-- Name: User_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public."User_id_seq"', 5, false);


--
-- Name: BikeImage BikeImage_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."BikeImage"
    ADD CONSTRAINT "BikeImage_pkey" PRIMARY KEY (id);


--
-- Name: BikeTag BikeTag_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."BikeTag"
    ADD CONSTRAINT "BikeTag_pkey" PRIMARY KEY (bike_id, tag_id);


--
-- Name: Bike Bike_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Bike"
    ADD CONSTRAINT "Bike_pkey" PRIMARY KEY (id);


--
-- Name: Brand Brand_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Brand"
    ADD CONSTRAINT "Brand_pkey" PRIMARY KEY (id);


--
-- Name: Model Model_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Model"
    ADD CONSTRAINT "Model_pkey" PRIMARY KEY (id);


--
-- Name: Tag Tag_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Tag"
    ADD CONSTRAINT "Tag_pkey" PRIMARY KEY (id);


--
-- Name: User User_email_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."User"
    ADD CONSTRAINT "User_email_key" UNIQUE (email);


--
-- Name: User User_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."User"
    ADD CONSTRAINT "User_pkey" PRIMARY KEY (id);


--
-- Name: _sqlx_migrations _sqlx_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public._sqlx_migrations
    ADD CONSTRAINT _sqlx_migrations_pkey PRIMARY KEY (version);


--
-- Name: BikeImage_bike_id_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX "BikeImage_bike_id_idx" ON public."BikeImage" USING btree (bike_id);


--
-- Name: Bike_model_id_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX "Bike_model_id_idx" ON public."Bike" USING btree (model_id);


--
-- Name: Model_brand_id_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX "Model_brand_id_idx" ON public."Model" USING btree (brand_id);


--
-- Name: BikeImage BikeImage_bike_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."BikeImage"
    ADD CONSTRAINT "BikeImage_bike_id_fkey" FOREIGN KEY (bike_id) REFERENCES public."Bike"(id) ON DELETE CASCADE;


--
-- Name: BikeTag BikeTag_bike_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."BikeTag"
    ADD CONSTRAINT "BikeTag_bike_id_fkey" FOREIGN KEY (bike_id) REFERENCES public."Bike"(id) ON DELETE CASCADE;


--
-- Name: BikeTag BikeTag_tag_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."BikeTag"
    ADD CONSTRAINT "BikeTag_tag_id_fkey" FOREIGN KEY (tag_id) REFERENCES public."Tag"(id) ON DELETE CASCADE;


--
-- Name: Bike Bike_model_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Bike"
    ADD CONSTRAINT "Bike_model_id_fkey" FOREIGN KEY (model_id) REFERENCES public."Model"(id) ON DELETE CASCADE;


--
-- Name: Model Model_brand_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Model"
    ADD CONSTRAINT "Model_brand_id_fkey" FOREIGN KEY (brand_id) REFERENCES public."Brand"(id) ON DELETE CASCADE;


--
-- PostgreSQL database dump complete
--

