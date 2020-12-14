CREATE TABLE leetcode_question (
    id          INTEGER PRIMARY KEY
                        NOT NULL,
    frontend_id INTEGER NOT NULL,
    title       TEXT    NOT NULL,
    slug        TEXT    NOT NULL,
    level       INTEGER NOT NULL
);
