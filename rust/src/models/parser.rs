use crate::models::object::ByteSlice;
use clap::ValueEnum;

// ValueEnum converts str to the enum type automatically
#[derive(ValueEnum, Clone, Debug)]
pub enum ObjectType {
    Blob,
    Commit,
    Tag,
    Tree,
}

impl ObjectType {
    pub fn as_bytes(&self) -> ByteSlice<'static> {
        match self {
            ObjectType::Blob => b"blob",
            ObjectType::Commit => b"commit",
            ObjectType::Tag => b"tag",
            ObjectType::Tree => b"tree",
        }
    }
}
