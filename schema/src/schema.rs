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
        frontend_id -> Integer,
        title -> Text,
        slug -> Text,
        level -> Integer,
    }
}

joinable!(leetcode_question -> leetcode_description (frontend_id));

allow_tables_to_appear_in_same_query!(
    leetcode_description,
    leetcode_question,
);
