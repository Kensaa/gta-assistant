use crate::constants;
use image::{imageops, DynamicImage, ImageReader, RgbImage};
use image_hasher::{Hasher, HasherConfig, ImageHash};
use log::{error, info};
use rust_embed::Embed;
use std::path::{Component, PathBuf};
use std::sync::{Arc, Mutex};
use std::{panic, thread};
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
pub type Resolution = (u32, u32);

#[derive(Embed)]
#[folder = "../assets"]
#[prefix = "assets/"]
struct Asset;

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

pub fn get_resolution() -> Resolution {
    let monitor = get_main_monitor().unwrap();
    return (monitor.width(), monitor.height());
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

fn hash_image(hasher: &Hasher, img: &RgbImage) -> ImageHash {
    hasher.hash_image(img)
}

pub fn compare_image(img1: &RgbImage, img2: &RgbImage) -> f64 {
    let hasher = HasherConfig::new().to_hasher();
    let (hash1, hash2) = thread::scope(|scope| {
        let hash1_thread = scope.spawn(|| hash_image(&hasher, img1));
        let hash2_thread = scope.spawn(|| hash_image(&hasher, img2));
        let hash1 = hash1_thread.join().unwrap();
        let hash2 = hash2_thread.join().unwrap();
        (hash1, hash2)
    });

    let distance = hash1.dist(&hash2);
    let similarity = 1.0 - (distance as f64 / (hash1.as_bytes().len() * 8) as f64);
    similarity
}

pub fn find_image_in_array(target: &RgbImage, images: &[RgbImage]) -> usize {
    thread::scope(|scope| {
        let mut threads = Vec::with_capacity(images.len());
        for (index, image) in images.iter().enumerate() {
            threads.push(scope.spawn(move || {
                let score = compare_image(target, image);
                (index, score)
            }))
        }

        let mut best_score = 0f64;
        let mut best_index = 0;
        for thread in threads {
            let (index, score) = thread.join().unwrap();
            if score > best_score {
                best_score = score;
                best_index = index;
            }
            if score == 1f64 {
                return index;
            }
        }
        return best_index;
    })
}

pub fn press(vk_code: i32) {
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
    thread::sleep(*constants::PRESS_DURATION);
    send(vk_code, false);
    thread::sleep(*constants::PRESS_DURATION);
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

pub fn load_image(path: PathBuf) -> RgbImage {
    let mut components = path.components();
    info!("loading image : {}", path.display());
    match components.next() {
        Some(Component::Normal(s)) if s.to_str().unwrap() == "assets" => {
            let asset_path = path
                .components()
                .map(|c| c.as_os_str().to_str().unwrap())
                .collect::<Vec<&str>>()
                .join("/");
            let file = match Asset::get(&asset_path) {
                Some(file) => file,
                None => {
                    error!("failed to get asset");
                    panic!("failed to get asset");
                }
            };

            match image::load_from_memory(&file.data) {
                Ok(img) => img.to_rgb8(),
                Err(err) => {
                    error!("error while decoding image : {}", err);
                    panic!("error while decoding image");
                }
            }
        }
        _ => {
            let img = match ImageReader::open(path) {
                Ok(img) => img,
                Err(err) => {
                    error!("failed to open image : {}", err);
                    panic!("");
                }
            };
            match img.decode() {
                Ok(img) => img.to_rgb8(),
                Err(err) => {
                    error!("failed to decode image : {}", err);
                    panic!("");
                }
            }
        }
    }
}
