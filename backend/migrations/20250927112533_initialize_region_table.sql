CREATE TABLE region_history
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    region     TEXT NOT NULL,
    start_time TEXT NOT NULL,
    stop_time  TEXT,
    duration   INTEGER,
    CONSTRAINT valid_region CHECK (region IN ('aa1', 'aa2', 'aa3', 'ac1', 'ac2', 'ac3'))
);