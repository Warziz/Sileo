use std::time::{SystemTime, UNIX_EPOCH};

pub fn color_text(text: &str, color: &str) -> String {
    let code = match color {
        "cyan" => "\x1b[36m",
        "green" => "\x1b[32m",
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