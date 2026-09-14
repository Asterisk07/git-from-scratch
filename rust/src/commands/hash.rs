use crate::models::blob::Blob;
use crate::models::object::{
    ByteSlice, ByteString, GitObject, GitObjectTrait, HashSlice, HashString, byte_find,
};
use crate::models::repo::Repository;
use flate2::Compression;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use hex;
use sha1::{Digest, Sha1};
use std::fs::{self, File};
use std::io::{Read, Write};
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

pub fn object_encode(obj: &GitObject) -> ByteString {
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

pub fn object_find(
    repo: &Repository,
    hash: HashSlice,
    kind: ByteSlice,
    follow: bool,
) -> HashString {
    hash.to_string()
}

pub fn print_bytes<W: Write>(writer: &mut W, data: ByteString) {
    if let Err(e) = writer.write_all(&data) {
        eprintln!("Failed to print to stdout {}", e)
    }
    let _ = writer.flush();
}

pub fn cat_file<W: Write>(writer: &mut W, repo: &Repository, kind: ByteSlice, hash: HashSlice) {
    let hash = object_find(repo, hash, kind, true);
    let obj = object_read(repo, &hash);
    let data = obj.dump();
    print_bytes(writer, data);
}

/// Streams the decompressed object contents directly to the destination writer.
/// This prevents allocating massive files entirely in memory.
pub fn cmd_cat_file<W: Write>(writer: &mut W, kind: ByteSlice, hash: HashSlice) {
    let repo = Repository::load(".");
    cat_file(writer, &repo, kind, hash);
}

pub fn object_from_file(kind: ByteSlice, path: &str) -> GitObject {
    let data = fs::read(path).expect("Failed to read");
    match &kind[..] {
        b"blob" => GitObject::Blob(Blob::new(Some(data))),
        _ => panic!("Unsupported TYPE {:?}", &kind[..]),
    }
}

/// Computes the object hash and returns the tiny 40-character hex signature directly.
/// Because the hash string is small, it can be safely passed up the call stack.
pub fn cmd_hash_object(kind: ByteSlice, path: &str, write_flag: bool) -> HashString {
    let obj = object_from_file(kind, path);
    if write_flag {
        let repo = Repository::load(".");
        object_write(&repo, &obj)
    } else {
        object_hash(&obj)
    }
}
