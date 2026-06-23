use std::{collections::HashMap, path::Path};
pub use sysinfo::{Disk, Disks};
use walkdir::{DirEntry, WalkDir};

pub fn get_mount_point_disk<'a>(disks: &'a Disks, mount_point: Option<&Path>) -> Option<&'a Disk> {
    let default_mount_point = "/";
    let mount_point = mount_point.unwrap_or(Path::new(default_mount_point));
    disks
        .list()
        .iter()
        .find(|disk| disk.mount_point() == mount_point)
}

pub fn get_largest_directories(
    n_directories: Option<u8>,
    root: Option<&str>,
) -> Result<(), walkdir::Error> {
    let n_directories = n_directories.unwrap_or(5);
    let root = root.unwrap_or("/");

    for entry in WalkDir::new(root).max_depth(1) {
        if let Ok(x) = entry {
            if let Ok(metadata) = x.metadata() {
                let mut storage_size = metadata.len();
                if metadata.is_dir() {
                    storage_size += WalkDir::new(x.path())
                        .into_iter()
                        .filter_map(|x| x.ok())
                        .filter_map(|x| x.metadata().ok())
                        .map(|x| x.len())
                        .sum();
                } else if metadata.is_symlink() {
                    todo!();
                }
            }
        }
    }

    Ok(())
}

struct DirSummary {
    path: String,
    storage_size: u64,
}
