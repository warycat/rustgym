#[derive(Default)]
pub struct Codec {}

impl Codec {
    pub fn new() -> Self {
        Codec {}
    }

    pub fn encode(&self, long: String) -> String {
        long
    }

    pub fn decode(&self, short: String) -> String {
        short
    }
}

#[test]
fn test() {
    let obj = Codec::new();
    let long = "design-tinyurl";
    assert_eq!(obj.decode(obj.encode(long.to_string())), long.to_string());
}
