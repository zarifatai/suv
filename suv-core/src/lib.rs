use std::path::{Path, PathBuf};
pub use sysinfo::{Disk, Disks};
use walkdir::WalkDir;

pub fn get_mount_point_disk<'a>(disks: &'a Disks, mount_point: Option<&Path>) -> Option<&'a Disk> {
    let mount_point = mount_point.unwrap_or(Path::new("/"));
    disks.iter().find(|disk| disk.mount_point() == mount_point)
}

pub fn get_directories_sizes(root: Option<&str>) -> Vec<(PathBuf, u64)> {
    let root = root.unwrap_or("/");
    let mut directories_sizes: Vec<(PathBuf, u64)> = Vec::new();

    for entry in WalkDir::new(root).max_depth(1) {
        if let Ok(x) = entry {
            if let Ok(metadata) = x.metadata() {
                if metadata.is_dir() {
                    let total_size_dir: u64 = WalkDir::new(x.path())
                        .into_iter()
                        .filter_map(|x| x.ok())
                        .filter_map(|x| x.metadata().ok())
                        .map(|x| x.len())
                        .sum();

                    directories_sizes.push((x.path().to_path_buf(), total_size_dir));
                } else if metadata.is_symlink() {
                    break;
                }
            }
        }
    }

    directories_sizes
}
