use std::collections::HashSet;
use std::os::unix::fs::MetadataExt;
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

pub fn get_directories_sizes(root_path: PathBuf) -> HashMap<PathBuf, u64> {
    let entries = WalkDir::new(&root_path)
        .same_file_system(true)
        .into_iter()
        .filter_map(|x| x.ok());

    let mut directories_sizes: HashMap<PathBuf, u64> = HashMap::new();
    let mut seen_inodes: HashSet<(u64, u64)> = HashSet::new();
    for entry in entries {
        if let Ok(metadata) = entry.metadata() {
            if metadata.nlink() > 1 {
                let inode_key = (metadata.dev(), metadata.ino());
                if !seen_inodes.insert(inode_key) {
                    continue;
                }
            }

            if let Some(tld) = get_top_level_directory(entry.path(), &root_path) {
                directories_sizes
                    .entry(tld)
                    .and_modify(|x| *x += metadata.blocks() * 512)
                    .or_insert(metadata.blocks() * 512);
            }
        }
    }
    directories_sizes
}

fn get_top_level_directory<'a>(path: &Path, root_path: &PathBuf) -> Option<PathBuf> {
    let remaining_path = path.strip_prefix(root_path).ok()?;
    let tld = remaining_path.components().next()?;
    Some(PathBuf::new().join(tld))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_top_directories() {
        let n_top_directories = 5;
        let root_path = PathBuf::from("/");
        let directories = get_directories_sizes(root_path);
        let mut directories: Vec<(PathBuf, u64)> = directories.into_iter().collect();
        directories.sort_by_key(|&(_, size)| size);
        let top_directories: Vec<_> = directories.iter().rev().take(n_top_directories).collect();
        println!("Top directories:\n{top_directories:?}");

        let total_size: u64 = directories.iter().map(|(_, s)| *s).sum();
        println!("Total size: {total_size}");
    }
}
