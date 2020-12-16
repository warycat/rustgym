table! {
    leetcode_description (id) {
        id -> Integer,
        filename -> Text,
        html -> Text,
    }
}

table! {
    leetcode_question (frontend_id) {
        id -> Integer,
        frontend_id -> Integer,
        title -> Text,
        slug -> Text,
        level -> Integer,
    }
}

table! {
    leetcode_solution (filename) {
        id -> Integer,
        filename -> Text,
        source -> Text,
    }
}

joinable!(leetcode_description -> leetcode_question (id));
joinable!(leetcode_solution -> leetcode_question (id));

allow_tables_to_appear_in_same_query!(
    leetcode_description,
    leetcode_question,
    leetcode_solution,
);
