use gta_assistant::{constants, utils, ThreadStatus};
use image::RgbImage;
use std::{path::Path, thread};
use winapi::um::winuser::{VK_RETURN, VK_RIGHT, VK_TAB};

pub fn handler(thread_status: ThreadStatus) {
    thread::spawn(move || {
        // INITIALIZATION
        let height = utils::get_main_monitor().unwrap().height();

        let header_pos = constants::CASINO_HEADER_POS.get(&height).unwrap();
        let fingerprint_pos = constants::CASINO_FINGERPRINT_POS.get(&height).unwrap();
        let parts_pos = constants::CASINO_PARTS_POS.get(&height).unwrap();
        let asset_folder = Path::new("../assets")
            .join(height.to_string())
            .join("casino");

        let header_image: RgbImage = utils::open_image(asset_folder.join("header.png"));

        let fingerprints: Vec<RgbImage> = (1..=*constants::CASINO_FINGERPRINT_COUNT)
            .map(|i| utils::open_image(asset_folder.join(i.to_string()).join("full.png")))
            .collect();

        let parts: Vec<Vec<RgbImage>> = (1..=*constants::CASINO_FINGERPRINT_COUNT)
            .map(|fingerprint| {
                (1..=4)
                    .map(|part| {
                        utils::open_image(
                            asset_folder
                                .join(fingerprint.to_string())
                                .join(format!("{}.png", part)),
                        )
                    })
                    .collect()
            })
            .collect();

        let monitor = utils::get_main_monitor().unwrap();
        loop {
            if !utils::check_thread_status(&thread_status) {
                break;
            };

            let header_screenshot = utils::capture_region(&monitor, header_pos).into_rgb8();
            let similarity = utils::compare_image(&header_image, &header_screenshot);
            if similarity > 0.99 {
                println!("Fingerprint detected ({} header similarity)", similarity);
                let fingerprint_screenshot =
                    utils::capture_region(&monitor, fingerprint_pos).into_rgb8();

                let fingerprint_index =
                    utils::find_image_in_array(&fingerprint_screenshot, &fingerprints);

                println!("Fingerprint detected: {}", fingerprint_index + 1);

                let solutions = parts
                    .get(fingerprint_index)
                    .expect("Invalid fingerprint index");

                let parts_screenshots: Vec<RgbImage> = utils::capture_regions(&monitor, &parts_pos)
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
                println!("pos to check: {:?}", pos_to_check);
                let pos_to_check = utils::relative_array(&pos_to_check);
                println!("relative pos to check: {:?}", pos_to_check);
                for move_count in pos_to_check {
                    for _ in 0..move_count {
                        utils::press(VK_RIGHT);
                    }
                    utils::press(VK_RETURN);
                }
                utils::press(VK_TAB);
                println!("Validating fingerprint...");
                thread::sleep(*constants::CASINO_WAIT_DELAY - *constants::LOOP_DELAY);
            }

            thread::sleep(*constants::LOOP_DELAY);
        }
    });
}
