use clap::ValueEnum;

// ValueEnum converts str to the enum type automatically
#[derive(ValueEnum, Clone, Debug)]
pub enum ObjectType {
    Blob,
    Commit,
    Tag,
    Tree,
}