use configparser::ini::Ini;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

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

    fn repo_path(&self, path: &str) -> PathBuf {
        // """Compute path under repo's gitdir."""
        self.gitdir.join(path)
    }

    fn repo_file(&self, path: &str, mkdir: bool) -> PathBuf {
        // """Compute path under repo's gitdir."""
        self.gitdir.join(path)
    }

    fn repo_create(path: &str) -> Self {
        let repo = Self::new(path, true);
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
        let con = Self::repo_default_config();
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
        return Self::repo_find(path);
    }
}

pub fn cmd_init(path: String) {
    // let path = match args.len() {
    //     2 => ".",      // git init
    //     3 => &args[2], // git init dir
    //     _ => panic!("error: too many arguments for `init`"),
    // };
    println!("creating repo at  {:?}", path);
    // repo_find(
    //     PathBuf::from(path)
    //         .canonicalize()
    //         .expect(&format!("Could not canocialise {:?}", path)),
    // )
    let path = &path;
    Repository::repo_create(path);
    println!("created repo at");
}
