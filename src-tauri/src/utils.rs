use std::thread;
use std::time::Duration;

use image::imageops;
use image::DynamicImage;
use rustdct::DctPlanner;
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};
use winapi::ctypes::c_int;
use winapi::um::winuser::{
    INPUT_u, MapVirtualKeyA, SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_EXTENDEDKEY,
    KEYEVENTF_KEYUP, MAPVK_VK_TO_VSC, VK_DELETE, VK_DOWN, VK_END, VK_HOME, VK_INSERT, VK_LEFT,
    VK_NEXT, VK_PAUSE, VK_PRIOR, VK_RCONTROL, VK_RIGHT, VK_UP,
};
use xcap::Monitor;

use crate::ThreadStatus;

pub type Region = [u32; 4];

pub fn check_thread_status(thread_status: &ThreadStatus) -> bool {
    let signal = thread_status.lock().unwrap();
    *signal
}

pub fn get_main_monitor() -> Result<Monitor, String> {
    let monitors = Monitor::all();
    let monitors = match monitors {
        Ok(monitors) => monitors,
        Err(err) => {
            return Err(err.to_string());
        }
    };
    let main_monitor = monitors.into_iter().find(|monitor| monitor.is_primary());
    let main_monitor = match main_monitor {
        Some(monitor) => Ok(monitor),
        None => {
            return Err("No primary monitor found".to_string());
        }
    };
    main_monitor
}

pub fn err_dialog(app: &tauri::AppHandle, message: &str) {
    app.dialog()
        .message(message)
        .kind(MessageDialogKind::Error)
        .blocking_show();
}

pub fn capture_region(monitor: &Monitor, region: &[u32; 4]) -> image::DynamicImage {
    let image = monitor.capture_image();
    let mut image = match image {
        Ok(image) => image,
        Err(err) => {
            panic!("Failed to capture image: {}", err);
        }
    };
    let image = imageops::crop(
        &mut image,
        region[0],
        region[1],
        region[2] - region[0],
        region[3] - region[1],
    );
    DynamicImage::ImageRgba8(image.to_image())
}

pub fn capture_regions(monitor: &Monitor, regions: &[Region]) -> Vec<image::DynamicImage> {
    let image = monitor.capture_image();
    let mut image = match image {
        Ok(image) => image,
        Err(err) => {
            panic!("Failed to capture image: {}", err);
        }
    };
    regions
        .iter()
        .map(|region| {
            let image = imageops::crop(
                &mut image,
                region[0],
                region[1],
                region[2] - region[0],
                region[3] - region[1],
            );
            DynamicImage::ImageRgba8(image.to_image())
        })
        .collect()
}

pub fn hash_image(img: &image::GrayImage) -> Vec<u8> {
    let size: usize = 32;
    let small: usize = 8;
    let resized_img = imageops::resize(
        img,
        size as u32,
        size as u32,
        imageops::FilterType::Gaussian,
    );
    let resized_img = resized_img.pixels().map(|p| p[0]).collect::<Vec<_>>();
    // dct
    let mut planner = DctPlanner::new();
    let dct = planner.plan_dct2(size * size);
    let mut dct_data = vec![0f64; size * size];
    for i in 0..size {
        for j in 0..size {
            dct_data[i * size + j] = resized_img[(i * size + j) as usize] as f64;
        }
    }
    dct.process_dct2(&mut dct_data);

    // dct-data is a 32x32 in line matrix
    // get only the 8x8 top-left corner
    let mut total = 0f64;
    // for i in 0..small {
    //     for j in 0..small {
    //         total += dct_data[i * size + j];
    //     }
    // }
    for i in 0..small * small {
        total += dct_data[i];
    }
    let avg = total / (small * small) as f64;
    let mut hash: Vec<u8> = Vec::with_capacity(8 * 8);
    for i in 0..small {
        for j in 0..small {
            hash.push(if dct_data[i * size + j] > avg { 1 } else { 0 });
        }
    }
    hash
}

