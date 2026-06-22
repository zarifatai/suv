use std::path::Path;
pub use sysinfo::{Disk, Disks};

pub fn get_mount_point_disk<'a>(disks: &'a Disks, mount_point: Option<&Path>) -> Option<&'a Disk> {
    let mount_point = mount_point.unwrap_or(Path::new("/"));
    disks
        .list()
        .iter()
        .find(|disk| disk.mount_point() == mount_point)
}
