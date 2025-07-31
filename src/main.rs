#![warn(clippy::all, clippy::pedantic)]
use std::env;
use std::fs;
use std::path::PathBuf;
use chrono::Utc;
use rdev::{grab, EventType, Key, Event};
use screenshots::Screen;

const TARGET_DIR: &str = "my_screens";

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Arguments: {:?}", args);
    let screens_dir = args.get(1).unwrap_or(&TARGET_DIR.to_string()).to_string();
    println!("Screenshots will be saved to: {}", screens_dir);

    let mut path = env::current_dir().unwrap();
    path.push(&screens_dir);
    
    create_screenshots_dir(&path.to_string_lossy());
    
    if is_dir_exists(&screens_dir) {
        print!("Directory {} already exists. ", screens_dir);
    } else {
        println!("Directory {} does not exist.", screens_dir);
    }

    if let Err(error) = grab(move |e| callback(e, &path.to_string_lossy())) {
        println!("Error: {error:?}");
    }
} 

fn callback(event: Event, path: &str) -> Option<Event> {
    if let EventType::KeyPress(Key::F12) = event.event_type {
        let mut path_buf = PathBuf::from(path);
        
        path_buf.push("");
        let total_path = path_buf.to_str().unwrap().to_string();

        std::thread::spawn(move || {
            println!("F12 pressed, taking screenshot...");
            make_screen(&total_path);
            println!("Screen made...");
        });
        
        None
    } else {
        Some(event)
    }
}

fn make_screen (screens_dir :&str) {
    let screens = Screen::all().unwrap();

    for i in 0..screens.len() {
        let screen = &screens[i];
        let image = screen.capture().unwrap();
        let current_time = Utc::now();

        let filename = format!("scr-{}-{}.png", i, current_time.format("%d-%m-%Y_%H_%M_%S"));

        let mut full_path = PathBuf::from(screens_dir);
        full_path.push(filename);

        if let Err(e) = image.save(full_path) {
            println!("Route -> {}", screens_dir.to_string());
            eprintln!("Failed to save screenshot: {}", e);
        }
    }
}

fn is_dir_exists(path: &str) -> bool {
    fs::metadata(path).map_or(false, |metadata| metadata.is_dir())
}

fn create_screenshots_dir(path: &str) {
    if let Err(e) = fs::create_dir_all(path) {
        println!("path: {}", path);
        eprintln!("Failed to create directory {}: {}", path, e);
        std::process::exit(1);
    } else {
        println!("Directory created successfully: {}", path);
    }
}

// fn delete_screenshots_dir(path: &str) {
//     if let Err(e) = fs::remove_dir_all(path) {
//         eprintln!("Failed to delete directory {}: {}", path, e);
//         std::process::exit(1);
//     } else {
//         println!("Directory deleted successfully: {}", path);
//     }
// }