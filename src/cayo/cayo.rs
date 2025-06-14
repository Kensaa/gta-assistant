use gta_assistant::{
    constants,
    utils::{self, TaskData, TaskResult},
};
use image::RgbImage;
use log::{debug, error, info};
use std::{path::Path, thread};
use windows::Win32::UI::Input::KeyboardAndMouse::{VK_DOWN, VK_LEFT, VK_RIGHT};

pub fn handler(task_data: TaskData) -> TaskResult {
    thread::Builder::new()
        .name("cayo solver".to_string())
        .spawn(move || {
            info!("Thread started");
            let resolution = utils::get_resolution();
            if !crate::cayo::SUPPORTED_RESOLUTIONS.contains(&resolution) {
                let err = "Cayo Fingerprints does not support your resolution";
                error!("{}", err);
                panic!("{}", err);
            }

            let asset_folder = Path::new("assets")
                .join(resolution.1.to_string())
                .join("cayo");

            let header_pos = constants::CAYO_HEADER_POS.get(&resolution).unwrap();
            let fingerprint_pos = constants::CAYO_FINGERPRINT_POS.get(&resolution).unwrap();
            let parts_pos = constants::CAYO_PARTS_POS.get(&resolution).unwrap();

            let header_image = utils::load_image(asset_folder.join("header.png"));
            info!("Header image loaded");

            let fingerprints: Vec<RgbImage> = (1..=*constants::CAYO_FINGERPRINT_COUNT)
                .map(|i| {
                    utils::load_image(asset_folder.join(i.to_string()).join("fingerprint.png"))
                })
                .collect();
            info!("Fingerprints image loaded");

            let parts: Vec<Vec<RgbImage>> = (1..=*constants::CAYO_FINGERPRINT_COUNT)
                .map(|fingerprint| {
                    (1..=8)
                        .map(|part| {
                            utils::load_image(
                                asset_folder
                                    .join(fingerprint.to_string())
                                    .join(format!("{}.png", part)),
                            )
                        })
                        .collect()
                })
                .collect();
            info!("fingerprints parts images loaded");

            let monitor = utils::get_main_monitor().unwrap();
            loop {
                if !utils::check_thread_status(&task_data.thread_status) {
                    break;
                };

                let header_screenshot = utils::capture_region(&monitor, header_pos).into_rgb8();
                let similarity = utils::compare_image(&header_image, &header_screenshot);
                debug!("similarity {}", similarity);
                if similarity > 0.99 {
                    info!("Fingerprint detected ({} header similarity)", similarity);
                    let fingerprint_screenshot =
                        utils::capture_region(&monitor, fingerprint_pos).into_rgb8();

                    let fingerprint_index =
                        utils::find_image_in_array(&fingerprint_screenshot, &fingerprints);
                    info!("Fingerprint index: {}", fingerprint_index);
                    let curr_parts = parts.get(fingerprint_index).unwrap();
                    let parts_screenshots: Vec<RgbImage> =
                        utils::capture_regions(&monitor, &parts_pos)
                            .into_iter()
                            .map(|img| img.into_rgb8())
                            .collect();

                    for i in 0..8 {
                        let part_screen = parts_screenshots.get(i).unwrap();
                        let part_index = utils::find_image_in_array(part_screen, &curr_parts);
                        debug!(
                            "part n°{} : current index: {}, target index : {}",
                            i, part_index, i
                        );

                        move_to(part_index, i);
                        utils::press(VK_DOWN);
                    }
                    thread::sleep(*constants::CAYO_WAIT_DELAY - *constants::LOOP_DELAY);
                }
                thread::sleep(*constants::LOOP_DELAY);
            }
            info!("Stopping thread");
        })
        .unwrap()
}

fn move_to(current: usize, target: usize) {
    if current == target {
        return;
    }

    if target > current {
        if target - current > 4 {
            let move_count = 8 - target + current;
            utils::multiple_press(VK_LEFT, move_count);
        } else {
            let move_count = target - current;
            utils::multiple_press(VK_RIGHT, move_count);
        }
    } else {
        if current - target > 4 {
            let move_count = 8 - current + target;
            utils::multiple_press(VK_RIGHT, move_count);
        } else {
            let move_count = current - target;
            utils::multiple_press(VK_LEFT, move_count);
        }
    }
}
