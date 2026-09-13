pub type ByteString = Vec<u8>;
pub type ByteSlice<'a> = &'a [u8];
use crate::models::blob::Blob;
pub use memchr::memchr as byte_find;

pub trait GitObjectTrait {
    fn get_type(&self) -> &'static str;
    fn dump(&self) -> ByteString;
    fn load(&mut self, data: ByteString);
}

pub enum GitObject {
    Blob(Blob),
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
