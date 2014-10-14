pub mod md5;
pub mod sha1;

pub trait Hasher
{
    /**
     * Reset the hasher's state.
     */
    fn reset(&mut self);

    /**
     * Provide input data.
     */
    fn update(&mut self, data: &[u8]);

    /**
     * Retrieve digest result. The output must be large enough to contains result
     * size (from output_size method).
     */
    fn output(&self, out: &mut [u8]);

    /**
     * Get the output size in bits.
     */
    fn output_size_bits(&self) -> uint;

    /**
     * Get the block size in bits.
     */
    fn block_size_bits(&self) -> uint;

    /**
     * Get the output size in bytes.
     */
    fn output_size(&self) -> uint
    {
        (self.output_size_bits() + 7) / 8
    }

    /**
     * Get the block size in bytes.
     */
    fn block_size(&self) -> uint
    {
        (self.block_size_bits() + 7) / 8
    }

    fn digest(&self) -> Vec<u8>
    {
        let size = self.output_size();
        let mut buf = Vec::from_elem(size, 0u8);

        self.output(buf.as_mut_slice());

        buf
    }
}

pub trait Hashable {
    /**
     * Feed the value to the hasher passed in parameter.
     */
     fn feed<H: Hasher>(&self, h: &mut H);

    /**
     * Hash the value to ~[u8].
     *
     * Reset the hasher passed in parameter, because we want
     * an empty hasher to get only the value's hash.
     */
    fn to_hash<H: Hasher>(&self, h: &mut H) -> Vec<u8>
    {
        h.reset();
        self.feed(h);
        h.digest()
    }
}

impl<'a> Hashable for &'a [u8] {
    fn feed<H: Hasher>(&self, h: &mut H)
    {
        h.update(*self)
    }
}
