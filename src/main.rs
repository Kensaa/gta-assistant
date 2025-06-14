#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod casino;
mod cayo;
mod misc;

use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::{fs, thread};

use directories::ProjectDirs;
use fltk::dialog;
use fltk::image::PngImage;
use fltk::prelude::ButtonExt;
use fltk::{
    app,
    button::ToggleButton,
    enums::{Color, FrameType},
    group::Flex,
    prelude::{GroupExt, WidgetExt, WindowExt},
    window::Window,
};
use gta_assistant::utils::{self, TaskData};
use gta_assistant::{
    ThreadStatus,
    utils::{Button, ButtonType},
};
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::filter::threshold::ThresholdFilter;

#[cfg(debug_assertions)]
const MIN_LOG_LEVEL: LevelFilter = LevelFilter::Debug;
#[cfg(not(debug_assertions))]
const MIN_LOG_LEVEL: LevelFilter = LevelFilter::Info;

struct AppState {
    running_threads: Mutex<HashMap<Button, ThreadStatus>>,
}
fn main() {
    // INIT LOGGER
    let log_folder = if cfg!(debug_assertions) {
        // Debug
        PathBuf::from("logs/")
    } else {
        // Release
        let proj_dirs = ProjectDirs::from("fr", "kensa", "gta-assistant").unwrap();
        let dir = proj_dirs.data_local_dir();
        dir.to_path_buf()
    };
    fs::create_dir_all(&log_folder).expect("Failed to create log directory");
    let log_file = log_folder.join("gta-assistant.log");
    if fs::exists(&log_file).unwrap() {
        fs::remove_file(&log_file).unwrap();
    }
    let log_format_pattern = Box::new(PatternEncoder::new(
        "[{d(%Y-%m-%d %H:%M:%S)}][{M}][{T}][{h({l})}] {m}{n}",
    ));
    let stdout = Box::new(
        ConsoleAppender::builder()
            .encoder(log_format_pattern.clone())
            .build(),
    );

    let file_appender = Box::new(
        FileAppender::builder()
            .encoder(log_format_pattern)
            .build(log_file)
            .unwrap(),
    );
    let logger_config = Config::builder()
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(MIN_LOG_LEVEL)))
                .build("stdout", stdout),
        )
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(MIN_LOG_LEVEL)))
                .build("file", file_appender),
        )
        .build(
            Root::builder()
                .appender("stdout")
                .appender("file")
                .build(MIN_LOG_LEVEL),
        )
        .unwrap();
    log4rs::init_config(logger_config).unwrap();
    // APP INIT

    let capture_enabled = std::env::var("ASSISTANT_CAPTURE").unwrap_or("0".to_string()) != "0"
        || cfg!(debug_assertions);
    let mut buttons = vec![
        vec![
            Button {
                disabled_text: "Enable Fingerprints (Casino)",
                enabled_text: "Disable Fingerprints (Casino)",
                task: casino::casino::handler,
                btn_type: ButtonType::Toggle,
            },
            Button {
                disabled_text: "Enable Fingerprints (Cayo)",
                enabled_text: "Disable Fingerprints (Cayo)",
                task: cayo::cayo::handler,
                btn_type: ButtonType::Toggle,
            },
        ],
        vec![Button {
            disabled_text: "Enable No AFK",
            enabled_text: "Disable No AFK",
            task: misc::no_afk::handler,
            btn_type: ButtonType::Toggle,
        }],
    ];
    if capture_enabled {
        buttons.push(vec![
            Button {
                disabled_text: "Enable Casino Capture",
                enabled_text: "Disable Casino Capture",
                task: casino::casino_capture::handler,
                btn_type: ButtonType::Toggle,
            },
            Button {
                disabled_text: "Enable Cayo Capture",
                enabled_text: "Disable Cayo Capture",
                task: cayo::cayo_capture::handler,
                btn_type: ButtonType::Toggle,
            },
        ]);
    }
    let app_state = Rc::new(AppState {
        running_threads: Mutex::new(HashMap::new()),
    });

    let app = app::App::default();

    let (s_err, r_err) = app::channel::<&str>();
    app::add_idle3(move |_| {
        if let Some(msg) = r_err.recv() {
            dialog::alert(0, 0, msg);
        }
    });

    let mut window = Window::default()
        .with_size(500, 200)
        .center_screen()
        .with_label("GTA Assistant");

    let image = PngImage::from_data(&utils::Asset::get("assets/icon.png").unwrap().data).unwrap();
    window.set_icon(Some(image));
    let mut col = Flex::default()
        .with_size(window.w(), window.h())
        .center_of_parent()
        .column();
    col.set_margins(10, 5, 10, 5);

    for row in buttons.iter() {
        let flex_row = Flex::default();

        for button_data in row {
            let mut toggle_button = create_button();
            toggle_button.set_label(button_data.disabled_text);

            let button_data = (*button_data).clone();
            let app_state_ref = app_state.clone();
            toggle_button.set_callback(move |toggle_button| {
                let value = toggle_button.value();
                change_button_value(toggle_button, &button_data, value);

                let mut running_threads = app_state_ref.running_threads.lock().unwrap();
                let is_running = match running_threads.get(&button_data) {
                    Some(signal) => *signal.lock().unwrap(),
                    None => false,
                };
                if value {
                    if is_running {
                        // already running
                        return;
                    }
                    let new_thread_status = Arc::new(Mutex::new(true));
                    running_threads.insert(button_data.clone(), new_thread_status.clone());
                    // spawn task from a thread that awaits for the end of the task to update ui
                    let button_data = button_data.clone();
                    let mut toggle_button = toggle_button.clone();
                    thread::spawn(move || {
                        let task_data = TaskData {
                            button: button_data.clone(),
                            thread_status: new_thread_status.clone(),
                        };
                        let handle = (button_data.task)(task_data);

                        let task_result = handle.join();
                        match task_result {
                            Ok(()) => {}
                            Err(err) => {
                                let error_str = err.downcast::<&str>().unwrap();
                                s_err.send(&error_str);
                            }
                        }
                        toggle_button.set_value(false);
                        if let Ok(mut guard) = new_thread_status.lock() {
                            *guard = false;
                        }
                        change_button_value(&mut toggle_button, &button_data, false);
                    });
                } else {
                    if is_running {
                        if let Some(thread_status) = running_threads.get(&button_data) {
                            if let Ok(mut signal) = thread_status.lock() {
                                *signal = false;
                            }
                        }
                    }
                }
            });
        }

        flex_row.end();
    }

    col.end();
    window.show();

    app.run().unwrap();
}

fn create_button() -> ToggleButton {
    let mut button = ToggleButton::default();
    button.set_color(Color::from_hex(0x0d6efd));
    button.set_selection_color(Color::from_hex(0x198754));
    button.set_label_color(Color::White);
    button.set_frame(FrameType::FlatBox);

    return button;
}

fn change_button_value(button: &mut ToggleButton, button_data: &Button, value: bool) {
    button.set_value(value);
    let new_label = if value {
        button_data.enabled_text
    } else {
        button_data.disabled_text
    };

    button.set_label(new_label);
}
