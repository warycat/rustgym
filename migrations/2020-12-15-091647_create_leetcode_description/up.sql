CREATE TABLE leetcode_description (
    id       INTEGER PRIMARY KEY
                     NOT NULL
                     REFERENCES leetcode_question (id),
    filename TEXT    NOT NULL,
    html     TEXT    NOT NULL
);
