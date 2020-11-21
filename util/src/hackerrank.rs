use std::io::*;

#[macro_export]
macro_rules! hackerrank {
    ($name:ident, $input:expr, $output:expr) => {
        #[test]
        fn $name() {
            let input = include_str!($input);
            let output = include_str!($output);
            let mut r = BufReader::new(input.as_bytes());
            let answer = solve(&mut r).unwrap();
            assert_eq!(answer, output);
        }
    };
}
