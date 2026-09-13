// tests/repo_tests.rs
use rustgit::models::repo::Repository;
use std::fs;
use std::path::PathBuf;
use tempfile::tempdir;

#[test]
fn test_repo_creation_capability() {
    // 1. Set up the path where we EXPECT the files to be
    let dir = tempdir().unwrap();
    let repo_path = dir.path().join("my_test_git_repo");
    let repo_str = repo_path.to_str().unwrap();

    // 2. Call the function (we don't even need to save the returned 'repo' variable!)
    let _repo = Repository::create(repo_str);

    // 3. Construct the paths manually to verify the side effects on disk
    let expected_worktree = PathBuf::from(repo_str);
    let expected_gitdir = expected_worktree.join(".git");

    // 4. Assert against the physical disk, completely ignoring the private struct fields
    assert!(
        expected_worktree.exists(),
        "Worktree directory was not created"
    );
    assert!(
        expected_gitdir.exists(),
        "Git directory (.git) was not created"
    );
    assert!(expected_gitdir.join("branches").is_dir());
    assert!(expected_gitdir.join("objects").is_dir());

    // 5. Verify file contents on disk
    let head_contents = fs::read_to_string(expected_gitdir.join("HEAD")).unwrap();
    assert_eq!(head_contents.trim(), "ref : refs/heads/master");
}

// --- 2. TEST FOR 'find' (SUCCESS CASE) ---
#[test]
fn test_repository_find_can_locate_git_dir_from_deep_subfolder() {
    let dir = tempdir().unwrap();
    let repo_path = dir.path().join("my_project");
    let repo_str = repo_path.to_str().unwrap();

    // Create the repository structure
    Repository::create(repo_str);

    // Make a deeply nested project subdirectory inside the worktree
    let nested_sub_dir = repo_path.join("src").join("controllers").join("auth");
    fs::create_dir_all(&nested_sub_dir).unwrap();

    // Call find from inside the deep subfolder. It should traverse up and succeed.
    Repository::find(nested_sub_dir);
}

// --- 3. TEST FOR 'find' (FAILURE CASE) ---
#[test]
#[should_panic(expected = "No git repo found at")]
fn test_repository_find_panics_if_no_repo_exists_in_tree() {
    let dir = tempdir().unwrap();
    let completely_empty_folder = dir.path().join("isolated_folder");
    fs::create_dir_all(&completely_empty_folder).unwrap();

    // This path has no .git folder above it, so it should hit your .expect() panic
    Repository::find(completely_empty_folder);
}
