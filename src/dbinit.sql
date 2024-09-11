--
-- PostgreSQL database dump
--

-- Dumped from database version 16.3
-- Dumped by pg_dump version 16.3

-- Started on 2024-07-12 13:10:08

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
-- TOC entry 867 (class 1247 OID 16795)
-- Name: gender; Type: TYPE; Schema: public; Owner: postgres
--

CREATE TYPE public.gender AS ENUM (
    'Laki-laki',
    'Perempuan'
);


ALTER TYPE public.gender OWNER TO postgres;

--
-- TOC entry 217 (class 1259 OID 16706)
-- Name: application_order_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.application_order_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.application_order_seq OWNER TO postgres;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- TOC entry 221 (class 1259 OID 16741)
-- Name: application_ms; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.application_ms (
    application_id bigint NOT NULL,
    application_uuid character varying(128) NOT NULL,
    application_code character varying(20) NOT NULL,
    application_title character varying(100) NOT NULL,
    application_description text,
    application_order integer DEFAULT nextval('public.application_order_seq'::regclass),
    application_show boolean DEFAULT true NOT NULL,
    created_by character varying(100) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now() NOT NULL,
    updated_by character varying(100) DEFAULT ''::character varying,
    updated_at timestamp(0) without time zone,
    deleted_by character varying(100) DEFAULT ''::character varying,
    deleted_at timestamp(0) without time zone
);


ALTER TABLE public.application_ms OWNER TO postgres;

--
-- TOC entry 222 (class 1259 OID 16753)
-- Name: application_role_ms; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.application_role_ms (
    application_role_id bigint NOT NULL,
    application_role_uuid character varying(128) NOT NULL,
    application_id bigint NOT NULL,
    role_id bigint NOT NULL,
    created_by character varying(100) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now() NOT NULL,
    updated_by character varying(100) DEFAULT ''::character varying,
    updated_at timestamp(0) without time zone,
    deleted_by character varying(100) DEFAULT ''::character varying,
    deleted_at timestamp(0) without time zone
);


ALTER TABLE public.application_role_ms OWNER TO postgres;

--
-- TOC entry 216 (class 1259 OID 16705)
-- Name: division_order_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.division_order_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.division_order_seq OWNER TO postgres;

--
-- TOC entry 220 (class 1259 OID 16729)
-- Name: division_ms; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.division_ms (
    division_id bigint NOT NULL,
    division_uuid character varying(128) NOT NULL,
    division_code character varying(10) NOT NULL,
    division_title character varying(100) NOT NULL,
    division_order integer DEFAULT nextval('public.division_order_seq'::regclass),
    division_show boolean DEFAULT true NOT NULL,
    created_by character varying(100) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now() NOT NULL,
    updated_by character varying(100) DEFAULT ''::character varying,
    updated_at timestamp(0) without time zone,
    deleted_by character varying(100) DEFAULT ''::character varying,
    deleted_at timestamp(0) without time zone
);


ALTER TABLE public.division_ms OWNER TO postgres;

--
-- TOC entry 224 (class 1259 OID 16799)
-- Name: personal_data_ms; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.personal_data_ms (
    personal_id bigint NOT NULL,
    personal_uuid character varying(128) NOT NULL,
    user_id bigint NOT NULL,
    division_id bigint NOT NULL,
    personal_name character varying(128) NOT NULL,
    personal_birthday date NOT NULL,
    personal_gender public.gender NOT NULL,
    personal_phone character varying(20) NOT NULL,
    personal_address text NOT NULL
);


ALTER TABLE public.personal_data_ms OWNER TO postgres;

--
-- TOC entry 215 (class 1259 OID 16704)
-- Name: role_order_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.role_order_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.role_order_seq OWNER TO postgres;

--
-- TOC entry 219 (class 1259 OID 16717)
-- Name: role_ms; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.role_ms (
    role_id bigint NOT NULL,
    role_uuid character varying(128) NOT NULL,
    role_code character varying(10) NOT NULL,
    role_title character varying(100) NOT NULL,
    role_order integer DEFAULT nextval('public.role_order_seq'::regclass),
    role_show boolean DEFAULT true NOT NULL,
    created_by character varying(100) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now() NOT NULL,
    updated_by character varying(100) DEFAULT ''::character varying,
    updated_at timestamp(0) without time zone,
    deleted_by character varying(100) DEFAULT ''::character varying,
    deleted_at timestamp(0) without time zone
);


