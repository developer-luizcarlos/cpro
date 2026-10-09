use std::{
    env,
    fs::DirBuilder,
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
