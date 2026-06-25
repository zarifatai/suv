use suv_core::get_directories_sizes;

fn main() {
    let n_top_directories = 5;
    let mut directories = get_directories_sizes(Some("/"));
    directories.sort_by_key(|&(_, size)| size);
    let top_directories: Vec<_> = directories.iter().rev().take(n_top_directories).collect();
    println!("Top directories:\n{top_directories:?}");
}
