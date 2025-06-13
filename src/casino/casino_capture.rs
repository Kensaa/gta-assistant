use gta_assistant::{
    constants,
    utils::{self, TaskData, TaskResult},
};
use std::{
    fs::{self},
    thread,
    time::Duration,
};

pub fn handler(task_data: TaskData) -> TaskResult {
    thread::Builder::new()
        .name("casino capture".to_string())
        .spawn(move || {
            let resolution = utils::get_resolution();
            if !crate::casino::SUPPORTED_RESOLUTIONS.contains(&resolution) {
                panic!("Casino Capture does not support your resolution");
            }

            let header_pos = constants::CASINO_HEADER_POS.get(&resolution).unwrap();
            let fingerprint_pos = constants::CASINO_FINGERPRINT_POS.get(&resolution).unwrap();
            let parts_pos = constants::CASINO_PARTS_POS.get(&resolution).unwrap();
            let output_folder = constants::OUTPUT_PATH
                .join(resolution.1.to_string())
                .join("casino");

            if !output_folder.exists() {
                fs::create_dir_all(&output_folder).expect("failed to create output folder");
            }
            let monitor = utils::get_main_monitor().unwrap();
            let mut curr_index = fs::read_dir(&output_folder)
                .unwrap()
                .map(|f| f.unwrap())
                .filter(|f| f.file_type().unwrap().is_dir())
                .map(|f| f.file_name().to_str().unwrap().to_string().parse::<usize>())
                .filter(|res| res.is_ok())
                .map(|res| res.unwrap())
                .max()
                .unwrap_or(0)
                + 1;

            thread::sleep(Duration::from_millis(5000));
            loop {
                if !utils::check_thread_status(&task_data.thread_status) {
                    break;
                };
                let fingerprint_screenshot =
                    utils::capture_region(&monitor, fingerprint_pos).into_rgb8();

                // try to find if we already saved that one
                let mut found = false;
                for file in fs::read_dir(&output_folder).unwrap().map(|f| f.unwrap()) {
                    if file.file_type().unwrap().is_dir() {
                        let fingerprint_path = file.path().join("full.png");
                        if fingerprint_path.exists() {
                            let prev_fingerprint_screenshot = utils::load_image(fingerprint_path);
                            let score = utils::compare_image(
                                &fingerprint_screenshot,
                                &prev_fingerprint_screenshot,
                            );
                            println!("n°{}, score: {}", file.file_name().to_str().unwrap(), score);
                            if score == 1f64 {
                                // already captured this one
                                found = true;
                                break;
                            }
                        }
                    }
                }
                if !found {
                    let curr_path = output_folder.join(curr_index.to_string());
                    if !curr_path.exists() {
                        fs::create_dir(&curr_path).expect("failed to create folder");
                    }
                    let header_screenshot = utils::capture_region(&monitor, header_pos).into_rgb8();
                    header_screenshot
                        .save(curr_path.join("header.png"))
                        .expect("failed to screenshot header");

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
                }
                thread::sleep(Duration::from_millis(5000));
            }
        })
        .unwrap()
}
