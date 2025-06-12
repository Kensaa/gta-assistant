use std::{
    fs::{self},
    path::Path,
    thread,
    time::Duration,
};

use gta_assistant::{constants, utils, ThreadStatus};
use tauri::AppHandle;
use winapi::um::winuser::{VK_DOWN, VK_RIGHT, VK_UP};

pub fn handler(thread_status: ThreadStatus, app_handle: AppHandle) {
    thread::spawn(move || {
        let height = utils::get_main_monitor().unwrap().height();
        if !crate::cayo::SUPPORTED_HEIGHTS.contains(&height) {
            utils::err_dialog(&app_handle, "Cayo Capture does not support your resolution");

            return;
        }

        let header_pos = constants::CAYO_HEADER_POS.get(&height).unwrap();
        let fingerprint_pos = constants::CAYO_FINGERPRINT_POS.get(&height).unwrap();
        let parts_pos = constants::CAYO_PARTS_POS.get(&height).unwrap();
        let output_folder = Path::new("../output").join(height.to_string()).join("cayo");

        if !output_folder.exists() {
            fs::create_dir_all(&output_folder).expect("failed to create output folder");
        }
        let monitor = utils::get_main_monitor().unwrap();
        let mut curr_index = 0;
        thread::sleep(Duration::from_millis(5000));
        loop {
            if !utils::check_thread_status(&thread_status) {
                break;
            };

            let fingerprint_screenshot =
                utils::capture_region(&monitor, fingerprint_pos).into_rgb8();

            // try to find if we already saved that one
            let mut found = false;
            for file in fs::read_dir(&output_folder).unwrap().map(|f| f.unwrap()) {
                if file.file_type().unwrap().is_dir() {
                    let fingerprint_path = file.path().join("fingerprint.png");
                    if fingerprint_path.exists() {
                        let prev_fingerprint_screenshot = utils::load_image(fingerprint_path);
                        if utils::compare_image(
                            &fingerprint_screenshot,
                            &prev_fingerprint_screenshot,
                        ) > 0.99
                        {
                            // already captured this one
                            found = true;
                            break;
                        }
                    }
                }
            }
            if found {
                continue;
            }

            let curr_path = output_folder.join(curr_index.to_string());
            if !curr_path.exists() {
                fs::create_dir(&curr_path).expect("failed to create folder");
            }
            let header_screenshot = utils::capture_region(&monitor, header_pos).into_rgb8();
            header_screenshot
                .save(curr_path.join("header.png"))
                .expect("failed to screenshot header");

            fingerprint_screenshot
                .save(curr_path.join("fingerprint.png"))
                .expect("failed to screenshot fingerprint");

            // save parts

            for i in 1..=8 {
                let pos = parts_pos.get(0).unwrap();
                utils::press(VK_DOWN);
                let part_screen = utils::capture_region(&monitor, pos);
                part_screen
                    .save(curr_path.join(i.to_string() + ".png"))
                    .expect("failed to save part");
                utils::press(VK_UP);
                utils::press(VK_RIGHT);
            }
            curr_index += 1;
            thread::sleep(Duration::from_millis(5000));
        }
    });
}
