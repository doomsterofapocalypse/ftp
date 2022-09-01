use flate3;
use libaes::Cipher;
use std::fs;

const  KEY:[u8;16] = [235, 112, 226, 204, 152, 93, 126, 244, 242, 214, 80, 172, 118, 19, 128, 42];
const IV:&[u8;16] = b"H@xor3llit3systm";



pub fn compress(file:&str) -> Vec<u8>{
    let filedata = fs::read(file).expect("Error reading file");
    let mut comp = flate3::Compressor::new();
    let cb = comp.deflate(&filedata);
    println!( "compressed size={}", cb.len() );
    encrypt(cb)
    
}


fn encrypt(file:Vec<u8>) -> Vec<u8>{
    let cipher = Cipher::new_128(&KEY);
    let encrypted = cipher.cbc_encrypt(IV, &file[..]);
    println!("Encryption done!!");
    encrypted
}


pub fn decrypt(file:Vec<u8>, filename:&str) {
    println!("Decryption in progress!!");
    let cipher = Cipher::new_128(&KEY);
    let decrypted = cipher.cbc_decrypt(IV, &file[..]);
    decompress(decrypted, filename);
}


fn decompress(file:Vec<u8>, filename:&str){
    println!("Decompression in progress!!");
    let filedata = flate3::inflate(&file);
    println!( "de-compressed size={}",filedata.len() );

    match std::fs::write(&filename, filedata){
        Ok(_) => {
            println!("[+] File write complete!");
        }
        Err(e)=> {
            println!("File write error:{}", e);
        }
    }
}