use std::process::Command;
use std::sync::{Arc, Mutex, OnceLock};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct DeltaPassword {
    pub location: String,
    pub password: String,
}

const FALLBACK_PASSWORDS: [(&str, &str); 5] = [
    ("DAM", "5575"),
    ("GORGE", "9879"),
    ("BAKSH", "7180"),
    ("AEROSPC", "3159"),
    ("PRISON", "8777"),
];

const FETCH_TIMEOUT_SECONDS: &str = "0.1";
const REFRESH_INTERVAL_SECS: u64 = 600;
const API_URL: &str = "https://i.elaina.vin/api/%E4%B8%89%E8%A7%92%E6%B4%B2/%E5%AF%86%E7%A0%81/";

#[derive(Debug, Clone)]
struct PasswordCache {
    items: Vec<DeltaPassword>,
}

static PASSWORD_CACHE: OnceLock<Arc<Mutex<PasswordCache>>> = OnceLock::new();
static PASSWORD_WORKER: OnceLock<()> = OnceLock::new();

pub fn load_passwords() -> Vec<DeltaPassword> {
    let cache = PASSWORD_CACHE
        .get_or_init(|| {
            Arc::new(Mutex::new(PasswordCache {
                items: fallback_passwords(),
            }))
        })
        .clone();

    PASSWORD_WORKER.get_or_init(|| {
        let cache = cache.clone();
        thread::spawn(move || loop {
            if let Some(items) = fetch_passwords_from_curl_once() {
                if let Ok(mut guard) = cache.lock() {
                    guard.items = items;
                }
            }
            thread::sleep(Duration::from_secs(REFRESH_INTERVAL_SECS));
        });
    });

    cache
        .lock()
        .map(|guard| guard.items.clone())
        .unwrap_or_else(|_| fallback_passwords())
}

fn fallback_passwords() -> Vec<DeltaPassword> {
    FALLBACK_PASSWORDS
        .iter()
        .map(|(location, password)| DeltaPassword {
            location: (*location).to_string(),
            password: (*password).to_string(),
        })
        .collect()
}

fn fetch_passwords_from_curl_once() -> Option<Vec<DeltaPassword>> {
    let curl_output = Command::new("curl")
        .args([
            "-fsSL",
            "--connect-timeout",
            FETCH_TIMEOUT_SECONDS,
            "--max-time",
            FETCH_TIMEOUT_SECONDS,
            API_URL,
        ])
        .output()
        .ok()?;

    if !curl_output.status.success() {
        return None;
    }

    let body = String::from_utf8(curl_output.stdout).ok()?;
    let parser = r#"import sys, json
obj = json.load(sys.stdin)
items = obj.get('data', [])
for item in items[:5]:
    name = str(item.get('name', '')).strip()
    password = str(item.get('password', '')).strip()
    if name and password:
        print(f"{name}\t{password}")
"#;

    let parse_output = Command::new("python3")
        .args(["-c", parser])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .ok()
        .and_then(|mut child| {
            use std::io::Write;
            if let Some(stdin) = child.stdin.as_mut() {
                let _ = stdin.write_all(body.as_bytes());
            }
            child.wait_with_output().ok()
        })?;

    if !parse_output.status.success() {
        return None;
    }

    let text = String::from_utf8(parse_output.stdout).ok()?;
    let parsed = parse_password_lines(&text);
    if parsed.is_empty() {
        None
    } else {
        Some(parsed)
    }
}

fn parse_password_lines(body: &str) -> Vec<DeltaPassword> {
    body.lines()
        .filter_map(|line| {
            let mut parts = line.split('\t');
            let name = parts.next()?.trim();
            let password = parts.next()?.trim();
            if name.is_empty() || password.is_empty() {
                return None;
            }
            Some(DeltaPassword {
                location: translit_location(name),
                password: password.to_string(),
            })
        })
        .take(5)
        .collect()
}

fn translit_location(s: &str) -> String {
    match s {
        "零号大坝" => "DAM".to_string(),
        "长弓溪谷" => "GORGE".to_string(),
        "巴克什" => "BAKSH".to_string(),
        "航天基地" => "AEROSPC".to_string(),
        "潮汐监狱" => "PRISON".to_string(),
        other => other
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .collect::<String>()
            .to_uppercase(),
    }
}
