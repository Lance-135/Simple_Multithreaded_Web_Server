use std::{
    {fs, thread},
    time:: Duration,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use hello::ThreadPool;
fn main(){
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let thread_pool = ThreadPool::new(5);
    for stream in listener.incoming().take(4){
        let stream = stream.unwrap();
        thread_pool.execute(||{
            handle_connection(stream);
        })
    }
}


fn handle_connection(mut stream: TcpStream){
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let (status_line, filename) = match &request_line[..]{
        "GET / HTTP/1.1" => ("HTTP/1.1 200 Ok", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        },
        _=> ("HTTP/1.1 404 NOT Found", "404.html")
    };
    
        let content = fs::read_to_string(filename).unwrap();
        let length = content.len();
        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");
        stream.write_all(response.as_bytes()).unwrap();
}