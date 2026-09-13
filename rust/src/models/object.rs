pub type ByteString = Vec<u8>;
pub type ByteSlice<'a> = &'a [u8];
pub type HashString = String;
pub type HashSlice<'a> = &'a str;
use crate::models::blob::Blob;
pub use memchr::memchr as byte_find;

pub trait GitObjectTrait {
    fn get_type(&self) -> &'static str;
    fn dump(&self) -> ByteString;
    fn load(&mut self, data: ByteString);
    fn new(data: Option<ByteString>) -> Self
    where
        Self: Sized + Default,
    {
        let mut obj = Self::default();
        match data {
            Some(d) => obj.load(d),
            _ => obj.load(vec![]),
        };
        obj
    }
}

pub enum GitObject {
    Blob(Blob),
}

impl GitObject {
    pub fn new(data: Option<ByteString>) -> Self {
        Self::Blob(Blob::new(data))
    }
}

macro_rules! implement_git_dispatch {
    ($enum_name:ident, $($variant:ident),*) => {
        impl GitObjectTrait for $enum_name {
            fn get_type(&self) -> &'static str {
                match self {
                    $( $enum_name::$variant(inner) => inner.get_type(), )*
                }
            }

            fn dump(&self) -> ByteString {
                match self {
                    $( $enum_name::$variant(inner) => inner.dump(), )*
                }
            }

            fn load(&mut self, data: ByteString) {
                match self {
                    // Clone the data if you have multiple variants,
                    // or pass it natively if it's consumed.
                    $( $enum_name::$variant(inner) => inner.load(data.clone()), )*
                }
            }
        }
    };
}

// 6. Invoke it immediately to glue everything together
implement_git_dispatch!(GitObject, Blob);