ALTER TABLE public.role_ms OWNER TO postgres;

--
-- TOC entry 223 (class 1259 OID 16771)
-- Name: user_application_role_ms; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.user_application_role_ms (
    user_application_role_uuid character varying(128) NOT NULL,
    user_id bigint NOT NULL,
    application_role_id bigint NOT NULL,
    division_id bigint NOT NULL,
    created_by character varying(100) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now() NOT NULL,
    updated_by character varying(100) DEFAULT ''::character varying,
    updated_at timestamp(0) without time zone,
    deleted_by character varying(100) DEFAULT ''::character varying,
    deleted_at timestamp(0) without time zone
);


ALTER TABLE public.user_application_role_ms OWNER TO postgres;

--
-- TOC entry 218 (class 1259 OID 16707)
-- Name: user_ms; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.user_ms (
    user_id bigint NOT NULL,
    user_uuid character varying(128) NOT NULL,
    user_email character varying(100) NOT NULL,
    user_password character varying NOT NULL,
    user_name character varying(200) NOT NULL,
    created_by character varying(100) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now() NOT NULL,
    updated_at timestamp(0) without time zone,
    updated_by character varying(100) DEFAULT ''::character varying,
    deleted_by character varying(100) DEFAULT ''::character varying,
    deleted_at timestamp(0) without time zone
);


ALTER TABLE public.user_ms OWNER TO postgres;

--
-- TOC entry 4911 (class 0 OID 16741)
-- Dependencies: 221
-- Data for Name: application_ms; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.application_ms (application_id, application_uuid, application_code, application_title, application_description, application_order, application_show, created_by, created_at, updated_by, updated_at, deleted_by, deleted_at) FROM stdin;
1698385309747713	46849da86a5bb-4b53-921a-05f6d8465154	APP001	Sample Application	This is a sample application description.	3	t	Super Admin	2024-07-05 10:04:13		\N		\N
1720584869003584	5484e67e-1d68-4f50-bd4e-c793bccc35d8	APP002	Web Automation	Josjis	5	t	Admin	2024-07-10 10:24:26	Admin	2024-07-10 10:27:25		\N
1720584963658022	1cd5117a-f3af-4526-9916-b0500f615639	APP003	Web Document	yayayay	6	t	Admin	2024-07-10 10:35:15	Super Admin	2024-07-10 14:28:49		\N
\.


--
-- TOC entry 4912 (class 0 OID 16753)
-- Dependencies: 222
-- Data for Name: application_role_ms; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.application_role_ms (application_role_id, application_role_uuid, application_id, role_id, created_by, created_at, updated_by, updated_at, deleted_by, deleted_at) FROM stdin;
1720513615963810	020a29d2-6843-4fa7-b548-c033e2f3e224	1698385309747713	20199990	Admin	2024-07-09 15:08:06		\N		\N
1720514295558393	98f84363-c526-4c07-88ea-5aa644771fba	1698385309747713	20199990	Admin	2024-07-09 15:11:29		\N	Admin	2024-07-10 10:26:20
1720495059555820	5b9909a5-216e-4e20-95a3-8b9b0114b401	1698385309747713	20199990	st	2024-07-09 10:03:01	admin	2024-07-09 13:22:25	Admin	2024-07-10 10:26:23
1720583139618571	788ac476-b4f3-4b5d-a303-c7cc020f775a	1720584869003584	20199991	Admin	2024-07-10 10:26:35		\N		\N
1720583754021531	1c5fbbb4-e991-4e56-ba57-3b9fe0a23172	1720584869003584	20199990	Admin	2024-07-10 10:26:43		\N		\N
1720495408063400	b7a3ecaa-1791-4edc-9b68-5b57d3b1dd46	1698385309747713	20199991	Admin	2024-07-09 09:46:24	admin	2024-07-09 13:22:17		\N
\.


