CREATE TABLE IF NOT EXISTS shopping_lists (
    guid TEXT
        PRIMARY KEY DEFAULT gen_random_uuid(),
    product TEXT REFERENCES products(guid),
    "user" TEXT REFERENCES users(guid),
    amount INT DEFAULT 1
);
