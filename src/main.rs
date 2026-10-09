#![allow(dead_code)]
#![allow(unused)]

//! # Programming projects boilerplate creator

use clap::{Parser, ValueEnum};
use std::{env, process};

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
    let dir = env::current_dir();

    if let Err(_) = dir {
        eprintln!("Cannot find out current directory");
        process::exit(1);
    }

    let args = Args::parse();
}
