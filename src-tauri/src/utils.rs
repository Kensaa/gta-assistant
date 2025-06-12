use image::{imageops, DynamicImage, ImageReader, RgbImage};
use image_hasher::HasherConfig;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};
use winapi::ctypes::c_int;
use winapi::um::winuser::{
    INPUT_u, MapVirtualKeyA, SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_EXTENDEDKEY,
    KEYEVENTF_KEYUP, MAPVK_VK_TO_VSC, VK_DELETE, VK_DOWN, VK_END, VK_HOME, VK_INSERT, VK_LEFT,
    VK_NEXT, VK_PAUSE, VK_PRIOR, VK_RCONTROL, VK_RIGHT, VK_UP,
};
use xcap::Monitor;

pub type ThreadStatus = Arc<Mutex<bool>>;
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

pub fn compare_image(img1: &RgbImage, img2: &RgbImage) -> f64 {
    let hasher = HasherConfig::new().to_hasher();
    let img1 = DynamicImage::ImageRgb8(img1.clone());
    let img2 = DynamicImage::ImageRgb8(img2.clone());
    let hash1 = hasher.hash_image(&img1);
    let hash2 = hasher.hash_image(&img2);

    let distance = hash1.dist(&hash2);
    let similarity = 1.0 - (distance as f64 / (hash1.as_bytes().len() * 8) as f64);
    similarity
}

pub fn find_image_in_array(target: &RgbImage, images: &[RgbImage]) -> usize {
    let mut best_index = 0;
    let mut best_score = 0f64;
    for (index, image) in images.iter().enumerate() {
        let score = compare_image(target, image);
        if score > best_score {
            best_score = score;
            best_index = index;
        }
        if score == 1f64 {
            break;
        }
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

pub fn open_image(path: PathBuf) -> RgbImage {
    return ImageReader::open(path)
        .expect("failed to open image")
        .decode()
        .expect("failed to decode image")
        .to_rgb8();
}
