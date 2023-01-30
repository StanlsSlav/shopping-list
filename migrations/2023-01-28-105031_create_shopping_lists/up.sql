CREATE TABLE IF NOT EXISTS shopping_lists (
    guid TEXT
        PRIMARY KEY DEFAULT gen_random_uuid()
);