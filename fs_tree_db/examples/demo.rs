use std::{path::PathBuf, time::Instant};

use fs_tree_db::{excludes::get_excludes, Tree};

// const DEFUALT_INITIAL_PATHL: &str = r#"C:\Users\Hyvnt\T"#;
const DEFUALT_INITIAL_PATHL: &str = r#"C:\"#;
const DEFAULT_SAVE_PATH: &str =
    r#"C:\Users\Hyvnt\T\Rust\file-explorer\fs_tree_db\save\tree.bincode"#;

#[tokio::main]
async fn main() {
    let save_path = PathBuf::from(DEFAULT_SAVE_PATH);

    // Load tree if already saved
    // if save_path.exists() {
    //     println!("Loading saved tree");
    //     let t = Tree::load(&save_path).expect("Couldn't load saved tree");
    // }

    println!("Creating new tree");

    let now = Instant::now(); // start the timer ⏱️

    let t = Tree::new(
        PathBuf::from(DEFUALT_INITIAL_PATHL),
        get_excludes()
    ).await;

    let elapsed = now.elapsed(); // stop the timer ⏱️

    println!("Tree built in {:.2?}", elapsed); // pretty format

    println!("Tree size: {}", t.len());

    t.save(&save_path).expect("Couldn't save tree");
}