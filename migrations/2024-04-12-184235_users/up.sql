CREATE TABLE public.users
(
    id character varying(36) NOT NULL,
    email character varying(255) NOT NULL,
    password character varying(255) NOT NULL,
    photo_profile_url text NOT NULL DEFAULT 'default.png',
    phone_number character varying(16) NOT NULL,
    birth_date date NOT NULL,
    bio_data text,
    created_at timestamp with time zone NOT NULL DEFAULT current_timestamp,
    updated_at time with time zone NOT NULL DEFAULT current_timestamp,
    PRIMARY KEY (id)
);


SELECT diesel_manage_updated_at('users');
