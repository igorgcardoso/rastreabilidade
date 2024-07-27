CREATE TABLE crops (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR(255) NOT NULL,
    area REAL NOT NULL,
    cultivation VARCHAR(255) NOT NULL,
    planted_at DATE NOT NULL,
    harvested_at DATE
);
