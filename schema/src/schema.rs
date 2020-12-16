table! {
    leetcode_description (did) {
        did -> Integer,
        filename -> Text,
        html -> Text,
    }
}

table! {
    leetcode_question (qid) {
        qid -> Integer,
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

joinable!(leetcode_description -> leetcode_question (did));
joinable!(leetcode_solution -> leetcode_question (question_id));

allow_tables_to_appear_in_same_query!(
    leetcode_description,
    leetcode_question,
    leetcode_solution,
);
