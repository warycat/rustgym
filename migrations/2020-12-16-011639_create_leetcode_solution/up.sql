CREATE TABLE leetcode_solution (
    id       INTEGER NOT NULL
                     REFERENCES leetcode_question (frontend_id),
    filename TEXT    NOT NULL
                     PRIMARY KEY,
    source   TEXT    NOT NULL
);
