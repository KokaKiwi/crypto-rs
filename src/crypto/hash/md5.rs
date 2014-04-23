use std::io::{MemWriter, BufWriter};

use super::Hasher;

static r: [u32, ..64]=  [
    7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,
    5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,
    4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,
    6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21,
];

static k: [u32, ..64] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
];

pub struct MD5
{
    h: [u32, ..4],
    data: Vec<u8>,
    length: u64,
}

impl MD5
{
    pub fn new() -> MD5
    {
        MD5 {
            h: [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476],
            data: Vec::new(),
            length: 0,
        }
    }

    fn process_block(&mut self, block: &[u8])
    {
        assert_eq!(block.len(), 64);

        let mut words = [0u32, ..16];
        for (i, chunk) in block.chunks(4).enumerate()
        {
            words[i] =
                    (chunk[0] as u32)
                |   (chunk[1] as u32 << 8)
                |   (chunk[2] as u32 << 16)
                |   (chunk[3] as u32 << 24)
            ;
        }

        let ff = |b: u32, c: u32, d: u32| d ^ (b & (c ^ d));
        let gg = |b: u32, c: u32, d: u32| c ^ (d & (b ^ c));
        let hh = |b: u32, c: u32, d: u32| (b ^ c ^ d);
        let ii = |b: u32, c: u32, d: u32| (c ^ (b | !d));

        let left_rotate = |x: u32, n: u32| (x << n) | (x >> (32 - n));

        let h = self.h;
        let (mut a, mut b, mut c, mut d) = (h[0], h[1], h[2], h[3]);

        for i in range(0u, 64u)
        {
            let (f, g) = match i {
                0..15   => (ff(b, c, d), i),
                16..31  => (gg(b, c, d), (5 * i + 1) % 16),
                32..47  => (hh(b, c, d), (3 * i + 5) % 16),
                48..63  => (ii(b, c, d), (7 * i) % 16),
                _ => (0, 0),
            };

            let tmp = d;
            d = c;
            c = b;
            b = left_rotate(a + f + k[i] + words[g], r[i]) + b;
            a = tmp;
        }

        self.h[0] += a;
        self.h[1] += b;
        self.h[2] += c;
        self.h[3] += d;
    }
}

impl Hasher for MD5
{
    fn reset(&mut self)
    {
        self.h = [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476];
        self.data.clear();
        self.length = 0;
    }

    fn update(&mut self, data: &[u8])
    {
        let mut d = self.data.clone();
        self.data.clear();

        d.push_all(data);

        for chunk in d.as_slice().chunks(64)
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
        let mut m = MD5 {
            h: self.h,
            data: Vec::new(),
            length: 0,
        };

        let mut w = MemWriter::new();
        w.write(self.data.as_slice());
        w.write_u8(0x80);
        w.write(Vec::from_elem(56 - self.data.len() - 1, 0x00 as u8).as_slice());
        w.write_le_u64(self.length * 8);
        m.process_block(w.get_ref());

        let mut w = BufWriter::new(out);
        for n in m.h.iter()
        {
            w.write_le_u32(*n);
        }
    }

    fn output_size_bits(&self) -> uint
    {
        128
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
    use hash::md5::MD5;

    use serialize::hex::ToHex;

    #[test]
    fn test_simple()
    {
        let tests = [
            ("The quick brown fox jumps over the lazy dog", ~"9e107d9d372bb6826bd81d3542a419d6"),
            ("The quick brown fox jumps over the lazy dog.", ~"e4d909c290d0fb1ca068ffaddf22cbd0"),
            ("", ~"d41d8cd98f00b204e9800998ecf8427e"),
        ];

        let mut m = MD5::new();

        for &(s, ref h) in tests.iter()
        {
            let data = s.as_bytes();

            m.reset();
            m.update(data);

            let hh = m.digest().to_hex();

            assert_eq!(hh.len(), h.len());
            assert_eq!(hh, *h);
        }
    }
}

#[cfg(test)]
mod bench
{
    use hash::Hasher;
    use hash::md5::MD5;
    use test::test::Bencher;

    #[bench]
    fn bench_10(bh: &mut Bencher)
    {
        let bytes = [1u8, ..10];

        bh.iter(|| {
            let mut m = MD5::new();
            m.reset();
            m.update(bytes);
            m.digest();
        });
        bh.bytes = bytes.len() as u64;
    }

    #[bench]
    fn bench_64(bh: &mut Bencher)
    {
        let bytes = [1u8, ..64];

        bh.iter(|| {
            let mut m = MD5::new();
            m.reset();
            m.update(bytes);
            m.digest();
        });
        bh.bytes = bytes.len() as u64;
    }

    #[bench]
    fn bench_1k(bh: &mut Bencher)
    {
        let bytes = [1u8, ..1024];

        bh.iter(|| {
            let mut m = MD5::new();
            m.reset();
            m.update(bytes);
            m.digest();
        });
        bh.bytes = bytes.len() as u64;
    }

    #[bench]
    fn bench_64k(bh: &mut Bencher)
    {
        let bytes = [1u8, ..64 * 1024];

        bh.iter(|| {
            let mut m = MD5::new();
            m.reset();
            m.update(bytes);
            m.digest();
        });
        bh.bytes = bytes.len() as u64;
    }

    #[bench]
    fn bench_update_64(bh: &mut Bencher)
    {
        let bytes = [1u8, ..64];
        let mut m = MD5::new();
        m.reset();

        bh.iter(|| {
            m.update(bytes);
        });
        bh.bytes = bytes.len() as u64;
    }

    #[bench]
    fn bench_update_64k(bh: &mut Bencher)
    {
        let bytes = [1u8, ..64 * 1024];
        let mut m = MD5::new();
        m.reset();

        bh.iter(|| {
            m.update(bytes);
        });
        bh.bytes = bytes.len() as u64;
    }

    #[bench]
    fn bench_update_128k(bh: &mut Bencher)
    {
        let bytes = [1u8, ..2 * 64 * 1024];
        let mut m = MD5::new();
        m.reset();

        bh.iter(|| {
            m.update(bytes);
        });
        bh.bytes = bytes.len() as u64;
    }
}
