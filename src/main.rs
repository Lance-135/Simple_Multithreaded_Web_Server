use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}, process::ExitStatus
};

fn main(){
    let reader = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in reader.incoming(){
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}


fn handle_connection(mut stream: TcpStream){
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    
    // let response = "HTTP/1.1 200 Ok\r\n\r\n";
    let status_line = "HTTP/1.1 200 OK";
    let content = fs::read_to_string("hello.html").unwrap();
    let length = content.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");
    stream.write_all(response.as_bytes()).unwrap();
}