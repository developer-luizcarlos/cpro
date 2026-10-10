#![allow(private_interfaces)]

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
pub struct Args {
    /// Project name
    #[arg(short, long)]
    pub name: String,

    /// Project kind
    #[arg(short, long, value_enum)]
    pub kind: Kind,

    /// Script files extension
    #[arg(short, long, value_enum)]
    pub ext: FileType,
}

#[derive(Clone, Debug, ValueEnum, PartialEq, Eq)]
pub enum FileType {
    /// TypeScript Files
    TS,
    /// JavaScript Files
    JS,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Kind {
    /// Create a front-end project
    Front,
    /// Create a back-end project
    Back,
}
