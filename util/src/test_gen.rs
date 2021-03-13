#[macro_export]
macro_rules! test_gen {
    ($name:ident, $input:expr, $output:expr) => {
        #[test]
        fn $name() {
            let input = include_str!($input);
            let output = include_str!($output);
            let mut reader = BufReader::new(input.as_bytes());
            let mut writer = "".to_string();
            assert!(solve(&mut reader, &mut writer).is_ok());
            assert_eq!(writer, output);
        }
    };
}
