use std::net::{UdpSocket};
use std::io::{self, Write};
use std::thread;
use std::sync::{Arc, Mutex};
use std::str;

use chrono::{DateTime, Utc};
use x25519_dalek::PublicKey;
use base64::{engine::general_purpose, Engine as _};

use crate::crypto::aes::{decrypt, encrypt};
use crate::utils::user::color_text;

pub fn init_sock(port: u16) -> io::Result<UdpSocket> {
    
    let addr = format!("0.0.0.0:{}",port);
    let socket = UdpSocket::bind(addr)?;
    socket.set_nonblocking(false)?; 
    Ok(socket)
}

pub fn hole_punching(socket: &UdpSocket, peer_addr: &str) -> io::Result<()> {

    let msg = format!("[*] Peer address: {}", peer_addr);
    println!("{}",color_text(&msg, "yellow"));

    let ctrl = "CTRL:PUNCH";
    let mut packet = Vec::new();
    packet.push(0x02);
    packet.extend_from_slice(ctrl.as_bytes());

    println!("{}", color_text("[!] Punching Hole", "magenta"));
    socket.send_to(&packet, peer_addr)?;
    Ok(())
}


pub fn listener(username: String, socket: UdpSocket, stdout : Arc<Mutex<io::Stdout>>, aes_key: [u8; 32]) {

    thread::spawn(move || {

        let mut buffer = [0; 1024];
    
            loop {

                if let Ok((len, _)) = socket.recv_from(&mut buffer){
                    let data = &buffer[..len];
                    
                    if data.is_empty(){
                        continue;
                    }
                    
                    let msg_type = data[0];

                    match msg_type {
                        0x01 => {
                            let payload = &data[1..];
                            if payload.len() < 12 {
                                continue;
                            }
                            //Get nonce & cipher_text
                            let (nonce_slice, ciphertext) = payload.split_at(12);
                            let nonce: [u8; 12] = match nonce_slice.try_into() {
                                Ok(n) => n,
                                Err(_) => {
                                    eprintln!("Invalid nonce length");
                                    continue;
                                }
                            };

                            //decipher the message
                            let plain = decrypt(&aes_key, ciphertext, &nonce);
                            let message = match String::from_utf8(plain) {
                                Ok(m) => m,
                                    Err(_) => {
                                    eprintln!("Invalid UTF-8 in decrypted message");
                                    return;
                                }
                            };
                        
                            display_text(&username, &stdout, &message);
                        
                        }

                        0x02 => {
                            let payload = &data[1..];
                            let message = match std::str::from_utf8(payload) {
                                Ok(m) => m,
                                Err(_) => {
                                    eprintln!("Invalid UTF-8 in CTRL message");
                                    continue;
                                }
                            };

                            if message.trim() == "CTRL:PUNCH" {
                                continue;
                            }
                        }

                        _ => continue,
                    }
                    
            }
        }
    });
}


pub fn start_input_loop(socket: UdpSocket, peer_addr: &str, aes_key: &[u8; 32]) {
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        input.clear();
        stdin.read_line(&mut input).unwrap();

        let msg = input.trim_end().as_bytes();
        if msg.is_empty() {
            continue;
        }
        let (cipher_text,nonce) = encrypt(aes_key, msg);
        let mut packet = Vec::new();
        packet.push(0x01);
        packet.extend_from_slice(&nonce);
        packet.extend_from_slice(&cipher_text);
        socket.send_to(&packet, peer_addr).unwrap();
    }
}


pub fn wait_for_peer(
    socket: &UdpSocket,
) -> io::Result<(String, u16, PublicKey, String)> {
    
    println!("{}", color_text("[*] Waiting for peer", "yellow"));
    let mut buf = [0u8; 4096];

    loop {
        let (len, _) = socket.recv_from(&mut buf)?;
        println!("{}",color_text("[+] Data recieved from relay server", "green"));
        let data = str::from_utf8(&buf[..len]).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid UTF-8: {}", e),
            )
        })?;

        let data = data.trim();

        let parts: Vec<&str> = data.split_whitespace().collect();

        if parts.len() != 4 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid peer info format: {}", data),
            ));
        }

        let ip = parts[0].to_string();

        let sport: u16 = parts[1].parse().map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid port: {}", parts[1]),
            )
        })?;

        //decode pubkey b64 -> bytes
        let pubkey_bytes = general_purpose::STANDARD
            .decode(parts[2])
            .map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidData, 
                    "Invalid base64 public key",
                )
            })?;

        if pubkey_bytes.len() != 32 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid public key length",
            ));
        }
        
        //reconstruct pubkey bytes -> Pubkey
        let mut pubkey_array = [0u8;32];
        pubkey_array.copy_from_slice(&pubkey_bytes);
        let pubkey_other = PublicKey::from(pubkey_array);

        let peer_username = parts[3].to_string();
        println!("{}, {}, {:?}, {}", ip, sport, pubkey_other, peer_username);

        return Ok((ip, sport, pubkey_other, peer_username));
    }
}


fn display_text (username: &String, stdout : &Arc<Mutex<io::Stdout>>, message: &String) {

    let utc_now : DateTime<Utc> = Utc::now();           
    let mut out = stdout.lock().unwrap();

    // clear current line
    write!(out, "\r\x1b[2K").unwrap();

    // print peer message
    write!(
        out,
        "{}\n",
        color_text(
            &format!("[{}] - peer > {}", utc_now, message.trim()),
            "cyan"
        )
            ).unwrap();
    // reprint prompt
    write!(
        out,
        "{}",
        color_text(
            &format!("[{}] - {}(you) > ", utc_now, username),
            "green"
        )
            ).unwrap();

        out.flush().unwrap();
}