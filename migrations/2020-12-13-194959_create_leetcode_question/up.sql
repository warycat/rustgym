CREATE TABLE leetcode_question (
    id          INTEGER NOT NULL,
    frontend_id INTEGER NOT NULL PRIMARY KEY,
    title       TEXT    NOT NULL,
    slug        TEXT    NOT NULL,
    level       INTEGER NOT NULL
);
