use std::{
    fs::{File, Metadata},
    path::Path,
};

#[derive(Debug)]
pub struct FileWrapper {
    pub native_file: Option<File>,
    pub metadata: Metadata,
    pub path: String,

    pub raw_path: String,
    pub is_write_only: bool,
}

impl FileWrapper {
    pub fn new(path: String) -> Result<Self, String> {
        let file_with_error = File::open(path.clone());
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
            is_write_only: false,
        })
    }

    pub fn create(path: String) -> Result<Self, String> {
        let file_with_error = File::create_new(path.clone());
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
            is_write_only: true,
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
        Self::new(self.raw_path.clone()).expect("clone error")
    }
}

impl PartialEq for FileWrapper {
    fn eq(&self, _other: &FileWrapper) -> bool {
        false
    }
}

impl Eq for FileWrapper {}
