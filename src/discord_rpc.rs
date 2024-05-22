#![allow(dead_code)]

use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};

use lazy_static::lazy_static;
use std::sync::Mutex;

use crate::log;
use crate::config;

lazy_static! {
    static ref CLIENT: Mutex<DiscordIpcClient> = Mutex::new(DiscordIpcClient::new("").unwrap());
}

pub fn init() {
    unsafe {
        let mut client = CLIENT.lock().unwrap();
        
        *client = DiscordIpcClient::new(config::cfg::CLIENT_ID.as_str()).unwrap();
        
        client.connect()
            .expect("Failed to connect to Discord IPC");
    }

    log::success("Successfully connected to Discord IPC".to_string());
}

pub fn set_activity(
    state: &str,
    details: &str,
    large_image: Option<&str>,
    large_text: Option<&str>,
    small_image: Option<&str>,
    small_text: Option<&str>,
    timestamp: Option<i64>,
    first_button_label: Option<&str>,
    first_button_url: Option<&str>,
    second_button_label: Option<&str>,
    second_button_url: Option<&str>
) -> bool {
    let mut activity_payload = activity::Activity::new()
        .state(state)
        .details(&details);

    let mut images = activity::Assets::new();

    // println!("{:?}", large_image);
    // println!("{:?}", large_image.is_some());
    // println!("{:?}", large_image.unwrap());

    if large_image.is_some() {
        images = images.large_image(large_image.unwrap());
    }

    if large_text.is_some() {
        images = images.large_text(large_text.unwrap());
    }

    if small_image.is_some() {
        images = images.small_image(small_image.unwrap());
    }

    if small_text.is_some() {
        images = images.small_text(small_text.unwrap());
    }
    
    activity_payload = activity_payload.assets(images);

    if timestamp.is_some() {
        activity_payload = activity_payload.timestamps(
            activity::Timestamps::new()
                .start(timestamp.unwrap())
        );
    }

    let mut buttons = Vec::new();
    
    if 
        first_button_label.is_some() && 
        first_button_url.is_some() {

        buttons.push(
            activity::Button::new(
                first_button_label.unwrap(), 
                first_button_url.unwrap()
            )
        )
    }

    if 
        second_button_label.is_some() && 
        second_button_url.is_some() {

        buttons.push(
            activity::Button::new(
                second_button_label.unwrap(), 
                second_button_url.unwrap()
            )
        )
    }

    if !buttons.is_empty() {
        activity_payload = activity_payload.buttons(buttons);
    }

    let result = CLIENT
        .lock()
        .unwrap()
        .set_activity(activity_payload);

    let status = result.is_ok();

    if status {
        log::success("Successfully set activity".to_string());
    } else {
        log::error(format!("Failed to set activity: {}", result.err().unwrap()));
    }

    return status;
}

pub fn clear_activity() -> bool {
    let result = CLIENT
        .lock()
        .unwrap()
        .clear_activity();

    let status = result.is_ok();
    if status {
        log::success("Successfully cleared activity".to_string());
    } else {
        log::error("Failed to clear activity".to_string());
    }

    return status;
}

pub fn disconnect() -> bool {
    let result = CLIENT
        .lock()
        .unwrap()
        .close();

    return result.is_ok();
}