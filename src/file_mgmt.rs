use crate::args::{FileType, Kind};
use crate::deps::install_front_deps;
use crate::file_content::{
    get_css_content, get_html_content, get_js_content, get_tsconfig_content,
};
use std::{
    env,
    fs::{self, DirBuilder},
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
    process,
};

pub fn create_dir(parent_dir: &PathBuf, dir_name: &String) -> Result<PathBuf, Error> {
    let path = Path::new(&parent_dir).join(dir_name);

    if path.exists() {
        let msg = format!("{} directory already exists", dir_name);
        let err = Error::new(ErrorKind::AlreadyExists, msg);

        return Err(err);
    }

    match DirBuilder::new().create(&path) {
        Ok(_) => {
            return Ok(path);
        }
        Err(err) => {
            return Err(err);
        }
    }
}

pub fn create_subdir(parent_dir: &PathBuf, kind: &Kind, ext: &FileType) {
    match kind {
        Kind::Front => create_front_project(parent_dir, ext),
        Kind::Back => (),
    };
}

fn create_front_project(parent_dir: &PathBuf, ext: &FileType) {
    let assets_dir = &create_dir(parent_dir, &String::from("assets")).unwrap();
    let styles_dir = &create_dir(assets_dir, &String::from("styles")).unwrap();
    let scripts_dir = &create_dir(assets_dir, &String::from("scripts")).unwrap();

    let html_file_path = parent_dir.join("index.html");
    let css_file_path = styles_dir.join("main.css");
    let script_file_path = match ext {
        FileType::JS => scripts_dir.join("main.js"),
        FileType::TS => scripts_dir.join("main.ts"),
    };

    let html_content = get_html_content();
    let css_content = get_css_content();
    let script_content = get_js_content();

    if *ext == FileType::TS {
        let tsconfig_path = parent_dir.join("tsconfig.json");
        let tsconfig_content = get_tsconfig_content();

        fs::write(tsconfig_path, tsconfig_content);
    }

    fs::write(html_file_path, html_content).unwrap();
    fs::write(css_file_path, css_content).unwrap();
    fs::write(script_file_path, script_content).unwrap();

    match install_front_deps() {
        Ok(_) => println!("Dependencies installed"),
        Err(err) => eprint!("{}", err.kind()),
    }
}
