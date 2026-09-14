use crate::models::object::{ByteString, GitObjectTrait};

#[derive(Default)]
pub struct Blob {
    data: ByteString,
}

impl GitObjectTrait for Blob {
    fn get_type(&self) -> &'static str {
        "blob"
    }
    fn dump(&self) -> ByteString {
        self.data.clone()
    }
    fn load(&mut self, data: ByteString) {
        self.data = data;
    }
}
