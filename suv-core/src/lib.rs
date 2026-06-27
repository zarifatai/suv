use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
pub use sysinfo::{Disk, Disks};
use walkdir::WalkDir;

pub fn get_mount_point_disk<'a>(disks: &'a Disks, mount_point: Option<&Path>) -> Option<&'a Disk> {
    let mount_point = mount_point.unwrap_or(Path::new("/"));
    disks.iter().find(|disk| disk.mount_point() == mount_point)
}

// pub fn get_directories_sizes(root: Option<&str>) -> Vec<(PathBuf, u64)> {
pub fn get_directories_sizes(root: Option<&str>) -> Vec<(String, u64)> {
    let root = root.unwrap_or("/");
    // let mut directories_sizes: Vec<(PathBuf, u64)> = Vec::new();
    let mut directories_sizes: HashMap<String, u64> = HashMap::new();

    let entries = WalkDir::new(root)
        .same_file_system(true)
        .into_iter()
        .filter_map(|x| x.ok());

    for entry in entries {
        let path = entry.path();

        if let Ok(metadata) = entry.metadata() {
            if let Some(tld) = get_top_level_directory(path.to_path_buf()) {
                directories_sizes
                    .entry(tld)
                    .and_modify(|x| *x += metadata.len())
                    .or_insert(0);
            }
        }
    }
    directories_sizes.into_iter().collect()
}

fn get_top_level_directory(path: PathBuf) -> Option<String> {
    let (prefix, _) = path.to_str()?.split_once('/')?;
    Some(prefix.to_owned())
}

pub fn get_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for entry in WalkDir::new("/").into_iter().filter_map(|e| e.ok()) {
        paths.push(entry.into_path());
    }
    paths
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_top_directories() {
        let n_top_directories = 5;
        let mut directories = get_directories_sizes(Some("/"));
        directories.sort_by_key(|&(_, size)| size);
        let top_directories: Vec<_> = directories.iter().rev().take(n_top_directories).collect();
        println!("Top directories:\n{top_directories:?}");
    }
}
