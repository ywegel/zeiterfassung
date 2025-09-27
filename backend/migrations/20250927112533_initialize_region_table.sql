CREATE TABLE region_history
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    region     TEXT NOT NULL,
    start_time TEXT NOT NULL,
    stop_time  TEXT,
    duration   INTEGER,
    CONSTRAINT valid_region CHECK (region IN ('north', 'south', 'east', 'west'))
);