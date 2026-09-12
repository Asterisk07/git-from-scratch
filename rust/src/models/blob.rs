use crate::models::object::{ByteString, GitObject};
pub struct Blob {
    data: ByteString,
}

impl GitObject for Blob {
    const TYPE: &'static str = "blob";
    fn dump(&self) -> ByteString {
        self.data.clone()
    }
    fn load(&mut self, data: ByteString) {
        self.data = data;
    }
}
