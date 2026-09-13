use crate::models::blob::Blob;
use crate::models::object::{ByteString, GitObject, GitObjectTrait, byte_find};
use crate::models::repo::Repository;
use flate2::read::ZlibDecoder;
use std::fs::File;
use std::io::Read;
// use std::path::PathBuf;
// use crate::models::object::GitObject;
use std::str::from_utf8;

pub fn object_read(repo: &Repository, hash: &str) -> GitObject {
    let objdir = &hash[..2];
    let objfile = &hash[2..];
    let path = repo.path("objects").join(objdir).join(objfile);
    if !path.try_exists().expect("Failed to read filesystem") {
        panic!("Required file does not exist: {:?}", path);
    }

    let file = File::open(path).expect("Failed to open file");
    let mut raw = ByteString::new();
    let mut decoder = ZlibDecoder::new(file);
    decoder.read_to_end(&mut raw).expect("Failed to read file");

    let x = byte_find(b' ', &raw).expect("Space missing from file");
    // let TYPE = &raw[..x];
    let type_tag = raw[..x].to_vec();

    let y = x + byte_find(b'\x00', &raw[x..]).expect("EOF missing from file");

    let size: usize = from_utf8(&raw[x + 1..y])
        .expect("Unable to extract size str")
        .parse()
        .expect("Unable to extract size int");
    assert_eq!(size, raw.len() - y - 1, "Malformed object, bad length");

    raw.drain(..y + 1);

    match &type_tag[..] {
        b"blob" => GitObject::Blob(Blob::new(Some(raw))),
        _ => panic!("Unsupported TYPE {:?}", &type_tag[..]),
    }
}