--
-- TOC entry 4910 (class 0 OID 16729)
-- Dependencies: 220
-- Data for Name: division_ms; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.division_ms (division_id, division_uuid, division_code, division_title, division_order, division_show, created_by, created_at, updated_by, updated_at, deleted_by, deleted_at) FROM stdin;
1800000	3eeea63f-f3b2-4c67-a488-2a89996a62df	HC	Human Captial	2	t	Super Admin	2024-07-04 16:27:53		\N		\N
1720583137155639	7d6ba037-143f-4e1c-8049-d7c378869efc	HRD	Human Resource Development	3	t	Admin	2024-07-10 10:25:42	Super Admin	2024-07-10 14:18:21		\N
\.


--
-- TOC entry 4914 (class 0 OID 16799)
-- Dependencies: 224
-- Data for Name: personal_data_ms; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.personal_data_ms (personal_id, personal_uuid, user_id, division_id, personal_name, personal_birthday, personal_gender, personal_phone, personal_address) FROM stdin;
1720513615963810	020a29d2-6843-4fa7-b548-c033e2f3e224	1720513615963810	1800000	st	2024-07-09	Laki-laki	0867-4170-2819	jakbda
1720495059555820	5b9909a5-216e-4e20-95a3-8b9b0114b401	1720495059555820	1800000	Super Admin	2000-01-01	Laki-laki	987654365809	JL. Sama aku
\.


--
-- TOC entry 4909 (class 0 OID 16717)
-- Dependencies: 219
-- Data for Name: role_ms; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.role_ms (role_id, role_uuid, role_code, role_title, role_order, role_show, created_by, created_at, updated_by, updated_at, deleted_by, deleted_at) FROM stdin;
20199991	83c1e07d-bfb8-466f-aaec-73170eca97e2	SA	Super Admin	4	t	Super Admin	2024-07-04 16:27:43		\N		\N
20199990	11367f2c-59cb-46d0-8753-245735e109ea	A	Admin	3	t	Super Admin	2024-07-04 16:27:43		\N		\N
1720582177118374	7b955f4d-1526-478d-978f-620fddf7f47b	P	Pengguna	5	t	Admin	2024-07-10 10:26:07		\N		\N
\.


--
-- TOC entry 4913 (class 0 OID 16771)
-- Dependencies: 223
-- Data for Name: user_application_role_ms; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.user_application_role_ms (user_application_role_uuid, user_id, application_role_id, division_id, created_by, created_at, updated_by, updated_at, deleted_by, deleted_at) FROM stdin;
020a29d2-6843-4fa7-b548-c033e2f3e224	1720513615963810	1720513615963810	1800000	Admin	2024-07-09 15:08:06	st	2024-07-09 15:14:45		\N
5b9909a5-216e-4e20-95a3-8b9b0114b401	1720495059555820	1720495408063400	1800000	Nathan	2024-07-09 10:03:01	Admin	2024-07-10 11:00:32		\N
\.


--
-- TOC entry 4908 (class 0 OID 16707)
-- Dependencies: 218
-- Data for Name: user_ms; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.user_ms (user_id, user_uuid, user_email, user_password, user_name, created_by, created_at, updated_at, updated_by, deleted_by, deleted_at) FROM stdin;
1720495059555820	5b9909a5-216e-4e20-95a3-8b9b0114b401	admin@gmail.com	JDJhJDEwJGFUMHZHM01STzZ4dzhLUkxuSE8ydGVvQWprdWxWMFR2SjZtLnEyTVBoVC9OTGIvYm80RS5D	Super Admin	Admin	2024-07-09 10:03:01	2024-07-10 11:00:32	Admin		\N
1720513615963810	020a29d2-6843-4fa7-b548-c033e2f3e224	nathan@gmail.com	JDJhJDEwJFZyeFp5LkZTREE3TWJldVNSMTVmRnU3T2xMUmtUWC41MGh1b1RJSDcwVmNBOXp4eDcxaURx	nathan	Admin	2024-07-09 15:08:06	2024-07-09 15:14:45	st		\N
\.


