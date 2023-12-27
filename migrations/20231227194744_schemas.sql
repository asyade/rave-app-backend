--
-- Extensions
--

CREATE EXTENSION IF NOT EXISTS "uuid-ossp" WITH SCHEMA public;
COMMENT ON EXTENSION "uuid-ossp" IS 'generate universally unique identifiers (UUIDs)';

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
    uid uuid DEFAULT public.uuid_generate_v4() NOT NULL
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

