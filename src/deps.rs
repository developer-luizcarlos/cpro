use clap::Parser;

use crate::args::{Args, FileType};
use std::{
    env::current_dir,
    io::Error,
    path::{Path, PathBuf},
    process::{Command, Output},
};

pub fn install_front_deps() -> Result<(), Error> {
    let args = Args::parse();

    let exec_dir = current_dir()?;
    let path = exec_dir.join(&args.name);

    let mut deps: Vec<&str> = Vec::new();

    deps.push("webpack");
    deps.push("webpack-cli");

    if *&args.ext == FileType::TS {
        deps.push("typescript");
        deps.push("ts-loader");
        deps.push("@types/node");
    }

    let deps = deps.join(" ");
    let install_cmd = format!("npm install {}", deps);

    let step_1 = Command::new("sh")
        .current_dir(&path)
        .arg("-c")
        .arg("npm init --init-type module -y")
        .output()?;

    let step_2 = Command::new("sh")
        .current_dir(&path)
        .arg("-c")
        .arg(install_cmd)
        .output()?;

    Ok(())
}
