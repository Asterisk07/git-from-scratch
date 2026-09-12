pub type ByteString = Vec<u8>;
pub type ByteSlice<'a> = &'a [u8];

pub trait GitObject {
    const TYPE: &'static str;
    fn dump(&self) -> ByteString;
    fn load(&mut self, data: ByteString);
}
