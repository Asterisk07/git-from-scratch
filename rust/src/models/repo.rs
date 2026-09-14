use configparser::ini::Ini;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf, absolute};

use crate::models::object::HashSlice;

pub struct Repository {
    worktree: PathBuf,
    gitdir: PathBuf,
    config_path: PathBuf,
    config: Ini,
}

impl Repository {
    fn new(worktree: impl AsRef<Path>, force: bool) -> Self {
        let worktree = absolute(worktree.as_ref()).expect("Invalid path");
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

    pub fn hash_path(&self, hash: HashSlice) -> PathBuf {
        let objdir = &hash[..2];
        let objfile = &hash[2..];
        let path = self.path("objects").join(objdir).join(objfile);
        path
    }

    pub fn path(&self, path: impl AsRef<Path>) -> PathBuf {
        self.gitdir.join(path)
    }

    pub fn file(&self, path: impl AsRef<Path>) -> PathBuf {
        let path = self.path(path);

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("Error creating parent dir")
        };
        path
    }

    pub fn create(path: impl AsRef<Path>) -> Self {
        let repo = Self::new(path, true);
        let path = &repo.worktree;

        let gitpath = &repo.gitdir;
        if path.exists() {
            assert!(path.is_dir(), "Path is an existing file : {:?}", path);
        } else {
            fs::create_dir(&path).expect("Failed to create worktree");
        }
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
        } else {
            fs::create_dir(&gitpath)
                .expect(&format!("Failed to create git directory at {:?}", gitpath));
        }

        fs::create_dir(gitpath.join("branches")).expect(&format!(
            "Failed to create branches dir inside worktree: {:?}",
            gitpath
        ));
        fs::create_dir(gitpath.join("objects")).unwrap();
        fs::create_dir(gitpath.join("refs")).unwrap();
        fs::create_dir(gitpath.join("refs").join("tags")).unwrap();
        fs::create_dir(gitpath.join("refs").join("heads")).unwrap();

        // Python: open("file.txt", "w")

        writeln!(
            File::create(gitpath.join("HEAD")).unwrap(),
            "ref : refs/heads/master"
        )
        .unwrap();
        writeln!(
            File::create(gitpath.join("description")).unwrap(),
            "Unnamed repo, edit description to name it"
        )
        .unwrap();
        // writeln!(
        //     File::create(path.join("config")).unwrap(),
        //     "ref : refs/heads/mastere"
        // )
        // .unwrap();
        let con = Self::default_config();
        con.write(&repo.config_path).unwrap();
        repo
    }

    fn default_config() -> Ini {
        let mut con = Ini::new();
        con.set("core", "repositoryformatversion", Some("0".to_string()));
        con.set("core", "filemode", Some("false".to_string()));
        con.set("core", "bare", Some("false".to_string()));
        con
    }

    pub fn find(path: PathBuf) {
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
        return Self::find(path);
    }
}
