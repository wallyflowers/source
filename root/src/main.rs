use std::collections::HashMap;
use sha2::{Sha256, Digest};
use bincode;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

type Hash = [u8; 32];       // A 256-bit SHA hash
type Time = i32;            // A 32-bit Unix timestamp
type Socket = [u8; 6];      // A 48-bit TCP/IP socket
type Value = f64;           // A 64-bit floating point number
type Data = Vec<u8>;        // A variable-length byte array
type Typecode = [u8; 2];    // A 16-bit type code

// TODO make presence and quality a type of knowledge (the data for the hash = 0, quality = 1, presence = 2, etc.)
type Presence = (Hash, Socket, Time);   // The sockets from and times at which a hash was confirmed to be have been provided.
                                        // "I have seen this hash at these sockets at these times"
type Quality = (Hash, Value);           // The quality of the data, as a value between f64::min and f64::max.
                                        // f64::min = "I believe this data to be not worth your resources to review."
                                        // 0.5 = "I am indifferent about this data."
                                        // f64::max = "I believe this data to be worth your resources to review."
                                        // 0 = "I believe this data to be harmful to review."
                                        // infinity = "I believe this data to be critical to review."
type Knowledge = (Hash, Typecode, Data);// The hash of the data, the type of data, and the data itself 
                                        // "I know this hash and this is the data and how to read it with our agreed upon lens."

type PresenceMap = HashMap<Hash, Vec<(Socket, Time)>>;
type QualityMap = HashMap<Hash, Vec<Value>>;


//TODO: type Knowledge = (Typecode, Data);
type KnowledgeMap =  HashMap<Hash, Vec<(Typecode, Data)>>;

type Stream = Vec<Socket>;

// A trait for a memory which can commit and recall data using the corresponding hash
trait Memory {
    fn commit(&mut self, hash: Hash, t: Typecode, data: &Data);
    fn recall(&self, hash: Hash) -> Option<Vec<(Typecode, Data)>>;
}


impl Memory for KnowledgeMap {
    // TODO verify that the hash is the hash of the data

    fn commit(&mut self, hash: Hash, t: Typecode, data: &Data) {
        let serialized_data = bincode::serialize(data).unwrap();
        let knowledge = self.get_mut(&hash);
        match knowledge {
            Some(knowledge) => {
                knowledge.push((t, serialized_data));
            }
            None => {
                self.insert(hash, vec![(t, serialized_data)]);
            }
        }
    }

    fn recall(&self, hash: Hash) -> Option<Vec<(Typecode, Data)>> {
        let knowledge = self.get(&hash);
        match knowledge {
            Some(knowledge) => {
                let knowledge: Vec<(Typecode, Data)> = knowledge.clone();
                Some(knowledge)
            }
            None => {
                None
            }
        }
    }
}


// TODO: make share take a stream and a vector of knowledge, send the knowledge to all the sockets in the stream
fn share<T: serde::Serialize>(socket: &Socket, prefix: &[u8; 2], data: &T) {
    let serialized_data = bincode::serialize(data).unwrap();
    let socket_address = format_socket_address(socket);
    if let Ok(mut stream) = TcpStream::connect(socket_address) {
        stream.write_all(prefix).unwrap();
        stream.write_all(&serialized_data).unwrap();
    }
}

// TODO: Remove the separate knowledge, presence, and quality sharing functions and make a single share function
fn share_knowledge(socket: &Socket, knowledge: &Knowledge) {
    share(socket, &[0, 0], knowledge);
}

fn share_presence(socket: &Socket, presence: &Presence) {
    share(socket, &[1, 0], presence);
}

fn share_quality(socket: &Socket, quality: &Quality) {
    share(socket, &[0, 1], quality);
}

fn format_socket_address(socket: &Socket) -> String {
    format!(
        "{}.{}.{}.{}:{}",
        socket[0], socket[1], socket[2], socket[3],
        (socket[4] as u16) << 8 | socket[5] as u16
    )
}

// TODO: make a single request stream function that takes a teacher socket and a student socket
// Send a request to the teacher containing a tuple of the student's socket as Knowledge of type "stream_request".
fn request_presence_stream(teacher: &Socket, student_port: u16) -> Result<(), std::io::Error> {
    let socket_address = format_socket_address(teacher);
    let mut stream = TcpStream::connect(socket_address)?;
    stream.write_all(&[0, 0])?; // Prefix for requesting presence stream
    stream.write_all(&student_port.to_be_bytes())?;
    Ok(())
}

fn request_quality_stream(teacher: &Socket, student_port: u16) -> Result<(), std::io::Error> {
    let socket_address = format_socket_address(teacher);
    let mut stream = TcpStream::connect(socket_address)?;
    stream.write_all(&[0, 1])?; // Prefix for requesting quality stream
    stream.write_all(&student_port.to_be_bytes())?;
    Ok(())
}

// TODO: send a request to the teacher containing the hash of the knowledge the student wants to learn about as Knowledge of type "knowledge_request".
fn request_knowledge(socket: &Socket, hash: Hash) -> Option<Knowledge> {
    let socket_address = format_socket_address(socket);
    if let Ok(mut stream) = TcpStream::connect(socket_address) {
        stream.write_all(&hash).unwrap();
        let mut serialized_knowledge = Vec::new();
        stream.read_to_end(&mut serialized_knowledge).unwrap();
        let knowledge: Knowledge = bincode::deserialize(&serialized_knowledge).unwrap();
        Some(knowledge)
    } else {
        None
    }
}

fn main() {
    let mut knowledge_map = KnowledgeMap::new();
    let mut source_map = KnowledgeMap::new();
    let mut presence_map = PresenceMap::new();
    let mut quality_map = QualityMap::new();

    let mut student: Vec<Socket> = Vec::new();

    // Start listening for incoming messages
    listen(34, &mut knowledge_map, &mut presence_map, &mut quality_map);
}