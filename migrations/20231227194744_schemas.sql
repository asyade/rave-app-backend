--
-- Extensions
--

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";



--
-- Entities table
--

CREATE SEQUENCE public.entities_sid_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

CREATE TABLE public.entities (
    sid integer DEFAULT nextval('public.entities_sid_seq'::regclass) PRIMARY KEY NOT NULL,
    uid uuid DEFAULT uuid_generate_v4() NOT NULL
);

ALTER SEQUENCE public.entities_sid_seq OWNED BY public.entities.sid;



-- 
-- External identities table
--

CREATE SEQUENCE public.external_identities_sid_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


CREATE TABLE public.external_identities (
    sid integer DEFAULT nextval('public.external_identities_sid_seq'::regclass) PRIMARY KEY NOT NULL,
    external_user_id character varying NOT NULL,
    email character varying NOT NULL,
    entity_sid integer NOT NULL,
    name character varying DEFAULT 'unamed'::character varying NOT NULL
);

ALTER SEQUENCE public.external_identities_sid_seq OWNED BY public.external_identities.sid;

ALTER TABLE ONLY public.external_identities
    ADD CONSTRAINT external_identities_fk FOREIGN KEY (entity_sid) REFERENCES public.entities(sid) ON UPDATE CASCADE ON DELETE CASCADE;



--
-- Public user view
--

CREATE VIEW public.public_users AS
 SELECT e.sid AS entity_sid,
    ei.external_user_id,
    ei.email,
    ei.name
   FROM (public.entities e
     JOIN public.external_identities ei ON ((ei.entity_sid = e.sid)));



-- 
-- Content table
--

CREATE SEQUENCE public.contents_sid_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

CREATE TABLE public.contents (
    sid integer DEFAULT nextval('public.contents_sid_seq'::regclass) PRIMARY KEY NOT NULL,
    uid uuid DEFAULT uuid_generate_v4() NOT NULL,
    entity_sid integer NOT NULL,
    content jsonb NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone DEFAULT now() NOT NULL
);

ALTER SEQUENCE public.contents_sid_seq OWNED BY public.contents.sid;

--
-- Posts table
--

CREATE SEQUENCE public.posts_sid_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

CREATE TABLE public.posts(
    sid integer DEFAULT nextval('public.posts_sid_seq'::regclass) PRIMARY KEY NOT NULL,
    content_sid integer NOT NULL,
    title character varying NOT NULL
);

ALTER SEQUENCE public.posts_sid_seq OWNED BY public.posts.sid;

ALTER TABLE ONLY public.posts
    ADD CONSTRAINT posts_fk FOREIGN KEY (content_sid) REFERENCES public.contents(sid) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Posts view
--

CREATE VIEW public.posts_view AS
 SELECT
    c.uid AS uid,
    c.entity_sid,
    p.title,
    c.content,
    c.created_at,
    c.updated_at
   FROM (public.posts p
     JOIN public.contents c ON ((c.sid = p.content_sid)));