#[macro_export]
macro_rules! advent_of_code {
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
