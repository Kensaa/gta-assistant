// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod casino;
mod casino_capture;
mod constants;
mod utils;

use serde::Serialize;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tauri::Manager;

const SUPPORTED_HEIGHTS: [u32; 2] = [1080, 1440];

pub type ThreadStatus = Arc<Mutex<bool>>;
// A task creating a long running thread that can be stopped by setting the value of the ThreadStatus to false
pub type LongTask = fn(ThreadStatus);
// A task that runs for a fixed amount of time
pub type Task = fn(u16);

// A button that can be toggled on and off, starting and stopping a long running thread
// ex : fingerprint solver

#[derive(Serialize, Clone, Debug)]
struct ToggleButton {
    id: String,
    #[serde(skip_serializing)]
    task: LongTask,
    enabled_text: String,  // text to display when the button is enabled
    disabled_text: String, // text to display when the button is disabled
}

// A button that can be clicked, starting a thread for a fixed amount of time
#[derive(Serialize, Clone, Debug)]
struct TimerButton {
    id: String,
    #[serde(skip_serializing)]
    task: Task,
    delay: u16,           // delay in seconds
    default_text: String, // text to display when the timer is not running
    running_text: String, // text to display when the timer is running
}

#[derive(Clone, Serialize, Debug)]
enum Button {
    Toggle(ToggleButton),
    Timer(TimerButton),
}

struct AppState {
    running_threads: Mutex<HashMap<String, ThreadStatus>>,
    buttons: Vec<Vec<Button>>,
}

#[tauri::command]
fn get_buttons(state: tauri::State<AppState>) -> Vec<Vec<Button>> {
    state.buttons.clone()
}

#[tauri::command]
fn handle_button(state: tauri::State<AppState>, id: String, action: bool) {
    // If action is true then start the task, else stop it
    let buttons: Vec<&Button> = state.buttons.iter().flatten().collect();
    let button = buttons.iter().find(|button| match button {
        Button::Toggle(toggle) => toggle.id == id,
        Button::Timer(timer) => timer.id == id,
    });
    let button = match button {
        Some(button) => button,
        None => return,
    };

    let mut running_threads = state.running_threads.lock().unwrap();
    let is_running = match running_threads.get(&id) {
        Some(signal) => *signal.lock().unwrap(),
        None => false,
    };

    match button {
        Button::Toggle(toggle) => {
            if action {
                if is_running {
                    println!("Task \"{}\" already running", id);
                    return;
                }
                let thread_status = Arc::new(Mutex::new(true));

                (toggle.task)(thread_status.clone());

                running_threads.insert(id, thread_status);
            } else {
                if let Some(signal) = running_threads.get(&id) {
                    let mut signal = signal.lock().unwrap();
                    *signal = false;
                }
            }
        }
        Button::Timer(timer) => {
            if action {
                if is_running {
                    println!("Task \"{}\" already running", id);
                    return;
                }

                (timer.task)(timer.delay);
            }
        }
    }
}

fn main() {
    let mut buttons = Vec::new();
    buttons.push(vec![
        Button::Toggle(ToggleButton {
            id: "casino-fingerprint".to_string(),
            task: casino::handler,
            enabled_text: "Disable Fingerprints (Casino)".to_string(),
            disabled_text: "Enable Fingerprints (Casino)".to_string(),
        }),
        Button::Toggle(ToggleButton {
            id: "cayo-fingerprint".to_string(),
            task: |_status| {
                // long running task
            },
            enabled_text: "Disable Fingerprints (Cayo)".to_string(),
            disabled_text: "Enable Fingerprints (Cayo)".to_string(),
        }),
    ]);

    buttons.push(vec![Button::Toggle(ToggleButton {
        id: "casino-capture".to_string(),
        task: casino_capture::handler,
        enabled_text: "Disable Casino Capture".to_string(),
        disabled_text: "Enable Casino Capture".to_string(),
    })]);

    let state: AppState = AppState {
        running_threads: Mutex::new(HashMap::new()),
        buttons,
    };
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![get_buttons, handle_button])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    let main_monitor = utils::get_main_monitor();
    let main_monitor = match main_monitor {
        Ok(main_monitor) => main_monitor,
        Err(err) => {
            utils::err_dialog(app.app_handle(), &err);
            return;
        }
    };
    let height = main_monitor.height();
    if !SUPPORTED_HEIGHTS.contains(&height) {
        utils::err_dialog(app.app_handle(), "Unsupported resolution");
        return;
    }

    app.run(|_, _| {});
}
