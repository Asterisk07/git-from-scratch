use crate::models::blob::Blob;
use crate::models::object::{
    ByteSlice, ByteString, GitObject, GitObjectTrait, HashSlice, HashString, byte_find,
};
use crate::models::repo::Repository;
use flate2::Compression;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use std::fs::File;
use std::io::{Read, Write};
// use std::path::PathBuf;
// use crate::models::object::GitObject;
use hex;
use sha1::{Digest, Sha1};
use std::str::from_utf8;

pub fn object_read(repo: &Repository, hash: HashSlice) -> GitObject {
    let path = repo.hash_path(&hash);
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

fn hash(raw: ByteSlice) -> HashString {
    let mut hash = Sha1::new();
    hash.update(raw);
    let hash = hash.finalize();
    let hash = hex::encode(hash);
    hash
}

fn object_encode(obj: &GitObject) -> ByteString {
    let type_tag = obj.get_type();
    let data = obj.dump();
    let mut raw = ByteString::new();
    raw.extend_from_slice(type_tag.as_bytes());
    raw.push(b' ');
    raw.extend_from_slice(data.len().to_string().as_bytes());
    raw.push(b'\x00');
    raw.extend_from_slice(&data);
    raw
}

pub fn object_hash(obj: &GitObject) -> HashString {
    let raw = object_encode(&obj);
    let hash = hash(&raw);
    hash
}

pub fn object_write(repo: &Repository, obj: &GitObject) -> HashString {
    let data = object_encode(&obj);
    let hash = hash(&data);
    let path = repo.hash_path(&hash);
    let path = repo.file(path);
    if !path.exists() {
        let file = File::create(path).expect("Failed to create file");
        let mut encoder = ZlibEncoder::new(file, Compression::default());
        encoder.write_all(&data).expect("Error writing to file");
        encoder.finish().expect("Failed to finish writing");
    }
    hash
}
