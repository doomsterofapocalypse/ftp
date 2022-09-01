
use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown, UdpSocket};
use std::io::{Read};

use crate::packet::structure::{packetype, filechunk, hello, ack, flags};
use crate::server::utils::{parse_buf, write_file};





pub fn start_udp_server(port:&str){
    let host = "0.0.0.0".to_string();
    let port = port;
    let connection_string = host+":"+port;

    let socket = UdpSocket::bind(connection_string).expect("Udp Bind not successful try another port");
    println!("UDP Server listening on port:{}", port);
    handle_udp_client(socket);

}


fn handle_udp_client(socket:UdpSocket){
    let mut data = [0_u8; 4136]; // using 4136 byte buffer
    let mut filename:String = "".to_string();
    let mut number_of_chunks = 0;
    let mut filevec:Vec<filechunk> = vec![];

    loop {
        let msg = socket.recv_from(&mut data);
        match msg{
            Ok((number_of_bytes,src_addr)) =>{
                println!("byte size {} recieved from:{}", number_of_bytes, src_addr);
                if number_of_bytes > 0{
                    let v = &data[0..number_of_bytes];
                    let ptype = parse_buf(v);
                    match ptype{
                        packetype::hello => {
                        let pckt:hello = bincode::deserialize(v).unwrap();
                            filename = pckt.name;
                            number_of_chunks = pckt.number_of_chunks;
                        }
                        packetype::filechunk => {
                            let pckt:filechunk = bincode::deserialize(v).unwrap();
                            println!("packet number:{}",&pckt.packetnumber);
                            let packetnumber = pckt.packetnumber;
                            filevec.push(pckt);
                            let ackpckt = ack::new(packetype::ack, packetnumber, flags::pass);
                            let reply = bincode::serialize(&ackpckt).unwrap();
                            socket.send_to(&reply, src_addr).expect("ack reply failed!");
                            
                        }
                        packetype::bye => {
                            write_file(filevec.clone(), filename.clone());
                            filevec = vec![];
                            filename = "".to_string();
                        }
        
                        _ => {
                            //println!("[!] Unrecognizable packet format");
                        }
                    
                    }

                }
             
            }
            Err(_) => {

            }
        }
    }

}



pub fn start_tcp_server(port:&str){
    let host = "0.0.0.0".to_string();
    let port = port;
    let connection_string = host+":"+port;
    let listener = TcpListener::bind(connection_string).unwrap();
    println!("Server listening on port:{}", port);


    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection succeeded
                    handle_tcp_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
        
    }
    // close the socket server
    drop(listener);
    
}



fn handle_tcp_client(mut stream: TcpStream) {
    let mut streamdata = [0_u8; 4136]; // using 4136 byte buffer
    let mut filename:String = "".to_string();
    let mut number_of_chunks = 0;
    let mut filevec:Vec<filechunk> = vec![];
    while match stream.read(&mut streamdata) {
        Ok(size) => {
            println!("size received: {}", size);
            if size > 0 {
                let v = &streamdata[0..size];
                let ptype = parse_buf(v);
                match ptype{
                    packetype::hello => {
                       let pckt:hello = bincode::deserialize(v).unwrap();
                        filename = pckt.name;
                        number_of_chunks = pckt.number_of_chunks;
                    }
                    packetype::filechunk => {
                        let pckt:filechunk = bincode::deserialize(v).unwrap();
                        println!("packet number:{}",&pckt.packetnumber);
                        filevec.push(pckt);
                        
                    }
                    packetype::bye => {
                        write_file(filevec.clone(), filename.clone());
                    }
    
                    _ => {
                        //println!("[!] Unrecognizable packet format");
                    }
                   
                }
                true
            }
            else{
                false
            }

        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    }{}
}




