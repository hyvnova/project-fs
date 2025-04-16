pub static DEFAULT_EXCLUDES: &[&str] = &[
    // Node.js / JS
    "node_modules",
    "dist",
    "build",
    ".parcel-cache",
    ".turbo",

    // Rust
    "target",

    // Python
    "__pycache__",
    ".mypy_cache",
    ".pytest_cache",

    // Java
    "bin",
    "out",

    // Git & VCS
    ".git",
    ".svn",
    ".hg",

    // OS + IDE junk
    ".DS_Store",
    ".idea",
    ".vscode",
    ".Trash",
    ".history",
    "thumbs.db",

    // Dependency managers
    ".venv",
    "env",
    "venv",
    ".env",
    ".yarn",
    ".pnpm-store",

    // Build system
    "cmake-build-debug",
    "cmake-build-release",
    ".gradle",

    // Misc big/trash folders
    "logs",
    "cache",
    "tmp",
    "temp",
    "coverage",
];

pub fn get_excludes() -> Vec<String> {
    DEFAULT_EXCLUDES.iter()
        .map(|s| s.to_string())
        .collect()
}
