mod server;
mod client;
mod packet;
mod common;



use clap::Parser;


use axum::{
    self,
    Router, 
    routing::{get,post, get_service}, 
    http::StatusCode,
    response::IntoResponse,
};
use axum_server::{ tls_rustls::RustlsConfig};
use tower_http::{services::ServeDir};
use crate::server::handlers::{save_file, show_upload};

///A Transfer Utility which can deploy servers using protocols UDP, TCP, HTTPS
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]

struct Args{
    ///mode to start the executable in either server or client 
    #[clap(short, long, value_parser)]
    mode:String,
    ///port number to use (mandatory to start server)
    #[clap(short, long, value_parser)]
    port:Option<String>,
    /// IP address to connect (to be used with client mode)
    #[clap(short, long, value_parser)]
    ip:Option<String>,
    ///filepath of the file to transfer
    #[clap(short, long, value_parser)]
    filepath:Option<String>,
    ///Use UDP protocol
    #[clap(short, long)]
    udp: bool,
    ///use TCP protocol - DEFAULT if no protocol is specifiec
    #[clap(short, long)]
    tcp: bool,
    ///use https protocol - default port of 443 is used
    #[clap(short, long)]
    https: bool,
    ///path to the cert.pem file
    #[clap(short, long, value_parser)]
    cert:Option<String>,
    ///path to key.pem file
    #[clap(short, long, value_parser)]
    key:Option<String>,
}

#[tokio::main]
pub async fn main() {
    let args = Args::parse();
    if args.mode == *"server"{
        if let Some(p) = args.port{
            if args.udp{
                server::init::start_udp_server(&p);
            }
            else{
                server::init::start_tcp_server(&p);
            }
        }
        else{
            if args.https{
                let (cert, key) = check_server_params(args).await;
                let app = Router::new()
                .route("/upload", get(show_upload))
                .route("/save_file", post(save_file))
                .fallback(get_service(ServeDir::new(".")).handle_error(handle_error));

                let config = RustlsConfig::from_pem_file(
                    &cert,
                    &key
                )
                .await
                .unwrap();

                println!("[+] Web Server started on port 443");
                let addr = std::net::SocketAddr::from(([0,0,0,0], 443));
                axum_server::bind_rustls(addr, config)
                    //.handle(handle)
                    .serve(app.into_make_service())
                    .await
                    .unwrap();
                
                
            }
            else{
                println!("Port number is required for starting the server");
                println!("usage: ftp -m server  -p <port number>");
            }
            
        }      
    }
    else if args.mode == *"client"{
        if let Some(path) = args.filepath.as_deref(){
            match args.ip {
                Some(ip) =>{
                    if let Some(port) = args.port{
                        if args.tcp{
                            client::init::start_tcp_transfer(&ip, &port, path).unwrap();
                        }
                        else{
                            client::init::start_udp_transfer(&ip, &port, path).unwrap();
                        }
                    }
                    else{
                        println!("Port number where the server is listening is required!");
                        println!("usage: ftp -m client  -p <server port number> -i <server ip> -f <file path to transfer>");
                    }
                }
                None => {
                    println!("IP where the server is listening is required!");
                    println!("usage: ftp -m client  -p <server port number> -i <server ip> -f <file path to transfer>");

                }
            }
        }
        else{
            println!("Full path of the file to be transferred is required!");
            println!("usage: ftp -m client  -p <server port number> -i <server ip> -f <file path to transfer>");

        }
    }
  
}

async fn handle_error(_err: std::io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

async fn check_server_params(args:Args) -> (String, String){
    if let Some(c) = args.cert.as_deref(){
        if let Some(k) = args.key.as_deref(){
            (c.to_owned(),k.to_owned())
        }
        else{
            panic!("path to key is required to start https server!!");
        }
    }
    else{
        panic!("Path to certificate is required to start https server!!");
    }
}