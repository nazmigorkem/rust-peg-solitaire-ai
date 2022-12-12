use std::{
    sync::mpsc::Sender,
    thread::{self, JoinHandle},
    time::Duration,
};

use psutil::process::Process;

use crate::peg_solitaire::Board;

use super::traits::Check;

impl Check for Board {
    fn memory_thread(tx: Sender<(u64, bool)>) -> JoinHandle<()> {
        thread::spawn(move || {
            let process = Process::current().unwrap();
            loop {
                let memory_usage_in_bytes = process.memory_info().unwrap().vms();
                tx.send((memory_usage_in_bytes, memory_usage_in_bytes > 1 << 33))
                    .unwrap();
                thread::sleep(Duration::from_millis(100));
            }
        })
    }
    fn timing_thread(time_limit_in_seconds: u64) -> JoinHandle<()> {
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(time_limit_in_seconds));
        })
    }
}
