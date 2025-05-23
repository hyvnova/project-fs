use std::fs;
use std::path::{ Path, PathBuf };
use fs_tree_db::excludes::get_excludes;
use fs_tree_db::Tree;
use tauri::{ AppHandle, Emitter };
use types::Node;
use parser::Parser;
use once_cell::sync::Lazy;
use walkdir::WalkDir;
use std::sync::RwLock;
use rayon::prelude::*;
use crossbeam::channel::unbounded;

mod parser;
mod macros;
mod functions;
mod types;

// const DEFUALT_INITIAL_PATHL: &str = r#"C:\Users\Hyvnt\T"#;
const DEFUALT_INITIAL_PATHL: &str = r#"C:\"#;
const DEFAULT_SAVE_PATH: &str =
    r#"C:\Users\Hyvnt\T\Rust\file-explorer\fs_tree_db\save\tree.bincode"#;

pub static GLOBAL_TREE: Lazy<RwLock<Option<Tree>>> = Lazy::new(|| { RwLock::new(None) });

pub fn with_tree<F, R>(f: F) -> R where F: FnOnce(&Tree) -> R {
    let tree = GLOBAL_TREE.read().unwrap();
    f(tree.as_ref().expect("Tree not initialized"))
}

#[tauri::command]
fn read_dir(
    initial_path: Option<String>,
    path: String,
    limit: u32,
    show_full_path: bool
) -> Result<Vec<String>, String> {
    let parent_path: String = initial_path.unwrap_or_else(|| DEFUALT_INITIAL_PATHL.to_string());
    dbg!(&parent_path, &path);

    let clean_path = path.trim_start_matches(&['/', '\\'][..]);
    let mut full_path = PathBuf::from(&parent_path);
    full_path.push(clean_path);

    dbg!(&full_path);

    if !full_path.exists() {
        return Err("Directory doesn't exist".to_string());
    }

    match fs::read_dir(&full_path) {
        Ok(entries) => {
            let files: Vec<String> = entries
                .filter_map(|entry| entry.ok().map(|e| e.path()))
                .take(limit as usize)
                .map(|path| {
                    if show_full_path {
                        path.display().to_string()
                    } else {
                        path.strip_prefix(&parent_path)
                            .unwrap_or_else(|_| Path::new("Cant"))
                            .display()
                            .to_string()
                    }
                })
                .collect();

            Ok(files)
        }
        Err(e) => Err(format!("Failed to read directory: {}", e)),
    }
}

#[tauri::command]
async fn load_tree() -> Result<(), String> {
    let save_path = PathBuf::from(DEFAULT_SAVE_PATH);

    let new_tree = if save_path.exists() {
        Tree::load(&save_path).expect("Couldn't load tree")
    } else {
        let t = Tree::new(PathBuf::from(DEFUALT_INITIAL_PATHL), get_excludes()).await;
        t.save(&save_path).expect("Couldn't save tree");
        t
    };

    let mut tree = GLOBAL_TREE.write().unwrap(); // <- Now safe, no `.await` after
    *tree = Some(new_tree);

    dbg!("Tree loaded");

    Ok(())
}


#[tauri::command]
async fn stream_query(
    app: AppHandle,
    q: String,
    limit: usize,
    chunk_size: usize
) -> Result<(), String> {
    let (sender, receiver) = unbounded();

    // Spawn filtering in background thread
    let app_clone = app.clone();

    std::thread::spawn(move || {
        let filters: Vec<Node> = Parser::parse(q.clone())
            .into_iter()
            .filter(|node| match node {
                Node::Call { func, args } => {
                    let dummy_path = PathBuf::from("/").to_path_buf();
                    func(&dummy_path.to_string_lossy().to_string(), args).is_ok()
                }
                Node::Fail(e) => {
                    app_clone.emit("parse-error", e).unwrap();
                    false
                }
            })
            .collect();

        let mut results = Vec::new();
        let mut total_sent = 0;

        for entry in WalkDir::new(DEFUALT_INITIAL_PATHL)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            if total_sent >= limit {
                break;
            }

            let path = entry.path().to_path_buf();
            let path_str = path.to_string_lossy().to_string();

            let valid = filters.iter().all(|filter| match filter {
                Node::Call { func, args } => func(&path_str, args).unwrap_or(false),
                _ => false,
            });

            if valid {
                results.push(path_str);
                total_sent += 1;

                if results.len() >= chunk_size {
                    sender.send(results.clone()).unwrap();
                    results.clear();
                }
            }
        }

        // Send remaining items if any
        if !results.is_empty() {
            sender.send(results).unwrap();
        }
    });

    // clear UI
    app.emit("clear", true).unwrap();

    tauri::async_runtime::spawn(async move {
        for chunk in receiver {
            app.emit("query-chunk", chunk).unwrap();
        }
    });

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder
        ::default()
        // .setup(|app| {
        //     // This runs once at launch, before frontend is ready.
        //     tauri::async_runtime::spawn(async move {
        //         match load_tree().await {
        //             Ok(_) => println!("Tree loaded at startup."),
        //             Err(e) => eprintln!("Failed to load tree: {e}"),
        //         }
        //     });

        //     Ok(())
        // })
        .plugin(tauri_plugin_opener::init())
        // << handlers >>
        .invoke_handler(tauri::generate_handler![read_dir, stream_query])

        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
