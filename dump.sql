--
-- PostgreSQL database dump
--

-- Dumped from database version 15.0 (Debian 15.0-1.pgdg110+1)
-- Dumped by pg_dump version 15.4

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

--
-- Name: _sqlx_test; Type: SCHEMA; Schema: -; Owner: postgres
--

CREATE SCHEMA _sqlx_test;


ALTER SCHEMA _sqlx_test OWNER TO postgres;

--
-- Name: database_ids; Type: SEQUENCE; Schema: _sqlx_test; Owner: postgres
--

CREATE SEQUENCE _sqlx_test.database_ids
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE _sqlx_test.database_ids OWNER TO postgres;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: databases; Type: TABLE; Schema: _sqlx_test; Owner: postgres
--

CREATE TABLE _sqlx_test.databases (
    db_name text NOT NULL,
    test_path text NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE _sqlx_test.databases OWNER TO postgres;

--
-- Name: Active_Audiobook; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public."Active_Audiobook" (
    user_id bigint NOT NULL,
    audiobook_id bigint NOT NULL,
    playback_position double precision DEFAULT 0 NOT NULL,
    edited_at timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE public."Active_Audiobook" OWNER TO postgres;

--
-- Name: Active_Audiobook_audiobook_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public."Active_Audiobook_audiobook_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public."Active_Audiobook_audiobook_id_seq" OWNER TO postgres;

--
-- Name: Active_Audiobook_audiobook_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public."Active_Audiobook_audiobook_id_seq" OWNED BY public."Active_Audiobook".audiobook_id;


--
-- Name: Active_Audiobook_user_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public."Active_Audiobook_user_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public."Active_Audiobook_user_id_seq" OWNER TO postgres;

--
-- Name: Active_Audiobook_user_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public."Active_Audiobook_user_id_seq" OWNED BY public."Active_Audiobook".user_id;


--
-- Name: Audiobook; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public."Audiobook" (
    id bigint NOT NULL,
    genre_id bigint NOT NULL,
    author_id bigint NOT NULL,
    name text NOT NULL,
    file_path text NOT NULL,
    length double precision DEFAULT 0 NOT NULL,
    thumbnail text,
    description text NOT NULL,
    stream_count bigint DEFAULT 0 NOT NULL,
    like_count bigint DEFAULT 0 NOT NULL,
    overall_rating double precision DEFAULT 0 NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    edited_at timestamp with time zone DEFAULT now() NOT NULL,
    deleted_at timestamp with time zone
);


ALTER TABLE public."Audiobook" OWNER TO postgres;

--
-- Name: Audiobook_author_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public."Audiobook_author_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public."Audiobook_author_id_seq" OWNER TO postgres;

--
-- Name: Audiobook_author_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public."Audiobook_author_id_seq" OWNED BY public."Audiobook".author_id;


--
-- Name: Audiobook_genre_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public."Audiobook_genre_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public."Audiobook_genre_id_seq" OWNER TO postgres;

--
-- Name: Audiobook_genre_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public."Audiobook_genre_id_seq" OWNED BY public."Audiobook".genre_id;


--
-- Name: Audiobook_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public."Audiobook_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public."Audiobook_id_seq" OWNER TO postgres;

--
-- Name: Audiobook_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public."Audiobook_id_seq" OWNED BY public."Audiobook".id;


--
-- Name: Bookmark; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public."Bookmark" (
    user_id bigint NOT NULL,
    audiobook_id bigint NOT NULL,
    edited_at timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE public."Bookmark" OWNER TO postgres;

--
-- Name: Bookmark_audiobook_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public."Bookmark_audiobook_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public."Bookmark_audiobook_id_seq" OWNER TO postgres;

--
-- Name: Bookmark_audiobook_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public."Bookmark_audiobook_id_seq" OWNED BY public."Bookmark".audiobook_id;


--
-- Name: Bookmark_user_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public."Bookmark_user_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public."Bookmark_user_id_seq" OWNER TO postgres;

--
-- Name: Bookmark_user_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public."Bookmark_user_id_seq" OWNED BY public."Bookmark".user_id;


--
-- Name: Chapter; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public."Chapter" (
    id bigint NOT NULL,
    name text NOT NULL,
    audiobook_id bigint NOT NULL,
    "position" double precision DEFAULT 0 NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    edited_at timestamp with time zone DEFAULT now() NOT NULL,
    deleted_at timestamp with time zone
);


ALTER TABLE public."Chapter" OWNER TO postgres;

--
-- Name: Chapter_audiobook_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public."Chapter_audiobook_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public."Chapter_audiobook_id_seq" OWNER TO postgres;

--
-- Name: Chapter_audiobook_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public."Chapter_audiobook_id_seq" OWNED BY public."Chapter".audiobook_id;


--
-- Name: Chapter_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public."Chapter_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public."Chapter_id_seq" OWNER TO postgres;

--
-- Name: Chapter_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public."Chapter_id_seq" OWNED BY public."Chapter".id;


--
-- Name: Genre; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public."Genre" (
    id bigint NOT NULL,
    name text NOT NULL,
    color text DEFAULT '#0000DC'::text NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    edited_at timestamp with time zone DEFAULT now() NOT NULL,
    deleted_at timestamp with time zone
);


ALTER TABLE public."Genre" OWNER TO postgres;

--
-- Name: Genre_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public."Genre_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public."Genre_id_seq" OWNER TO postgres;

--
-- Name: Genre_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public."Genre_id_seq" OWNED BY public."Genre".id;


--
-- Name: Rating; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public."Rating" (
    id bigint NOT NULL,
    user_id bigint NOT NULL,
    audiobook_id bigint NOT NULL,
    rating smallint NOT NULL,
    review text,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    edited_at timestamp with time zone DEFAULT now() NOT NULL,
    deleted_at timestamp with time zone
);


ALTER TABLE public."Rating" OWNER TO postgres;

--
-- Name: Rating_audiobook_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public."Rating_audiobook_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public."Rating_audiobook_id_seq" OWNER TO postgres;

--
-- Name: Rating_audiobook_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public."Rating_audiobook_id_seq" OWNED BY public."Rating".audiobook_id;


--
-- Name: Rating_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public."Rating_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public."Rating_id_seq" OWNER TO postgres;

--
-- Name: Rating_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public."Rating_id_seq" OWNED BY public."Rating".id;


--
-- Name: Rating_user_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public."Rating_user_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public."Rating_user_id_seq" OWNER TO postgres;

--
-- Name: Rating_user_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public."Rating_user_id_seq" OWNED BY public."Rating".user_id;


--
-- Name: User; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public."User" (
    id bigint NOT NULL,
    username text NOT NULL,
    email text NOT NULL,
    name text NOT NULL,
    surname text NOT NULL,
    bio text NOT NULL,
    profile_picture text,
    password_hash text NOT NULL,
    password_salt text NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    edited_at timestamp with time zone DEFAULT now() NOT NULL,
    deleted_at timestamp with time zone
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


ALTER TABLE public."User_id_seq" OWNER TO postgres;

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
-- Name: Active_Audiobook user_id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Active_Audiobook" ALTER COLUMN user_id SET DEFAULT nextval('public."Active_Audiobook_user_id_seq"'::regclass);


--
-- Name: Active_Audiobook audiobook_id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Active_Audiobook" ALTER COLUMN audiobook_id SET DEFAULT nextval('public."Active_Audiobook_audiobook_id_seq"'::regclass);


--
-- Name: Audiobook id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Audiobook" ALTER COLUMN id SET DEFAULT nextval('public."Audiobook_id_seq"'::regclass);


--
-- Name: Audiobook genre_id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Audiobook" ALTER COLUMN genre_id SET DEFAULT nextval('public."Audiobook_genre_id_seq"'::regclass);


--
-- Name: Audiobook author_id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Audiobook" ALTER COLUMN author_id SET DEFAULT nextval('public."Audiobook_author_id_seq"'::regclass);


--
-- Name: Bookmark user_id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Bookmark" ALTER COLUMN user_id SET DEFAULT nextval('public."Bookmark_user_id_seq"'::regclass);


--
-- Name: Bookmark audiobook_id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Bookmark" ALTER COLUMN audiobook_id SET DEFAULT nextval('public."Bookmark_audiobook_id_seq"'::regclass);


--
-- Name: Chapter id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Chapter" ALTER COLUMN id SET DEFAULT nextval('public."Chapter_id_seq"'::regclass);


--
-- Name: Chapter audiobook_id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Chapter" ALTER COLUMN audiobook_id SET DEFAULT nextval('public."Chapter_audiobook_id_seq"'::regclass);


--
-- Name: Genre id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Genre" ALTER COLUMN id SET DEFAULT nextval('public."Genre_id_seq"'::regclass);


--
-- Name: Rating id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Rating" ALTER COLUMN id SET DEFAULT nextval('public."Rating_id_seq"'::regclass);


--
-- Name: Rating user_id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Rating" ALTER COLUMN user_id SET DEFAULT nextval('public."Rating_user_id_seq"'::regclass);


--
-- Name: Rating audiobook_id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Rating" ALTER COLUMN audiobook_id SET DEFAULT nextval('public."Rating_audiobook_id_seq"'::regclass);


--
-- Name: User id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."User" ALTER COLUMN id SET DEFAULT nextval('public."User_id_seq"'::regclass);


--
-- Data for Name: databases; Type: TABLE DATA; Schema: _sqlx_test; Owner: postgres
--

COPY _sqlx_test.databases (db_name, test_path, created_at) FROM stdin;
\.


--
-- Data for Name: Active_Audiobook; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public."Active_Audiobook" (user_id, audiobook_id, playback_position, edited_at) FROM stdin;
1	3	50	2024-01-26 18:43:13.002446+00
2	1	456	2024-01-26 18:43:13.002446+00
3	1	200	2024-01-26 18:43:13.002446+00
5	45	11628.174154	2024-01-27 21:59:43.874566+00
3	4	407	2024-01-26 20:19:31.960273+00
5	1	201.885195	2024-01-27 19:14:21.889024+00
5	10	20469.610586	2024-01-27 20:45:36.621436+00
1	2	4467.573696	2024-01-27 20:49:09.33824+00
5	4	409.756371	2024-01-27 19:24:28.315995+00
5	37	15437.285897	2024-01-27 19:27:44.992226+00
4	10	0	2024-01-27 20:30:08.387679+00
2	3	461.744799	2024-01-27 18:17:11.968344+00
\.


--
-- Data for Name: Audiobook; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public."Audiobook" (id, genre_id, author_id, name, file_path, length, thumbnail, description, stream_count, like_count, overall_rating, created_at, edited_at, deleted_at) FROM stdin;
2	27	2	Antigone	/static/examples/antigone.mp3	4446.702	/static/examples/antigone.jpg	Antigone is an Athenian tragedy written by Sophocles in 441 BC and first performed at the Festival of Dionysus of the same year.	1	2	4	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
1	26	1	Oliver Twist - Example	/static/examples/ot.mp3	456.378	/static/examples/ot.jpg	Oliver Twist unromantically portrays the sordid lives of criminals and exposes the cruel treatment of the many orphans in London in the mid-19th century.	6	1	3.33	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
10	13	4	"Boy" The Wandering Dog	/media/audiobook_57453d3b-e3ec-46ed-a0df-94a4040c9e6e_audio.mp3	33068.282	/media/audiobook_7ef081ad-21bc-4f4c-bd1f-8b993a435694_image.jpg	Another 'dog's-eye view' book for children by this early activist for the American Humane Society. In this tale, we follow the travels and adventures of Boy, a loveable and loyal wire-haired fox-terrier in city and country.	4	2	0	2024-01-26 21:16:47.671922+00	2024-01-27 20:48:55.282552+00	\N
27	1	5	Life on the Mississippi	/media/audiobook_53b87ae9-318a-4cbb-aff8-2564e65f7bc5_audio.mp3	53373.762	/media/audiobook_53b87ae9-318a-4cbb-aff8-2564e65f7bc5_image.jpg	Life on the Mississippi is a memoir by Mark Twain of his days as a steamboat pilot on the Mississippi River before the American Civil War published in 1883.	0	0	0	2024-01-27 18:12:41.803641+00	2024-01-27 18:12:41.803641+00	\N
28	27	5	Hamlet	/media/audiobook_88337186-e567-4fb1-b98c-ef108fe00285_audio.mp3	13935.168	/media/audiobook_88337186-e567-4fb1-b98c-ef108fe00285_image.jpg	The Tragedy of Hamlet, Prince of Denmark, often shortened to Hamlet (/ˈhæmlɪt/), is a tragedy written by William Shakespeare sometime between 1599 and 1601. It is Shakespeare's longest play, with 29,551 words. Set in Denmark, the play depicts Prince Hamlet and his attempts to exact revenge against his uncle, Claudius, who has murdered Hamlet's father in order to seize his throne and marry Hamlet's mother. Hamlet is considered among the "most powerful and influential tragedies in the English language", with a story capable of "seemingly endless retelling and adaptation by others".[1] It is widely considered one of the greatest plays of all time.[2] Three different early versions of the play are extant: the First Quarto (Q1, 1603); the Second Quarto (Q2, 1604); and the First Folio (F1, 1623). Each version includes lines and passages missing from the others.[3]\n\n	0	0	0	2024-01-27 18:20:49.128194+00	2024-01-27 18:20:49.128194+00	\N
3	3	3	Adventures of Sherlock Holmes - Example	/static/examples/sh.mp3	1662.856	/static/examples/sh.jpg	A collection of twelve short stories featuring Conan Doyle's legendary detective, originally published as single stories in Strand Magazine and subsequently collected into a single volume. There is not always a crime committed nor a culprit to find, and when there is, Holmes does not invariably get his man.	2	3	3.25	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
29	16	5	Tale of Two Cities	/media/audiobook_0d71db05-6728-4d22-92b2-50216c2928a4_audio.mp3	53740.804	/media/audiobook_0d71db05-6728-4d22-92b2-50216c2928a4_image.jpg	A Tale of Two Cities is a historical novel published in 1859 by Charles Dickens, set in London and Paris before and during the French Revolution.	0	0	0	2024-01-27 18:22:34.94824+00	2024-01-27 18:22:34.94824+00	\N
30	3	5	Adventures of Sherlock Holmes	/media/audiobook_80c5924f-0fce-452b-91e7-7586df28dce9_audio.mp3	48350.432	/media/audiobook_80c5924f-0fce-452b-91e7-7586df28dce9_image.jpg	The Adventures of Sherlock Holmes is a collection of twelve short stories by British writer Arthur Conan Doyle, first published on 14 October 1892.	0	0	0	2024-01-27 18:25:09.255137+00	2024-01-27 18:25:09.255137+00	\N
4	16	1	Tale of Two Cities - Example	/static/examples/totc.mp3	407.914	/static/examples/totc.jpg	A Tale of Two Cities is a historical novel published in 1859 by Charles Dickens, set in London and Paris before and during the French Revolution.	2	1	3	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
31	26	5	Oliver Twist	/media/audiobook_8c0e00ba-e87d-47c2-81e5-025619f96ec2_audio.mp3	61075.066	/media/audiobook_8c0e00ba-e87d-47c2-81e5-025619f96ec2_image.jpg	Oliver Twist; or, The Parish Boy's Progress, is the second novel by English author Charles Dickens. It was originally published as a serial from 1837 to 1839 and as a three-volume book in 1838.[1] The story follows the titular orphan, who, after being raised in a workhouse, escapes to London, where he meets a gang of juvenile pickpockets led by the elderly criminal Fagin, discovers the secrets of his parentage, and reconnects with his remaining family.	0	0	0	2024-01-27 18:27:11.105827+00	2024-01-27 18:27:16.172984+00	\N
32	21	5	Going Away To College	/media/audiobook_cc58ab75-fbc5-4eaa-ac75-6ee3db92c348_audio.mp3	178.542	/media/audiobook_cc58ab75-fbc5-4eaa-ac75-6ee3db92c348_image.jpg	By Blink-182	0	0	0	2024-01-27 18:28:12.875631+00	2024-01-27 18:28:12.875631+00	\N
34	15	5	A Christmas Carol	/media/audiobook_3aa81de5-fe38-4863-b81b-578c37d05bfb_audio.mp3	11616.176	/media/audiobook_68769191-e68d-4176-8be2-1be277b28291_image.jpg	A Christmas Carol. In Prose. Being a Ghost Story of Christmas, commonly known as A Christmas Carol, is a novella by Charles Dickens, first published in London by Chapman & Hall in 1843 and illustrated by John Leech. It recounts the story of Ebenezer Scrooge, an elderly miser who is visited by the ghost of his former business partner Jacob Marley and the spirits of Christmas Past, Present and Yet to Come. In the process, Scrooge is transformed into a kinder, gentler man. 	0	0	0	2024-01-27 18:35:14.432284+00	2024-01-27 18:36:32.587383+00	\N
35	16	5	The Great Gatsby	/media/audiobook_95885c85-384d-41d8-883e-ba362657fefa_audio.mp3	20188.584	/media/audiobook_95885c85-384d-41d8-883e-ba362657fefa_image.jpg	The Great Gatsby is a 1925 novel by American writer F. Scott Fitzgerald. Set in the Jazz Age on Long Island, near New York City, the novel depicts first-person narrator Nick Carraway's interactions with mysterious millionaire Jay Gatsby and Gatsby's obsession to reunite with his former lover, Daisy Buchanan. 	0	0	0	2024-01-27 18:41:22.979215+00	2024-01-27 18:41:22.979215+00	\N
36	16	5	Pride and Prejudice	/media/audiobook_5717e495-88de-41bc-b76b-c3399d76ff3f_audio.mp3	47219.04	/media/audiobook_5717e495-88de-41bc-b76b-c3399d76ff3f_image.jpg	Pride and Prejudice is the second novel by English author Jane Austen, published in 1813. A novel of manners, it follows the character development of Elizabeth Bennet, the protagonist of the book, who learns about the repercussions of hasty judgments and comes to appreciate the difference between superficial goodness and actual goodness. 	0	0	0	2024-01-27 18:53:05.179936+00	2024-01-27 18:53:05.179936+00	\N
33	26	5	The Picture of Dorian Gray	/media/audiobook_592cf3f9-b047-4c7a-be90-4d7270a882db_audio.mp3	34064.238	/media/audiobook_592cf3f9-b047-4c7a-be90-4d7270a882db_image.jpg	The Picture of Dorian Gray is a philosophical novel by Irish writer Oscar Wilde. A shorter novella-length version was published in the July 1890 issue of the American periodical Lippincott's Monthly Magazine.The novel-length version was published in April 1891. 	0	0	0	2024-01-27 18:33:19.770586+00	2024-01-27 19:05:01.599666+00	\N
38	18	5	The Oddysey	/media/audiobook_1ece8fa2-ea46-40e9-9c27-cdf77719678b_audio.mp3	42419.546	/media/audiobook_1ece8fa2-ea46-40e9-9c27-cdf77719678b_image.jpg	The Odyssey is one of two major ancient Greek epic poems attributed to Homer. It is one of the oldest extant works of literature still widely read by modern audiences. 	0	0	0	2024-01-27 19:03:32.87693+00	2024-01-27 19:04:30.049101+00	\N
37	18	5	Moby Dick	/media/audiobook_fcf94357-e622-46c0-86b5-a3f6b08c58ef_audio.mp3	88271.69	/media/audiobook_fcf94357-e622-46c0-86b5-a3f6b08c58ef_image.jpg	Moby-Dick; or, The Whale is an 1851 novel by American writer Herman Melville. The book is the sailor Ishmael's narrative of the maniacal quest of Ahab, captain of the whaling ship Pequod, for vengeance against Moby Dick, the giant white sperm whale that bit off his leg on the ship's previous voyage. A contribution to the literature of the American Renaissance, Moby-Dick was published to mixed reviews, was a commercial failure, and was out of print at the time of the author's death in 1891. Its reputation as a Great American Novel was established only in the 20th century, after the 1919 centennial of its author's birth. William Faulkner said he wished he had written the book himself, and D. H. Lawrence called it "one of the strangest and most wonderful books in the world" and "the greatest book of the sea ever written". Its opening sentence, "Call me Ishmael", is among world literature's most famous.	1	0	0	2024-01-27 18:59:05.869725+00	2024-01-27 19:15:14.098248+00	\N
43	26	4	The Agony Column	/media/audiobook_6365fc90-aa07-4478-a8ce-681121815fe8_audio.mp3	8138.234	/media/audiobook_6365fc90-aa07-4478-a8ce-681121815fe8_image.jpg	English romantic adventure starring a young American in London and inspired by the personal ads (agony columns) in the London papers. In this treacherous tale of murder and intrigue young American Geoffrey West tracks a killer from the posh dining room of the Carlton Hotel to the opium dens of London's Limehouse district in search of the truth and the heart of his true love only to find the culprit all too close to home.\n\nEarl Derr Biggers is better known as the author of numerous Charlie Chan novels,\n\nThe Agony Column was released as a movie under the name Second Floor Mystery in 1930. While this movie was in production, its two stars, Loretta Young and Grant Withers, eloped.	0	0	0	2024-01-27 19:59:36.198845+00	2024-01-27 19:59:36.198845+00	\N
44	19	4	The Game	/media/audiobook_185ca4f5-a89d-4a8a-9f53-696d134b9172_audio.mp3	6083.376	/media/audiobook_185ca4f5-a89d-4a8a-9f53-696d134b9172_image.jpg	Jack London wrote at least four stories about boxing; A Piece of Steak (1909), The Mexican (1911), The Abysmal Brute (1911), and The Game (1905). The Game is told, in part, from the point of view of a woman, the fiancée of one of the competitors. This is to be his last fight and they are to be married on the morrow. Against her better judgment, she agrees to watch the bout. (Introduction by Tom Crawford)	0	0	0	2024-01-27 20:09:22.742963+00	2024-01-27 20:09:22.742963+00	\N
45	11	4	The Adventures of Master F.J.	/media/audiobook_bf2dd0ae-b1fa-44c5-8228-71dfc619b477_audio.mp3	13075.842	/media/audiobook_bf2dd0ae-b1fa-44c5-8228-71dfc619b477_image.jpg	This story presents through letters, poems and third-person commentary the love affair between a young man named Freeman Jones and a married woman named Elinor, lady of the castle he is visiting in Scotland. Events in the affair are traced from initial attraction through seduction to (somewhat) graphic sexual encounters and their aftermath. (Allegedly based on a real-life scandal, the author, in re-issuing his story two years later, transplanted the action to Italy, renaming the principals Fernando Jeronimi and Leonora.) 	1	0	0	2024-01-27 20:23:46.075376+00	2024-01-27 20:23:46.075376+00	\N
46	11	4	Emma	/media/audiobook_29dc32df-d4c4-4d04-a7ea-b5f5d4712106_audio.mp3	64669.436	/media/audiobook_29dc32df-d4c4-4d04-a7ea-b5f5d4712106_image.jpg	Sherry reads Jane Austen’s sparkling comedy of manners with wit and vivacity, and brings the characters to life. Mr. Woodhouse worries and frets, Miss Bates chatters on, and Emma blithely manipulates and misunderstands her friends and family until she finally learns her lesson!	0	0	0	2024-01-27 21:14:11.236135+00	2024-01-27 21:14:11.236135+00	\N
\.


--
-- Data for Name: Bookmark; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public."Bookmark" (user_id, audiobook_id, edited_at) FROM stdin;
1	2	2024-01-26 18:43:13.002446+00
1	3	2024-01-26 18:43:13.002446+00
1	4	2024-01-26 18:43:13.002446+00
2	3	2024-01-26 18:43:13.002446+00
2	4	2024-01-26 18:43:13.002446+00
3	4	2024-01-26 18:43:13.002446+00
5	10	2024-01-26 21:30:21.935259+00
1	10	2024-01-27 20:48:55.280212+00
\.


--
-- Data for Name: Chapter; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public."Chapter" (id, name, audiobook_id, "position", created_at, edited_at, deleted_at) FROM stdin;
1	Introduction	2	0	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
2	Conclusion	2	2540	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
3	A	1	10	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
4	B	1	60	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
5	C	1	250	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
6	d	1	300	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
7	Adventure 1 - Intro	3	0	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
8	Adventure 1 - Story	3	100	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
9	Adventure 1 - Conclusion	3	1400	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
78	8	44	5242.363429	2024-01-27 20:10:23.285626+00	2024-01-27 20:10:23.285626+00	\N
17	I Seek and Find a Friend	10	890.278072	2024-01-26 21:18:04.240846+00	2024-01-26 21:18:04.240846+00	\N
79	9	44	5726.149297	2024-01-27 20:10:25.578022+00	2024-01-27 20:10:25.578022+00	\N
19	I Lose My Friend	10	2094.686782	2024-01-26 21:18:58.169968+00	2024-01-26 21:18:58.169968+00	\N
20	I Find A Second Friend	10	3021.155021	2024-01-26 21:19:13.579609+00	2024-01-26 21:19:13.579609+00	\N
21	My New Master	10	3947.623259	2024-01-26 21:19:28.177599+00	2024-01-26 21:19:28.177599+00	\N
22	An Old Friend, and an Adventure	10	4484.232506	2024-01-26 21:19:41.578225+00	2024-01-26 21:19:41.578225+00	\N
23	Beanie Loses his Home	10	5736.51081	2024-01-26 21:19:57.552561+00	2024-01-26 21:19:57.552561+00	\N
24	The Woman by the River	10	6080.887344	2024-01-26 21:20:09.04491+00	2024-01-26 21:20:09.04491+00	\N
25	Stanna and Napoleon	10	6988.789115	2024-01-26 21:20:30.106847+00	2024-01-26 21:20:30.106847+00	\N
26	I Meet Gringo Again	10	7583.621309	2024-01-26 21:20:39.310503+00	2024-01-26 21:20:39.310503+00	\N
27	Master Gets Two Shocks	10	8773.285698	2024-01-26 21:20:50.679647+00	2024-01-26 21:20:50.679647+00	\N
28	Napoleon and the Wasp	10	11309.149265	2024-01-26 21:21:03.700039+00	2024-01-26 21:21:03.700039+00	\N
29	The Great Secret	10	14377.231111	2024-01-26 21:21:19.922543+00	2024-01-26 21:21:19.922543+00	\N
30	The Lady Gay Cat	10	15191.212008	2024-01-26 21:21:30.416463+00	2024-01-26 21:21:30.416463+00	\N
31	His Mother's Boy	10	16067.806821	2024-01-26 21:21:38.885002+00	2024-01-26 21:21:38.885002+00	\N
32	Poor Amarilla	10	17633.154702	2024-01-26 21:21:52.56296+00	2024-01-26 21:21:52.56296+00	\N
33	To Love or Not to Love the Country	10	17977.531236	2024-01-26 21:22:03.152919+00	2024-01-26 21:22:03.152919+00	\N
34	The Arrival of the Twins	10	18948.046922	2024-01-26 21:22:12.674536+00	2024-01-26 21:22:12.674536+00	\N
35	The Showman's Dogs	10	19824.641735	2024-01-26 21:22:20.559007+00	2024-01-26 21:22:20.559007+00	\N
36	Good King Harry	10	20419.473929	2024-01-26 21:22:32.944254+00	2024-01-26 21:22:32.944254+00	\N
37	The Reformed Showman	10	21452.60353	2024-01-26 21:22:44.149212+00	2024-01-26 21:22:44.149212+00	\N
38	Master Carty's Bottle	10	22986.644453	2024-01-26 21:23:04.733876+00	2024-01-26 21:23:04.733876+00	\N
39	Mrs. Waverlee's School	10	23874.664589	2024-01-26 21:23:13.939264+00	2024-01-26 21:23:13.939264+00	\N
40	Master's Brother-Boys	10	25459.894104	2024-01-26 21:23:27.164164+00	2024-01-26 21:23:27.164164+00	\N
41	Sir Edward Medlington	10	26993.935027	2024-01-26 21:23:38.473158+00	2024-01-26 21:23:38.473158+00	\N
42	The Boy Montmorency	10	28841.045526	2024-01-26 21:23:50.647211+00	2024-01-26 21:23:50.647211+00	\N
43	The Most Painful Event of my Life	10	29534.307584	2024-01-26 21:24:00.646155+00	2024-01-26 21:24:00.646155+00	\N
44	Weary Days and a Rescue	10	31564.750838	2024-01-26 21:24:22.895411+00	2024-01-26 21:24:22.895411+00	\N
45	The Happiest Time of my Life	10	32410.038694	2024-01-26 21:24:35.306106+00	2024-01-26 21:24:35.306106+00	\N
46	My Own Dear Home	10	32879.643058	2024-01-26 21:24:45.622777+00	2024-01-26 21:24:45.622777+00	\N
80	The Printer to the Reader (Q1)	45	474.14779	2024-01-27 20:24:14.998063+00	2024-01-27 20:24:14.998063+00	\N
81	Page 1	45	575.599022	2024-01-27 20:24:32.180382+00	2024-01-27 20:24:32.180382+00	\N
82	Page 2	45	913.760068	2024-01-27 20:24:41.268129+00	2024-01-27 20:24:41.268129+00	\N
83	Page 3	45	1438.85251	2024-01-27 20:24:47.633058+00	2024-01-27 20:24:47.633058+00	\N
84	Page 5	45	2415.768682	2024-01-27 20:24:56.021213+00	2024-01-27 20:24:56.021213+00	\N
85	Page 6	45	3506.265977	2024-01-27 20:25:01.023027+00	2024-01-27 20:25:01.023027+00	\N
86	Page 7	45	6347.856276	2024-01-27 20:25:10.954986+00	2024-01-27 20:25:10.954986+00	\N
59	A	37	15437.285897	2024-01-27 19:26:16.212826+00	2024-01-27 19:26:16.212826+00	\N
60	B	37	38946.423277	2024-01-27 19:26:20.827559+00	2024-01-27 19:26:20.827559+00	\N
61	C	37	65626.159	2024-01-27 19:26:24.618456+00	2024-01-27 19:26:24.618456+00	\N
62	1	43	857.521844	2024-01-27 20:00:03.326552+00	2024-01-27 20:00:03.326552+00	\N
63	2	43	1822.753467	2024-01-27 20:00:19.068356+00	2024-01-27 20:00:19.068356+00	\N
64	3	43	2658.780857	2024-01-27 20:00:28.574504+00	2024-01-27 20:00:28.574504+00	\N
65	4	43	3662.013726	2024-01-27 20:00:39.983055+00	2024-01-27 20:00:39.983055+00	\N
66	5	43	5125.474631	2024-01-27 20:00:52.542617+00	2024-01-27 20:00:52.542617+00	\N
67	6	43	6373.646586	2024-01-27 20:01:12.052491+00	2024-01-27 20:01:12.052491+00	\N
68	7	43	7406.085117	2024-01-27 20:01:26.607391+00	2024-01-27 20:01:26.607391+00	\N
69	8	43	7806.732905	2024-01-27 20:01:34.244328+00	2024-01-27 20:01:34.244328+00	\N
70	9	43	8045.580625	2024-01-27 20:01:49.468079+00	2024-01-27 20:01:49.468079+00	\N
71	1	44	566.195522	2024-01-27 20:10:02.375847+00	2024-01-27 20:10:02.375847+00	\N
72	2	44	909.642899	2024-01-27 20:10:05.363172+00	2024-01-27 20:10:05.363172+00	\N
73	3	44	1356.837309	2024-01-27 20:10:08.330521+00	2024-01-27 20:10:08.330521+00	\N
74	4	44	2101.078329	2024-01-27 20:10:12.433204+00	2024-01-27 20:10:12.433204+00	\N
75	5	44	3112.337122	2024-01-27 20:10:15.214808+00	2024-01-27 20:10:15.214808+00	\N
76	6	44	3964.521497	2024-01-27 20:10:17.980998+00	2024-01-27 20:10:17.980998+00	\N
77	7	44	4689.465293	2024-01-27 20:10:20.82378+00	2024-01-27 20:10:20.82378+00	\N
87	Page 8	45	9448.368411	2024-01-27 20:25:15.884802+00	2024-01-27 20:25:15.884802+00	\N
88	-	46	676.118146	2024-01-27 21:14:35.39053+00	2024-01-27 21:14:35.39053+00	\N
89	-	46	2276.035101	2024-01-27 21:14:38.747999+00	2024-01-27 21:14:38.747999+00	\N
90	-	46	3357.736036	2024-01-27 21:14:41.446072+00	2024-01-27 21:14:41.446072+00	\N
91	-	46	3985.566433	2024-01-27 21:14:44.566262+00	2024-01-27 21:14:44.566262+00	\N
92	-	46	6519.027267	2024-01-27 21:14:47.605775+00	2024-01-27 21:14:47.605775+00	\N
93	-	46	7569.486638	2024-01-27 21:14:51.186756+00	2024-01-27 21:14:51.186756+00	\N
94	-	46	9052.488101	2024-01-27 21:14:54.473677+00	2024-01-27 21:14:54.473677+00	\N
95	-	46	10226.530927	2024-01-27 21:14:58.159378+00	2024-01-27 21:14:58.159378+00	\N
96	-	46	11524.157208	2024-01-27 21:15:03.870315+00	2024-01-27 21:15:03.870315+00	\N
97	-	46	14057.618042	2024-01-27 21:15:07.398067+00	2024-01-27 21:15:07.398067+00	\N
98	-	46	15355.244323	2024-01-27 21:15:10.65892+00	2024-01-27 21:15:10.65892+00	\N
99	-	46	17517.954791	2024-01-27 21:15:13.501541+00	2024-01-27 21:15:13.501541+00	\N
100	-	46	18691.997616	2024-01-27 21:15:16.144803+00	2024-01-27 21:15:16.144803+00	\N
101	-	46	20723.909068	2024-01-27 21:15:18.996134+00	2024-01-27 21:15:18.996134+00	\N
102	-	46	21781.583999	2024-01-27 21:15:23.78081+00	2024-01-27 21:15:23.78081+00	\N
103	-	46	23326.377191	2024-01-27 21:15:26.755952+00	2024-01-27 21:15:26.755952+00	\N
104	-	46	25118.337293	2024-01-27 21:15:30.237119+00	2024-01-27 21:15:30.237119+00	\N
105	-	46	28146.131948	2024-01-27 21:15:32.781055+00	2024-01-27 21:15:32.781055+00	\N
106	-	46	29876.300322	2024-01-27 21:15:35.35473+00	2024-01-27 21:15:35.35473+00	\N
107	-	46	32162.594246	2024-01-27 21:15:38.195911+00	2024-01-27 21:15:38.195911+00	\N
108	-	46	33645.59571	2024-01-27 21:15:41.43444+00	2024-01-27 21:15:41.43444+00	\N
109	-	46	34572.471624	2024-01-27 21:15:43.727236+00	2024-01-27 21:15:43.727236+00	\N
110	-	46	36735.182093	2024-01-27 21:15:45.784783+00	2024-01-27 21:15:45.784783+00	\N
111	-	46	39028.449168	2024-01-27 21:15:47.880486+00	2024-01-27 21:15:47.880486+00	\N
112	-	46	40991.74685	2024-01-27 21:15:50.19972+00	2024-01-27 21:15:50.19972+00	\N
113	-	46	43519.025556	2024-01-27 21:15:53.8026+00	2024-01-27 21:15:53.8026+00	\N
114	-	46	46512.743148	2024-01-27 21:15:56.231488+00	2024-01-27 21:15:56.231488+00	\N
115	-	46	49069.876925	2024-01-27 21:15:58.958015+00	2024-01-27 21:15:58.958015+00	\N
116	-	46	54537.740016	2024-01-27 21:16:05.860615+00	2024-01-27 21:16:05.860615+00	\N
117	-	46	55618.634157	2024-01-27 21:16:08.766333+00	2024-01-27 21:16:08.766333+00	\N
118	-	46	57676.815002	2024-01-27 21:16:11.67655+00	2024-01-27 21:16:11.67655+00	\N
119	-	46	59298.412031	2024-01-27 21:16:14.695516+00	2024-01-27 21:16:14.695516+00	\N
120	-	46	62292.129623	2024-01-27 21:16:17.393961+00	2024-01-27 21:16:17.393961+00	\N
\.


--
-- Data for Name: Genre; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public."Genre" (id, name, color, created_at, edited_at, deleted_at) FROM stdin;
1	Biography	#9ACD32	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
2	SciFi	#6495ED	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
3	Mystery	#483D8B	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
4	Fantasy	#800080	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
5	Crime	#FF4500	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
6	Horror	#8B0000	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
7	Thriller	#FFD700	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
8	Dystopian	#2E8B57	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
9	Magic Realism	#FFA500	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
10	Educational	#87CEEB	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
11	Romance	#FF69B4	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
12	Business and Economics	#4169E1	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
13	Kids	#00BFFF	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
14	Cooking	#CD853F	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
15	Fairy Tales	#FF6347	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
16	Novels	#008080	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
17	History	#8B4513	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
18	Adventure	#228B22	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
19	Sports	#FF8C00	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
20	Entertainment	#FFD700	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
21	Travel	#32CD32	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
22	Politics	#800000	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
23	Motorsport	#FF0000	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
24	Computers	#00CED1	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
25	Art	#FFD700	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
26	Fiction	#9400D3	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
27	Tragedy	#FF5E00	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
28	Documentary	#4000FF	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
\.


--
-- Data for Name: Rating; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public."Rating" (id, user_id, audiobook_id, rating, review, created_at, edited_at, deleted_at) FROM stdin;
1	1	2	4	I appreciate this book, but it feels kinda old these days.	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
2	3	2	3	I think we got past such boring stories.	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
3	2	2	5	You are mean guys.	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
4	1	1	5	Absolute banger	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
5	2	1	1	A very twisted story	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
6	3	1	4	Great	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
7	2	4	1	I did not like either of the cities	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
8	1	4	5	Great	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
9	3	4	3	Nah	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
10	2	3	1	Confusing	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
11	1	3	3	Average	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
12	3	3	5	Thrilling	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
13	4	3	4	Mysterious	2024-01-26 20:38:34.907231+00	2024-01-26 20:38:34.907231+00	\N
14	5	4	3	I got lost in both cities	2024-01-27 12:28:32.336439+00	2024-01-27 12:28:32.336439+00	\N
\.


--
-- Data for Name: User; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public."User" (id, username, email, name, surname, bio, profile_picture, password_hash, password_salt, created_at, edited_at, deleted_at) FROM stdin;
1	charlie	c@c.com	Charles	Dickens	We forge the chains we wear in life.	/static/examples/c.jpg	$pbkdf2-sha256$i=600000,l=32$xHr+sXyp5BtpPCIvIRrRvA$yjmxouyWA7I4mHhTPutuHThixR0gz7nuhYTgFJYAOYw	xHr+sXyp5BtpPCIvIRrRvA	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
2	sop	s@s.com	Sophocles	of Greece	The only true wisdom is in knowing you know nothing.	/static/examples/s.jpg	$pbkdf2-sha256$i=600000,l=32$xHr+sXyp5BtpPCIvIRrRvA$yjmxouyWA7I4mHhTPutuHThixR0gz7nuhYTgFJYAOYw	xHr+sXyp5BtpPCIvIRrRvA	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
3	archie	a@a.com	Arthur Conan	Doyle	I like detectives	/static/examples/a.jpg	$pbkdf2-sha256$i=600000,l=32$xHr+sXyp5BtpPCIvIRrRvA$yjmxouyWA7I4mHhTPutuHThixR0gz7nuhYTgFJYAOYw	xHr+sXyp5BtpPCIvIRrRvA	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
6	v	v@v.com	Vojta	Syk		\N	$pbkdf2-sha256$i=600000,l=32$xHr+sXyp5BtpPCIvIRrRvA$yjmxouyWA7I4mHhTPutuHThixR0gz7nuhYTgFJYAOYw	xHr+sXyp5BtpPCIvIRrRvA	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
7	p	p@p.com	Pavel	Koh		\N	$pbkdf2-sha256$i=600000,l=32$xHr+sXyp5BtpPCIvIRrRvA$yjmxouyWA7I4mHhTPutuHThixR0gz7nuhYTgFJYAOYw	xHr+sXyp5BtpPCIvIRrRvA	2024-01-26 18:43:13.002446+00	2024-01-26 18:43:13.002446+00	\N
5	r	r@r.com	Roman	Mar	Zoberte mu bicykel	/media/user_96f1fa33-9fcc-4c98-a3ae-c27bd83410db_image.jpeg	$pbkdf2-sha256$i=600000,l=32$xHr+sXyp5BtpPCIvIRrRvA$yjmxouyWA7I4mHhTPutuHThixR0gz7nuhYTgFJYAOYw	xHr+sXyp5BtpPCIvIRrRvA	2024-01-26 18:43:13.002446+00	2024-01-27 18:24:10.6038+00	\N
4	n	n@n.com	Ninka	Rybka		/media/user_911a0e15-50f2-45e6-a881-23281124622b_image.jpg	$pbkdf2-sha256$i=600000,l=32$xHr+sXyp5BtpPCIvIRrRvA$yjmxouyWA7I4mHhTPutuHThixR0gz7nuhYTgFJYAOYw	xHr+sXyp5BtpPCIvIRrRvA	2024-01-26 18:43:13.002446+00	2024-01-27 19:31:44.447504+00	\N
\.


--
-- Data for Name: _sqlx_migrations; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public._sqlx_migrations (version, description, installed_on, success, checksum, execution_time) FROM stdin;
20231119114802	init	2024-01-26 18:43:12.487205+00	t	\\x5579e964401793ec2b5c37314505c51f0c53dffa6e30a6db5e73e150ca7b63b6055abcc5f02520862ceacf96b4114e32	452893626
20240126171646	seeding	2024-01-26 23:36:39.65056+00	t	\\x529a05b2b16194ac02c7334907147d86cc3e08256caa4ca0ba7954dfd820a85c33c641ce21d85243e88ff4ccedf113ea	144426095
\.


--
-- Name: database_ids; Type: SEQUENCE SET; Schema: _sqlx_test; Owner: postgres
--

SELECT pg_catalog.setval('_sqlx_test.database_ids', 365, true);


--
-- Name: Active_Audiobook_audiobook_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public."Active_Audiobook_audiobook_id_seq"', 1, false);


--
-- Name: Active_Audiobook_user_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public."Active_Audiobook_user_id_seq"', 1, false);


--
-- Name: Audiobook_author_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public."Audiobook_author_id_seq"', 1, false);


--
-- Name: Audiobook_genre_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public."Audiobook_genre_id_seq"', 1, false);


--
-- Name: Audiobook_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public."Audiobook_id_seq"', 46, true);


--
-- Name: Bookmark_audiobook_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public."Bookmark_audiobook_id_seq"', 1, false);


--
-- Name: Bookmark_user_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public."Bookmark_user_id_seq"', 1, false);


--
-- Name: Chapter_audiobook_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public."Chapter_audiobook_id_seq"', 1, false);


--
-- Name: Chapter_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public."Chapter_id_seq"', 120, true);


--
-- Name: Genre_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public."Genre_id_seq"', 29, false);


--
-- Name: Rating_audiobook_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public."Rating_audiobook_id_seq"', 1, false);


--
-- Name: Rating_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public."Rating_id_seq"', 14, true);


--
-- Name: Rating_user_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public."Rating_user_id_seq"', 1, false);


--
-- Name: User_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public."User_id_seq"', 8, false);


--
-- Name: databases databases_pkey; Type: CONSTRAINT; Schema: _sqlx_test; Owner: postgres
--

ALTER TABLE ONLY _sqlx_test.databases
    ADD CONSTRAINT databases_pkey PRIMARY KEY (db_name);


--
-- Name: Active_Audiobook Active_Audiobook_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Active_Audiobook"
    ADD CONSTRAINT "Active_Audiobook_pkey" PRIMARY KEY (user_id, audiobook_id);


--
-- Name: Audiobook Audiobook_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Audiobook"
    ADD CONSTRAINT "Audiobook_pkey" PRIMARY KEY (id);


--
-- Name: Bookmark Bookmark_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Bookmark"
    ADD CONSTRAINT "Bookmark_pkey" PRIMARY KEY (user_id, audiobook_id);


--
-- Name: Chapter Chapter_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Chapter"
    ADD CONSTRAINT "Chapter_pkey" PRIMARY KEY (id);


--
-- Name: Genre Genre_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Genre"
    ADD CONSTRAINT "Genre_pkey" PRIMARY KEY (id);


--
-- Name: Rating Rating_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Rating"
    ADD CONSTRAINT "Rating_pkey" PRIMARY KEY (id);


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
-- Name: User User_username_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."User"
    ADD CONSTRAINT "User_username_key" UNIQUE (username);


--
-- Name: _sqlx_migrations _sqlx_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public._sqlx_migrations
    ADD CONSTRAINT _sqlx_migrations_pkey PRIMARY KEY (version);


--
-- Name: databases_created_at; Type: INDEX; Schema: _sqlx_test; Owner: postgres
--

CREATE INDEX databases_created_at ON _sqlx_test.databases USING btree (created_at);


--
-- Name: Audiobook_author_id_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX "Audiobook_author_id_idx" ON public."Audiobook" USING btree (author_id);


--
-- Name: Audiobook_genre_id_id_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX "Audiobook_genre_id_id_idx" ON public."Audiobook" USING btree (genre_id);


--
-- Name: Chapter_audiobook_id_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX "Chapter_audiobook_id_idx" ON public."Chapter" USING btree (audiobook_id);


--
-- Name: Rating_audiobook_id_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX "Rating_audiobook_id_idx" ON public."Rating" USING btree (audiobook_id);


--
-- Name: Rating_userid_id_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX "Rating_userid_id_idx" ON public."Rating" USING btree (user_id);


--
-- Name: Active_Audiobook Active_Audiobook_audiobook_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Active_Audiobook"
    ADD CONSTRAINT "Active_Audiobook_audiobook_id_fkey" FOREIGN KEY (audiobook_id) REFERENCES public."Audiobook"(id) ON DELETE CASCADE;


--
-- Name: Active_Audiobook Active_Audiobook_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Active_Audiobook"
    ADD CONSTRAINT "Active_Audiobook_user_id_fkey" FOREIGN KEY (user_id) REFERENCES public."User"(id) ON DELETE CASCADE;


--
-- Name: Audiobook Audiobook_author_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Audiobook"
    ADD CONSTRAINT "Audiobook_author_id_fkey" FOREIGN KEY (author_id) REFERENCES public."User"(id) ON DELETE CASCADE;


--
-- Name: Audiobook Audiobook_genre_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Audiobook"
    ADD CONSTRAINT "Audiobook_genre_id_fkey" FOREIGN KEY (genre_id) REFERENCES public."Genre"(id) ON DELETE CASCADE;


--
-- Name: Bookmark Bookmark_audiobook_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Bookmark"
    ADD CONSTRAINT "Bookmark_audiobook_id_fkey" FOREIGN KEY (audiobook_id) REFERENCES public."Audiobook"(id) ON DELETE CASCADE;


--
-- Name: Bookmark Bookmark_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Bookmark"
    ADD CONSTRAINT "Bookmark_user_id_fkey" FOREIGN KEY (user_id) REFERENCES public."User"(id) ON DELETE CASCADE;


--
-- Name: Chapter Chapter_audiobook_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Chapter"
    ADD CONSTRAINT "Chapter_audiobook_id_fkey" FOREIGN KEY (audiobook_id) REFERENCES public."Audiobook"(id) ON DELETE CASCADE;


--
-- Name: Rating Rating_audiobook_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Rating"
    ADD CONSTRAINT "Rating_audiobook_id_fkey" FOREIGN KEY (audiobook_id) REFERENCES public."Audiobook"(id) ON DELETE CASCADE;


--
-- Name: Rating Rating_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Rating"
    ADD CONSTRAINT "Rating_user_id_fkey" FOREIGN KEY (user_id) REFERENCES public."User"(id) ON DELETE CASCADE;


--
-- PostgreSQL database dump complete
--

