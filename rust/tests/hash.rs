use flate2::Compression;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use rustgit::commands::hash::{
    cmd_cat_file, cmd_hash_object, object_encode, object_hash, object_read, object_write,
};
use rustgit::models::object::{ByteString, GitObject, GitObjectTrait};
use rustgit::models::repo::Repository;
use std::fs::{self, File, create_dir_all, write};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::Command;

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

    let generated_hash = object_hash(&mock_blob);

    let expected_git_hash = "ce013625030ba8dba906f756967f9e9ca394464a";
    assert_eq!(generated_hash, expected_git_hash);
}

#[test]
fn test_object_write_capability_matches_real_git_sha1_local_dir() {
    // 1. Set up a fixed local folder inside your project's target directory
    let repo_path = PathBuf::from("target/debug_matches_sha1");

    // Clean up any old test run data so you always inspect a fresh state
    if repo_path.exists() {
        std::fs::remove_dir_all(&repo_path).unwrap();
    }
    std::fs::create_dir_all(&repo_path).unwrap();

    let repo_str = repo_path.to_str().unwrap();
    let repo = Repository::create(repo_str);

    // 2. Draft raw Git object content matching your exact "hello\n" reference
    let file_content = b"hello\n".to_vec();
    let mock_blob = GitObject::new(Some(file_content));

    // 3. Execute object_write to encode, hash, and compress it to disk
    let generated_hash = object_write(&repo, &mock_blob);

    // Verify it generates the exact Git SHA1 signature for "hello\n"
    let expected_git_hash = "ce013625030ba8dba906f756967f9e9ca394464a";
    assert_eq!(generated_hash, expected_git_hash);

    // 4. Verify the folder structure creation and file existence
    let objdir = &generated_hash[..2]; // "ce"
    let objfile = &generated_hash[2..]; // "013625030b..."
    let expected_file_path = repo.path("objects").join(objdir).join(objfile);

    println!("Expected file path {:?}", expected_file_path);
    assert!(
        expected_file_path.exists(),
        "Object was not written to the expected disk path"
    );

    // 5. Read back the zlib compressed payload to verify data integrity
    let compressed_data = std::fs::read(&expected_file_path).unwrap();
    let mut decoder = ZlibDecoder::new(&compressed_data[..]);
    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data).unwrap();

    // Verify that the file's data perfectly matches your internal encoder output
    let expected_encoded_data = object_encode(&mock_blob);
    assert_eq!(decompressed_data, expected_encoded_data);
}

#[test]
fn test_object_write_capability_matches_real_git_sha1() {
    // 1. Set up a mock repo sandbox using your initialization hook
    let dir = tempdir().unwrap();
    let repo_path = dir.path().join("mock_repo");
    let repo_str = repo_path.to_str().unwrap();
    let repo = Repository::create(repo_str);

    // 2. Draft raw Git object content matching your exact "hello\n" reference
    let file_content = b"hello\n".to_vec();
    let mock_blob = GitObject::new(Some(file_content));

    // 3. Execute object_write to encode, hash, and compress it to disk
    let generated_hash = object_write(&repo, &mock_blob);

    // Verify it generates the exact Git SHA1 signature for "hello\n"
    let expected_git_hash = "ce013625030ba8dba906f756967f9e9ca394464a";
    assert_eq!(generated_hash, expected_git_hash);

    // 4. Verify the folder structure creation and file existence
    let objdir = &generated_hash[..2]; // "ce"
    let objfile = &generated_hash[2..]; // "013625030b..."
    let expected_file_path = repo.path("objects").join(objdir).join(objfile);

    assert!(
        expected_file_path.exists(),
        "Object was not written to the expected disk path"
    );

    // 5. Read back the zlib compressed payload to verify data integrity
    let compressed_data = std::fs::read(&expected_file_path).unwrap();
    let mut decoder = ZlibDecoder::new(&compressed_data[..]);
    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data).unwrap();

    // Verify that the file's data perfectly matches your internal encoder output
    let expected_encoded_data = object_encode(&mock_blob);
    assert_eq!(decompressed_data, expected_encoded_data);
}

#[test]

