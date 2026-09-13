use crate::models::repo::Repository;

pub fn cmd_init(path: String) {
    // let path = match args.len() {
    //     2 => ".",      // git init
    //     3 => &args[2], // git init dir
    //     _ => panic!("error: too many arguments for `init`"),
    // };
    println!("creating repo at  {:?}", path);
    // find(
    //     PathBuf::from(path)
    //         .canonicalize()
    //         .expect(&format!("Could not canocialise {:?}", path)),
    // )
    let path = &path;
    Repository::create(path);
    println!("created repo at");
}
