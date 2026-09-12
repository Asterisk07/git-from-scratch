#![allow(unused_variables)]
#![allow(dead_code)]
// use std::env;

pub mod init; 
pub mod args;
pub mod object;

// use clap::Parser;
// use init::cmd_init;
use args::{parse};
// use args::{Cli, Commands};

// fn print_help() {
//     println!("Usage: git <command> [options]");
//     println!("Commands:");
//     println!("    help          Show this info");
//     // println!("    init          Initialize a repository");
//     // println!("    add           Add files to staging area");
//     // println!("    cat-file      Show object content");
//     // println!("    check-ignore  Check ignored files");
// }

pub fn run() {
    parse();
    // let cli = Cli::parse();
    // match cli.command {
    //     Commands::CatFile { object_type, object } => {
    //         println!("Reading {:?} object {}", object_type, object);
    //         // call your cmd_cat_file(object_type, object) here
    //     }
    //     Commands::HashObject { object_type, write, path } => {
    //         if write {
    //             println!("Writing {:?} to database from {}", object_type, path);
    //         }
    //         // call your cmd_hash_object(...) here
    //     }
    // }
    // let args: Vec<String> = env::args().collect();
    // if (args.len()) == 1 {
    //     print_help();
    //     return;
    // }
    // let command = &*args[1];
    // let args: &[String] = &args;

    // match command {
    //     "help" => print_help(),
    //     "init" => cmd_init(args),
    //     // "cat-file"     => cmd_cat_file(args)
    //     // "check-ignore" => cmd_check_ignore(args)
    //     // "checkout"     => cmd_checkout(args)
    //     // "commit"       => cmd_commit(args)
    //     // "hash-object"  => cmd_hash_object(args)
    //     // "log"          => cmd_log(args)
    //     // "ls-files"     => cmd_ls_files(args)
    //     // "ls-tree"      => cmd_ls_tree(args)
    //     // "rev-parse"    => cmd_rev_parse(args)
    //     // "rm"           => cmd_rm(args)
    //     // "show-ref"     => cmd_show_ref(args)
    //     // "status"       => cmd_status(args)
    //     // "tag"          => cmd_tag(args)
    //     _ => println!("Bad command."),
    // }
}
