use std::sync::Arc;
use tokio::sync::Mutex;
use std::path::PathBuf;
use std::collections::VecDeque;
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
pub struct Tree {
    // Holds all paths files found
    data: Vec<PathBuf>,
}

type SharedQueue = Arc<Mutex<VecDeque<PathBuf>>>;
type SharedData = Arc<Mutex<Vec<PathBuf>>>;

const THREAD_MULTIPLIER: usize = {
    #[cfg(feature = "low")]
    { 64 }

    #[cfg(all(not(feature = "low"), feature = "medium"))]
    { 128 }

    #[cfg(all(not(feature = "low"), not(feature = "medium"), feature = "high"))]
    { 512 }

    #[cfg(all(not(feature = "low"), not(feature = "medium"), not(feature = "high"), feature = "peak"))]
    { 1024 }

    #[cfg(not(any(feature = "low", feature = "medium", feature = "high", feature = "peak")))]
    { 32 } // fallback
};


impl Tree {
    pub async fn new(path: PathBuf, lazy_exclude: Vec<String>) -> Self {

        let data: SharedData = Arc::new(Mutex::new(Vec::new()));
        let queue: SharedQueue = Arc::new(Mutex::new(VecDeque::new()));
        let exclude = Arc::new(lazy_exclude);

        queue.lock().await.push_back(path.clone());

        let mut tasks = Vec::new();

        let num_workers = std::thread::available_parallelism()
            .map(|n| n.get() * THREAD_MULTIPLIER) // ← try 8x core count
            .unwrap_or(32);       // safety net

        for _ in 0..num_workers {
            let q = queue.clone();
            let d = data.clone();
            let e = exclude.clone();

            tasks.push(
                tokio::spawn(async move {
                    Tree::worker_loop(q, d, e).await;
                })
            );
        }

        for t in tasks {
            t.await.unwrap();
        }

        let data_final = Arc::try_unwrap(data).unwrap().into_inner();

        Tree { data: data_final }
    }

    async fn worker_loop(queue: SharedQueue, data: SharedData, exclude: Arc<Vec<String>>) {
        loop {
            let next_path = {
                let mut q = queue.lock().await;
                q.pop_front()
            };

            let Some(path) = next_path else {
                break;
            };
            let _ = Tree::process_path(&path, &queue, &data, &exclude).await;
        }
    }

    async fn process_path(
        path: &PathBuf,
        queue: &SharedQueue,
        data: &SharedData,
        exclude: &Arc<Vec<String>>
    ) -> Result<(), String> {
        if !path.exists() {
            return Ok(());
        }

        let entries = match tokio::fs::read_dir(path).await {
            Ok(dir) => dir,
            Err(_) => {
                return Ok(());
            }
        };

        let mut dir = entries;
        while let Ok(Some(entry)) = dir.next_entry().await {
            let file_type = entry.file_type().await.unwrap();
            let entry_path = entry.path();

            if file_type.is_file() {
                // dbg!(&entry_path);
                data.lock().await.push(entry_path);
            } else if file_type.is_dir() {
                let name = entry.file_name();
                let name_str = name.to_string_lossy();

                if exclude.iter().any(|e| e == &name_str) {
                    continue;
                }

                queue.lock().await.push_back(entry_path);
            }
        }

        Ok(())
    }

    pub fn len(&self) -> usize {
        return self.data.len();
    }

    pub fn save(&self, path: &PathBuf) -> std::io::Result<()> {
        let bytes = bincode::serialize(self).unwrap();
        std::fs::write(path, bytes)
    }

    pub fn load(path: &PathBuf) -> std::io::Result<Self> {
        let bytes = std::fs::read(path)?;
        let tree: Tree = bincode::deserialize(&bytes).unwrap();
        Ok(tree)
    }

    pub fn get_data(&self) -> Vec<PathBuf> {
        self.data.clone()
    }
}