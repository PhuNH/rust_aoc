use std::fs;
use std::env;

pub fn read_input() -> String {
    let path = env::current_exe().unwrap();
    let exe_name = path.file_name().unwrap().to_str().unwrap();
    fs::read_to_string(format!("input/{}", exe_name)).unwrap()
} 
