use gta_assistant::utils::{self, TaskData, TaskResult};
use log::info;
use std::{thread, time::Duration};
use windows::Win32::UI::Input::KeyboardAndMouse::{VK_S, VK_Z};

pub fn handler(task_data: TaskData) -> TaskResult {
    thread::Builder::new()
        .name("no afk".to_string())
        .spawn(move || {
            info!("Thread started");

            let sleep_duration = Duration::from_secs(10);
            loop {
                if !utils::check_thread_status(&task_data.thread_status) {
                    break;
                }

                utils::press(VK_Z);
                thread::sleep(sleep_duration);
                utils::press(VK_S);
                thread::sleep(sleep_duration);
            }
            info!("Stopping thread");
        })
        .unwrap()
}
