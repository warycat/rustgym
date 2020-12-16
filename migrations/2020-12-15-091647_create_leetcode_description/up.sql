CREATE TABLE leetcode_description (
    did       INTEGER PRIMARY KEY
                     NOT NULL
                     REFERENCES leetcode_question (qid),
    filename TEXT    NOT NULL,
    html     TEXT    NOT NULL
);
