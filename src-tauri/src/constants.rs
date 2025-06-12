use crate::utils::{Region, Resolution};
use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    time::Duration,
};

pub const R1080: Resolution = (1920, 1080);
pub const R1440: Resolution = (2560, 1440);

lazy_static! {
    // CASINO
    pub static ref CASINO_FINGERPRINT_COUNT: u16 = 4;
    pub static ref CASINO_HEADER_POS: HashMap<Resolution, Region> = {
        let mut m = HashMap::new();
        m.insert(R1080, [370, 90, 1550, 120]);
        add_resolution_to_map(&mut m, R1440);
        m
    };
    pub static ref CASINO_FINGERPRINT_POS: HashMap<Resolution, Region> = {
        let mut m = HashMap::new();
        m.insert(R1080, [974, 157, 1320, 685]);
        add_resolution_to_map(&mut m, R1440);
        m
    };
    pub static ref CASINO_PARTS_POS: HashMap<Resolution, Vec<Region>> = {
        let mut m = HashMap::new();
        m.insert(
            R1080,
            vec![
                [475, 271, 595, 391],
                [618, 271, 738, 391],
                [475, 414, 595, 535],
                [618, 414, 738, 535],
                [475, 558, 595, 680],
                [618, 558, 738, 680],
                [475, 702, 595, 823],
                [618, 702, 738, 823],
            ],
        );
        add_resolution_to_array_map(&mut m, R1440);
        m
    };
    pub static ref CASINO_WAIT_DELAY:Duration = Duration::from_millis(4350);

    // CAYO
    pub static ref CAYO_FINGERPRINT_COUNT: u16 = 7;
    pub static ref CAYO_HEADER_POS: HashMap<Resolution, Region> = {
        let mut m = HashMap::new();
        m.insert(R1080, [600, 60, 1661, 127]);
        add_resolution_to_map(&mut m, R1440);
        m
    };

    pub static ref CAYO_FINGERPRINT_POS: HashMap<Resolution, Region> = {
        let mut m = HashMap::new();
        m.insert(R1080, [905, 321, 1565, 979]);
        add_resolution_to_map(&mut m, R1440);
        m
    };

    pub static ref CAYO_PARTS_POS: HashMap<Resolution, Vec<Region>> = {
        let mut m = HashMap::new();
        m.insert(
            R1080,
            vec![
                [413, 357, 820, 417],
                [413, 433, 820, 493],
                [413, 509, 820, 569],
                [413, 585, 820, 645],
                [413, 661, 820, 721],
                [413, 737, 820, 797],
                [413, 813, 820, 873],
                [413, 889, 820, 949]
            ],
        );
        add_resolution_to_array_map(&mut m, R1440);
        m
    };
    pub static ref CAYO_WAIT_DELAY:Duration = Duration::from_millis(2200);


    // GENERAL
    pub static ref PRESS_DURATION: Duration = Duration::from_millis(30);
    pub static ref UPDATE_RATE:u16 = 10;
    pub static ref LOOP_DELAY:Duration = Duration::from_millis(1000 / *UPDATE_RATE as u64);

    pub static ref OUTPUT_PATH: PathBuf = {
        if cfg!(debug_assertions) {
            Path::new("../output")
        }else {
            Path::new("output")
        }.to_path_buf()
    };
}

fn add_resolution_to_map(map: &mut HashMap<Resolution, Region>, resolution: Resolution) {
    map.insert(
        resolution,
        resolution_remap(
            map.get(&R1080).expect("could not find 1080p regions"),
            R1080,
            resolution,
        ),
    );
}

fn add_resolution_to_array_map(map: &mut HashMap<Resolution, Vec<Region>>, resolution: Resolution) {
    let vec: &Vec<Region> = map.get(&R1080).expect("could not find 1080p regions");

    let res: Vec<Region> = vec
        .into_iter()
        .map(|region| resolution_remap(region, R1080, resolution))
        .collect();

    map.insert(resolution, res);
}
fn resolution_remap(region: &Region, original_res: Resolution, target_res: Resolution) -> Region {
    let original_res_x = original_res.0 as f64;
    let original_res_y = original_res.1 as f64;
    let target_res_x = target_res.0 as f64;
    let target_res_y = target_res.1 as f64;

    let x1 = region[0] as f64;
    let y1 = region[1] as f64;
    let x2 = region[2] as f64;
    let y2 = region[3] as f64;

    let new_x1 = (x1 / original_res_x * target_res_x).round() as u32;
    let new_y1 = (y1 / original_res_y * target_res_y).round() as u32;
    let new_x2 = (x2 / original_res_x * target_res_x).round() as u32;
    let new_y2 = (y2 / original_res_y * target_res_y).round() as u32;

    return [new_x1, new_y1, new_x2, new_y2];
}
