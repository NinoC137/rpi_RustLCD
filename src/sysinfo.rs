use std::fs;
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub struct SystemStatus {
    pub cpu_percent: u8,
    pub mem_percent: u8,
}

pub fn read_system_status() -> SystemStatus {
    SystemStatus {
        cpu_percent: read_cpu_percent().unwrap_or(0),
        mem_percent: read_mem_percent().unwrap_or(0),
    }
}

fn read_cpu_percent() -> Option<u8> {
    let a = read_cpu_times()?;
    thread::sleep(Duration::from_millis(120));
    let b = read_cpu_times()?;

    let total_delta = b.total.saturating_sub(a.total);
    let idle_delta = b.idle.saturating_sub(a.idle);
    if total_delta == 0 {
        return Some(0);
    }
    let used = total_delta.saturating_sub(idle_delta);
    let pct = ((used as f64 / total_delta as f64) * 100.0).round() as u8;
    Some(pct.min(100))
}

fn read_mem_percent() -> Option<u8> {
    let text = fs::read_to_string("/proc/meminfo").ok()?;
    let mut total_kb = None;
    let mut avail_kb = None;

    for line in text.lines() {
        if line.starts_with("MemTotal:") {
            total_kb = line.split_whitespace().nth(1)?.parse::<u64>().ok();
        } else if line.starts_with("MemAvailable:") {
            avail_kb = line.split_whitespace().nth(1)?.parse::<u64>().ok();
        }
    }

    let total = total_kb?;
    let avail = avail_kb?;
    if total == 0 {
        return Some(0);
    }
    let used = total.saturating_sub(avail);
    let pct = ((used as f64 / total as f64) * 100.0).round() as u8;
    Some(pct.min(100))
}

#[derive(Debug, Clone, Copy)]
struct CpuTimes {
    total: u64,
    idle: u64,
}

fn read_cpu_times() -> Option<CpuTimes> {
    let text = fs::read_to_string("/proc/stat").ok()?;
    let line = text.lines().next()?;
    let mut parts = line.split_whitespace();
    let label = parts.next()?;
    if label != "cpu" {
        return None;
    }
    let nums: Vec<u64> = parts.filter_map(|p| p.parse::<u64>().ok()).collect();
    if nums.len() < 4 {
        return None;
    }
    let idle = nums[3] + nums.get(4).copied().unwrap_or(0);
    let total: u64 = nums.iter().sum();
    Some(CpuTimes { total, idle })
}
