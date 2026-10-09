#![allow(dead_code)]
#![allow(unused)]

//! # Programming projects boilerplate creator
pub mod args;
pub mod file_mgmt;

use args::Args;
use clap::{Parser, ValueEnum};
use file_mgmt::create_dir;
use std::env;

fn main() {
    let current_dir = env::current_dir().unwrap();
    let args = Args::parse();

    create_dir(&args.name);
}
