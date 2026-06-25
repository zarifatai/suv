use suv_core::get_directories_sizes;

fn main() {
    let n_top_directories = 5;
    let directories = get_directories_sizes(Some("/"));

    if let Ok(mut dirs) = directories {
        dirs.sort_by_key(|&(_, size)| size);
        let top_directories: Vec<_> = dirs.iter().rev().take(n_top_directories).collect();
        println!("Top directories:\n{top_directories:?}");
    }
}
