use crate::packet::structure::*;
use xxhash_rust::xxh3::xxh3_64;
use crate::common::utils::decrypt;



pub fn parse_buf(stream:&[u8]) -> packetype{
    let newpacket:filechunk = bincode::deserialize(stream).unwrap_or(filechunk::default());
    if newpacket.header == packetype::filechunk{
        println!("[+] Recieved filechunk packet");
        return packetype::filechunk
    }
    let newpacket2:hello = bincode::deserialize(stream).unwrap_or(hello::default());
    if newpacket2.header == packetype::hello{
        println!("[+] Recieved hello packet");
        return packetype::hello
    }
    let newpacket3:ack = bincode::deserialize(stream).unwrap_or(ack::default());
    if newpacket3.header == packetype::ack{
        println!("[+] Recieved ack packet");
       return  packetype::ack
    }
    let newpacket4:bye = bincode::deserialize(stream).unwrap_or(bye::default());
    if newpacket4.header == packetype::bye{
        println!("[+] Recieved bye packet");
        packetype::bye
    }

    else{
        println!("[!] Recieved non existent format");
        packetype::nonexistent
    }

}

pub fn write_file(filevec:Vec<filechunk>, filename:String) {
    let collection_size = filevec.len();
    println!("total collection recieved: {}", collection_size);
    let mut filedata:Vec<u8> = vec![];
    for i in 0..filevec.len(){
        if filevec[i].padding == 0_usize{
            filedata.extend(&filevec[i].data);
        }
        else{
            let index = 4096-&filevec[i].padding;
            filedata.extend(&filevec[i].data[..index]);
        }        
    }

    decrypt(filedata, &filename);

    /*match std::fs::write(&filename, filedata){
        Ok(_) => {
            println!("[+] File write complete!");
        }
        Err(e)=> {
            println!("File write error:{}", e);
        }
    }*/
    
}


fn is_pckt_valid(pckt:filechunk) -> bool {
    let original_checksum = pckt.checksum;
    let new_checksum = xxh3_64(&pckt.data);
    original_checksum == new_checksum
   
}