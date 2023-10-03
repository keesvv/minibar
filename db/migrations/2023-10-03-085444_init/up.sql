CREATE TABLE
    Beverage (
        id INTEGER NOT NULL PRIMARY KEY,
        description NOT NULL,
        capacity NUMERIC NOT NULL,
        amount NUMERIC NOT NULL
    );

CREATE TABLE
    Beverage_Metadata (
        id INTEGER NOT NULL PRIMARY KEY,
        category TEXT,
        image_uri TEXT,
        alc_percent NUMERIC,
        packaging INTEGER
    )