fn test_object_write_is_readable_by_real_git_cli_local_dir() {
    // 1. Set up a fixed local folder inside your project's target directory
    let repo_path = PathBuf::from("target/debug_git_test");

    // Clean up any old test run data so you always start fresh
    if repo_path.exists() {
        std::fs::remove_dir_all(&repo_path).unwrap();
    }
    std::fs::create_dir_all(&repo_path).unwrap();

    // FIRST: Let your code initialize the repository layout
    let repo_str = repo_path.to_str().unwrap();
    let repo = Repository::create(repo_str);

    // SECOND: Fix the missing HEAD file so the Git CLI recognizes the repo
    let git_dir = repo_path.join(".git");
    std::fs::create_dir_all(git_dir.join("refs/heads")).unwrap();
    std::fs::write(git_dir.join("HEAD"), b"ref: refs/heads/main\n").unwrap();

    // 2. Draft the mock object ("hello\n" matches hash ce013625...)
    let file_content = b"hello\n".to_vec();
    let mock_blob = GitObject::new(Some(file_content));

    // 3. Execute YOUR Rust function to write the compressed file to disk
    let generated_hash = object_write(&repo, &mock_blob).to_string();
    // let generated_hash = object_write(&repo, &mock_blob).unwrap().to_string();

    // 4. Use the REAL Git installation on your machine to query the database
    let git_output = Command::new("git")
        .arg("-C")
        .arg(&repo_path)
        .arg("cat-file")
        .arg("-p")
        .arg(&generated_hash)
        .output()
        .expect("Failed to execute real git command");

    // 5. Assertions
    if !git_output.status.success() {
        let expected_object_path = git_dir
            .join("objects")
            .join(&generated_hash[0..2])
            .join(&generated_hash[2..]);

        panic!(
            "\n================ TEST FAILURE DETAILED REPORT ================\n\
             Real Git rejected our object database file!\n\n\
             [Command executed]: git -C {:?} cat-file -p {}\n\
             [Generated Hash]:   {}\n\
             [Git CLI Error]:    {}\n\n\
             [Inspection Guide]:\n\
             - Go inspect this folder: {:?}\n\
             - Does the object file exist? {}\n\
             - Object path checked:    {:?}\n\
             ==============================================================",
            repo_path,
            generated_hash,
            generated_hash,
            String::from_utf8_lossy(&git_output.stderr).trim(),
            git_dir,
            expected_object_path.exists(),
            expected_object_path
        );
    }

    let git_read_string = String::from_utf8(git_output.stdout).unwrap();
    assert_eq!(git_read_string, "hello\n");
}

#[test]
fn test_cmd_cat_file_execution() {
    // 1. Pick a file in your repo to act as the target
    let target_file = "src/main.rs";
    let object_type = "blob";

    // 2. Query git hash-object to find out what its hash should be
    let hash_output = Command::new("git")
        .args(["hash-object", "-w", target_file])
        .output()
        .expect("Failed to run git hash-object");

    let hash_str = String::from_utf8(hash_output.stdout)
        .expect("Invalid UTF-8 from git")
        .trim()
        .to_string();

    // 3. Query git cat-file to get the exact expected stdout payload
    let git_cat_output = Command::new("git")
        .args(["cat-file", "-p", &hash_str])
        .output()
        .expect("Failed to run git cat-file");

    let expected_bytes = git_cat_output.stdout;

    // 4. Mirror the variables you are passing to your production command
    // Adjust these if your project wraps strings/bytes into custom types
    let object_type_bytes = object_type.as_bytes();

    // 5. Create a dynamic vector to capture the output instead of stdout
    let mut actual_bytes = ByteString::new();

    // 6. Run your exact line of code
    cmd_cat_file(&mut actual_bytes, object_type_bytes, &hash_str);

    // 7. Verify the output bytes match what git itself printed
    assert_eq!(actual_bytes, expected_bytes);
}

