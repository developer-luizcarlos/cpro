#![allow(dead_code)]
#![allow(unused)]

//! # Programming projects boilerplate creator
pub mod args;
pub mod deps;
pub mod file_content;
pub mod file_mgmt;

use args::{Args, FileType};
use clap::{Parser, ValueEnum};
use file_mgmt::{create_dir, create_project};
use std::env;

fn main() {
    let current_dir = env::current_dir().unwrap();
    let args = Args::parse();

    let current_dir = env::current_dir().unwrap();

    match create_dir(&current_dir, &args.name) {
        Ok(dir) => {
            println!("{} directory created successfully", &args.name);
            println!("Wait while project structure is been created...");

            match create_project(&dir, &args.kind, &args.ext) {
                Ok(_) => println!("Project created successfully!"),
                Err(err) => eprintln!("{}", err),
            }
        }
        Err(err) => eprintln!("{}", err),
    };
}
