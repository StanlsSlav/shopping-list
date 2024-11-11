CREATE TABLE IF NOT EXISTS billing_types (
    guid TEXT
        PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL
        UNIQUE
);