#![allow(dead_code)]
#![allow(unused)]

//! # Programming projects boilerplate creator
pub mod args;

use args::Args;
use clap::{Parser, ValueEnum};
use std::{
    env,
    fs::DirBuilder,
    path::{Path, PathBuf},
    process,
};

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
