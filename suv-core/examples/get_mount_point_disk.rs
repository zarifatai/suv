use suv_core::{Disks, get_mount_point_disk};

fn main() {
    let all_disks = Disks::new_with_refreshed_list();
    let mount_point = None;
    let disk = get_mount_point_disk(&all_disks, mount_point);
    if let Some(x) = disk {
        println!("{x:?}");
    }
}
