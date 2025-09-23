use std::thread;
use std::time::Duration;
use std::collections::VecDeque;
use chrono::{DateTime, Local, Timelike};
struct Measurement {
    timestamp: DateTime<Local>,
    val: f32,
}
fn main() {
    let win_size=5;
    let mut i = 0;
    let mut temp:f32;
    let mut window: VecDeque<Measurement> = VecDeque::new();
    loop {
        temp=24.750+(i as f32).sin()*0.5;
        window.push_back(Measurement{timestamp:Local::now(),val:temp});
        if window.len() > win_size {
            window.pop_front();
        }
        for (_, m)in window.iter().enumerate() {
            print!(" {:>8}",format!("{:2.2}", m.val));
        }
        println!();
        for (_, m)in window.iter().enumerate() {
            let now = m.timestamp;
            print!(" {:>8}", format!("{:02}:{:02}:{:02}",now.hour(),now.minute(),now.second()));
        }
        println!();
        i+=1;
        thread::sleep(Duration::from_secs(1));
    }
}
