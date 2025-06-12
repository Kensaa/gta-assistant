use gta_assistant::{constants, utils, ThreadStatus};
use image::RgbImage;
use std::{path::Path, thread, time::Instant};
use winapi::um::winuser::{VK_DOWN, VK_LEFT, VK_RIGHT};

pub fn handler(thread_status: ThreadStatus) {
    thread::spawn(move || {
        let height = utils::get_main_monitor().unwrap().height();
        let asset_folder = Path::new("../assets").join(height.to_string()).join("cayo");

        let header_pos = constants::CAYO_HEADER_POS.get(&height).unwrap();
        let fingerprint_pos = constants::CAYO_FINGERPRINT_POS.get(&height).unwrap();
        let parts_pos = constants::CAYO_PARTS_POS.get(&height).unwrap();

        let header_image = utils::open_image(asset_folder.join("header.png"));

        let fingerprints: Vec<RgbImage> = (1..=*constants::CAYO_FINGERPRINT_COUNT)
            .map(|i| utils::open_image(asset_folder.join(i.to_string()).join("fingerprint.png")))
            .collect();

        let parts: Vec<Vec<RgbImage>> = (1..=*constants::CAYO_FINGERPRINT_COUNT)
            .map(|fingerprint| {
                (1..=8)
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
                let before_screens = Instant::now();
                let fingerprint_screenshot =
                    utils::capture_region(&monitor, fingerprint_pos).into_rgb8();

                let time1 = Instant::now();
                let fingerprint_index =
                    utils::find_image_in_array(&fingerprint_screenshot, &fingerprints);
                println!(
                    "fingerprint detect time : {}ms",
                    time1.elapsed().as_millis()
                );
                let curr_parts = parts.get(fingerprint_index).unwrap();
                let time2 = Instant::now();
                let parts_screenshots: Vec<RgbImage> = utils::capture_regions(&monitor, &parts_pos)
                    .into_iter()
                    .map(|img| img.into_rgb8())
                    .collect();

                println!("screen time : {}ms", before_screens.elapsed().as_millis());
                println!("only parts screen time : {}ms", time2.elapsed().as_millis());
                for i in 0..8 {
                    let part_screen = parts_screenshots.get(i).unwrap();
                    let part_index = utils::find_image_in_array(part_screen, &curr_parts);
                    println!(
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
    });
}

fn move_to(current: usize, target: usize) {
    if current == target {
        return;
    }

    if target > current {
        if target - current > 4 {
            let move_count = 8 - target + current;
            multiple_press(VK_LEFT, move_count);
        } else {
            let move_count = target - current;
            multiple_press(VK_RIGHT, move_count);
        }
    } else {
        if current - target > 4 {
            let move_count = 8 - current + target;
            multiple_press(VK_RIGHT, move_count);
        } else {
            let move_count = current - target;
            multiple_press(VK_LEFT, move_count);
        }
    }
}

fn multiple_press(key: i32, count: usize) {
    for _ in 0..count {
        utils::press(key);
        // MAYBE ADD DELAY
    }
}
