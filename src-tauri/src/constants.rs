use crate::utils::Region;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    // CASINO
    pub static ref CASINO_FINGERPRINT_COUNT: u16 = 4;
    pub static ref CASINO_HEADER_POS: HashMap<u32, Region> = {
        let mut m = HashMap::new();
        m.insert(1080, [370, 90, 1550, 120]);
        m.insert(1440, [495, 126, 2060, 155]);
        m
    };
    pub static ref CASINO_FINGERPRINT_POS: HashMap<u32, Region> = {
        let mut m = HashMap::new();
        m.insert(1080, [974, 157, 1320, 685]);
        m.insert(1440, [1215, 208, 1825, 910]);
        m
    };
    pub static ref CASINO_PARTS_POS: HashMap<u32, Vec<Region>> = {
        let mut m = HashMap::new();
        m.insert(
            1080,
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
        m.insert(
            1440,
            vec![
                [632, 360, 791, 519],
                [825, 360, 983, 519],
                [632, 550, 791, 710],
                [825, 550, 983, 710],
                [632, 743, 791, 900],
                [825, 743, 983, 900],
                [632, 937, 791, 1095],
                [825, 937, 983, 1095],
            ],
        );
        m
    };

    // CAYO
    pub static ref CAYO_FINGERPRINT_COUNT: u16 = 7;
    pub static ref CAYO_HEADER_POS: HashMap<u32, Region> = {
        let mut m = HashMap::new();
        m.insert(1080, [449, 60, 1661, 127]);
        m.insert(1440, [0, 0, 0, 0]);
        m
    };

    pub static ref CAYO_FINGERPRINT_POS: HashMap<u32, Region> = {
        let mut m = HashMap::new();
        m.insert(1080, [905, 321, 1565, 979]);
        m.insert(1440, [0, 0, 0, 0]);
        m
    };

    pub static ref CAYO_PARTS_POS: HashMap<u32, Vec<Region>> = {
        let mut m = HashMap::new();
        m.insert(
            1080,
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
        m.insert(
            1440,
            vec![
                [0,0,0,0],
                [0,0,0,0],
                [0,0,0,0],
                [0,0,0,0],
                [0,0,0,0],
                [0,0,0,0],
                [0,0,0,0],
                [0,0,0,0],
            ],
        );
        m
    };


    // GENERAL
    pub static ref UPDATE_RATE:u16 = 10;
}
