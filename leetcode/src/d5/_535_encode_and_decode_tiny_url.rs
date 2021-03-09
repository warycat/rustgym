#[derive(Default)]
struct Codec {}

impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn encode(&self, long: String) -> String {
        long
    }

    fn decode(&self, short: String) -> String {
        short
    }
}

#[test]
fn test() {
    let obj = Codec::new();
    let long = "design-tinyurl";
    assert_eq!(obj.decode(obj.encode(long.to_string())), long.to_string());
}
