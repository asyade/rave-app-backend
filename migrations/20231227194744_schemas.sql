--
-- Extensions
--

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";



--
-- Entities table
--

CREATE SEQUENCE public.entity_sid_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

CREATE TABLE public.entity (
    sid integer DEFAULT nextval('public.entity_sid_seq'::regclass) PRIMARY KEY NOT NULL,
    uid uuid DEFAULT uuid_generate_v4() NOT NULL UNIQUE
);

ALTER SEQUENCE public.entity_sid_seq OWNED BY public.entity.sid;



-- 
-- External identities table
--

CREATE SEQUENCE public.external_identity_sid_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


CREATE TABLE public.external_identity (
    sid integer DEFAULT nextval('public.external_identity_sid_seq'::regclass) PRIMARY KEY NOT NULL,
    external_user_id character varying NOT NULL UNIQUE,
    email character varying NOT NULL UNIQUE,
    entity_sid integer NOT NULL,
    name character varying DEFAULT 'unamed'::character varying NOT NULL
);

ALTER SEQUENCE public.external_identity_sid_seq OWNED BY public.external_identity.sid;

ALTER TABLE ONLY public.external_identity
    ADD CONSTRAINT external_identities_fk FOREIGN KEY (entity_sid) REFERENCES public.entity(sid) ON UPDATE CASCADE ON DELETE CASCADE;



--
-- Public user view
--

CREATE VIEW public.public_user AS
 SELECT e.sid AS entity_sid,
    ei.external_user_id,
    ei.email,
    ei.name
   FROM (public.entity e
     JOIN public.external_identity ei ON ((ei.entity_sid = e.sid)));



--
-- Asset table
--

CREATE TYPE AssetKind AS ENUM ('image', 'video', 'audio');

CREATE SEQUENCE public.asset_sid_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

CREATE TABLE public.asset(
    sid integer DEFAULT nextval('public.asset_sid_seq'::regclass) PRIMARY KEY NOT NULL,
    uid uuid DEFAULT uuid_generate_v4() NOT NULL UNIQUE,
    owner_sid integer NOT NULL,
    kind AssetKind NOT NULL,
    data jsonb NOT NULL
);

ALTER SEQUENCE public.asset_sid_seq OWNED BY public.asset.sid;



-- 
-- Content table
--

CREATE SEQUENCE public.content_sid_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

CREATE TABLE public.content (
    sid integer DEFAULT nextval('public.content_sid_seq'::regclass) PRIMARY KEY NOT NULL,
    uid uuid DEFAULT uuid_generate_v4() NOT NULL UNIQUE,
    entity_sid integer NOT NULL,
    content jsonb NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone DEFAULT now() NOT NULL
);

ALTER SEQUENCE public.content_sid_seq OWNED BY public.content.sid;



--
-- Post table
--

CREATE SEQUENCE public.post_sid_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

CREATE TABLE public.post(
    sid integer DEFAULT nextval('public.post_sid_seq'::regclass) PRIMARY KEY NOT NULL,
    content_sid integer NOT NULL,
    title character varying NOT NULL
);

ALTER SEQUENCE public.post_sid_seq OWNED BY public.post.sid;

ALTER TABLE ONLY public.post
    ADD CONSTRAINT post_fk FOREIGN KEY (content_sid) REFERENCES public.content(sid) ON UPDATE CASCADE ON DELETE CASCADE;



--
-- Post view
--

CREATE VIEW public.post_view AS
 SELECT
    c.uid AS uid,
    c.entity_sid,
    p.title,
    c.content,
    c.created_at,
    c.updated_at
   FROM (public.post p
     JOIN public.content c ON ((c.sid = p.content_sid)));