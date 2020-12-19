CREATE TABLE leetcode_solution (
    question_id       INTEGER NOT NULL
                     REFERENCES leetcode_question (id),
    filename TEXT    NOT NULL
                     PRIMARY KEY,
    source   TEXT    NOT NULL
);
