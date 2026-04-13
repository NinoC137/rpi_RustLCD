use std::fs;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct SystemStatus {
    pub cpu_percent: u8,
    pub mem_percent: u8,
    pub top_label: String,
    pub top_cpu: u8,
    pub top_mem: u8,
}

pub fn read_system_status() -> SystemStatus {
    let (cpu_percent, mem_percent) = read_overall_usage().unwrap_or((0, 0));
    let (top_label, top_cpu, top_mem) = read_top_process().unwrap_or(("idle".to_string(), 0, 0));
    SystemStatus {
        cpu_percent,
        mem_percent,
        top_label,
        top_cpu,
        top_mem,
    }
}

fn read_overall_usage() -> Option<(u8, u8)> {
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
    let used = total.saturating_sub(avail);
    let mem_pct = if total == 0 {
        0
    } else {
        (((used as f64 / total as f64) * 100.0).round() as u8).min(100)
    };

    let output = Command::new("sh")
        .arg("-c")
        .arg("top -bn1 | sed -n '3p'")
        .output()
        .ok()?;
    let line = String::from_utf8(output.stdout).ok()?;
    let mut cpu_pct = 0u8;
    if let Some(idle_pos) = line.find(" id") {
        let prefix = &line[..idle_pos];
        let idle_num = prefix
            .split_whitespace()
            .last()
            .and_then(|s| s.trim_end_matches(',').parse::<f64>().ok())
            .unwrap_or(100.0);
        cpu_pct = (100.0 - idle_num).round().clamp(0.0, 100.0) as u8;
    }

    Some((cpu_pct, mem_pct))
}

fn read_top_process() -> Option<(String, u8, u8)> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("ps -eo comm,pcpu,pmem --sort=-pcpu | sed -n '2p'")
        .output()
        .ok()?;
    let line = String::from_utf8(output.stdout).ok()?;
    let parts: Vec<_> = line.split_whitespace().collect();
    if parts.len() < 3 {
        return None;
    }
    let label = parts[0].to_string();
    let cpu = parts[1].parse::<f64>().ok().unwrap_or(0.0).round().clamp(0.0, 100.0) as u8;
    let mem = parts[2].parse::<f64>().ok().unwrap_or(0.0).round().clamp(0.0, 100.0) as u8;
    Some((label, cpu, mem))
}