pub fn compare_image(img1: &image::GrayImage, img2: &image::GrayImage) -> f64 {
    let (hash1, hash2) = thread::scope(|scope| {
        let hash1_thread = scope.spawn(|| hash_image(img1));
        let hash2_thread = scope.spawn(|| hash_image(img2));
        let hash1 = hash1_thread.join().unwrap();
        let hash2 = hash2_thread.join().unwrap();
        (hash1, hash2)
    });

    let mut count = 0;
    for (h1, h2) in hash1.iter().zip(hash2.iter()) {
        if h1 == h2 {
            count += 1;
        }
    }
    count as f64 / hash1.len() as f64
}

// pub fn compare_image(img1: &image::GrayImage, img2: &image::GrayImage) -> f64 {
//     match image_compare::gray_similarity_structure(
//         &image_compare::Algorithm::RootMeanSquared,
//         img1,
//         img2,
//     ) {
//         Ok(similarity) => similarity.score,
//         Err(_) => 0f64,
//     }
// }
// pub fn compare_image(img1: &image::GrayImage, img2: &image::GrayImage) -> f64 {
//     let pixel1: Vec<_> = img1.pixels().collect();
//     let pixel2: Vec<_> = img2.pixels().collect();
//     if pixel1.len() != pixel2.len() {
//         return 0f64;
//     }
//     let mut count: u32 = 0;
//     for (p1, p2) in pixel1.iter().zip(pixel2.iter()) {
//         if p1[0] == p2[0] {
//             count += 1;
//         }
//     }
//     count as f64 / pixel1.len() as f64
// }

pub fn find_image_in_array(target: &image::GrayImage, images: &[image::GrayImage]) -> usize {
    let mut best_index = 0;
    let mut best_score = 0f64;
    for (index, image) in images.iter().enumerate() {
        let score = compare_image(target, image);
        println!(
            "Index : {}, Score: {}, Best Score : {}, Best Index : {}",
            index, score, best_score, best_index
        );
        if score > best_score {
            best_score = score;
            best_index = index;
        }
        // if score == 1f64 {
        //     break;
        // }
    }
    println!();
    best_index
}

pub fn press(vk_code: i32) {
    const PRESS_DURATION: Duration = Duration::from_millis(30);
    fn send(vk_code: i32, down: bool) {
        let extended_keys = vec![
            VK_RCONTROL,
            VK_PAUSE,
            VK_HOME,
            VK_PRIOR,
            VK_UP,
            VK_LEFT,
            VK_DOWN,
            VK_RIGHT,
            VK_NEXT,
            VK_END,
            VK_INSERT,
            VK_DELETE,
        ];
        let mut flags = if down { 0 } else { KEYEVENTF_KEYUP };
        if extended_keys.contains(&vk_code) {
            flags |= KEYEVENTF_EXTENDEDKEY;
        }

        let scan = unsafe { MapVirtualKeyA((vk_code & 0xff) as u32, MAPVK_VK_TO_VSC) };
        let mut union: INPUT_u = unsafe { std::mem::zeroed() };
        let inner_union = unsafe { union.ki_mut() };

        *inner_union = KEYBDINPUT {
            wScan: scan as u16,
            dwFlags: flags,
            dwExtraInfo: 0,
            time: 0,
            wVk: 0,
        };

        let mut input = [INPUT {
            type_: INPUT_KEYBOARD,
            u: union,
        }; 1];

        unsafe {
            SendInput(1, input.as_mut_ptr(), size_of::<INPUT>() as c_int);
        }
    }
    send(vk_code, true);
    thread::sleep(PRESS_DURATION);
    send(vk_code, false);
    thread::sleep(PRESS_DURATION);
}

pub fn relative_array(arr: &[usize]) -> Vec<usize> {
    let mut result = Vec::new();
    let mut last = 0;
    for i in arr {
        result.push(i - last);
        last = *i;
    }
    result
}
