use std::path::PathBuf;

mod test_file;

pub use self::test_file::*;

/// # Panics
/// Invalid Project Root
pub fn project_root() -> PathBuf {
    project_root::get_project_root().unwrap()
}
