use std::env;

fn cmd_add(args: &[String]) {
    println!("cmd_add called with {:?}", args);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &*args[1];
    let args: &[String] = &args;  

    match command {
        "add"          => cmd_add(args),
        // "cat-file"     => cmd_cat_file(args)
        // "check-ignore" => cmd_check_ignore(args)
        // "checkout"     => cmd_checkout(args)
        // "commit"       => cmd_commit(args)
        // "hash-object"  => cmd_hash_object(args)
        // "init"         => cmd_init(args)
        // "log"          => cmd_log(args)
        // "ls-files"     => cmd_ls_files(args)
        // "ls-tree"      => cmd_ls_tree(args)
        // "rev-parse"    => cmd_rev_parse(args)
        // "rm"           => cmd_rm(args)
        // "show-ref"     => cmd_show_ref(args)
        // "status"       => cmd_status(args)
        // "tag"          => cmd_tag(args)
        _              => println!("Bad command.")
    }
}
