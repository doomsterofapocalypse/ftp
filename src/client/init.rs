use std::{path::Path, net::UdpSocket};
use crate::client::utils::{send_tcp_file, send_udp_file};

use super::utils::read_enc_file;

pub fn start_tcp_transfer(ip:&str, port:&str, filepath:&str) -> Result<(), Box<dyn std::error::Error>>{
    let host = ip.to_string();
    let connection_string = host+":"+port;

    let filename = Path::new(&filepath).file_name().unwrap().to_str().unwrap(); 
    let filechunks = read_enc_file(filepath)?;
    send_tcp_file(connection_string, filechunks, filename);
    Ok(())
}



pub fn start_udp_transfer(ip:&str, port:&str, filepath:&str) -> Result<(), Box<dyn std::error::Error>>{
    let host = ip.to_string();
    let connection_string = host+":"+port;

    let filename = Path::new(&filepath).file_name().unwrap().to_str().unwrap(); 
    let filechunks = read_enc_file(filepath)?;

    let socket = UdpSocket::bind("0.0.0.0:34254").expect("couldn't bind to local address");
    println!("[+] UDP socket bind successful on 0.0.0.0:34254");
    send_udp_file(connection_string, socket, filechunks, filename);
    Ok(())
}