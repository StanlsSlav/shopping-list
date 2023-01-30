CREATE TABLE IF NOT EXISTS products (
    guid         TEXT
        PRIMARY KEY DEFAULT gen_random_uuid(),
    name         TEXT                   NOT NULL,
    price        FLOAT4                 NOT NULL,
    billing_type TEXT
        REFERENCES billing_types (guid) NOT NULL,
    qr_code      TEXT                   NOT NULL
);
