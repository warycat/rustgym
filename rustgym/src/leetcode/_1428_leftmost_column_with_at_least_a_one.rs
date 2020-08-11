#[test]
fn test() {
    unsafe {
        use rustgym_util::*;
        let data = vec_vec_i32![[0, 0], [1, 1]];
        let matrix = testcpp::BinaryMatrix::new(data);
        let res = 0;
        assert_eq!(testcpp::leftMostColumnWithOne(&matrix), res);

        let data = vec_vec_i32![[0, 0], [0, 1]];
        let matrix = testcpp::BinaryMatrix::new(data);
        let res = 1;
        assert_eq!(testcpp::leftMostColumnWithOne(&matrix), res);

        let data = vec_vec_i32![[0, 0], [0, 0]];
        let matrix = testcpp::BinaryMatrix::new(data);
        let res = -1;
        assert_eq!(testcpp::leftMostColumnWithOne(&matrix), res);

        let data = vec_vec_i32![[0, 0, 0, 1], [0, 0, 1, 1], [0, 1, 1, 1]];
        let matrix = testcpp::BinaryMatrix::new(data);
        let res = 1;
        assert_eq!(testcpp::leftMostColumnWithOne(&matrix), res);
    }
}
