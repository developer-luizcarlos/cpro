#![allow(dead_code)]
#![allow(unused)]

//! # Programming projects boilerplate creator
pub mod args;
pub mod deps;
pub mod file_content;
pub mod file_mgmt;

use args::{Args, FileType};
use clap::{Parser, ValueEnum};
use file_mgmt::{create_dir, create_subdir};
use std::env;

fn main() {
    let current_dir = env::current_dir().unwrap();
    let args = Args::parse();

    let current_dir = env::current_dir().unwrap();
    let created_dir = create_dir(&current_dir, &args.name);

    match created_dir {
        Ok(dir) => {
            println!("{} directory created successfully", &args.name);
            create_subdir(&dir, &args.kind, &args.ext);
        }
        Err(err) => eprintln!("{}", err),
    };
}
