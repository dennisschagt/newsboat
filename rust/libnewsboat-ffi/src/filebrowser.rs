use libnewsboat::filebrowser;
use std::path::Path;

// cxx doesn't allow to share types from other crates, so we have to wrap it
// cf. https://github.com/dtolnay/cxx/issues/496
struct FileBrowser(filebrowser::FileBrowser);

#[cxx::bridge(namespace = "newsboat::filebrowser::bridged")]
mod bridged {
    extern "Rust" {
        type FileBrowser;

        fn create(dir: String) -> Box<FileBrowser>;
    }
}

fn create(dir: String) -> Box<FileBrowser> {
    let dir = Path::new(&dir);
    Box::new(FileBrowser(filebrowser::FileBrowser::new(dir.to_owned())))
}
