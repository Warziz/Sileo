use std::net::UdpSocket;
use chrono::{DateTime, Utc};
use x25519_dalek::PublicKey;
use std::io::{self, Write};
use std::thread;
use std::sync::{Arc, Mutex};
use std::str;
use base64::{engine::general_purpose, Engine as _};

use crate::crypto::crypto::generate_keypair;
use crate::utils::user::color_text;

pub fn init_sock(port: u16) -> io::Result<UdpSocket> {
    
    let addr = format!("0.0.0.0:{}",port);
    let socket = UdpSocket::bind(addr)?;
    socket.set_nonblocking(false)?; 
    Ok(socket)
}


pub fn listener(username: String, socket: UdpSocket, stdout : Arc<Mutex<io::Stdout>>) {

    thread::spawn(move || {

        let mut buffer = [0; 1024];
    
            loop {

                if let Ok((len, _)) =  socket.recv_from(&mut buffer){
                    let data = String::from_utf8_lossy(&buffer[..len]);
                    
                    if data == "CTRL:PUNCH"{
                        continue;
                    } else {
                        let utc_now : DateTime<Utc> = Utc::now();
                
                    let mut out = stdout.lock().unwrap();
                
                    write!(out, "\r{}", " ".repeat(80)).unwrap();
                    write!(
                        out,
                        "\r{}\n",
                        color_text(
                            &format!("[{}] - peer > {}", utc_now, data),
                            "cyan"
                        )
                    ).unwrap();

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
                    
            }
        }
    });
}

pub fn start_input_loop(socket: UdpSocket, peer_addr: &str) {
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        socket.send_to(input.as_bytes(), peer_addr).unwrap();
    }
}

pub fn wait_for_peer(
    socket: &UdpSocket,
) -> io::Result<(String, u16, PublicKey, String)> {
    
    println!("Wait for peer");
    let mut buf = [0u8; 4096];

    loop {
        let (len, _) = socket.recv_from(&mut buf)?;
        println!("Data Recieved !");
        let data = str::from_utf8(&buf[..len]).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid UTF-8: {}", e),
            )
        })?;

        let data = data.trim();

        if data == "ready" {
            println!("[*] Checked in with server, waiting");
            continue;
        }

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

        return Ok((ip, sport, pubkey_other, peer_username));
    }
}
