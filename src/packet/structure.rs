use serde::{Serialize, Deserialize};



#[derive(Debug, Clone, Serialize, Deserialize,)]
pub enum flags{
    Encrypted,
    Compressed,
    pass,
    fail,
    miss,
    complete,
    client_fin,
    server_fin,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]

pub enum packetype{
    hello,
    ack,
    bye,
    filechunk,
    nonexistent
}

#[derive(Debug, Clone, Serialize, Deserialize,)]
pub struct hello{
    pub header:packetype, //hello
    pub name:String,
    pub flags:Option<Vec<flags>>,
    pub number_of_chunks:usize,
}

impl Default for hello{
    fn default() -> Self {
        hello { header: packetype::nonexistent, name: "".to_owned(), flags: None, number_of_chunks: 0 }
    }
}

impl hello {
    pub fn new(header:packetype, name:String, flags:Option<Vec<flags>>,number_of_chunks:usize) -> Self{
        hello{
            header,
            name,
            flags,
            number_of_chunks,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize,)]
pub struct ack{
    pub header:packetype, //ack
    pub packetnumber:i32,
    pub flag:flags
}
impl ack{
    pub fn new(header:packetype, packetnumber:i32, flag:flags) -> Self {
        ack{
            header,
            packetnumber,
            flag
        }
    }
}

impl Default for ack{
    fn default() -> Self {
        ack { header: packetype::nonexistent, packetnumber: 0, flag: flags::fail }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize,)]
pub struct bye{
    pub header:packetype, //bye
    pub flag:flags
}

impl Default for bye {
    fn default() -> Self {
        bye { header: packetype::nonexistent, flag: flags::client_fin }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize,)]
pub struct filechunk {
    pub header:packetype, //filechunk
    pub packetnumber: i32,
    pub  totalsize: usize,
    pub data: Vec<u8>,
    pub padding:usize,
    pub checksum:u64,
    
}
impl filechunk {
    pub fn new(header:packetype, packetnumber:i32, totalsize:usize, data:Vec<u8>, padding:usize, checksum:u64)->Self{
        filechunk{
            header,
            packetnumber,
            totalsize,
            data,
            padding,
            checksum
        }
    }
}


impl Default for filechunk {
    fn default() -> Self {
        let data:Vec<u8> = vec![0];
        filechunk { header: packetype::nonexistent, packetnumber: 0, totalsize: 0, data, padding: 0, checksum: 0 }
    }
}