struct Codec;

impl Codec {
    fn new() -> Self {
        Codec {}
    }
    fn encode(&self, strs: Vec<String>) -> String {
        let n = strs.len();
        let mut res: String = "".to_string();
        let v = Self::encode_usize(n);
        res.push_str(&v);
        for s in strs {
            let n = s.len();
            let v = Self::encode_usize(n);
            res.push_str(&v);
            res.push_str(&s);
        }
        res
    }

    fn decode(&self, s: String) -> Vec<String> {
        let mut res = vec![];
        let v: Vec<char> = s.chars().collect();
        let n = Self::decode_usize(&v[0..4]);
        let mut index = 4;
        for _ in 0..n {
            let m = Self::decode_usize(&v[index..index + 4]);
            index += 4;
            let ss: String = v[index..index + m].iter().collect();
            index += m;
            res.push(ss);
        }
        res
    }

    fn encode_usize(x: usize) -> String {
        let x = x as u32;
        vec![
            (x >> 24 & 0xff) as u8 as char,
            (x >> 16 & 0xff) as u8 as char,
            (x >> 8 & 0xff) as u8 as char,
            (x & 0xff) as u8 as char,
        ]
        .into_iter()
        .collect()
    }

    fn decode_usize(v: &[char]) -> usize {
        ((v[0] as u32) << 24 | (v[1] as u32) << 16 | (v[2] as u32) << 8 | (v[3] as u32)) as usize
    }
}

#[test]
fn test() {
    let obj = Codec::new();
    let strs = vec_string!("123", "321");
    let encoded = obj.encode(strs);
    let decoded = obj.decode(encoded);
    let res = vec_string!("123", "321");
    assert_eq!(decoded, res);
}
