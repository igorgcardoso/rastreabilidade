CREATE TABLE batches (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    crop_id INTEGER NOT NULL,
    classification VARCHAR(255),
    processing VARCHAR(255),
    packing VARCHAR(255) NOT NULL,
    quantity REAL NOT NULL,
    tracking_code VARCHAR(255) NOT NULL,
    date DATE NOT NULL,
    FOREIGN KEY (crop_id) REFERENCES crops (id)
);
CREATE UNIQUE INDEX tracking_code_key ON batches (tracking_code);
