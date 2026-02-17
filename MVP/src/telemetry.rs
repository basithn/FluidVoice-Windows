use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::Mutex;
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UsageStats {
    pub total_recordings: usize,
    pub total_audio_seconds: f64,
    pub errors_encountered: usize,
    pub last_used: Option<SystemTime>,
}

pub static STATS: Mutex<UsageStats> = Mutex::new(UsageStats {
    total_recordings: 0,
    total_audio_seconds: 0.0,
    errors_encountered: 0,
    last_used: None,
});

pub fn load_stats() {
    if let Ok(content) = fs::read_to_string("stats.json") {
        if let Ok(loaded) = serde_json::from_str::<UsageStats>(&content) {
            *STATS.lock().unwrap() = loaded;
        }
    }
}

pub fn save_stats() {
    let stats = STATS.lock().unwrap();
    if let Ok(json) = serde_json::to_string_pretty(&*stats) {
        let _ = fs::write("stats.json", json);
    }
}

pub fn record_usage(duration_secs: f64) {
    let mut stats = STATS.lock().unwrap();
    stats.total_recordings += 1;
    stats.total_audio_seconds += duration_secs;
    stats.last_used = Some(SystemTime::now());
}

pub fn record_error() {
    let mut stats = STATS.lock().unwrap();
    stats.errors_encountered += 1;
}

pub fn print_summary() {
    let stats = STATS.lock().unwrap();
    println!("\n📊 Session Summary:");
    println!("  Recordings: {}", stats.total_recordings);
    println!("  Total Audio: {:.1}s", stats.total_audio_seconds);
    println!("  Errors: {}", stats.errors_encountered);
}
