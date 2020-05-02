#[test]
fn test() {
    unsafe {
        testcpp::VERSION = 10;
        assert_eq!(testcpp::firstBadVersion(100), 10);
        testcpp::VERSION = 20;
        assert_eq!(testcpp::firstBadVersion(100), 20);
    }
}
