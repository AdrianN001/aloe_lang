use std::path::{Path, PathBuf};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct PathWrapper {
    pub native_object: PathBuf,

    pub repr_str: String,
}

impl PathWrapper {
    pub fn new(path: &str) -> Result<PathWrapper, String> {
        let native_path = Path::new(path).to_path_buf();

        Ok(PathWrapper {
            repr_str: path.to_string(),
            native_object: native_path,
        })
    }

    pub fn new_from_pathbuf(path_buf: PathBuf) -> PathWrapper {
        let repr_str = path_buf.to_string_lossy().to_string();

        PathWrapper {
            repr_str,
            native_object: path_buf,
        }
    }

    pub fn inspect(&self) -> String {
        format!("[PathWrapper for {:?}]", self.repr_str)
    }

    pub fn type_name(&self) -> String {
        "<native object 'PathWrapper'>".into()
    }
}
