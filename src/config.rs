use std::{fs::File, io::Read};
// use json::JsonValue;
// use lazy_static::lazy_static;
// use std::sync::Mutex;

use crate::log;

pub mod cfg {
    pub static mut CLIENT_ID: String = String::new();

    pub static mut ANIMATED: bool = false;
    pub static mut ANIMATED_AMOUNT: i64 = 0;
    pub static mut ANIMATED_IMG: String = String::new();
}

fn parse(content: String) -> bool {
    let json = json::parse(&content);

    if json.is_err() {
        log::error(format!("Failed to parse config file: {}", json.err().unwrap()));
        return false;
    }

    let json = json.unwrap();

    if json.has_key("client_id") {
        let client_id_raw = json["client_id"].as_str();

        if client_id_raw.is_some() {
            let client_id = client_id_raw.unwrap().to_string();
            unsafe { cfg::CLIENT_ID = client_id; }
            log::success(format!("parsed client_id as {}", client_id_raw.unwrap().to_string()));            
        } else {
            log::error("Failed to parse client_id".to_string());
            return false;
        }
    } else {
        log::error("Failed to find client_id in config file".to_string());
    }

    if json.has_key("animated") {
        let animated_raw: Option<bool> = json["animated"].as_bool();

        if animated_raw.is_some() {
            let animated_img = animated_raw.unwrap();
            unsafe { cfg::ANIMATED = animated_img }
            log::success(format!("parsed animated as {:?}", animated_img));            
        } else {
            log::error("Failed to parse animated".to_string());
            return false;
        }
    } else {
        log::error("Failed to find animated in config file".to_string());
    }

    if json.has_key("animated_img") {
        let animated_img_raw = json["animated_img"].as_str();

        if animated_img_raw.is_some() {
            let animated_img = animated_img_raw.unwrap().to_string();
            unsafe { cfg::ANIMATED_IMG = animated_img }
            log::success(format!("parsed animated as {:?}", animated_img_raw.unwrap().to_string()));            
        } else {
            log::error("Failed to parse animated_img".to_string());
            return false;
        }
    } else {
        log::error("Failed to find animated_img in config file".to_string());
    }

    if json.has_key("animated_amount") {
        let animated_amount_raw = json["animated_amount"].as_i64();

        if animated_amount_raw.is_some() {
            let animated_amount = animated_amount_raw.unwrap();
            unsafe { cfg::ANIMATED_AMOUNT = animated_amount }
            log::success(format!("parsed animated as {:?}", animated_amount));            
        } else {
            log::error("Failed to parse animated_amount".to_string());
            return false;
        }
    } else {
        log::error("Failed to find animated_amount in config file".to_string());
    }

    return true;
}

pub fn read() -> bool {
    let file = File::open("config.json");

    if file.is_err() {
        log::error(format!("Failed to open config file: {}", file.err().unwrap()));
        return false;
    }

    let mut contents = String::new();
    file.unwrap().read_to_string(&mut contents).unwrap();

    if !parse(contents) {
        log::error("Failed to parse config file".to_string());
        return false;
    }

    log::success("Successfully parsed config file".to_string());

    return true;
}