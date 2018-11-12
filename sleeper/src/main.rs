extern crate rand;

use std::thread;
use std::time;

use rand::Rng;


const MIN_SLEEP: u64 = 10;
const MAX_SLEEP: u64 = 1000;


fn main() {
    let ms = rand::thread_rng().gen_range(MIN_SLEEP, MAX_SLEEP);
    let duration = time::Duration::from_millis(ms);

    thread::sleep(duration);

    println!("slept for: {}ms", ms);
}
