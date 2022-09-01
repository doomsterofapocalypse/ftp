use crate::packet::structure::*;
use xxhash_rust::xxh3::xxh3_64;
use std::fs::{File, self};
use std::io::{Read, Write};
use std::net::{TcpStream, UdpSocket, SocketAddr};
use crate::server::utils::parse_buf;
use std::{thread, time};
use crate::common::utils::compress;

// read a file and return a vector of packetstructs
pub fn read_file_buffer(filepath: &str) -> Result<Vec<filechunk>, Box<dyn std::error::Error>> {
    const BUFFER_LEN: usize = 4096;
    let mut buffer = [0u8; BUFFER_LEN];
    let mut file = File::open(filepath)?;
    let mut filevec:Vec<filechunk> = Vec::new();
    let mut packetnumber = 0;
    let mut padding = 0;

    loop {
        let read_count = file.read(&mut buffer)?;
        if read_count == 0 {
            break;
        }
        packetnumber+=1;
        let curead = &buffer[..BUFFER_LEN];
        let totalsize = curead.len();
        if read_count < 4096{
            padding = 4096-read_count;
        }
        let data:Vec<u8> = curead.to_vec();
        let checksum = xxh3_64(curead);
        let header = packetype::filechunk;
        let pckt = filechunk::new(header, packetnumber, totalsize, data, padding, checksum);
        filevec.push(pckt);
        //filevec.extend_from_slice(curead);

        
    }
    Ok(filevec)
}

//compress and encrypt a file and read it as vector of packetstruct
pub fn read_enc_file(filepath: &str) -> Result<Vec<filechunk>, Box<dyn std::error::Error>>{
    const BUFFER_LEN: usize = 4096;
    let mut buffer = [0u8; BUFFER_LEN];
    let mut filevec:Vec<filechunk> = Vec::new();
    let mut packetnumber = 0;
    let mut padding = 0;

    let enc_file = compress(filepath);
    fs::write("tmpenc", enc_file).expect("Temporary file could not be written");
    let mut file = File::open("tmpenc")?;

    loop {
        let read_count = file.read(&mut buffer)?;
        if read_count == 0 {
            break;
        }
        packetnumber+=1;
        let curead = &buffer[..BUFFER_LEN];
        let totalsize = curead.len();
        if read_count < 4096{
            padding = 4096-read_count;
        }
        //let data:Vec<u8> = curead.iter().cloned().collect();
        let data:Vec<u8> = curead.to_vec();
        let checksum = xxh3_64(curead);
        let header = packetype::filechunk;
        let pckt = filechunk::new(header, packetnumber, totalsize, data, padding, checksum);
        filevec.push(pckt);
        //filevec.extend_from_slice(curead);

        
    }
    Ok(filevec)
}



pub fn send_tcp_file(connection_string:String, filechunks:Vec<filechunk>, filename:&str){
    let chunks = filechunks;
    let total_chunks = chunks.len();
    let flagvec = vec![flags::Encrypted, flags::Compressed];
    let flags = Some(flagvec);
    let hellopckt = hello::new(packetype::hello, filename.to_string(), flags, total_chunks);
    let byepckt = bye{header:packetype::bye, flag:flags::complete};
    
    
    let stream = TcpStream::connect(connection_string);
    match stream{
        Ok(mut stream) => {
            println!("[+] Successfully connected to server");
            println!("[+] Sending Hello Packet..");
            let hellobytes = bincode::serialize(&hellopckt).unwrap();
            stream.write(&hellobytes).unwrap();
            for i in 0..total_chunks{
                println!("[+] sending chunk no:{}", i);
                let bytes = bincode::serialize(&chunks[i]).unwrap();
                println!("chunk size: {}", bytes.len());
                stream.write(&bytes).unwrap();
            }
            println!("[+] File transfer complete..");
            let byebytes = bincode::serialize(&byepckt).unwrap();
            println!("[+] Sending bye...");
            stream.write(&byebytes).unwrap();
            
            
        }
    
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    
    println!("Terminated.")
}



pub fn send_udp_file(connection_string:String, socket:UdpSocket, filechunks:Vec<filechunk>, filename:&str) {
    let chunks = filechunks;
    let total_chunks = chunks.len();
    let flagvec = vec![flags::Encrypted, flags::Compressed];
    let flags = Some(flagvec);
    let hellopckt = hello::new(packetype::hello, filename.to_string(), flags, total_chunks);
    let byepckt = bye{header:packetype::bye, flag:flags::complete};
    let mut ackbuffer = [0u8; 12];

    
    println!("[+] Sending Hello Packet to: {}", &connection_string);
    let hellobytes = bincode::serialize(&hellopckt).unwrap();
    socket.send_to(&hellobytes, &connection_string).expect("hello packet could not be sent");
    //for (i, <item>) in chunks.iter().enumerate().take(total_chunks)
    for i in 0..total_chunks{
        println!("[+] sending chunk no:{}", i);
        let bytes = bincode::serialize(&chunks[i]).unwrap();
        println!("chunk size: {}", bytes.len());
        socket.send_to(&bytes, &connection_string).expect("Chunk could not be sent");
        let ack = socket.recv_from(&mut ackbuffer).unwrap();
        if ackchecker(ack, i, ackbuffer){
            continue;
        }
        else{
            let socket_clone = socket.try_clone().expect("couldn't clone the socket");
            slow_retransmit(&bytes, &connection_string, socket_clone, i);
        }

    }
    println!("[+] File transfer complete..");
    let byebytes = bincode::serialize(&byepckt).unwrap();
    println!("[+] Sending bye...");
    socket.send_to(&byebytes, &connection_string).expect("bye packet could not be sent");
            
            

    
    println!("Terminated.")
}


fn ackchecker(ack:(usize, SocketAddr), index:usize, ackbuffer:[u8;12]) -> bool {
    //let mut ackbuffer = [0u8; 100];
    let packet_number:i32 = index as i32+1;
    println!("byte size {} recieved from:{}", ack.0, ack.1);
    if ack.0 > 0{
        let v = &ackbuffer[0..ack.0];
        let ptype = parse_buf(v);
        match ptype{
            packetype::ack =>{
                let pckt:ack = bincode::deserialize(v).unwrap();
                let acknumber = pckt.packetnumber;
                acknumber == packet_number
            }

            _ => {
                println!("unrecognized pack type");
                false
            }
        }
    }
    else {
        false
    }

}


fn slow_retransmit(bytes:&Vec<u8>, connection_string:&str, socket:UdpSocket, i:usize) {

    loop{
        let mut ackbuffer = [0u8; 12];
        socket.send_to(bytes, &connection_string).expect("retry Chunk could not be sent");
        let ack = socket.recv_from(&mut ackbuffer).unwrap();
        if ackchecker(ack,i, ackbuffer){
            break;
        }
        else{
            println!("[+] Waiting before retransmission");
            let wait_time = time::Duration::from_millis(30);
            thread::sleep(wait_time);
            continue;
        }

    }

}
