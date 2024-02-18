////////////////////////////////////////////////////////////////////////////////

use std::thread::sleep;
use std::time::{Duration,Instant};

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct TimeManager {
    target_fps : usize,
    target_spf : Duration,
    last_t     : Instant,
    total_t    : Duration,
    frame_count: usize,
    pause_count: isize,
}

impl TimeManager {
    pub fn new(target_fps : usize) -> Self {
        let spf = 1.0f64 / target_fps as f64;
        let spf_sec = spf.floor() as u64;
        let spf_subnanos = ((spf - spf_sec as f64) * 1e9).round() as u32;
        let target_spf = Duration::new(spf_sec, spf_subnanos);
        let last_t = Instant::now();
        let total_t = Duration::default();
        let frame_count = 0;
        let pause_count = 1;
        TimeManager {
            target_fps, target_spf, last_t, total_t,
            frame_count, pause_count,
        }
    }

    pub fn tick(&mut self) {
        if self.pause_count > 0 { return; }
        let t = Instant::now();
        let tdiff = t - self.last_t;
        if tdiff < self.target_spf {
            sleep(self.target_spf - tdiff);
        }
        self.frame_count += 1;
        let t = Instant::now();
        let tdiff = t - self.last_t;
        self.last_t = t;
        self.total_t += tdiff;
    }

    pub fn pause(&mut self) {
        self.pause_count += 1;
    }

    pub fn unpause(&mut self) {
        self.pause_count -= 1;
        if self.pause_count == 0 {
            self.last_t = Instant::now();
        }
    }

    pub fn target_fps(&self) -> usize {
        self.target_fps
    }

    pub fn average_fps(&self) -> f64 {
        (self.frame_count as f64
            / (self.total_t.as_secs() as f64
                + self.total_t.subsec_nanos() as f64*1e-9))
    }

    pub fn frame_count(&self) -> usize {
        self.frame_count
    }
}

////////////////////////////////////////////////////////////////////////////////
