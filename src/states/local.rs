use std::path::PathBuf;

pub struct LocalState {
    pub working_dir: PathBuf,
    pub selected_files: Vec<PathBuf>,
}

impl LocalState {
    pub fn new() -> Self {
        let working_dir = std::env::current_dir().unwrap_or_else(|_| std::env::temp_dir());
        let selected_files = Vec::new();

        return LocalState {
            working_dir,
            selected_files,
        };
    }
}
