CREATE TABLE
    beverage (
        id TEXT NOT NULL PRIMARY KEY,
        description TEXT NOT NULL,
        capacity REAL NOT NULL,
        amount REAL NOT NULL
    );

CREATE TABLE
    beverage_meta (
        id TEXT NOT NULL PRIMARY KEY,
        category TEXT,
        image_uri TEXT,
        alc_percent NUMERIC,
        packaging INTEGER
    )