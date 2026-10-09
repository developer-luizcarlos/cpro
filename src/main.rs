#![allow(dead_code)]
#![allow(unused)]

//! # Programming projects boilerplate creator

use clap::{Parser, ValueEnum};
use std::{
    env,
    fs::DirBuilder,
    path::{Path, PathBuf},
    process,
};

#[derive(Parser, Debug)]
#[command(name = "cpro")]
#[command(version = "1.0.0")]
#[command(about = "Programming projects boilerplate creator")]
struct Args {
    /// Project name
    #[arg(short, long)]
    name: String,

    /// Project kind
    #[arg(short, long, value_enum)]
    kind: Kind,

    /// Script files extension
    #[arg(short, long, value_enum)]
    ext: FileType,
}

#[derive(Clone, Debug, ValueEnum)]
enum FileType {
    /// TypeScript Files
    TS,
    /// JavaScript Files
    JS,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Kind {
    /// Create a front-end project
    Front,
    /// Create a back-end project
    Back,
}

fn main() {
    let current_dir = env::current_dir().unwrap();
    let args = Args::parse();

    create_dir(&args.name);
}

fn create_path(dir_name: &String) -> PathBuf {
    let current_dir = env::current_dir().unwrap();
    let path = Path::new(&current_dir).join(dir_name.clone());

    path
}

fn create_dir(dir_name: &String) {
    let path = create_path(dir_name);

    if path.exists() {
        eprintln!("{:?} directory already exists", dir_name);
        process::exit(1);
    }

    match DirBuilder::new().create(&path) {
        Ok(_) => println!("{:?} directory created", dir_name),
        Err(err) => eprintln!("{}", err),
    }
}
