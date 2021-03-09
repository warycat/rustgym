#[macro_export]
macro_rules! adventofcode {
    ($name:ident, $input:expr, $output:expr) => {
        #[test]
        fn $name() {
            let input = include_str!($input);
            let output = include_str!($output);
            let mut reader = BufReader::new(input.as_bytes());
            let mut writer = "".to_string();
            solve(&mut reader, &mut writer);
            assert_eq!(writer, output);
        }
    };
}

#[macro_export]
macro_rules! adventofcode_ignore {
    ($name:ident, $input:expr, $output:expr) => {
        #[test]
        #[ignore]
        fn $name() {
            let input = include_str!($input);
            let output = include_str!($output);
            let mut reader = BufReader::new(input.as_bytes());
            let mut writer = "".to_string();
            solve(&mut reader, &mut writer);
            assert_eq!(writer, output);
        }
    };
}
