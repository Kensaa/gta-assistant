use crate::{constants, utils, ThreadStatus};
use std::{
    fs::{self},
    path::Path,
    thread,
    time::Duration,
};

pub fn handler(thread_status: ThreadStatus) {
    thread::spawn(move || {
        let height = utils::get_main_monitor().unwrap().height();
        let header_pos = constants::CASINO_HEADER_POS.get(&height).unwrap();
        let fingerprint_pos = constants::CASINO_FINGERPRINT_POS.get(&height).unwrap();
        let parts_pos = constants::CASINO_PARTS_POS.get(&height).unwrap();
        let output_folder = Path::new("../output")
            .join(height.to_string())
            .join("casino");

        if !output_folder.exists() {
            fs::create_dir_all(&output_folder).expect("failed to create output folder");
        }
        let monitor = utils::get_main_monitor().unwrap();
        let mut curr_index = 0;
        loop {
            if !utils::check_thread_status(&thread_status) {
                break;
            };
            let curr_path = output_folder.join(curr_index.to_string());
            if !curr_path.exists() {
                fs::create_dir(&curr_path).expect("failed to create folder");
            }
            let header_screenshot = utils::capture_region(&monitor, header_pos).into_rgb8();
            header_screenshot
                .save(output_folder.join("header.png"))
                .expect("failed to screenshot header");

            let fingerprint_screenshot =
                utils::capture_region(&monitor, fingerprint_pos).into_rgb8();
            fingerprint_screenshot
                .save(curr_path.join("full.png"))
                .expect("failed to screenshot fingerprint");

            utils::capture_regions(&monitor, &parts_pos)
                .into_iter()
                .map(|img| img.into_rgb8())
                .enumerate()
                .for_each(|(i, img)| {
                    img.save(curr_path.join(i.to_string() + ".png"))
                        .expect("failed to write part screenshot");
                });

            curr_index += 1;
            thread::sleep(Duration::from_millis(10000));
        }
    });
}
