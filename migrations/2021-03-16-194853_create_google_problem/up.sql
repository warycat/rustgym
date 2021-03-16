CREATE TABLE google_problem (
    id       INTEGER PRIMARY KEY
                     NOT NULL,
    division INTEGER NOT NULL,
    year     INTEGER NOT NULL,
    round    INTEGER NOT NULL,
    number   INTEGER NOT NULL,
    title    TEXT    NOT NULL,
    problem  TEXT    NOT NULL,
    solution TEXT    NOT NULL,
    analysis TEXT    NOT NULL
);
