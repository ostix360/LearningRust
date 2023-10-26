use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listening =  TcpListener::bind("127.0.0.1:7878").unwrap();

    for flow in listening.incoming(){
        let flow = flow.unwrap();
        connexion_handler(flow)
    }
}

fn connexion_handler(mut flow: TcpStream){
    let mut buffer = [0; 1024];
    flow.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (statue, file) = if buffer.starts_with(get) {
        ( "HTTP/1.1 200 OK", "hello.html")
    }else{
        ( "HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let content = fs::read_to_string(file).unwrap();
    let response = format!(
            "{} OK\r\nContent-Length: {}\r\n\r\n{}",
            statue,
            content.len(),
            content
    );
    flow.write(response.as_bytes()).unwrap();

    flow.flush().unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer))
}
