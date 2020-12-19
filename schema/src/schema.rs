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
    leetcode_description,
    leetcode_question,
    leetcode_solution,
);
