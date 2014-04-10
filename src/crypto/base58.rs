use std::str;

use num::bigint::BigUint;
use num::bigint::ToBigUint;

static BASE58_ALPHABET: &'static str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

pub fn to_base58(data: &[u8]) -> ~str
{
    let mut s = str::from_utf8(data).unwrap();
    let origlen = s.len();
    s = s.trim_left_chars(&'\0');
    let newlen = s.len();

    let mut num = BigUint::new(Vec::new());
    let mut p = 1u.to_biguint().expect("wtf");
    for c in s.bytes_rev()
    {
        let mut n = c.to_biguint().expect("wtf");
        n = n * p;
        num = num + n;
        p = p << 8;
    }

    let mut result = ~"";
    while num >= 58u.to_biguint().expect("wtf")
    {
        let d = num / 58u.to_biguint().expect("wtf");
        let r = num % 58u.to_biguint().expect("wtf");

        num = d;
        result.push_char(BASE58_ALPHABET[r.to_uint().expect("wtf")] as char);
    }

    result.push_char(BASE58_ALPHABET[num.to_uint().expect("wtf")] as char);

    let fill = Vec::from_elem(origlen - newlen, BASE58_ALPHABET[0] as char);
    result.push_str(str::from_chars(fill.as_slice()));

    fn str_reverse(s: &str) -> ~str
    {
        s.chars_rev().collect()
    }
    str_reverse(result)
}

#[cfg(test)]
mod test
{
    use super::to_base58;

    #[test]
    fn test_simple()
    {
        let s = "hello world";
        let r = to_base58(s.as_bytes());

        assert_eq!(r, ~"StV1DL6CwTryKyV");
    }

    #[test]
    fn test_padding()
    {
        let s = "\0\0\0hello world";
        let r = to_base58(s.as_bytes());

        assert_eq!(r, ~"111StV1DL6CwTryKyV");
    }
}

#[cfg(test)]
mod bench
{
    use super::to_base58;
    use test::test::BenchHarness;

    #[bench]
    fn bench_simple(bh: &mut BenchHarness)
    {
        let s = "hello world";

        bh.iter(|| {
            to_base58(s.as_bytes());
        });
        bh.bytes = s.len() as u64;
    }
}
