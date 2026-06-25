use std::path::{Path, PathBuf};
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

pub fn get_directories_sizes(root: Option<&str>) -> Result<Vec<(PathBuf, u64)>, walkdir::Error> {
    let default_root = "/";

    let root = root.unwrap_or(default_root);
    let mut directories_sizes: Vec<(PathBuf, u64)> = Vec::new();

    for entry in WalkDir::new(root).max_depth(1) {
        if let Ok(x) = entry {
            if let Ok(metadata) = x.metadata() {
                let mut storage_size = metadata.len();
                if metadata.is_dir() {
                    let total_size_dir: u64 = WalkDir::new(x.path())
                        .into_iter()
                        .filter_map(|x| x.ok())
                        .filter_map(|x| x.metadata().ok())
                        .map(|x| x.len())
                        .sum();

                    storage_size += total_size_dir;
                    directories_sizes.push((x.path().to_path_buf(), storage_size));
                } else if metadata.is_symlink() {
                    break;
                }
            }
        }
    }

    Ok(directories_sizes)
}
