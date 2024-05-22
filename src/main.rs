use core::time;
// use std::{env, thread, time};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

mod discord_rpc;
mod config;
mod log;

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    if !config::read() {
        log::error("Failed to read config file, exiting...".to_string());
        return;
    }

    discord_rpc::init();

    unsafe {
        if config::cfg::ANIMATED {
            log::warn("Animated presence is enabled".to_string());

            let mut i = 0;
            loop {
                discord_rpc::set_activity(
                    "Rusty Rpc", "Testing",
                    Some(&("catgif".to_string() + &i.to_string())), None, None, None,
                    None, 
                    None, None, None, None
                );

                i += 1;
                if i >= config::cfg::ANIMATED_AMOUNT {
                    i = 0;
                }

                thread::sleep(time::Duration::from_millis(500));
            }

        } else {
            discord_rpc::set_activity(
                "Rusty Rpc", "Testing",
                Some("catto"), None, None, None,
                None, 
                Some("Teste"), Some("discord://-/apps"), None, None
            );
        }
    }

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    while running.load(Ordering::SeqCst) {}

    log::warn("Exit request event received, exiting...".to_string());

    if discord_rpc::disconnect() {
        log::success("Successfully disconnected from Discord IPC".to_string());
    } else {
        log::error("Failed to disconnect from Discord IPC".to_string());
    }
}
