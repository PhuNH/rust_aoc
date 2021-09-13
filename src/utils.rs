use std::fs;
use std::env;
use md5;

pub fn read_input() -> String {
    let path = env::current_exe().unwrap();
    let exe_name = path.file_name().unwrap().to_str().unwrap();
    fs::read_to_string(format!("input/{}", exe_name)).unwrap()
}

pub fn hex_md5(salt: &str, additional: &String) -> String {
    format!("{:x}", md5::compute(salt.to_owned() + additional))
}
