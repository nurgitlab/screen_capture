#![warn(clippy::all, clippy::pedantic)]
use std::env;
use std::fs;
use std::path;
use rdev::{grab, EventType, Key, Button, Event};

const TARGET_DIR: &str = "my_screens";

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Arguments: {:?}", args);
    let screens_dir = args.get(1).unwrap_or(&TARGET_DIR.to_string()).to_string();
    println!("Screenshots will be saved to: {}", screens_dir);



    create_screenshots_dir(&screens_dir);
    if is_dir_exists(&screens_dir) {
        print!("Directory {} already exists. ", screens_dir);
    } else {
        println!("Directory {} does not exist.", screens_dir);
    }

    if let Err(error) = grab(move |e| callback(e, &screens_dir)) {
        println!("Error: {error:?}");
    }
}

fn callback(event: Event, path: &str) -> Option<Event> {
    match event.event_type {
        EventType::KeyPress(Key::F12) => {
            println!("F12 pressed, exiting...");
            None
        }
        _ => Some(event),
    }
}
fn is_dir_exists(path: &str) -> bool {
    fs::metadata(path).map_or(false, |metadata| metadata.is_dir())
}

fn create_screenshots_dir(path: &str) {
    if let Err(e) = fs::create_dir_all(path) {
        eprintln!("Failed to create directory {}: {}", path, e);
        std::process::exit(1);
    } else {
        println!("Directory created successfully: {}", path);
    }
}

fn delete_screenshots_dir(path: &str) {
    if let Err(e) = fs::remove_dir_all(path) {
        eprintln!("Failed to delete directory {}: {}", path, e);
        std::process::exit(1);
    } else {
        println!("Directory deleted successfully: {}", path);
    }
}