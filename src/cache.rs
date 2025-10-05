use std::{
    fs,
    path::{Path, PathBuf},
    time::UNIX_EPOCH,
};

fn get_cache_path(base_path: &Path) -> Option<PathBuf> {
    let mut cache_path = dirs::home_dir()?;
    let cache_file_name = base_path
        .display()
        .to_string()
        .replace("/", "_")
        .replace("\\", "_")
        .replace(":", "_");

    cache_path.push(".cache");
    cache_path.push("fuzzit");

    if !cache_path.exists() {
        fs::create_dir_all(&cache_path).ok()?;
    }

    cache_path.push(cache_file_name);
    Some(cache_path)
}

pub fn create_cache(base_path: &Path, repo_paths: &[PathBuf]) -> Option<()> {
    let cache_path = get_cache_path(base_path)?;

    let mtime = fs::metadata(base_path).ok()?.modified().ok()?;
    let mtime_secs = mtime.duration_since(UNIX_EPOCH).ok()?.as_secs();

    let mut content = format!("{mtime_secs}\n");
    for path in repo_paths {
        content.push_str(&format!("{}\n", path.display()));
    }

    fs::write(&cache_path, &content).ok()?;
    Some(())
}

// Check if mtime (modified time) of base path in cache is the same as current
pub fn mtime_matches_cache(base_path: &Path) -> Option<Vec<PathBuf>> {
    let cache_path = get_cache_path(base_path)?;

    let current_mtime = fs::metadata(base_path).ok()?.modified().ok()?;
    let current_mtime_secs = current_mtime.duration_since(UNIX_EPOCH).ok()?.as_secs();

    let content = fs::read_to_string(&cache_path).ok()?;
    let mut lines = content.lines();
    let cached_mtime: u64 = lines.next()?.parse().ok()?;
    let cached_repo_paths: Vec<PathBuf> = lines.map(PathBuf::from).collect();

    if cached_mtime == current_mtime_secs {
        Some(cached_repo_paths)
    } else {
        None
    }
}
