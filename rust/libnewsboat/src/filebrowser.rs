use std::path::PathBuf;

pub struct FileBrowser {
    current_directory: PathBuf,
}

impl FileBrowser {
    pub fn new(dir: PathBuf) -> FileBrowser {
        FileBrowser {
            current_directory: dir,
        }
    }

    pub fn listDirectoryEntries(&self) {}
}

// TODO: Test behaviour if initial directory does not exist
// TODO: Test behaviour if initial directory is a file
// TODO: Test going up a directory
// TODO: Test going down a directory
