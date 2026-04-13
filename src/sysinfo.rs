use std::fs;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct SystemStatus {
    pub cpu_percent: u8,
    pub mem_percent: u8,
    pub top_threads: Vec<ProcEntry>,
}

#[derive(Debug, Clone)]
pub struct ProcEntry {
    pub label: String,
    pub cpu: u8,
    pub mem: u8,
}

pub fn read_system_status() -> SystemStatus {
    let (cpu_percent, mem_percent) = read_overall_usage().unwrap_or((0, 0));
    let top_threads = read_top_processes(3);
    SystemStatus {
        cpu_percent,
        mem_percent,
        top_threads,
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

fn read_top_processes(limit: usize) -> Vec<ProcEntry> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("ps -eo comm,pcpu,pmem --sort=-pcpu | sed -n '2,12p'")
        .output();

    let Ok(output) = output else {
        return fallback_entries();
    };
    let Ok(text) = String::from_utf8(output.stdout) else {
        return fallback_entries();
    };

    let mut items = Vec::new();
    for line in text.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
        if parts.len() < 3 {
            continue;
        }
        let label = truncate_label(parts[0], 12);
        let cpu = parts[1]
            .parse::<f64>()
            .ok()
            .unwrap_or(0.0)
            .round()
            .clamp(0.0, 100.0) as u8;
        let mem = parts[2]
            .parse::<f64>()
            .ok()
            .unwrap_or(0.0)
            .round()
            .clamp(0.0, 100.0) as u8;
        items.push(ProcEntry { label, cpu, mem });
        if items.len() >= limit {
            break;
        }
    }

    if items.is_empty() {
        fallback_entries()
    } else {
        items
    }
}

fn truncate_label(s: &str, max_len: usize) -> String {
    s.chars().take(max_len).collect()
}

fn fallback_entries() -> Vec<ProcEntry> {
    vec![
        ProcEntry { label: "idle".to_string(), cpu: 0, mem: 0 },
        ProcEntry { label: "system".to_string(), cpu: 0, mem: 0 },
        ProcEntry { label: "worker".to_string(), cpu: 0, mem: 0 },
    ]
}
