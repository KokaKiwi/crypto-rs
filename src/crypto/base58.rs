use num::bigint::ToBigUint;
use num::Integer;

static BASE58_ALPHABET: &'static str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

pub fn to_base58(data: &[u8]) -> String
{
    let mut n = 0u.to_biguint().unwrap();
    for (i, c) in data.iter().rev().enumerate() {
        let c = c.to_biguint().unwrap();

        n =n + (c << (i * 8));
    }

    let mut result = String::new();
    let limit = 58u.to_biguint().unwrap();
    while n >= limit {
        let (d, r) = n.div_rem(&limit);
        let r = r.to_uint().unwrap();

        n = d;
        result.push_char(BASE58_ALPHABET[r] as char);
    }
    let r = n.to_uint().unwrap();

    if r > 0 {
        result.push_char(BASE58_ALPHABET[r] as char);
    }

    for c in data.iter() {
        if *c == 0 {
            result.push_char(BASE58_ALPHABET[0] as char);
        } else {
            break;
        }
    }

    result.as_slice().chars().rev().collect()
}

#[cfg(test)]
mod test
{
    use super::to_base58;

    #[test]
    fn test_simple()
    {
        let tests = [
            ("hello world".as_bytes(), "StV1DL6CwTryKyV"),
            ("Hello World".as_bytes(), "JxF12TrwUP45BMd"),
        ];

        for &(s, n) in tests.iter() {
            let r = to_base58(s);

            assert_eq!(r.as_slice(), n);
        }
    }
}

#[cfg(test)]
mod bench
{
    use super::to_base58;
    use test::test::Bencher;

    #[bench]
    fn bench_simple(bh: &mut Bencher)
    {
        let s = "hello world";

        bh.iter(|| {
            to_base58(s.as_bytes());
        });
        bh.bytes = s.len() as u64;
    }
}
