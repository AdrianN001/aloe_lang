use std::{
    fs::{File, Metadata},
    path::Path,
};

pub struct FileWrapper {
    pub native_file: Option<File>,
    pub metadata: Metadata,
    pub path: String,

    pub raw_path: String,
    pub mode: String,
}

impl FileWrapper {
    pub fn new(path: String, mode: &str) -> Result<Self, String> {
        let file_with_error = match mode {
            "r" => File::options()
                .read(true)
                .write(false)
                .append(false)
                .create(false)
                .open(&path),
            "w" => File::options()
                .read(false)
                .write(true)
                .append(false)
                .create(false)
                .open(&path),
            "a" => File::options()
                .read(false)
                .write(false)
                .append(true)
                .create(false)
                .open(&path),
            "x" => File::create_new(&path),
            _ => return Err("illegal mode parameter".into()),
        };

        let file = match file_with_error {
            Ok(file) => file,
            Err(err) => return Err(err.to_string()),
        };

        let metadata = match file.metadata() {
            Ok(metadata) => metadata,
            Err(err) => return Err(err.to_string()),
        };

        let file_path = match Path::new(&path).to_str() {
            Some(file_path) => file_path.to_string(),
            None => return Err("could not get the path of file.".into()),
        };

        Ok(Self {
            native_file: Some(file),
            metadata,
            path: file_path,

            raw_path: path,
            mode: mode.to_string(),
        })
    }

    pub fn type_name(&self) -> String {
        "<native object 'FileWrapper'>".into()
    }

    pub fn inspect(&self) -> String {
        format!("[FileWrapper for {:?}]", self.path)
    }
}

impl Clone for FileWrapper {
    fn clone(&self) -> Self {
        Self::new(self.raw_path.clone(), &self.mode.clone()).expect("clone error")
    }
}

impl PartialEq for FileWrapper {
    fn eq(&self, other: &FileWrapper) -> bool {
        self.path == other.path && self.mode == other.mode
    }
}

impl Eq for FileWrapper {}
