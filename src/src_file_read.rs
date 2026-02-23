use std::fs;



pub fn read_source_file(file_path: &str) -> String{

    fs::read_to_string(file_path).unwrap()
} 
