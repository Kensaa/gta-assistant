use enigo::Enigo;
use enigo::Keyboard;
use enigo::Settings;
use image::imageops;
use image::DynamicImage;
use image_compare;
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};
use xcap::Monitor;

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

pub fn capture_region(monitor: &Monitor, region: [u32; 4]) -> Result<image::DynamicImage, String> {
    let image = monitor.capture_image();
    let mut image = match image {
        Ok(image) => image,
        Err(err) => {
            return Err(err.to_string());
        }
    };
    let image = imageops::crop(
        &mut image,
        region[0],
        region[1],
        region[2] - region[0],
        region[3] - region[1],
    );
    Ok(DynamicImage::ImageRgba8(image.to_image()))
}

pub fn compare_image(img1: &image::GrayImage, img2: &image::GrayImage) -> Result<f64, String> {
    match image_compare::gray_similarity_structure(
        &image_compare::Algorithm::MSSIMSimple,
        img1,
        img2,
    ) {
        Ok(similarity) => Ok(similarity.score),
        Err(err) => Err(err.to_string()),
    }
}

pub fn find_image_in_array(
    target: &image::GrayImage,
    images: &[image::GrayImage],
) -> Result<usize, String> {
    let mut best_index = 0;
    let mut best_score = 0.0;
    for (index, image) in images.iter().enumerate() {
        let score = compare_image(target, image)?;
        if score < best_score {
            best_score = score;
            best_index = index;
        }
        if score < 0.1 {
            break;
        }
    }
    Ok(best_index)
}

pub fn press(key: enigo::Key) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo
        .key(key, enigo::Direction::Press)
        .expect("Failed to press key");
    enigo
        .key(key, enigo::Direction::Release)
        .expect("Failed to release key");
}
