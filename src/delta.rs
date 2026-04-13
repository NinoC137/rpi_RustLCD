use std::process::Command;

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

pub fn load_passwords() -> Vec<DeltaPassword> {
    fetch_passwords_from_curl().unwrap_or_else(fallback_passwords)
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

fn fetch_passwords_from_curl() -> Option<Vec<DeltaPassword>> {
    let output = Command::new("curl")
        .args(["-fsSL", "https://api.icofun.cn/api/delta_mima.php"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let body = String::from_utf8(output.stdout).ok()?;
    let parsed = parse_passwords(&body);
    if parsed.is_empty() {
        None
    } else {
        Some(parsed)
    }
}

fn parse_passwords(body: &str) -> Vec<DeltaPassword> {
    let mut locations = Vec::new();
    let mut passwords = Vec::new();

    for line in body.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("地点:") {
            locations.push(normalize_location(rest.trim()));
        } else if let Some(rest) = trimmed.strip_prefix("密码:") {
            let digits: String = rest.chars().filter(|c| c.is_ascii_digit()).collect();
            if !digits.is_empty() {
                passwords.push(digits);
            }
        }
    }

    let count = locations.len().min(passwords.len()).min(5);
    (0..count)
        .map(|i| DeltaPassword {
            location: locations[i].clone(),
            password: passwords[i].clone(),
        })
        .collect()
}

fn normalize_location(raw: &str) -> String {
    if let Some(start) = raw.find('【') {
        if let Some(end_rel) = raw[start + '【'.len_utf8()..].find('】') {
            let inner_start = start + '【'.len_utf8();
            let inner_end = inner_start + end_rel;
            let inner = &raw[inner_start..inner_end];
            return translit_location(inner);
        }
    }
    translit_location(raw)
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
