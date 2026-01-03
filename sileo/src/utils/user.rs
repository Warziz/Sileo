use std::time::{SystemTime, UNIX_EPOCH};

pub fn color_text(text: &str, color: &str) -> String {
    let code = match color {
        "red" => "\033[91m",
        "green" => "\033[92m",
        "yellow" => "\033[93m",
        "blue" => "\033[94m",
        "magenta" => "\033[95m",
        _ => "",
    };
    format!("{}{}\x1b[0m", code, text)
}

pub fn generate_username() -> String {

    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
    const  LENGTH: usize = 8;

    let mut username = String::with_capacity(LENGTH);

    let mut seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;

    for _ in 0..LENGTH {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;

        let index = (seed % CHARSET.len() as u64) as usize;
        username.push(CHARSET[index] as char);

    }

    username
}