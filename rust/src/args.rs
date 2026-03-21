use clap::{Parser, Subcommand};
use crate::init::cmd_init;
use crate::object::ObjectType;

#[derive(Parser)]
#[command(author, version, about = "The stupidest content tracker")]
 struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
 enum Commands {
    Init{
        #[arg(default_value = ".")]
        path: String
    }
    ,
    /// Provide content of repository objects
    CatFile {
        /// Specify the type
        #[arg(value_enum)]
        object_type: ObjectType,
        
        /// The object to display
        object: String,
    },
    
    /// Compute object ID and optionally creates a blob from a file
    HashObject {
        /// Specify the type
        #[arg(short = 't', value_enum, default_value = "blob")]
        object_type: ObjectType,
        
        /// Actually write the object into the database
        #[arg(short = 'w')]
        write: bool,
        
        /// Read object from <file>
        path: String,
    },
}

pub fn parse() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { path } => cmd_init(path),
        Commands::CatFile { object_type, object } => {
            println!("Reading {:?} object {}", object_type, object);
            // call your cmd_cat_file(object_type, object) here
        }
        Commands::HashObject { object_type, write, path } => {
            if write {
                println!("Writing {:?} to database from {}", object_type, path);
            }
            println!("Computing hash of database from {}", path);
            // call your cmd_hash_object(...) here
        }
    }
}