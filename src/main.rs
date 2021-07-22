use ring::digest;
use std::fs::{self, File};
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

static BASE_PATH: &'static str = "data";

fn main() {
    let path = Path::new(BASE_PATH);
    let manifest = visit_dirs(&path)
        .expect("Failed to visit dirs")
        .into_iter()
        .map(handle_dir_entry)
        .collect::<Vec<_>>();

    dbg!(&manifest);
}

// X load all files in a dir recursively
// X create a description of them
// Serve that up over HTTP
// do that in two places
// create a diff plan
// execute that and see if it works

#[derive(Debug)]
struct Entry {
    hash: String,
    path: String,
    modified: u128,
}

fn handle_dir_entry(path: PathBuf) -> Entry {
    let path = clean_path(path);

    // Get Hash
    let mut buf = vec![];
    let mut f = File::open(&path).expect("Could not open file");
    let modified = f
        .metadata()
        .unwrap()
        .modified()
        .unwrap()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    f.read_to_end(&mut buf).expect("Could not read file");
    let hash = digest::digest(&digest::SHA256, &buf);
    let hash = format!("{:?}", hash)[7..].to_string();

    // Format Path
    let path = path
        .into_iter()
        .skip(1)
        .collect::<PathBuf>()
        .to_string_lossy()
        .to_string();

    Entry {
        path,
        hash,
        modified,
    }
}

fn clean_path(path: PathBuf) -> PathBuf {
    let base = Path::new(BASE_PATH);
    let to_skip = base.canonicalize().unwrap().components().count();
    path.canonicalize()
        .unwrap()
        .components()
        .into_iter()
        .skip(to_skip - 1)
        .collect::<PathBuf>()
}

fn visit_dirs(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut paths = vec![];
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                paths.extend(visit_dirs(&path)?);
            } else {
                paths.push(path);
            }
        }
    }
    Ok(paths)
}
