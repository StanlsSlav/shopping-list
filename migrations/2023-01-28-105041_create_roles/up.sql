CREATE TABLE IF NOT EXISTS roles (
    guid TEXT
        PRIMARY KEY DEFAULT gen_random_uuid()
);