#[test]
fn test_cmd_hash_object_read_only_with_unique_file() {
    // 1. Fabricate a completely unique temporary file name and string content
    let temp_filename = "test_transient_sample_file.txt";
    let file_content = b"Completely unique string payload to guarantee a fresh SHA-1 hash! 554433";

    // 2. Write the file onto the hard drive so the official git binary can see it
    let mut file = File::create(temp_filename).expect("Failed to create temporary test file");
    file.write_all(file_content)
        .expect("Failed to write test file payload");

    // 3. Query the official git command to calculate the expected reference hash string
    let git_hash_output = Command::new("git")
        .args(["hash-object", temp_filename])
        .output()
        .expect("Failed to run official git hash-object");

    let expected_hash_str = String::from_utf8(git_hash_output.stdout)
        .expect("Invalid UTF-8 output from git")
        .trim()
        .to_string();

    // 4. Mirror your application's input types (Adjust conversions if necessary)
    let object_type_bytes = b"blob";

    // 5. Execute your code in memory (write_flag = false ensures NO write happens to .git/objects)
    let actual_hash_string = cmd_hash_object(object_type_bytes, temp_filename, false);

    // 6. CRITICAL HOUSEKEEPING: Delete the transient file from disk immediately
    // Even if the upcoming assertion fails, we run this step so it doesn't leave clutter.
    let _ = fs::remove_file(temp_filename);

    // 7. Verify that your application's calculation matches Git's baseline perfectly
    assert_eq!(actual_hash_string.to_string(), expected_hash_str);
}

#[test]
fn test_cmd_hash_object_with_live_write_and_cleanup() {
    // 1. Setup a unique name and text body to guarantee a brand-new object hash
    let temp_filename = "test_transient_write_target.txt";
    let unique_payload = b"Transient write token signature verification framework: 123456789";

    // 2. Write the file onto the disk workspace so your loader can grab it
    let mut file = File::create(temp_filename).expect("Failed to create transient test file");
    file.write_all(unique_payload)
        .expect("Failed to populate test file content");

    // 3. Mirror your module's types
    let object_type_bytes = b"blob";

    // 4. Run your write loop (write_flag = true)
    // This calculates the hash string AND saves the compressed object to your .git folder
    let hash_str = cmd_hash_object(object_type_bytes, temp_filename, true);
    // let hash_str = generated_hash.to_string();

    // 5. Clean up the original text file from your workspace immediately
    let _ = fs::remove_file(temp_filename);

    // 6. VERIFICATION A: Confirm the compressed file was created in your .git database
    let repo = Repository::load(".");
    let git_db_object_path = repo.file(repo.hash_path(&hash_str));

    assert!(
        git_db_object_path.exists(),
        "The object file was not successfully persisted to the database path at {:?}",
        git_db_object_path
    );

    // 7. VERIFICATION B: Verify your cat_file engine can decompress and read it back perfectly
    let mut actual_readback_bytes = Vec::new();
    cmd_cat_file(&mut actual_readback_bytes, object_type_bytes, &hash_str);

    // Assert that the data retrieved from the database matches the original payload exactly
    assert_eq!(actual_readback_bytes, unique_payload);

    // 8. ABSOLUTE CLEANUP: Permanently delete the generated database file
    // This ensures the test leaves zero footprints or dangling blobs behind in your repo
    let _ = fs::remove_file(&git_db_object_path);

    // Optional: Attempt to remove the 2-character parent folder if it is now empty
    if let Some(parent_dir) = git_db_object_path.parent() {
        let _ = fs::remove_dir(parent_dir); // Fails safely if other legitimate objects are inside it
    }
}

#[test]
fn test_hash_object_to_cat_file_pipeline() {
    // 1. Target the actual file from your command-line example
    let target_file = "src/main.rs";
    let object_type_bytes = b"blob";

    // 2. Step 1 of your CLI flow: Compute the hash string from the file
    // This simulates running: git hash-object src/main.rs
    let hash_str = cmd_hash_object(object_type_bytes, target_file, false);

    // 3. Prepare an in-memory byte buffer to act as your standard output (stdout) writer
    let mut actual_stdout_bytes = Vec::new();

    // 4. Step 2 of your CLI flow: Feed that generated hash directly into cat-file
    // This simulates running: git cat-file blob <hash>
    cmd_cat_file(&mut actual_stdout_bytes, object_type_bytes, &hash_str);

    // 5. Read the true physical contents of src/main.rs directly from disk as a baseline
    let expected_file_bytes =
        std::fs::read(target_file).expect("Failed to read src/main.rs baseline for verification");

    // 6. Assert that your cat-file streaming output perfectly matches the original file text
    assert_eq!(actual_stdout_bytes, expected_file_bytes);
}
