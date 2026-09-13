use flate2::Compression;
use flate2::write::ZlibEncoder;
use rustgit::commands::hash::{hash, object_read};
use rustgit::models::object::{GitObject, GitObjectTrait};
use rustgit::models::repo::Repository;
use std::fs::{create_dir_all, write};
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_object_read_capability_for_valid_blob() {
    // 1. Set up a mock repo sandbox
    let dir = tempdir().unwrap();
    let repo_path = dir.path().join("mock_repo");
    let repo_str = repo_path.to_str().unwrap();
    let repo = Repository::create(repo_str);

    // 2. Draft raw Git object content: "type size\0content"
    let content_str = "hello git clone content";
    let content_body = content_str.as_bytes();
    let size = content_body.len();

    // Construct exactly what a Git blob looks like uncompressed: "blob 23\0hello git clone content"
    let mut raw_uncompressed = Vec::new();
    write!(raw_uncompressed, "blob {}\x00", size).unwrap();
    raw_uncompressed.extend_from_slice(content_body);

    // 3. Compress the mock Git object data using Zlib
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&raw_uncompressed).unwrap();
    let compressed_data = encoder.finish().unwrap();

    let mock_hash = "abcdef1234567890123456789012345678901234";
    let objdir = &mock_hash[..2]; // "ab"
    let objfile = &mock_hash[2..]; // "cdef123456..."

    let object_folder = repo.path("objects").join(objdir);
    create_dir_all(&object_folder).unwrap();

    let object_file_path = object_folder.join(objfile);
    write(&object_file_path, compressed_data).unwrap();

    let result = object_read(&repo, mock_hash);

    let readable_string = String::from_utf8(result.dump()).expect("Found invalid UTF-8 bytes");

    assert_eq!(readable_string, content_str);
}

#[test]
#[should_panic(expected = "Required file does not exist")]
fn test_object_read_panics_on_missing_hash() {
    let dir = tempdir().unwrap();
    let repo_path = dir.path().join("mock_repo_empty");
    let repo = Repository::create(repo_path.to_str().unwrap());

    // Call a random hash that has no matching compressed file written on disk
    object_read(&repo, "00112233445566778899aabbccddeeff00112233");
}

#[test]
fn test_hash_capability_matches_real_git_sha1() {
    let file_content = b"hello\n".to_vec();

    let mock_blob = GitObject::new(Some(file_content));

    let generated_hash = hash(mock_blob);

    let expected_git_hash = "ce013625030ba8dba906f756967f9e9ca394464a";
    assert_eq!(generated_hash, expected_git_hash);
}
