use std::{collections::HashMap, path::Path};
pub use sysinfo::{Disk, Disks};
use walkdir::WalkDir;

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

    let mut top_directories = HashMap::new();

    for entry in WalkDir::new(root).max_depth(1) {
        if let Ok(entry) = entry {
            let path = entry.path().display().to_string();
            let storage_size = entry.metadata().unwrap().len();
            if top_directories.len() as u8 <= n_directories {
                top_directories.insert(path, storage_size);
            } else {
                if let Some((key, value)) = top_directories.iter().min_by_key(|(_, v)| v) {
                    if storage_size > *value {
                        top_directories.remove(key);
                        top_directories.insert(path, storage_size);
                    }
                }
            }
        }
    }
    Ok(())
}

struct StorageEntry {
    path: String,
    storage_size: u64,
}
