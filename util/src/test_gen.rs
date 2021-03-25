#[macro_export]
macro_rules! google_test_gen {
    ($name:ident, $input:expr, $output:expr) => {
        #[test]
        fn $name() {
            let input = include_str!($input);
            let output = include_str!($output);
            let mut reader = BufReader::new(input.as_bytes());
            let mut writer = "".to_string();
            let t: usize = reader.parse_line();
            for i in 1..=t {
                solve(i, &mut reader, &mut writer);
            }
            let writer_lines: Vec<&str> = writer.lines().collect();
            let output_lines: Vec<&str> = output.lines().collect();
            assert_eq!(writer_lines.len(), output_lines.len());
            for i in 0..writer_lines.len() {
                assert_eq!(writer_lines[i], output_lines[i]);
            }
        }
    };
}

#[macro_export]
macro_rules! test_gen {
    ($name:ident, $input:expr, $output:expr) => {
        #[test]
        fn $name() {
            let input = include_str!($input);
            let output = include_str!($output);
            let mut reader = BufReader::new(input.as_bytes());
            let mut writer = "".to_string();
            solve(&mut reader, &mut writer);
            let writer_lines: Vec<&str> = writer.lines().collect();
            let output_lines: Vec<&str> = output.lines().collect();
            assert_eq!(writer_lines.len(), output_lines.len());
            for i in 0..writer_lines.len() {
                assert_eq!(writer_lines[i], output_lines[i]);
            }
        }
    };
}
