use gta_assistant::{
    constants,
    utils::{self, TaskData, TaskResult},
};
use image::RgbImage;
use log::{debug, error, info};
use std::{path::Path, thread};
use windows::Win32::UI::Input::KeyboardAndMouse::{VK_DOWN, VK_RETURN, VK_RIGHT, VK_TAB};

pub fn handler(task_data: TaskData) -> TaskResult {
    thread::Builder::new()
        .name("casino solver".to_string())
        .spawn(move || {
            info!("Thread started");
            // INITIALIZATION
            let resolution = utils::get_resolution();
            if !crate::casino::SUPPORTED_RESOLUTIONS.contains(&resolution) {
                let err = "Casino Fingerprints does not support your resolution";
                error!("{}", err);
                panic!("{}", err);
            }

            let header_pos = constants::CASINO_HEADER_POS.get(&resolution).unwrap();
            let fingerprint_pos = constants::CASINO_FINGERPRINT_POS.get(&resolution).unwrap();
            let parts_pos = constants::CASINO_PARTS_POS.get(&resolution).unwrap();
            let asset_folder = Path::new("assets")
                .join(resolution.1.to_string())
                .join("casino");

            let header_image: RgbImage = utils::load_image(asset_folder.join("header.png"));
            info!("Header image loaded");
            let fingerprints: Vec<RgbImage> = (1..=*constants::CASINO_FINGERPRINT_COUNT)
                .map(|i| utils::load_image(asset_folder.join(i.to_string()).join("full.png")))
                .collect();
            info!("Fingerprints image loaded");

            let parts: Vec<Vec<RgbImage>> = (1..=*constants::CASINO_FINGERPRINT_COUNT)
                .map(|fingerprint| {
                    (1..=4)
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

                    info!("Fingerprint index: {}", fingerprint_index + 1);

                    let solutions = parts
                        .get(fingerprint_index)
                        .expect("Invalid fingerprint index");

                    let parts_screenshots: Vec<RgbImage> =
                        utils::capture_regions(&monitor, &parts_pos)
                            .into_iter()
                            .map(|img| img.into_rgb8())
                            .collect();

                    // Potential improvement here
                    let mut pos_to_check: Vec<usize> = Vec::with_capacity(4);
                    for solution in solutions {
                        let index = utils::find_image_in_array(solution, &parts_screenshots);
                        pos_to_check.push(index);
                    }

                    pos_to_check.sort();
                    debug!("pos to check: {:?}", pos_to_check);
                    let pos_to_check = utils::relative_array(&pos_to_check);
                    for move_count in pos_to_check {
                        let right_moves = move_count % 2;
                        let down_moves = move_count / 2;
                        utils::multiple_press(VK_RIGHT, right_moves);
                        utils::multiple_press(VK_DOWN, down_moves);
                        utils::press(VK_RETURN);
                    }
                    utils::press(VK_TAB);
                    info!("Validating fingerprint...");
                    thread::sleep(*constants::CASINO_WAIT_DELAY - *constants::LOOP_DELAY);
                }

                thread::sleep(*constants::LOOP_DELAY);
            }
            info!("Stopping thread");
        })
        .unwrap()
}
