use super::Hasher;
use std::io::{MemWriter, BufWriter};

use std::vec;

pub struct SHA1
{
    priv h: [u32, ..5],
    priv data: ~[u8],
    priv length: u64,
}

impl SHA1
{
    pub fn new() -> SHA1
    {
        SHA1 {
            h: [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476, 0xc3d2e1f0],
            data: ~[],
            length: 0,
        }
    }

    fn process_block(&mut self, block: &[u8])
    {
        assert_eq!(block.len(), 64);

        let mut words = [0u32, ..80];
        for (i, chunk) in block.chunks(4).enumerate()
        {
            words[i] =
                    (chunk[3] as u32)
                |   (chunk[2] as u32 << 8)
                |   (chunk[1] as u32 << 16)
                |   (chunk[0] as u32 << 24)
            ;
        }

        let ff = |b: u32, c: u32, d: u32| d ^ (b & (c ^ d));
        let gg = |b: u32, c: u32, d: u32| b ^ c ^ d;
        let hh = |b: u32, c: u32, d: u32| (b & c) | (d & (b | c));
        let ii = |b: u32, c: u32, d: u32| b ^ c ^ d;

        let left_rotate = |x: u32, n: u32| (x << n) | (x >> (32 - n));

        for i in range(16, 80)
        {
            let n = words[i - 3] ^ words[i - 8] ^ words[i - 14] ^ words[i - 16];
            words[i] = left_rotate(n, 1);
        }

        let mut a = self.h[0];
        let mut b = self.h[1];
        let mut c = self.h[2];
        let mut d = self.h[3];
        let mut e = self.h[4];

        for i in range(0, 80)
        {
            let (f, k) = match i {
                0..19  => (ff(b, c, d), 0x5a827999),
                20..39 => (gg(b, c, d), 0x6ed9eba1),
                40..59 => (hh(b, c, d), 0x8f1bbcdc),
                60..79 => (ii(b, c, d), 0xca62c1d6),
                _ => (0, 0),
            };

            let tmp = left_rotate(a, 5) + f + e + k + words[i];
            e = d;
            d = c;
            c = left_rotate(b, 30);
            b = a;
            a = tmp;
        }

        self.h[0] += a;
        self.h[1] += b;
        self.h[2] += c;
        self.h[3] += d;
        self.h[4] += e;
    }
}

impl Hasher for SHA1
{
    fn reset(&mut self)
    {
        self.h = [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476, 0xc3d2e1f0];
        self.data = ~[];
        self.length = 0;
    }

    fn update(&mut self, data: &[u8])
    {
        let mut d = self.data.clone();
        self.data = ~[];

        d.push_all(data);

        for chunk in d.chunks(64)
        {
            self.length += chunk.len() as u64;

            if chunk.len() == 64
            {
                self.process_block(chunk);
            }
            else
            {
                self.data.push_all(chunk);
            }
        }
    }

    #[allow(unused_must_use)]
    fn output(&self, out: &mut [u8])
    {
        let mut m = SHA1 {
            h: self.h,
            data: ~[],
            length: 0,
        };

        let mut w = MemWriter::new();
        w.write(self.data);
        w.write_u8(0x80 as u8);
        w.write(vec::from_elem(56 - self.data.len() - 1, 0x00 as u8));
        w.write_be_u64(self.length * 8);
        m.process_block(w.get_ref());

        let mut w = BufWriter::new(out);
        for n in m.h.iter()
        {
            w.write_be_u32(*n);
        }
    }

    fn output_size_bits(&self) -> uint
    {
        160
    }

    fn block_size_bits(&self) -> uint
    {
        512
    }
}

#[cfg(test)]
mod test
{
    use hash::Hasher;
    use hash::sha1::SHA1;

    #[test]
    fn test_simple()
    {
        fn to_hex(data: &[u8]) -> ~str
        {
            data.map(|c| format!("{:02x}", *c)).concat()
        }

        let mut m = SHA1::new();

        let tests = [
            ("The quick brown fox jumps over the lazy dog", ~"2fd4e1c67a2d28fced849ee1bb76e7391b93eb12"),
            ("The quick brown fox jumps over the lazy cog", ~"de9f2c7fd25e1b3afad3e85a0bd17d9b100db4b3"),
            ("", ~"da39a3ee5e6b4b0d3255bfef95601890afd80709"),
        ];

        for &(s, ref h) in tests.iter()
        {
            let data = s.as_bytes();

            m.reset();
            m.update(data);

            let hh = to_hex(m.digest());

            assert_eq!(hh.len(), h.len());
            assert_eq!(hh, *h);
        }
    }
}

#[cfg(test)]
mod bench
{
    use hash::Hasher;
    use hash::sha1::SHA1;
    use test::test::BenchHarness;

    #[bench]
    fn bench_10(bh: &mut BenchHarness)
    {
        let bytes = [1u8, ..10];

        bh.iter(|| {
            let mut m = SHA1::new();
            m.reset();
            m.update(bytes);
            m.digest();
        });
        bh.bytes = bytes.len() as u64;
    }

    #[bench]
    fn bench_64(bh: &mut BenchHarness)
    {
        let bytes = [1u8, ..64];

        bh.iter(|| {
            let mut m = SHA1::new();
            m.reset();
            m.update(bytes);
            m.digest();
        });
        bh.bytes = bytes.len() as u64;
    }

    #[bench]
    fn bench_1k(bh: &mut BenchHarness)
    {
        let bytes = [1u8, ..1024];

        bh.iter(|| {
            let mut m = SHA1::new();
            m.reset();
            m.update(bytes);
            m.digest();
        });
        bh.bytes = bytes.len() as u64;
    }

    #[bench]
    fn bench_64k(bh: &mut BenchHarness)
    {
        let bytes = [1u8, ..64 * 1024];

        bh.iter(|| {
            let mut m = SHA1::new();
            m.reset();
            m.update(bytes);
            m.digest();
        });
        bh.bytes = bytes.len() as u64;
    }

    #[bench]
    fn bench_update_64(bh: &mut BenchHarness)
    {
        let bytes = [1u8, ..64];
        let mut m = SHA1::new();
        m.reset();

        bh.iter(|| {
            m.update(bytes);
        });
        bh.bytes = bytes.len() as u64;
    }

    #[bench]
    fn bench_update_64k(bh: &mut BenchHarness)
    {
        let bytes = [1u8, ..64 * 1024];
        let mut m = SHA1::new();
        m.reset();

        bh.iter(|| {
            m.update(bytes);
        });
        bh.bytes = bytes.len() as u64;
    }

    #[bench]
    fn bench_update_128k(bh: &mut BenchHarness)
    {
        let bytes = [1u8, ..2 * 64 * 1024];
        let mut m = SHA1::new();
        m.reset();

        bh.iter(|| {
            m.update(bytes);
        });
        bh.bytes = bytes.len() as u64;
    }
}
