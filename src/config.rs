use std::{fs::File, io::{Read, Write}};
// use json::JsonValue;
// use lazy_static::lazy_static;
// use std::sync::Mutex;

use crate::log;

pub mod cfg {
    pub static mut CLIENT_ID: String = String::new();

    pub static mut STATE: String = String::new();
    pub static mut DETAILS: String = String::new();
    pub static mut TIMESPAMP: bool = false;

    pub static mut LARGE_ANIMATED: bool = false;
    pub static mut LARGE_ANIMATED_SPEED: u64 = 0;
    pub static mut LARGE_ANIMATED_AMOUNT: u64 = 0;
    pub static mut LARGE_ANIMATED_IMG: String = String::new();
    pub static mut LARGE_IMAGE: String = String::new();
    pub static mut LARGE_TEXT: String = String::new();
    pub static mut SMALL_IMAGE: String = String::new();
    pub static mut SMALL_TEXT: String = String::new();

    pub static mut FIRST_BUTTON_LABEL: String = String::new();
    pub static mut FIRST_BUTTON_URL: String = String::new();
    pub static mut SECOND_BUTTON_LABEL: String = String::new();
    pub static mut SECOND_BUTTON_URL: String = String::new();
}

fn parse(content: String) -> bool {
    let json = json::parse(&content);

    if json.is_err() {
        log::error(format!("Failed to parse config file: {}", json.err().unwrap()));
        return false;
    }

    let json = json.unwrap();

    unsafe {
        if let Some(client_id) = json["client_id"].as_str() {
            cfg::CLIENT_ID = client_id.to_string();
            log::success(format!("Parsed client_id as {}", client_id));
        } else {
            log::error("Failed to parse client_id, this is a required field!".to_string());
            return false;
        }

        if let Some(state) = json["state"].as_str() {
            cfg::STATE = state.to_string();
            log::success(format!("Parsed state as {}", state));
        } else {
            log::error("Failed to parse state, this is a required field!".to_string());
            return false;
        }

        if let Some(details) = json["details"].as_str() {
            cfg::DETAILS = details.to_string();
            log::success(format!("Parsed details as {}", details));
        } else {
            log::error("Failed to parse details, this is a required field!".to_string());
            return false;
        }

        if let Some(timestamp) = json["timestamp"].as_bool() {
            cfg::TIMESPAMP = timestamp;
            log::success(format!("Parsed timestamp as {}", timestamp));
        }

        if json["images"].is_object() {
            let images = &json["images"];

            if let Some(large_animated) = images["large_animated"].as_bool() {
                cfg::LARGE_ANIMATED = large_animated;
                log::success(format!("Parsed large_animated as {}", large_animated));
            }

            if let Some(large_animate_speed) = images["large_animate_speed"].as_u64() {
                cfg::LARGE_ANIMATED_SPEED = large_animate_speed;
                log::success(format!("Parsed large_animate_speed as {}", large_animate_speed));
            }
            if let Some(large_animated_amount) = images["large_animated_amount"].as_u64() {
                cfg::LARGE_ANIMATED_AMOUNT = large_animated_amount;
                log::success(format!("Parsed large_animated_amount as {}", large_animated_amount));
            }

            if let Some(large_animated_img) = images["large_animated_img"].as_str() {
                cfg::LARGE_ANIMATED_IMG = large_animated_img.to_string();
                log::success(format!("Parsed large_animated_img as {}", large_animated_img));
            }

            if let Some(large_image) = images["large_image"].as_str() {
                cfg::LARGE_IMAGE = large_image.to_string();
                log::success(format!("Parsed large_image as {}", large_image));
            }

            if let Some(large_text) = images["large_text"].as_str() {
                cfg::LARGE_TEXT = large_text.to_string();
                log::success(format!("Parsed large_text as {}", large_text));
            }

            if let Some(small_image) = images["small_image"].as_str() {
                cfg::SMALL_IMAGE = small_image.to_string();
                log::success(format!("Parsed small_image as {}", small_image));
            }

            if let Some(small_text) = images["small_text"].as_str() {
                cfg::SMALL_TEXT = small_text.to_string();
                log::success(format!("Parsed small_text as {}", small_text));
            }
        }

        if json["buttons"].is_object() {
            let buttons = &json["buttons"];

            if buttons["first"].is_object() {
                let first_button = &buttons["first"];

                if let Some(label) = first_button["label"].as_str() {
                    cfg::FIRST_BUTTON_LABEL = label.to_string();
                    log::success(format!("Parsed first_button label as {}", label));
                }

                if let Some(url) = first_button["url"].as_str() {
                    cfg::FIRST_BUTTON_URL = url.to_string();
                    log::success(format!("Parsed first_button url as {}", url));
                }
            }

            if buttons["second"].is_object() {
                let second_button = &buttons["second"];

                if let Some(label) = second_button["label"].as_str() {
                    cfg::SECOND_BUTTON_LABEL = label.to_string();
                    log::success(format!("Parsed second_button label as {}", label));
                }

                if let Some(url) = second_button["url"].as_str() {
                    cfg::SECOND_BUTTON_URL = url.to_string();
                    log::success(format!("Parsed second_button url as {}", url));
                }
            }
        }
    }

    true
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

pub fn write() -> bool {
    let file = File::create("config.json");

    if file.is_err() {
        log::error(format!("Failed to create config file: {}", file.err().unwrap()));
        return false;
    }

    let mut file = file.unwrap();

    let content = r#"{
    "client_id": "1164950312914800850",

    "state": "Try out Rusty RPC, a Rust client for Discord Rich Presence!",
    "details": "Rusty RPC",
    "timestamp": true,

    "images": {
        "large_animated": false,
        "large_animate_speed": 3000,
        "large_animated_amount": 12,
        "large_animated_img": "catgif",

        "large_image": "catto",
        "large_text": "Im a cat!",
    
        "small_image": "ferris",
        "small_text": "Im a tiny crab!"
    },

    "buttons": {
        "first": {
            "label": "Developer",
            "url": "https://noob.bio"
        },
        "second": {
            "label": "Repository",
            "url": "https://github.com/IMXNOOBX/rusty-rpc"
        }
    }
}"#;

    file.write_all(content.as_bytes()).unwrap();

    log::success("Successfully wrote config file".to_string());

    return true;
}