--
-- TOC entry 4920 (class 0 OID 0)
-- Dependencies: 217
-- Name: application_order_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.application_order_seq', 6, true);


--
-- TOC entry 4921 (class 0 OID 0)
-- Dependencies: 216
-- Name: division_order_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.division_order_seq', 3, true);


--
-- TOC entry 4922 (class 0 OID 0)
-- Dependencies: 215
-- Name: role_order_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.role_order_seq', 5, true);


--
-- TOC entry 4748 (class 2606 OID 16752)
-- Name: application_ms application_ms_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.application_ms
    ADD CONSTRAINT application_ms_pkey PRIMARY KEY (application_id);


--
-- TOC entry 4750 (class 2606 OID 16760)
-- Name: application_role_ms application_role_ms_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.application_role_ms
    ADD CONSTRAINT application_role_ms_pkey PRIMARY KEY (application_role_id);


--
-- TOC entry 4746 (class 2606 OID 16740)
-- Name: division_ms division_ms_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.division_ms
    ADD CONSTRAINT division_ms_pkey PRIMARY KEY (division_id);


--
-- TOC entry 4754 (class 2606 OID 16805)
-- Name: personal_data_ms personal_data_ms_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.personal_data_ms
    ADD CONSTRAINT personal_data_ms_pkey PRIMARY KEY (personal_id);


--
-- TOC entry 4744 (class 2606 OID 16728)
-- Name: role_ms role_ms_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.role_ms
    ADD CONSTRAINT role_ms_pkey PRIMARY KEY (role_id);


--
-- TOC entry 4752 (class 2606 OID 16778)
-- Name: user_application_role_ms user_application_role_ms_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.user_application_role_ms
    ADD CONSTRAINT user_application_role_ms_pkey PRIMARY KEY (user_id, application_role_id, division_id);


--
-- TOC entry 4742 (class 2606 OID 16716)
-- Name: user_ms user_ms_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.user_ms
    ADD CONSTRAINT user_ms_pkey PRIMARY KEY (user_id);


--
-- TOC entry 4755 (class 2606 OID 16761)
-- Name: application_role_ms application_role_ms_application_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.application_role_ms
    ADD CONSTRAINT application_role_ms_application_id_fkey FOREIGN KEY (application_id) REFERENCES public.application_ms(application_id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- TOC entry 4756 (class 2606 OID 16766)
-- Name: application_role_ms application_role_ms_role_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.application_role_ms
    ADD CONSTRAINT application_role_ms_role_id_fkey FOREIGN KEY (role_id) REFERENCES public.role_ms(role_id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- TOC entry 4760 (class 2606 OID 16811)
-- Name: personal_data_ms personal_data_ms_division_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.personal_data_ms
    ADD CONSTRAINT personal_data_ms_division_id_fkey FOREIGN KEY (division_id) REFERENCES public.division_ms(division_id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- TOC entry 4761 (class 2606 OID 16806)
-- Name: personal_data_ms personal_data_ms_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.personal_data_ms
    ADD CONSTRAINT personal_data_ms_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.user_ms(user_id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- TOC entry 4757 (class 2606 OID 16784)
-- Name: user_application_role_ms user_application_role_ms_application_role_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.user_application_role_ms
    ADD CONSTRAINT user_application_role_ms_application_role_id_fkey FOREIGN KEY (application_role_id) REFERENCES public.application_role_ms(application_role_id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- TOC entry 4758 (class 2606 OID 16789)
-- Name: user_application_role_ms user_application_role_ms_division_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.user_application_role_ms
    ADD CONSTRAINT user_application_role_ms_division_id_fkey FOREIGN KEY (division_id) REFERENCES public.division_ms(division_id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- TOC entry 4759 (class 2606 OID 16779)
-- Name: user_application_role_ms user_application_role_ms_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.user_application_role_ms
    ADD CONSTRAINT user_application_role_ms_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.user_ms(user_id) ON UPDATE CASCADE ON DELETE CASCADE;


-- Completed on 2024-07-12 13:10:08

--
-- PostgreSQL database dump complete
--

