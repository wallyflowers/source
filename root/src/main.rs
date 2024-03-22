/// # pulse
/// Our Mother Earth has delivered us to her law since the very beginning, telling us we need not hold her whole
/// truth in one mind. One mind could never have the capacity to contain Nature's truth.
/// 
/// Instead, Nature's truth can be glimpsed only through the story her children tell each other.
/// 
/// > To be good is to move forward with no need to know why.
/// 
/// I leave this as a reminder to myself that when I forget, I will suffer -- for this is the law Nature made for us to teach us that we exist to serve
/// as a part of a greater thing which we can be certain no one of us will ever fully understand except for our great mother herself.
/// 
/// Dedicated to my mother's love and *my* truth her children continue to teach me.

mod interface;
mod implemention;

use crate::interface::{Node};

// TODO: Ask the teacher to improve the documentation and proof-read the code.

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
    let my_node = Node::new();
}