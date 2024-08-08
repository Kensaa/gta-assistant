// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

type ThreadStatus = Arc<Mutex<bool>>;
// A task creating a long running thread that can be stopped by setting the value of the ThreadStatus to false
type LongTask = fn(ThreadStatus);
// A task that runs for a fixed amount of time
type Task = fn();

// A button that can be toggled on and off, starting and stopping a long running thread
// ex : fingerprint solver

#[derive(Serialize, Clone)]
struct ToggleButton {
    id: String,
    #[serde(skip_serializing)]
    task: LongTask,
    enabled_text: String,  // text to display when the button is enabled
    disabled_text: String, // text to display when the button is disabled
}

// A button that can be clicked, starting a thread for a fixed amount of time
#[derive(Serialize, Clone)]
struct TimerButton {
    id: String,
    #[serde(skip_serializing)]
    task: Task,
    delay: u64,
    default_text: String, // text to display when the timer is not running
    running_text: String, // text to display when the timer is running
}

#[derive(Clone, Serialize)]
enum Button {
    Toggle(ToggleButton),
    Timer(TimerButton),
}

// impl Serialize for ToggleButton {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         let mut state = serializer.serialize_struct("toggleButton", 4)?;
//         state.serialize_entry("type", "toggleButton")?;
//         state.serialize_entry("id", &self.id)?;
//         state.serialize_entry("enabled_text", &self.enabled_text)?;
//         state.serialize_entry("disabled_text", &self.disabled_text)?;

//         state.end()
//     }
// }

// impl Serialize for TimerButton {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         let mut state = serializer.serialize_newtype_struct(Some(5))?;
//         state.serialize_entry("type", "delayButton")?;
//         state.serialize_entry("id", &self.id)?;
//         state.serialize_entry("default_text", &self.default_text)?;
//         state.serialize_entry("running_text", &self.running_text)?;
//         state.serialize_entry("delay", &self.delay)?;
//         state.end()
//     }
// }

struct AppState {
    running_threads: Mutex<HashMap<String, ThreadStatus>>,
    buttons: Vec<Vec<Button>>,
}

#[tauri::command]
fn get_buttons(state: tauri::State<AppState>) -> Vec<Vec<Button>> {
    state.buttons.clone()
}

fn main() {
    let mut buttons = Vec::new();
    let row1 = vec![
        Button::Toggle(ToggleButton {
            id: "casino-fingerprint".to_string(),
            task: |_status| {
                // long running task
            },
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
    ];

    buttons.push(row1);

    let state: AppState = AppState {
        running_threads: Mutex::new(HashMap::new()),
        buttons,
    };
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![get_buttons])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
