table! {
    adventofcode_description (id) {
        id -> Integer,
        year -> Integer,
        day -> Integer,
        title -> Text,
        filename -> Text,
        html -> Text,
    }
}

table! {
    adventofcode_solution (id) {
        id -> Integer,
        year -> Integer,
        day -> Integer,
        filename -> Text,
        source -> Text,
    }
}

table! {
    google_problem (id) {
        id -> Integer,
        division -> Integer,
        year -> Integer,
        round -> Integer,
        number -> Integer,
        title -> Text,
        problem -> Text,
        input -> Text,
        output -> Text,
        solution -> Text,
        analysis -> Text,
    }
}

table! {
    leetcode_description (id) {
        id -> Integer,
        filename -> Text,
        html -> Text,
    }
}

table! {
    leetcode_question (id) {
        id -> Integer,
        title -> Text,
        slug -> Text,
        level -> Integer,
    }
}

table! {
    leetcode_solution (filename) {
        question_id -> Integer,
        filename -> Text,
        source -> Text,
    }
}

joinable!(leetcode_description -> leetcode_question (id));
joinable!(leetcode_solution -> leetcode_question (question_id));

allow_tables_to_appear_in_same_query!(
    adventofcode_description,
    adventofcode_solution,
    google_problem,
    leetcode_description,
    leetcode_question,
    leetcode_solution,
);
