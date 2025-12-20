#![allow(unused_variables)]
#![allow(dead_code)]
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
fn cmd_add(args: &[String]) {
    println!("cmd_add called with {:?}", args);
}

fn cmd_init(args: &[String]) {
    let path = match args.len() {
        2 => ".",      // git init
        3 => &args[2], // git init dir
        _ => panic!("error: too many arguments for `init`"),
    };
    println!("creating repo at  {:?}", path);
    // repo_find(
    //     PathBuf::from(path)
    //         .canonicalize()
    //         .expect(&format!("Could not canocialise {:?}", path)),
    // )
    repo_create(path);
    println!("created repo at");
}

use configparser::ini::Ini;

fn print_help() {
    println!("Usage: git <command> [options]");
    println!("Commands:");
    println!("    help          Show this info");
    // println!("    init          Initialize a repository");
    // println!("    add           Add files to staging area");
    // println!("    cat-file      Show object content");
    // println!("    check-ignore  Check ignored files");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if (args.len()) == 1 {
        print_help();
        return;
    }
    let command = &*args[1];
    let args: &[String] = &args;

    match command {
        "help" => print_help(),
        "init" => cmd_init(args),
        // "cat-file"     => cmd_cat_file(args)
        // "check-ignore" => cmd_check_ignore(args)
        // "checkout"     => cmd_checkout(args)
        // "commit"       => cmd_commit(args)
        // "hash-object"  => cmd_hash_object(args)
        // "log"          => cmd_log(args)
        // "ls-files"     => cmd_ls_files(args)
        // "ls-tree"      => cmd_ls_tree(args)
        // "rev-parse"    => cmd_rev_parse(args)
        // "rm"           => cmd_rm(args)
        // "show-ref"     => cmd_show_ref(args)
        // "status"       => cmd_status(args)
        // "tag"          => cmd_tag(args)
        _ => println!("Bad command."),
    }
}

struct Repository {
    worktree: PathBuf,
    gitdir: PathBuf,
    config_path: PathBuf,
    config: Ini,
}

impl Repository {
    fn new(path1: &str, force: bool) -> Self {
        let worktree = PathBuf::from(path1);
        let gitdir = worktree.join(".git");
        let config_path = gitdir.join("config");
        let mut config = Ini::new();
        if force == false {
            let config_path = config_path.to_str().expect("Invalid config path");
            let x = config.load(config_path).expect("Loading config");

            let version = config
                .getint("core", "repositoryformatversion")
                .expect("Not found key")
                .expect("Couldnt convert key to int");
            assert!(
                version == 0,
                "repositoryformatversion is non zero : {}",
                version
            );
        };
        Self {
            worktree: worktree,
            gitdir: gitdir,
            config_path: config_path,
            config: config,
        }
    }
}

fn repo_path(repo: Repository, path: &str) -> PathBuf {
    // """Compute path under repo's gitdir."""
    repo.gitdir.join(path)
}

fn repo_file(repo: Repository, path: &str, mkdir: bool) -> PathBuf {
    // """Compute path under repo's gitdir."""
    repo.gitdir.join(path)
}

fn repo_create(path: &str) -> Repository {
    let repo = Repository::new(path, true);
    let path = &repo.worktree;

    // Ok(entries.next().is_none())
    let gitpath = &repo.gitdir;
    if path.exists() {
        assert!(path.is_dir(), "Path is an existing file : {:?}", path);
        if gitpath.exists() {
            assert!(
                gitpath.is_dir(),
                "gitdir is an existing file : {:?}",
                gitpath
            );
            assert!(
                fs::read_dir(&gitpath)
                    .expect("Error reading git dir")
                    .next()
                    .is_none(),
                "Git dir non empty : {:?}",
                gitpath
            );
        }
    } else {
        fs::create_dir(&path).expect("Failed to create worktree");
        fs::create_dir(&gitpath)
            .expect(&format!("Failed to create git directory at {:?}", gitpath));
    }

    fs::create_dir(gitpath.join("branches")).unwrap();
    fs::create_dir(gitpath.join("objects")).unwrap();
    fs::create_dir(gitpath.join("refs")).unwrap();
    fs::create_dir(gitpath.join("refs").join("tags")).unwrap();
    fs::create_dir(gitpath.join("refs").join("heads")).unwrap();

    // Python: open("file.txt", "w")

    writeln!(
        File::create(path.join("HEAD")).unwrap(),
        "ref : refs/heads/master"
    )
    .unwrap();
    writeln!(
        File::create(path.join("description")).unwrap(),
        "Unnamed repo, edit description to name it"
    )
    .unwrap();
    writeln!(
        File::create(path.join("config")).unwrap(),
        "ref : refs/heads/mastere"
    )
    .unwrap();
    let con = repo_default_config();
    con.write(&repo.config_path).unwrap();
    repo
}

fn repo_default_config() -> Ini {
    let mut con = Ini::new();
    con.set("core", "repositoryformatversion", Some("0".to_string()));
    con.set("core", "filemode", Some("false".to_string()));
    con.set("core", "bare", Some("false".to_string()));
    con
}

fn repo_find(path: PathBuf) {
    // need canocical path
    let x = path.join(".git");
    if x.is_dir() {
        return println!("found git dir at {:?}", x);
    }
    // println!("Se at {:?}", path);
    let path = path
        .parent()
        .expect(&format!("No git repo found at {:?}", path))
        .to_path_buf();
    return repo_find(path);
}
