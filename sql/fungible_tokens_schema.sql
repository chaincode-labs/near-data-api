-- Table: public.assets__fungible_tokens

-- DROP TABLE IF EXISTS public.assets__fungible_tokens;

CREATE TABLE IF NOT EXISTS public.assets__fungible_tokens
(
    token_id text COLLATE pg_catalog."default" NOT NULL,
    total_supply text COLLATE pg_catalog."default" DEFAULT 0,
    burn_amount text COLLATE pg_catalog."default" DEFAULT 0,
    holder_count integer DEFAULT 0,
    transaction_count integer DEFAULT 0,
    transaction_amount text COLLATE pg_catalog."default" DEFAULT 0,
    metadata text COLLATE pg_catalog."default",
    CONSTRAINT assets__fungible_tokens_pkey PRIMARY KEY (token_id)
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.assets__fungible_tokens
    OWNER to postgres;