use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time;

mod discord_rpc;
mod config;
mod log;

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = Arc::clone(&running);

    if !config::read() {
        handle_config();
    }

    discord_rpc::init();

    // Im a noobie, i dont want to be unsafe, help me
    unsafe {
        if config::cfg::LARGE_ANIMATED {
            log::warn("Animated presence is enabled".to_string());
        }

        thread::spawn(move || presence_loop());
    }

    /*
     * Control hander for Ctrl-C or SIGINT
     * Important to disconnect from Discord IPC before exiting
     */
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

/*
 * Prompt the user to create a new config file
 */
fn handle_config() {
    let mut input = String::new();
    log::warn("Would you like to create a new config file with the default values? (Y/n)".to_string());
    std::io::stdin().read_line(&mut input).unwrap();

    if input.trim().to_lowercase() == "y" {
        if !config::write() {
            log::error("Failed to write config file, please check the docs at https://github.com/IMXNOOBX/rusty-rpc#-config".to_string());
            std::process::exit(1);
        }

        if !config::read() {
            log::error("Failed to read config file, exiting...".to_string());
            std::process::exit(1);
        }
    } else {
        log::error("Please create a config file with the values provided in the docs at https://github.com/IMXNOOBX/rusty-rpc#-config".to_string());
        std::process::exit(1);
    }
}

/*
 * Update the presence with the config values, and in case
 * The image should be animated, loop through the images. Make sure the images are uploaded to discord 
 */
unsafe fn presence_loop() {
    let mut i = 0;
    loop {
        let status = discord_rpc::set_activity(
            config::cfg::STATE.as_str(),
            config::cfg::DETAILS.as_str(),
            Some(get_large_image().as_str()),
            Some(config::cfg::LARGE_TEXT.as_str()),
            Some(config::cfg::SMALL_IMAGE.as_str()),
            Some(config::cfg::SMALL_TEXT.as_str()),
            get_timestamp(),
            Some(config::cfg::FIRST_BUTTON_LABEL.as_str()),
            Some(config::cfg::FIRST_BUTTON_URL.as_str()),
            Some(config::cfg::SECOND_BUTTON_LABEL.as_str()),
            Some(config::cfg::SECOND_BUTTON_URL.as_str())
        );

        if config::cfg::LARGE_ANIMATED {
            i = (i + 1) % config::cfg::LARGE_ANIMATED_AMOUNT;
            thread::sleep(time::Duration::from_millis(config::cfg::LARGE_ANIMATED_SPEED));
        }

        if status {
            loop {}
        } else {
            discord_rpc::disconnect();
            thread::sleep(time::Duration::from_secs(5));
            discord_rpc::init();
        }
    }
}

/*
 * Helper method to get the animated image or the static image depending on the config 
 */
unsafe fn get_large_image() -> String {
    if config::cfg::LARGE_ANIMATED {
        config::cfg::LARGE_ANIMATED_IMG.to_string()
    } else {
        config::cfg::LARGE_IMAGE.to_string()
    }
}

/*
 * Helper method to get the current timestamp if the config allows it
 */
unsafe fn get_timestamp() -> Option<i64> {
    if config::cfg::TIMESPAMP {
        Some(time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_secs() as i64)
    } else {
        None
    }
}
