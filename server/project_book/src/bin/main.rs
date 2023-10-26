use std::{fs, thread};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use project_book::ThreadPool;

fn main() {
    let listening =  TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(50);
    for flow in listening.incoming().take(10){
        let flow = flow.unwrap();
        pool.execute(|| {
            connexion_handler(flow)
        });
    }
    println!("Everything is shut down")
}

fn connexion_handler(mut flow: TcpStream){
    let mut buffer = [0; 1024];
    flow.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let pause = b"GET /pause HTTP/1.1\r\n";

    let (statue, file) = if buffer.starts_with(get) {
        ( "HTTP/1.1 200 OK", "hello.html")
    }else if buffer.starts_with(pause){
        thread::sleep(Duration::from_secs(5));
        ( "HTTP/1.1 200 OK", "hello.html")
    }else {
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
}
