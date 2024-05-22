#![allow(dead_code)]

use chrono;

fn get_time() -> String {
    let time = chrono::Local::now();
    return time.format("%H:%M:%S").to_string();
}

pub fn out(content: String) {
    println!("{} - [\x1b[34mOUT\x1b[0m] {}", get_time(), content);
}

pub fn error(content: String) {
    println!("{} - [\x1b[31mERR\x1b[0m] {}", get_time(), content);
}

pub fn warn(content: String) {
    println!("{} - [\x1b[33mWARN\x1b[0m] {}", get_time(), content);
}

pub fn success(content: String) {
    println!("{} - [\x1b[32mOK\x1b[0m] {}", get_time(), content);
}