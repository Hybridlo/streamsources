pub mod front_common;

use rand::prelude::*;
use serde::{Deserialize, Serialize};

pub const GLOBAL_DELAY_VALUE_SECONDS: u32 = 1;
pub const GLOBAL_DELAY_VALUE: u32 = GLOBAL_DELAY_VALUE_SECONDS * 1_000;

pub const FPS: u32 = 60;

#[derive(Debug, Deserialize, Serialize)]
pub struct MyTestStruct {
    v1: usize,
    v2: Option<String>,
}

impl MyTestStruct {
    pub fn new() -> Self {
        Self {
            v1: rand::thread_rng().gen_range(0..1000),
            v2: None,
        }
    }

    pub fn from(v1: i32) -> Self {
        Self {
            v1: v1 as usize,
            v2: None,
        }
    }
}