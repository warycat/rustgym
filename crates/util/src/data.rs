#[macro_export]
macro_rules! vec_vec_i32 {
    ($($tail:tt),*) => {
        vec![$(vec!$tail),*] as Vec<Vec<i32>>
    };
}

#[macro_export]
macro_rules! vec_string {
    ($($tail:tt),*) => {
        vec![$($tail.to_string()),*] as Vec<String>
    };
}

#[macro_export]
macro_rules! vec_vec_string {
    ($($tail:tt),*) => {
        vec![$(vec_string!$tail),*] as Vec<Vec<String>>
    };
}

#[macro_export]
macro_rules! vec_vec_char {
    ($($tail:tt),*) => {
        vec![$(vec!$tail),*] as Vec<Vec<char>>
    };
}
