CREATE TABLE IF NOT EXISTS Recipes
(
    id   SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS Malts
(
    id   SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS RecipeMalts
(
    id        SERIAL PRIMARY KEY,
    recipe_id INTEGER REFERENCES Recipes(id),
    malt_id   INTEGER REFERENCES Malts(id),
    grams     DECIMAL
);
